"""Diagnostic: detect the wirebonder AprilTag and print the 4 slot poses in
base_link, side by side with the DEVICES ground truth -- so the camera->robot
transform and the OpenCV-tag-vs-model frame convention can be validated by eye
BEFORE wiring vision into the pick/place flow (TODO Task 2).

Runs in the SYSTEM ROS env (cv2 + tf2 there); does NOT import
wirebonder_pick_place (that pulls pinocchio, which the system python lacks), so
the ground truth is hardcoded here.

Bring the sim up, park the AMR facing wb1, send the arm to the hub (run
wirebonder_pick_place.py once), then:
    source /opt/ros/humble/setup.bash
    source ~/dobot_ws/install/setup.bash
    cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
    python3 wirebonder_vision_node.py

It prints once a second. If vision and ground truth disagree by more than a few
mm / deg, fix R_TAGCV_TO_MODEL in wirebonder_vision.py and re-run.
"""

import math

import numpy as np
import rclpy
from rclpy.node import Node
from rclpy.duration import Duration
from sensor_msgs.msg import Image, CameraInfo
from geometry_msgs.msg import PoseStamped
from tf2_ros import Buffer, TransformListener

import wirebonder_vision as wv

# Ground-truth device world (odom) poses, mirrored from wirebonder_pick_place.DEVICES.
DEVICES_GT = {
    'wb1': (2.35, 0.5, 0.0, 0.0),
}
DEVICE = 'wb1'


def quat_to_R(x, y, z, w):
    """Quaternion (x,y,z,w) -> 3x3 rotation matrix."""
    n = math.sqrt(x * x + y * y + z * z + w * w) or 1.0
    x, y, z, w = x / n, y / n, z / n, w / n
    return np.array([
        [1 - 2 * (y * y + z * z), 2 * (x * y - z * w), 2 * (x * z + y * w)],
        [2 * (x * y + z * w), 1 - 2 * (x * x + z * z), 2 * (y * z - x * w)],
        [2 * (x * z - y * w), 2 * (y * z + x * w), 1 - 2 * (x * x + y * y)],
    ])


def _image_to_bgr(msg):
    """sensor_msgs/Image (rgb8/bgr8) -> HxWx3 BGR ndarray. No cv_bridge dep."""
    arr = np.frombuffer(msg.data, np.uint8).reshape(msg.height, msg.width, 3)
    if msg.encoding == 'rgb8':
        arr = arr[:, :, ::-1]
    return np.ascontiguousarray(arr)


class Diag(Node):
    def __init__(self):
        super().__init__('wirebonder_vision_diag')
        self.bgr = None
        self.K = None
        self.tf_buffer = Buffer()
        self.tf_listener = TransformListener(self.tf_buffer, self)
        self.create_subscription(Image, '/d405/color/image_raw', self._img_cb, 10)
        self.create_subscription(CameraInfo, '/d405/color/camera_info', self._info_cb, 10)
        # Device pose in ODOM (device is static there, so the planner can cache it
        # and reuse its odom->base_link TF). This is the vision-layer contract the
        # .venv flows + dispatcher consume; the AI magazine detector publishes the
        # same shape later.
        self.pub = self.create_publisher(PoseStamped, '/vision/device_pose', 10)
        # Publish fast (the planner medians a burst of frames); print throttled.
        self._print_ctr = 0
        self.create_timer(0.1, self._tick)
        self.get_logger().info("waiting for image / camera_info / TF ...")

    def _img_cb(self, msg):
        self.bgr = _image_to_bgr(msg)

    def _info_cb(self, msg):
        self.K = np.array(msg.k, dtype=float).reshape(3, 3)

    def _lookup_T(self, target, source):
        """T_target_source as a 4x4, or None if the TF is unavailable."""
        try:
            tf = self.tf_buffer.lookup_transform(
                target, source, rclpy.time.Time(), timeout=Duration(seconds=0.5))
        except Exception as e:
            self.get_logger().warn(f"[TF] {target}<-{source} failed: {e}")
            return None
        t = tf.transform.translation
        q = tf.transform.rotation
        return wv.make_T(quat_to_R(q.x, q.y, q.z, q.w), [t.x, t.y, t.z])

    def _tick(self):
        if self.bgr is None or self.K is None:
            return
        T_base_opt = self._lookup_T('base_link', 'd405_optical_frame')
        T_base_odom = self._lookup_T('base_link', 'odom')
        if T_base_opt is None or T_base_odom is None:
            return

        det = wv.detect_tag(self.bgr, self.K)
        if det is None:
            self.get_logger().info(f"tag {wv.TAG_ID} not detected")
            return
        rvec, tvec = det

        T_base_model_vis = wv.device_pose_in_base(T_base_opt, rvec, tvec)
        slots_vis = wv.slots_in_base(T_base_model_vis)

        # Publish the device pose in odom (T_odom_model = T_odom_base @ T_base_model).
        T_odom_model_vis = wv.inv_T(T_base_odom) @ T_base_model_vis
        self._publish_pose(T_odom_model_vis)

        # Side-by-side print throttled to ~1 Hz (publish runs at the full tick rate).
        self._print_ctr = (self._print_ctr + 1) % 10
        if self._print_ctr != 0:
            return

        dx, dy, dz, dyaw = DEVICES_GT[DEVICE]
        T_odom_model = wv.make_T(wv.rpy_to_R(0, 0, dyaw), [dx, dy, dz])
        T_base_model_gt = T_base_odom @ T_odom_model
        slots_gt = wv.slots_in_base(T_base_model_gt)

        lines = [f"--- {DEVICE}: vision vs ground-truth slot centres (base_link, m) ---"]
        for L in 'ABCD':
            v = slots_vis[L][0]
            g = slots_gt[L][0]
            d = np.linalg.norm(v - g)
            lines.append(
                f"  {L}: vis=({v[0]:+.3f},{v[1]:+.3f},{v[2]:+.3f})  "
                f"gt=({g[0]:+.3f},{g[1]:+.3f},{g[2]:+.3f})  |dist|={d * 1000:5.1f} mm")
        self.get_logger().info("\n".join(lines))

    def _publish_pose(self, T_odom_model):
        ps = PoseStamped()
        ps.header.stamp = self.get_clock().now().to_msg()
        ps.header.frame_id = 'odom'
        t = T_odom_model[:3, 3]
        qx, qy, qz, qw = wv.R_to_quat(T_odom_model[:3, :3])
        ps.pose.position.x, ps.pose.position.y, ps.pose.position.z = map(float, t)
        ps.pose.orientation.x = float(qx)
        ps.pose.orientation.y = float(qy)
        ps.pose.orientation.z = float(qz)
        ps.pose.orientation.w = float(qw)
        self.pub.publish(ps)


def main():
    rclpy.init()
    node = Diag()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()
