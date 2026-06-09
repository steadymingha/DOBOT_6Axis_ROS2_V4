#!/usr/bin/env python3
"""Reachability map for the CR7 arm (Link6), via Monte-Carlo FK sampling.

What it does
------------
1. Samples joint configurations uniformly inside the joint limits (J1..J5;
   J6 is fixed because a wrist roll changes neither the Link6 origin position
   nor the approach-axis tilt, and the gripper joints are held at neutral).
2. Computes Link6 forward kinematics (pinocchio) for every sample.
3. Rejects self-colliding configurations (pinocchio + hpp-fcl GeometryModel,
   with SRDF-disabled pairs and neutral-config always-colliding pairs removed),
   so only joint-limit-respecting AND collision-free poses count as reachable.
4. Builds two clouds in one pass:
     * "all"  - every reachable Link6 origin (orientation-agnostic envelope),
     * "down" - reachable poses whose approach axis (tool z = R[:,2]) points
                down (within a tilt tolerance) -> where the gripper can grasp.
5. Voxel-dedupes both clouds so the point count stays bounded.
6. Publishes them to RViz as latched PointCloud2 (/reach_all, /reach_down).
7. Saves them for offline analysis: one CSV (x,y,z,down) + two PCD files
   (all / down) + a JSON metadata sidecar (joint range, N, voxel, tilt, seed,
   counts), all timestamped so runs are reproducible/comparable.

The joint ranges are parameters, so the same script is used to study how the
real-robot joint-limit tuning changes the reachable / graspable region and to
pick valid grasp / place locations.

Pure FK + collision check: no simulator is required to compute or save. RViz
(with robot_state_publisher providing the base_link TF) is only needed to *view*
the published clouds.
"""

import os
import sys
import json
import math
import tempfile
import argparse
import datetime
import numpy as np
import pinocchio as pin


# Default model / config -------------------------------------------------------

DEFAULT_XACRO = os.path.expanduser(
    '~/dobot_ws/install/cra_description/share/cra_description/urdf/cr7_robot.xacro')
DEFAULT_SRDF = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                            'cr7_moveit', 'config', 'cr7_robot.srdf')

ARM_JOINTS = ('joint1', 'joint2', 'joint3', 'joint4', 'joint5', 'joint6')
GRIPPER_JOINTS = ('gripper_gripper_joint', 'gripper_right_finger_joint')
EE_FRAME = 'Link6'

# Default joint limits (radians) = the limits used by test_w_gripper.py.
# (lower, upper) per joint J1..J6. These are the parameters to vary.
DEFAULT_LIMITS_DEG = [
    (-101.0, 10.0),    # J1
    (-70.0, 60.0),     # J2
    (-180.0, 180.0),   # J3
    (0.0, 120.0),      # J4
    (-120.0, 120.0),   # J5
    (-180.0, 180.0),   # J6 (fixed during sampling; range kept for metadata)
]

DOWN_WORLD = np.array([0.0, 0.0, -1.0])  # world -Z; gripper "down" approach axis

# Offset from Link6 origin to TCP along the tool z-axis (gripper body length).
# OnRobot 2FG7: gripper_2fg7_attach_joint has xyz="0 0 0", finger attachment
# joints are at z=0.12005 from gripper_base_link → TCP ≈ 0.12 m from Link6.
# Change this value when swapping grippers.
TCP_OFFSET_M = 0.12005


# Model + self-collision -------------------------------------------------------

