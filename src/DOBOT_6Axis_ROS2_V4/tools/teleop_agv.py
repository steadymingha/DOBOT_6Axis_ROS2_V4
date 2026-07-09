#!/usr/bin/env python3
"""Low-latency keyboard teleop for the MPO-700 AGV.

Unlike the shell `ros2 topic pub --once` approach (which spawns a new node and
re-runs DDS discovery on every key, adding ~0.3-0.5 s), this keeps a single
publisher alive so each key press publishes immediately.

planar_move latches the last command, so one publish per key starts the motion
and it keeps going until you press SPACE.

Keep THIS terminal focused while pressing keys (not the Gazebo window).
"""

import sys
import termios
import tty
import select

import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Twist

HELP = """\
================ AGV Teleop ================
  (keep THIS terminal focused, not Gazebo)

  Arrow Up    : forward
  Arrow Down  : backward
  Arrow Left  : turn left
  Arrow Right : turn right
  a / d       : strafe left / right
  SPACE or s  : STOP
  + / -       : speed up / down
  q           : quit (stops the AGV)
============================================"""


def get_key(timeout=0.1):
    """Read one keypress; decode arrow keys (ESC [ A/B/C/D) into 'UP'/'DOWN'/..."""
    fd = sys.stdin.fileno()
    rlist, _, _ = select.select([fd], [], [], timeout)
    if not rlist:
        return None
    ch = sys.stdin.read(1)
    if ch == '\x1b':  # escape sequence (arrow key)
        seq = sys.stdin.read(2)
        return {'[A': 'UP', '[B': 'DOWN', '[C': 'RIGHT', '[D': 'LEFT'}.get(seq, None)
    return ch


def main():
    rclpy.init()
    node = Node('agv_teleop')
    pub = node.create_publisher(Twist, '/cmd_vel', 10)

    lin = 0.3   # m/s
    ang = 0.6   # rad/s
    step = 0.1

    def send(x=0.0, y=0.0, yaw=0.0):
        t = Twist()
        t.linear.x = float(x)
        t.linear.y = float(y)
        t.angular.z = float(yaw)
        pub.publish(t)

    fd = sys.stdin.fileno()
    old = termios.tcgetattr(fd)
    print(HELP)
    print(f"  linear = {lin} m/s   angular = {ang} rad/s")
    try:
        tty.setraw(fd)
        while True:
            key = get_key()
            if key is None:
                continue
            if key == 'UP':
                send(x=lin);  msg = f"forward  (x={lin})"
            elif key == 'DOWN':
                send(x=-lin); msg = f"backward (x=-{lin})"
            elif key == 'LEFT':
                send(yaw=ang);  msg = f"turn left  (yaw={ang})"
            elif key == 'RIGHT':
                send(yaw=-ang); msg = f"turn right (yaw=-{ang})"
            elif key in ('a', 'A'):
                send(y=lin);  msg = f"strafe left  (y={lin})"
            elif key in ('d', 'D'):
                send(y=-lin); msg = f"strafe right (y=-{lin})"
            elif key in (' ', 's', 'S'):
                send(); msg = "STOP"
            elif key in ('+', '='):
                lin += step; ang += step; msg = f"speed up   (lin={lin:.1f} ang={ang:.1f})"
            elif key in ('-', '_'):
                lin = max(0.1, lin - step); ang = max(0.1, ang - step)
                msg = f"speed down (lin={lin:.1f} ang={ang:.1f})"
            elif key in ('q', 'Q', '\x03'):  # q or Ctrl-C
                break
            else:
                continue
            # \r keeps lines tidy in raw mode
            sys.stdout.write('\r' + msg + ' ' * 10 + '\r\n')
            sys.stdout.flush()
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, old)
        send()  # stop
        node.get_logger().info("stopped.")
        rclpy.shutdown()


if __name__ == '__main__':
    main()
