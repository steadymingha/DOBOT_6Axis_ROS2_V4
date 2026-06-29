#!/usr/bin/env python3
"""Standalone single-topic image viewer (replaces rqt_image_view).

One window, one topic, no rqt plugin framework. Default topic is the D405 color
image. Run directly (the launch env already has rclpy/cv_bridge):
    python3 view_d405.py [topic]
Focus the window and press 'q' (or Ctrl+C in the terminal) to quit.
"""
import sys

import cv2
import rclpy
from cv_bridge import CvBridge
from rclpy.node import Node
from sensor_msgs.msg import Image


class Viewer(Node):
    def __init__(self, topic):
        super().__init__('image_viewer')
        self.bridge = CvBridge()
        self.win = topic
        self.create_subscription(Image, topic, self._cb, 10)
        cv2.namedWindow(self.win, cv2.WINDOW_NORMAL)
        self.get_logger().info(f"Viewing {topic} (focus window, press 'q' to quit)")

    def _cb(self, msg):
        # imshow/waitKey share the (single) spin thread, so calling them here is safe.
        img = self.bridge.imgmsg_to_cv2(msg, desired_encoding='bgr8')
        cv2.imshow(self.win, img)
        if cv2.waitKey(1) & 0xFF == ord('q'):
            rclpy.shutdown()


def main():
    topic = sys.argv[1] if len(sys.argv) > 1 else '/d405/color/image_raw'
    rclpy.init()
    node = Viewer(topic)
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        cv2.destroyAllWindows()
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()