class ReachabilityModel:
    """Reduced 6-DOF CR7 model (gripper locked) with FK and self-collision."""

    def __init__(self, xacro_path=DEFAULT_XACRO, srdf_path=DEFAULT_SRDF,
                 ee_frame=EE_FRAME):
        urdf_path = self._xacro_to_urdf(xacro_path)

        # Full model + collision geometry, then lock the gripper joints so the
        # planning model is the 6-DOF arm; the gripper geometry stays attached
        # (frozen at neutral) and still participates in self-collision.
        full = pin.buildModelFromUrdf(urdf_path)
        full_geom = pin.buildGeomFromUrdf(full, urdf_path, pin.GeometryType.COLLISION)
        locked_ids = [full.getJointId(n) for n in GRIPPER_JOINTS
                      if full.existJointName(n)]
        self.model, geoms = pin.buildReducedModel(
            full, [full_geom], locked_ids, pin.neutral(full))
        self.geom = geoms[0]
        self.data = self.model.createData()

        # Collision pairs: all pairs, minus SRDF-disabled (adjacent/never) pairs,
        # minus any pair already colliding at the neutral config (mounting
        # overlaps such as gripper<->Link6) so they don't cause false positives.
        self.geom.addAllCollisionPairs()
        n_all = len(self.geom.collisionPairs)
        if srdf_path and os.path.exists(srdf_path):
            pin.removeCollisionPairs(self.model, self.geom, srdf_path)
        n_srdf = len(self.geom.collisionPairs)
        self.geom_data = self.geom.createData()
        n_neutral = self._disable_pairs_colliding_at(pin.neutral(self.model))
        self.pair_stats = (n_all, n_srdf, len(self.geom.collisionPairs), n_neutral)

        self.frame_id = self.model.getFrameId(ee_frame)
        # arm-joint order -> reduced-model configuration indices
        self.q_index = [self.model.idx_qs[self.model.getJointId(n)]
                        for n in ARM_JOINTS]

    @staticmethod
    def _xacro_to_urdf(xacro_path):
        import xacro
        doc = xacro.process_file(xacro_path)
        path = os.path.join(tempfile.gettempdir(), 'cr7_reach_model.urdf')
        with open(path, 'w') as f:
            f.write(doc.toxml())
        return path

    def _disable_pairs_colliding_at(self, q):
        """Remove collision pairs that are already in collision at config q."""
        pin.computeCollisions(self.model, self.data, self.geom, self.geom_data,
                              q, False)
        bad = [(cp.first, cp.second)
               for i, cp in enumerate(self.geom.collisionPairs)
               if self.geom_data.collisionResults[i].isCollision()]
        for a, b in bad:
            self.geom.removeCollisionPair(pin.CollisionPair(a, b))
        if bad:
            self.geom_data = self.geom.createData()
        return len(bad)

    def pin_q(self, j1_to_j6):
        """Map an arm-joint vector (J1..J6) to a full reduced-model config."""
        qp = pin.neutral(self.model)
        for k, idx in enumerate(self.q_index):
            qp[idx] = j1_to_j6[k]
        return qp

    def fk(self, qp):
        """TCP placement for a reduced-model config -> (position, rotation).

        Returns the TCP position (Link6 origin + TCP_OFFSET_M along tool z-axis)
        and the Link6 rotation.  The rotation itself is unchanged; only the
        reported position is shifted to the gripper tip so the cloud reflects
        where the gripper actually contacts an object.
        """
        pin.forwardKinematics(self.model, self.data, qp)
        pin.updateFramePlacement(self.model, self.data, self.frame_id)
        oMf = self.data.oMf[self.frame_id]
        tool_z = oMf.rotation[:, 2]          # tool z-axis in world frame
        tcp_pos = oMf.translation + TCP_OFFSET_M * tool_z
        return tcp_pos.copy(), oMf.rotation.copy()

    def self_collides(self, qp):
        """True if the config is in self-collision (stops at first contact)."""
        return pin.computeCollisions(self.model, self.data, self.geom,
                                     self.geom_data, qp, True)


