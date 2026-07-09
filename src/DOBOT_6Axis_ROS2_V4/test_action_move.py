#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Minimal test: move the real robot through action_move_server.py.

Reads the current joints from /joint_states, adds a tiny offset to one joint,
and sends it as a FollowJointTrajectory goal to the action server (which relays
it to the robot via ServoJ). Set JOG_RAD = 0.0 to hold position (feedback only).

Run (bringup + action_move_server already up):
    source /opt/ros/humble/setup.bash
    source ~/dobot_ws/install/setup.bash
    python3 test_action_move.py
"""

import math
import os

import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
from control_msgs.action import FollowJointTrajectory
from trajectory_msgs.msg import JointTrajectoryPoint
from sensor_msgs.msg import JointState

JOG_JOINT = 0          # which joint to nudge (0..5)
JOG_RAD = math.radians(10.0)   # offset; set 0.0 for a no-move pipeline check
MOVE_TIME_S = 3.0       # time to reach the point


class MoveTester(Node):
    def __init__(self):
        super().__init__('action_move_tester')
        name = os.getenv('DOBOT_TYPE')
        action_name = f'/{name}_group_controller/follow_joint_trajectory'
        self.client = ActionClient(self, FollowJointTrajectory, action_name)
        self.js = None
        self.create_subscription(JointState, '/joint_states', self._on_js, 10)
        self.get_logger().info(f'Waiting for action server: {action_name}')

    def _on_js(self, msg):
        self.js = msg

    def run(self):
        # Wait for current joint state.
        while rclpy.ok() and self.js is None:
            rclpy.spin_once(self, timeout_sec=0.5)
        if not self.client.wait_for_server(timeout_sec=5.0):
            self.get_logger().error('Action server not available')
            return

        names = list(self.js.name)
        pos = list(self.js.position)
        self.get_logger().info(f'Current: {dict(zip(names, [round(p, 4) for p in pos]))}')
        pos[JOG_JOINT] += JOG_RAD

        goal = FollowJointTrajectory.Goal()
        goal.trajectory.joint_names = names
        point = JointTrajectoryPoint()
        point.positions = pos
        point.time_from_start.sec = int(MOVE_TIME_S)
        goal.trajectory.points = [point]

        self.get_logger().info(f'Sending goal (joint {JOG_JOINT} += {JOG_RAD:.4f} rad)')
        future = self.client.send_goal_async(goal)
        rclpy.spin_until_future_complete(self, future)
        handle = future.result()
        if not handle.accepted:
            self.get_logger().error('Goal rejected')
            return
        result_future = handle.get_result_async()
        rclpy.spin_until_future_complete(self, result_future)
        self.get_logger().info(f'Result error_code: {result_future.result().result.error_code}')


def main():
    rclpy.init()
    node = MoveTester()
    node.run()
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
