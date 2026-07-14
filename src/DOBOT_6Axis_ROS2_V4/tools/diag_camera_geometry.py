#!/usr/bin/env python3
"""Camera-geometry diagnosis for the wirebonder vision pipeline.

The depth-hybrid device pose is viewpoint-dependent (x/z swing ~30 mm between
capture configs). This script separates WHERE that error enters, per viewpoint:

  1. reproj  : project the GROUND-TRUTH tag corners (cr.world device pose,
               odom==world verified) through TF(image stamp) + K and compare
               with the DETECTED pixels. Nonzero -> camera TF chain / K /
               detection. Depth is not involved.
  2. depth   : fit a plane to the depth over the tag; deproject the detected
               corner pixels with plane-Z and compare with the GT corners in
               odom. Nonzero beyond (1) -> depth alignment.
  3. centre  : the tag-centre estimate 2 ways --
                 current  = corner-MEAN pixel + ROI-MEDIAN Z (the pipeline)
                 proposed = DIAGONAL-INTERSECTION pixel + plane-ray Z
               vs the GT centre. Quantifies the projective-approximation bias.
  4. device  : the full device_pose_in_base output (CURRENT = the depth-upright
               construction) and the OLD PnP-hybrid variant (PnP rotation with
               depth normal snap + depth centre) vs the GT device pose
               (x, y, z, yaw). The hybrid line documents the error the upright
               construction removed; on a real bring-up drop the GT compare and
               use the cross-view spread as the pass gate.
  5. tf-skew : |optical origin at TF(latest) - TF(image stamp)| -- the error a
               moving/settling arm injects when TF is looked up at Time().

Run with the sim up and the arm free to move (it visits the capture viewpoints
and returns to the start config):

    source /opt/ros/humble/setup.bash
    source ~/dobot_ws/install/setup.bash
    cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
    python3 tools/diag_camera_geometry.py
"""

import math
import os
import sys
import time
import threading

import numpy as np
import rclpy
from rclpy.node import Node
from rclpy.parameter import Parameter
from rclpy.duration import Duration
from rclpy.action import ActionClient
from rclpy.executors import MultiThreadedExecutor
from sensor_msgs.msg import Image, CameraInfo
from control_msgs.action import FollowJointTrajectory
from trajectory_msgs.msg import JointTrajectoryPoint
from tf2_ros import Buffer, TransformListener

sys.path.insert(0, os.path.join(os.path.dirname(os.path.dirname(
    os.path.abspath(__file__))), 'vision'))
import wirebonder_vision as wv  # noqa: E402

# Ground truth: device pose from cr.world (odom == world, verified against
# `gz model -m wirebonder -p` and the mpo700 model pose vs /odom).
GT_DEVICE = (2.35, 0.5, 0.0, 0.0)

JOINTS = ['joint1', 'joint2', 'joint3', 'joint4', 'joint5', 'joint6']
ACTION = '/cr7_group_controller/follow_joint_trajectory'

# Viewpoints: the pinned capture config A, three small perturbations of it
# (pan / shoulder / wrist-pitch), and the old far view B. All keep the tag in
# the 87 deg FOV; A and B are proven reachable by the sequence script.
A = (-0.42842, -0.13806, -1.94614, 0.19484, -1.70985, -0.39863)
B = (-0.22080, -0.67615, -1.38912, +0.09719, -1.63472, -0.59292)
VIEWS = [
    ('A',    A),
    ('A_j1', (A[0] + 0.12,) + A[1:]),
    ('A_j2', (A[0], A[1] - 0.08) + A[2:]),
    ('A_j5', A[:4] + (A[4] - 0.12, A[5])),
    ('B',    B),
]
N_FRAMES = 6
SETTLE_S = 3.5      # arm keeps creeping ~2 s after the action result (known)


def T_of(x, y, z, yaw):
    c, s = math.cos(yaw), math.sin(yaw)
    return np.array([[c, -s, 0, x], [s, c, 0, y], [0, 0, 1, z], [0, 0, 0, 1]], float)


# GT tag corners/centre in odom, in the OpenCV tag frame convention the
# detector uses (same composition as wirebonder_vision._demo case 5).
T_ODOM_TAGCV = (T_of(*GT_DEVICE) @ wv.T_MODEL_TAG
                @ wv.inv_T(wv.make_T(wv.R_CV_TO_SDF, np.zeros(3))))