def parse_args(argv=None):
    p = argparse.ArgumentParser(description=__doc__,
                                formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument('--xacro', default=DEFAULT_XACRO, help='robot xacro path')
    p.add_argument('--srdf', default=DEFAULT_SRDF,
                   help='SRDF with disabled collision pairs')
    p.add_argument('--samples', '-N', type=int, default=1000000,
                   help='number of Monte-Carlo joint samples (default 1000000)')
    p.add_argument('--voxel', type=float, default=0.02,
                   help='voxel size in meters for dedupe (default 0.02)')
    p.add_argument('--down-tol-deg', type=float, default=5.0,
                   help='tilt tolerance (deg) for the "down" set (default 15)')
    p.add_argument('--seed', type=int, default=1, help='RNG seed')
    p.add_argument('--limits-deg', type=str, default=None,
                   help='override joint limits, 12 comma values: '
                        'j1lo,j1hi,...,j6lo,j6hi (degrees)')
    p.add_argument('--out-dir', default=os.path.join(
        os.path.dirname(os.path.abspath(__file__)), 'reachability_out'),
        help='directory for CSV/PCD/meta output')
    p.add_argument('--no-self-collision', action='store_true',
                   help='disable self-collision filtering (envelope only)')
    p.add_argument('--no-viz', action='store_true',
                   help='skip ROS publishing (compute + save + sanity only)')
    p.add_argument('--frame-id', default='base_link',
                   help='frame_id for the published clouds (default base_link)')
    return p.parse_args(argv)


def resolve_limits(args):
    if args.limits_deg:
        vals = [float(v) for v in args.limits_deg.split(',')]
        if len(vals) != 12:
            raise SystemExit('--limits-deg needs 12 comma-separated values')
        deg = [(vals[2 * i], vals[2 * i + 1]) for i in range(6)]
    else:
        deg = DEFAULT_LIMITS_DEG
    rad = [(math.radians(lo), math.radians(hi)) for lo, hi in deg]
    return deg, rad


def voxel_keys(points, voxel):
    """Set of integer voxel indices occupied by the points."""
    if len(points) == 0:
        return set()
    idx = np.floor(np.asarray(points) / voxel).astype(np.int64)
    return set(map(tuple, idx))


def voxelize_with_down(all_pts, down_pts, voxel):
    """Dedupe to voxel centers and flag voxels that contain a 'down' sample.

    Returns (centers Nx3, down_flags N bool, down_centers Mx3). Every 'down'
    sample is also a reachable sample, so down voxels are a subset of all voxels.
    """
    all_k = voxel_keys(all_pts, voxel)
    down_k = voxel_keys(down_pts, voxel)
    if not all_k:
        z = np.zeros((0, 3))
        return z, np.zeros((0,), dtype=bool), z
    keys = list(all_k)
    centers = (np.asarray(keys, dtype=np.float64) + 0.5) * voxel
    down_flags = np.array([k in down_k for k in keys], dtype=bool)
    return centers, down_flags, centers[down_flags]


def write_pcd(path, pts):
    """Write an ASCII PCD (xyz) readable by pcl_viewer/CloudCompare/open3d."""
    pts = np.asarray(pts, dtype=np.float32)
    n = len(pts)
    header = (
        "# .PCD v0.7 - Point Cloud Data file format\n"
        "VERSION 0.7\nFIELDS x y z\nSIZE 4 4 4\nTYPE F F F\nCOUNT 1 1 1\n"
        f"WIDTH {n}\nHEIGHT 1\nVIEWPOINT 0 0 0 1 0 0 0\n"
        f"POINTS {n}\nDATA ascii\n")
    with open(path, 'w') as f:
        f.write(header)
        for p in pts:
            f.write(f"{p[0]:.6f} {p[1]:.6f} {p[2]:.6f}\n")


def save_clouds(centers, down_flags, down_centers, args, limits_deg, stats):
    """Save CSV (x,y,z,down) + all/down PCD + JSON metadata, timestamped."""
    os.makedirs(args.out_dir, exist_ok=True)
    ts = datetime.datetime.now().strftime('%Y%m%d_%H%M%S')
    base = os.path.join(args.out_dir, f'reach_{ts}')
    csv_path, all_pcd, down_pcd, meta_path = (
        base + '.csv', base + '_all.pcd', base + '_down.pcd', base + '_meta.json')

    with open(csv_path, 'w') as f:
        f.write('x,y,z,down\n')
        for p, d in zip(centers, down_flags):
            f.write(f'{p[0]:.6f},{p[1]:.6f},{p[2]:.6f},{int(d)}\n')

        # Append per-height reach summary: for each z level, the farthest
        # down-reachable point and its horizontal distance from the base.
        if len(down_centers) > 0:
            dc = np.asarray(down_centers)
            dist_xy = np.sqrt(dc[:, 0]**2 + dc[:, 1]**2)
            z_bins = np.round(dc[:, 2] / args.voxel) * args.voxel
            f.write('\n# reach_down summary: farthest TCP per height\n')
            f.write('# z_m,max_dist_from_base_m,farthest_x,farthest_y\n')
            for z in sorted(np.unique(z_bins)):
                mask = z_bins == z
                idx_max = int(np.argmax(dist_xy[mask]))
                max_d = dist_xy[mask][idx_max]
                px, py = dc[mask][idx_max, 0], dc[mask][idx_max, 1]
                f.write(f'# {z:+.3f},{max_d:.3f},{px:.3f},{py:.3f}\n')
    write_pcd(all_pcd, centers)
    write_pcd(down_pcd, down_centers)

    meta = {
        'timestamp': ts,
        'xacro': args.xacro,
        'srdf': args.srdf,
        'joint_limits_deg': {f'J{i+1}': list(limits_deg[i]) for i in range(6)},
        'j6_fixed_deg': 0.0,
        'samples': args.samples,
        'voxel_m': args.voxel,
        'down_tol_deg': args.down_tol_deg,
        'seed': args.seed,
        'self_collision_filter': not args.no_self_collision,
        'frame_id': args.frame_id,
        'stats': stats,
        'files': {'csv': csv_path, 'all_pcd': all_pcd, 'down_pcd': down_pcd},
    }
    with open(meta_path, 'w') as f:
        json.dump(meta, f, indent=2)
    print(f'[reach] saved:\n  {csv_path}\n  {all_pcd}\n  {down_pcd}\n  {meta_path}',
          flush=True)
    return meta


def _make_xyz_cloud(header, pts):
    """Build a PointCloud2 with XYZ only (use AxisColor in RViz Color Transformer)."""
    from sensor_msgs.msg import PointCloud2, PointField
    import struct
    fields = [
        PointField(name='x', offset=0,  datatype=PointField.FLOAT32, count=1),
        PointField(name='y', offset=4,  datatype=PointField.FLOAT32, count=1),
        PointField(name='z', offset=8,  datatype=PointField.FLOAT32, count=1),
    ]
    data = bytearray()
    for p in pts:
        data += struct.pack('fff', float(p[0]), float(p[1]), float(p[2]))
    msg = PointCloud2()
    msg.header = header
    msg.height = 1
    msg.width = len(pts)
    msg.fields = fields
    msg.is_bigendian = False
    msg.point_step = 12
    msg.row_step = 12 * len(pts)
    msg.data = bytes(data)
    msg.is_dense = True
    return msg


def publish_clouds(centers, down_centers, args):
    """Publish /reach_all and /reach_down as XYZ-only PointCloud2.

    Republishes every 3 s with default QoS so RViz2 always catches a message
    regardless of startup order. Use AxisColor (Z axis) in RViz Color Transformer.
    """
    import rclpy
    from rclpy.node import Node
    from std_msgs.msg import Header
    from sensor_msgs.msg import PointCloud2  # noqa: F401

    rclpy.init()
    node = Node('reachability_map')
    pub_all  = node.create_publisher(PointCloud2, '/reach_all',  10)
    pub_down = node.create_publisher(PointCloud2, '/reach_down', 10)

    def make_msgs():
        h = Header()
        h.frame_id = args.frame_id
        h.stamp = node.get_clock().now().to_msg()
        return (_make_xyz_cloud(h, centers),
                _make_xyz_cloud(h, down_centers))

    def republish():
        ma, md = make_msgs()
        pub_all.publish(ma)
        pub_down.publish(md)

    republish()
    node.create_timer(3.0, republish)
    node.get_logger().info(
        f'Publishing /reach_all ({len(centers)} pts) and '
        f'/reach_down ({len(down_centers)} pts) every 3s. '
        f'RViz: Fixed Frame={args.frame_id}, Color Transformer=AxisColor (Z). '
        f'Ctrl+C to stop.')
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


def sample_reachability(rm, args, limits_rad):
    """Monte-Carlo sample J1..J5, FK, and self-collision filter.

    Returns (envelope_pts, all_pts, down_pts, stats):
      envelope_pts - Link6 origin of every in-limit sample (filter OFF),
      all_pts      - collision-free reachable origins (filter ON),
      down_pts     - collision-free origins whose approach axis points down.
    """
    rng = np.random.default_rng(args.seed)
    lo = np.array([limits_rad[i][0] for i in range(5)])  # J1..J5
    hi = np.array([limits_rad[i][1] for i in range(5)])
    j6_fixed = 0.0  # wrist roll: irrelevant to Link6 position and tilt
    cos_tol = math.cos(math.radians(args.down_tol_deg))

    envelope_pts, all_pts, down_pts = [], [], []
    n_collision = 0
    N = args.samples
    report_every = max(1, N // 10)
    for i in range(N):
        j = rng.uniform(lo, hi)
        qp = rm.pin_q([j[0], j[1], j[2], j[3], j[4], j6_fixed])
        p, R = rm.fk(qp)
        envelope_pts.append(p)
        if not args.no_self_collision and rm.self_collides(qp):
            n_collision += 1
            continue
        all_pts.append(p)
        # approach axis = tool z = R[:,2]; "down" if aligned with world -Z
        if float(R[:, 2] @ DOWN_WORLD) >= cos_tol:
            down_pts.append(p)
        if (i + 1) % report_every == 0:
            print(f'[reach]   sampled {i + 1}/{N} '
                  f'(reachable {len(all_pts)}, down {len(down_pts)})', flush=True)

    stats = {
        'samples': N,
        'in_limit': N,
        'collisions': n_collision,
        'reachable_samples': len(all_pts),
        'down_samples': len(down_pts),
    }
    return (np.asarray(envelope_pts), np.asarray(all_pts),
            np.asarray(down_pts), stats)


def main(argv=None):
    args = parse_args(argv)
    limits_deg, limits_rad = resolve_limits(args)
    print('[reach] loading model ...', flush=True)
    rm = ReachabilityModel(args.xacro, args.srdf)
    n_all, n_srdf, n_active, n_neutral = rm.pair_stats
    print(f'[reach] collision pairs: {n_all} all -> {n_srdf} after SRDF '
          f'-> {n_active} active ({n_neutral} disabled as neutral-colliding)',
          flush=True)
    print(f'[reach] sampling {args.samples} configs '
          f'(self-collision filter {"OFF" if args.no_self_collision else "ON"}) ...',
          flush=True)
    envelope_pts, all_pts, down_pts, stats = sample_reachability(rm, args, limits_rad)

    # Voxel dedupe (Task 3): bound the point count.
    centers, down_flags, down_centers = voxelize_with_down(all_pts, down_pts, args.voxel)
    envelope_voxels = len(voxel_keys(envelope_pts, args.voxel))
    stats.update({
        'reachable_voxels': int(len(centers)),
        'down_voxels': int(len(down_centers)),
        'envelope_voxels': int(envelope_voxels),
    })
    print(f'[reach] voxels @ {args.voxel} m: reachable={stats["reachable_voxels"]}, '
          f'down={stats["down_voxels"]}, envelope(filter-off)={envelope_voxels}',
          flush=True)
    print(f'[reach] samples: {stats["samples"]} total, '
          f'{stats["collisions"]} self-colliding, '
          f'{stats["reachable_samples"]} reachable, '
          f'{stats["down_samples"]} down', flush=True)

    # Save for offline analysis (Task 5) - always, even headless.
    save_clouds(centers, down_flags, down_centers, args, limits_deg, stats)

    # Publish to RViz (Task 4) unless headless.
    if not args.no_viz:
        publish_clouds(centers, down_centers, args)

    return centers, down_flags, down_centers, stats


if __name__ == '__main__':
    main()
