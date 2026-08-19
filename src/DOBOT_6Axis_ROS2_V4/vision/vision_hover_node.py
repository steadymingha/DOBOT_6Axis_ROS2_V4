#!/usr/bin/env python3
"""Real-robot vision shim: the topic contract of tag_vision_node.py, fed by the
verified vision_bridge chain (docs/real_robot_pipeline_plan.md 4.1b).

    /vision/capture (Int32)  1 -> vision_target.snapshot() once (runner MAGAZINE,
                             gated 10-frame average, arm must be still)
                             0 -> forget the last result
    /vision/device_pose      the detected magazine's BOX CENTRE (base_link == odom on
                             the real robot, see arm.launch.py) republished at 10 Hz until
                             reset, so main.py's median-15 + spread gate works unchanged.
                             = P_base (front-face centre, mid-height) + MAGAZINE_INWARD_M
                             along INWARD_AXIS. The shelf sequence takes it as box_world.

Nothing here is new maths: P_base/hover come from vision_target.hover_from_snapshot
(the --vision --run path), tool_vector from ONE cr7_pnp.robot_feed.RobotFeed. Runs
in the ros2_dobot container next to bringup; the vision_runner owns the camera on
the host and answers over ZMQ (vision_bridge/config.py).

Orientation is published as identity (the magazine yaw is not measured here; the
shelf sequence derives its grasp yaw from the shelf yaw in env/real.json). ARUCO (/vision/shelf_pose) and pocket
(/vision/pocket_state) modes are stage 3b/3c: one more mode + topic each.

    DOBOT_ENV=real /root/dobot_ws/.venv/bin/python3 vision/vision_hover_node.py [--ip IP]
"""
import argparse
import os
import sys
import threading

import rclpy
from rclpy.node import Node
from geometry_msgs.msg import PoseStamped
from std_msgs.msg import Int32

_PKG_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, _PKG_ROOT)
sys.path.insert(0, os.path.join(_PKG_ROOT, 'test'))
from cr7_pnp.robot_feed import RobotFeed, default_robot_ip   # noqa: E402
import vision_target as vt                                   # noqa: E402  (mocks cv2 if broken)


class VisionHover(Node):
    def __init__(self, ip):
        super().__init__('vision_hover')
        self.mon = RobotFeed(ip)                 # the ONE feed instance of this process
        self.mon.start()
        if not self.mon.wait_ready(5.0):
            raise SystemExit(f"[vision_hover] no 30004 feed from {ip}: {self.mon.error}")
        self.tf = vt.Transform()                 # hand-eye T_flange_cam, loaded once
        self._hover = None
        self._busy = False
        self.pub = self.create_publisher(PoseStamped, '/vision/device_pose', 10)
        self.create_subscription(Int32, '/vision/capture', self._capture_cb, 10)
        self.create_timer(0.1, self._tick)
        self.get_logger().info(f"[vision_hover] ready (feed {ip}, runner "
                               f"{vt.vcfg.RUNNER_HOST}); waiting for /vision/capture")

    def _capture_cb(self, msg):
        if msg.data == 0:
            self._hover = None
            return
        if self._busy:
            self.get_logger().warn("[vision_hover] snapshot already running; ignored")
            return
        self._busy = True
        threading.Thread(target=self._snapshot, daemon=True).start()   # blocks up to 30 s

    def _snapshot(self):
        try:
            snap, tool, _payload = vt.snapshot(self.mon, verbose=False)
            P_base, hover, inward, _ = vt.hover_from_snapshot(snap, tool, self.tf)
            self._hover = P_base + inward            # box centre (see docstring)
            self.get_logger().info(
                "[vision_hover] P_base [%+.4f %+.4f %+.4f] -> box centre [%+.4f %+.4f %+.4f] "
                "(hover would be [%+.4f %+.4f %+.4f]) m  track %s, %d frames, valid %.0f%%, sd %.2f cm"
                % (*P_base, *self._hover, *hover, snap['track_id'], snap['n_frames'],
                   snap['valid_pct'], snap['sd_cm']))
        except vt.VisionTargetError as e:
            self._hover = None
            self.get_logger().error(f"[vision_hover] {e}")
        finally:
            self._busy = False

    def _tick(self):
        if self._hover is None:
            return
        ps = PoseStamped()
        ps.header.stamp = self.get_clock().now().to_msg()
        ps.header.frame_id = 'odom'              # == base_link on the real robot
        ps.pose.position.x, ps.pose.position.y, ps.pose.position.z = map(float, self._hover)
        ps.pose.orientation.w = 1.0
        self.pub.publish(ps)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('--ip', default=None, help="robot IP (default: param.json)")
    a = ap.parse_args()
    rclpy.init()
    node = VisionHover(a.ip or default_robot_ip())
    try:
        rclpy.spin(node)
    except (KeyboardInterrupt, rclpy.executors.ExternalShutdownException):
        pass
    finally:
        node.mon.stop()
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()