OBJP = wv._marker_object_points(wv.TAG_SIZE_M)
CORNERS_GT = np.array([(T_ODOM_TAGCV @ np.append(p, 1.0))[:3] for p in OBJP])
CENTRE_GT = T_ODOM_TAGCV[:3, 3]


quat_to_R = wv.quat_to_R


def diag_intersect(c):
    """Intersection of the two diagonals of the corner quad = the true image of
    the tag CENTRE under perspective (the corner mean is not)."""
    p, r = c[0], c[2] - c[0]
    q, s = c[1], c[3] - c[1]
    denom = r[0] * s[1] - r[1] * s[0]
    d = q - p
    t = (d[0] * s[1] - d[1] * s[0]) / denom
    return p + t * r


def plane_fit(depth, corners, K):
    """(unit normal toward camera, centroid) of the depth points over the tag
    ROI, in the optical frame; None if too few finite pixels."""
    roi, x0, y0 = wv._tag_roi(depth, corners)
    ys, xs = np.where(np.isfinite(roi))
    if xs.size < 8:
        return None
    z = roi[ys, xs].astype(float)
    X = (xs + x0 - K[0, 2]) / K[0, 0] * z
    Y = (ys + y0 - K[1, 2]) / K[1, 1] * z
    P = np.stack([X, Y, z], axis=1)
    c0 = P.mean(0)
    _, _, Vt = np.linalg.svd(P - c0, full_matrices=False)
    n = Vt[2]
    if n[2] > 0:
        n = -n
    return n / np.linalg.norm(n), c0


def plane_z(px, n, c0, K):
    """Z (optical-axis depth) where the ray through pixel px meets the plane."""
    ray = np.array([(px[0] - K[0, 2]) / K[0, 0], (px[1] - K[1, 2]) / K[1, 1], 1.0])
    return float(n @ c0) / float(n @ ray)


def pose_xyzyaw(T):
    return (T[0, 3], T[1, 3], T[2, 3], math.atan2(T[1, 0], T[0, 0]))


class Diag(Node):
    def __init__(self):
        super().__init__('diag_camera_geometry', parameter_overrides=[
            Parameter('use_sim_time', Parameter.Type.BOOL, True)])
        self.bgr, self.stamp = None, None
        self.K = None
        self.depth_msg = None
        self.tfb = Buffer()
        self.tfl = TransformListener(self.tfb, self)
        self.create_subscription(Image, '/camera/d405/color/image_raw', self._img, 10)
        self.create_subscription(CameraInfo, '/camera/d405/color/camera_info', self._info, 10)
        self.create_subscription(Image, '/camera/d405/aligned_depth_to_color/image_raw', self._dep, 10)
        self.client = ActionClient(self, FollowJointTrajectory, ACTION)

    def _img(self, m):
        arr = np.frombuffer(m.data, np.uint8).reshape(m.height, m.width, 3)
        self.bgr = np.ascontiguousarray(arr[:, :, ::-1] if m.encoding == 'rgb8' else arr)
        self.stamp = m.header.stamp

    def _info(self, m):
        self.K = np.array(m.k, float).reshape(3, 3)

    def _dep(self, m):
        self.depth_msg = m

    def lookup(self, stamp):
        try:
            tf = self.tfb.lookup_transform('odom', 'd405_optical_frame', stamp,
                                           timeout=Duration(seconds=1.0))
        except Exception as e:
            self.get_logger().warn(f"TF failed: {e}")
            return None
        t, q = tf.transform.translation, tf.transform.rotation
        return wv.make_T(quat_to_R(q.x, q.y, q.z, q.w), [t.x, t.y, t.z])

    def move(self, q, dur=5.0):
        if not self.client.wait_for_server(timeout_sec=5.0):
            raise RuntimeError(f'action server {ACTION} unavailable')
        goal = FollowJointTrajectory.Goal()
        goal.trajectory.joint_names = JOINTS
        pt = JointTrajectoryPoint()
        pt.positions = list(q)
        pt.time_from_start.sec = int(dur)
        pt.time_from_start.nanosec = int((dur % 1) * 1e9)
        goal.trajectory.points.append(pt)
        done = threading.Event()
        gh_box = {}

        def on_goal(fut):
            gh = fut.result()
            gh_box['gh'] = gh
            if not gh.accepted:
                done.set()
                return
            gh.get_result_async().add_done_callback(lambda f: done.set())

        self.client.send_goal_async(goal).add_done_callback(on_goal)
        if not done.wait(timeout=dur + 10.0):
            raise RuntimeError('trajectory did not finish')
        if not gh_box['gh'].accepted:
            raise RuntimeError('trajectory goal rejected')

    def grab_frame(self):
        """One fresh, mutually-consistent (bgr, depth, K, stamp) tuple."""
        t0 = time.time()
        last = self.stamp
        while time.time() - t0 < 5.0:
            s, d = self.stamp, self.depth_msg
            if (s is not None and d is not None and self.K is not None
                    and (last is None or s != last)):
                ds = d.header.stamp
                if abs((s.sec + s.nanosec * 1e-9) - (ds.sec + ds.nanosec * 1e-9)) < 0.06:
                    depth = wv.read_depth(d.data, d.height, d.width, d.encoding)
                    return self.bgr, depth, self.K.copy(), s
            time.sleep(0.02)
        return None


