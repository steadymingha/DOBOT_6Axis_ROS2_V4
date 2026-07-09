#!/usr/bin/env python3
"""Manually jog ONE arm joint by a relative angle, no planner/pinocchio.

Reads the current /joint_states, adds the delta to the chosen joint (others held),
and sends a single-point trajectory to the arm controller. For quick hardware/sim
pokes like "rotate J1 by 10 deg".

    source /opt/ros/humble/setup.bash
    source ~/dobot_ws/install/setup.bash
    python3 tools/jog_joint.py            # J1 +10 deg (default)
    python3 tools/jog_joint.py 1 -10      # J1 -10 deg
    python3 tools/jog_joint.py 6 15 --time 1.5
"""
import argparse
import math
import sys

import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
from sensor_msgs.msg import JointState
from control_msgs.action import FollowJointTrajectory
from trajectory_msgs.msg import JointTrajectoryPoint

JOINTS = ['joint1', 'joint2', 'joint3', 'joint4', 'joint5', 'joint6']
ACTION = '/cr7_group_controller/follow_joint_trajectory'


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('joint', nargs='?', type=int, default=1, help='joint number 1..6')
    ap.add_argument('deg', nargs='?', type=float, default=10.0, help='relative degrees')
    ap.add_argument('--time', type=float, default=2.0, help='move duration (s)')
    args = ap.parse_args()
    if not 1 <= args.joint <= 6:
        sys.exit('joint must be 1..6')

    rclpy.init()
    node = Node('jog_joint')
    cur = {}
    node.create_subscription(
        JointState, '/joint_states',
        lambda m: cur.update(zip(m.name, m.position)), 10)

    # Wait for one /joint_states with all 6 arm joints.
    while rclpy.ok() and not all(j in cur for j in JOINTS):
        rclpy.spin_once(node, timeout_sec=0.1)

    positions = [cur[j] for j in JOINTS]
    i = args.joint - 1
    positions[i] += math.radians(args.deg)
    node.get_logger().info(
        f"jog {JOINTS[i]} by {args.deg:+.1f} deg -> {math.degrees(positions[i]):+.1f} deg")

    client = ActionClient(node, FollowJointTrajectory, ACTION)
    if not client.wait_for_server(timeout_sec=5.0):
        sys.exit(f'action server {ACTION} not available')

    goal = FollowJointTrajectory.Goal()
    goal.trajectory.joint_names = JOINTS
    pt = JointTrajectoryPoint()
    pt.positions = positions
    pt.time_from_start.sec = int(args.time)
    pt.time_from_start.nanosec = int((args.time % 1) * 1e9)
    goal.trajectory.points.append(pt)

    fut = client.send_goal_async(goal)
    rclpy.spin_until_future_complete(node, fut)
    gh = fut.result()
    if not gh.accepted:
        sys.exit('goal rejected')
    res = gh.get_result_async()
    rclpy.spin_until_future_complete(node, res)
    node.get_logger().info('done')
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