def measure_view(node, name):
    rows = []
    for _ in range(N_FRAMES):
        fr = node.grab_frame()
        if fr is None:
            continue
        bgr, depth, K, stamp = fr
        corners = wv.detect_tag_corners(bgr)
        det = wv.detect_tag(bgr, K)
        T_stamp = node.lookup(stamp)
        T_now = node.lookup(rclpy.time.Time())
        if corners is None or det is None or T_stamp is None or T_now is None:
            continue
        T_opt_odom = wv.inv_T(T_stamp)

        # 1. reprojection: GT corners -> pixels vs detected pixels
        proj = []
        for X in CORNERS_GT:
            p = T_opt_odom[:3, :3] @ X + T_opt_odom[:3, 3]
            p = K @ (p / p[2])
            proj.append(p[:2])
        reproj = corners - np.array(proj)                    # px, per corner

        # 2. corners via depth-plane Z -> odom vs GT
        pf = plane_fit(depth, corners, K)
        if pf is None:
            continue
        n_pl, c0 = pf
        cd = []
        for px in corners:
            P = wv._deproject(px, plane_z(px, n_pl, c0, K), K)
            cd.append(T_stamp[:3, :3] @ P + T_stamp[:3, 3])
        corner_err = np.array(cd) - CORNERS_GT               # m, per corner, odom

        # 3. centre estimators
        px_mean = wv._tag_center_px(corners)
        px_diag = diag_intersect(corners.astype(float))
        roi, _, _ = wv._tag_roi(depth, corners)
        z_med = float(np.median(roi[np.isfinite(roi)]))
        cen_cur = T_stamp[:3, :3] @ wv._deproject(px_mean, z_med, K) + T_stamp[:3, 3]
        cen_pro = (T_stamp[:3, :3] @ wv._deproject(
            px_diag, plane_z(px_diag, n_pl, c0, K), K) + T_stamp[:3, 3])

        # 4. device pose: current pipeline (stamp TF) and proposed-centre variant
        T_cur = wv.device_pose_in_base(T_stamp, det, depth=depth, K=K, corners=corners)
        n_opt = wv._tag_plane_normal(depth, corners, K)
        rvec, _ = max(det, key=lambda rt: float(
            wv.T_optical_tagcv(rt[0], rt[1])[:3, 2] @ n_opt)) if n_opt is not None else det[0]
        import cv2
        R_sel = wv.T_optical_tagcv(rvec, np.zeros(3))[:3, :3]
        if n_opt is not None:
            R_sel = wv._align_normal(R_sel, n_opt)
        rv = cv2.Rodrigues(R_sel)[0].ravel()
        tv = wv._deproject(px_diag, plane_z(px_diag, n_pl, c0, K), K)
        T_pro = wv._T_base_model_from(T_stamp, rv, tv)

        # 5. TF skew latest vs stamp
        skew = float(np.linalg.norm(T_now[:3, 3] - T_stamp[:3, 3]))

        rows.append(dict(
            reproj=reproj, corner_err=corner_err,
            cen_cur=cen_cur - CENTRE_GT, cen_pro=cen_pro - CENTRE_GT,
            dev_cur=pose_xyzyaw(T_cur), dev_pro=pose_xyzyaw(T_pro),
            z_med=z_med, z_pln=plane_z(px_diag, n_pl, c0, K), skew=skew,
            px_mean=px_mean, px_diag=px_diag))
        time.sleep(0.25)

    if not rows:
        print(f"[{name}] NO valid frames (tag not detected / no depth / TF)")
        return None

    def med(key):
        return np.median(np.array([r[key] for r in rows]), axis=0)

    reproj = med('reproj')                    # 4x2 px
    corner = med('corner_err') * 1000.0       # 4x3 mm
    cc, cp = med('cen_cur') * 1000.0, med('cen_pro') * 1000.0
    dc, dp = med('dev_cur'), med('dev_pro')
    gt = np.array(GT_DEVICE)
    print(f"\n=== view {name}  ({len(rows)} frames) ===")
    print(f"  reproj err (det-proj, px): mean=({reproj[:,0].mean():+.2f},{reproj[:,1].mean():+.2f})"
          f"  per-corner={[f'({u:+.1f},{v:+.1f})' for u, v in reproj]}")
    print(f"  corner err via depth-plane (odom, mm): mean=({corner[:,0].mean():+.1f},"
          f"{corner[:,1].mean():+.1f},{corner[:,2].mean():+.1f})  max|.|={np.abs(corner).max():.1f}")
    print(f"  centre err  CURRENT (mean-px + median-Z): ({cc[0]:+.1f},{cc[1]:+.1f},{cc[2]:+.1f}) mm")
    print(f"  centre err  PROPOSED(diag-px + plane-Z) : ({cp[0]:+.1f},{cp[1]:+.1f},{cp[2]:+.1f}) mm")
    print(f"  Z centre: median={med('z_med'):.4f}  plane={med('z_pln'):.4f}  "
          f"(diff {1000*(med('z_med')-med('z_pln')):+.1f} mm)")
    print(f"  px centre: mean-of-corners vs diagonal: "
          f"d=({med('px_mean')[0]-med('px_diag')[0]:+.2f},{med('px_mean')[1]-med('px_diag')[1]:+.2f}) px")
    print(f"  device pose err CURRENT   : dx={1000*(dc[0]-gt[0]):+.1f} dy={1000*(dc[1]-gt[1]):+.1f} "
          f"dz={1000*(dc[2]-gt[2]):+.1f} mm  dyaw={math.degrees(dc[3]-gt[3]):+.2f} deg")
    print(f"  device pose err PNP-HYBRID: dx={1000*(dp[0]-gt[0]):+.1f} dy={1000*(dp[1]-gt[1]):+.1f} "
          f"dz={1000*(dp[2]-gt[2]):+.1f} mm  dyaw={math.degrees(dp[3]-gt[3]):+.2f} deg")
    print(f"  TF skew latest-vs-stamp: {1000*med('skew'):.1f} mm")
    return dict(dev_cur=dc, dev_pro=dp)


def main():
    rclpy.init()
    node = Diag()
    ex = MultiThreadedExecutor()
    ex.add_node(node)
    threading.Thread(target=ex.spin, daemon=True).start()

    # remember the start config to restore at the end
    from sensor_msgs.msg import JointState
    start = {}
    node.create_subscription(JointState, '/joint_states',
                             lambda m: start.update(zip(m.name, m.position)), 10)
    t0 = time.time()
    while not all(j in start for j in JOINTS) and time.time() - t0 < 10:
        time.sleep(0.05)
    q_start = [start[j] for j in JOINTS]
    print(f"start config: {[f'{q:+.3f}' for q in q_start]}")

    results = {}
    try:
        for name, q in VIEWS:
            print(f"\n>>> moving to view {name} ...")
            node.move(q, dur=5.0)
            time.sleep(SETTLE_S)
            r = measure_view(node, name)
            if r:
                results[name] = r
    finally:
        print("\n>>> returning to start config ...")
        node.move(q_start, dur=5.0)

    if results:
        gt = np.array(GT_DEVICE)
        print("\n" + "=" * 72)
        print("device-pose error per viewpoint (mm / deg):")
        for tag, key in (('CURRENT   ', 'dev_cur'), ('PNP-HYBRID', 'dev_pro')):
            vals = np.array([results[n][key] for n in results])
            errs = vals - gt
            print(f"  {tag}:")
            for n, e in zip(results, errs):
                print(f"    {n:5s} dx={1000*e[0]:+7.1f} dy={1000*e[1]:+7.1f} "
                      f"dz={1000*e[2]:+7.1f}  dyaw={math.degrees(e[3]):+6.2f}")
            spread = (vals.max(0) - vals.min(0))
            print(f"    cross-view spread: x={1000*spread[0]:.1f} y={1000*spread[1]:.1f} "
                  f"z={1000*spread[2]:.1f} mm  yaw={math.degrees(spread[3]):.2f} deg")
    rclpy.shutdown()


if __name__ == '__main__':
    main()
