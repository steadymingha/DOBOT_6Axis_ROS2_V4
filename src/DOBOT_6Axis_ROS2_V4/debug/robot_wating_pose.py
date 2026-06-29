#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
import os
import math

from control_msgs.action import FollowJointTrajectory
from trajectory_msgs.msg import JointTrajectoryPoint


class DobotOverheadPose(Node):
    def __init__(self):
        super().__init__('dobot_overhead_pose')
        robot_type = os.getenv("DOBOT_TYPE", "cr7")
        action_name = f'/{robot_type}_group_controller/follow_joint_trajectory'
        self.traj_client = ActionClient(self, FollowJointTrajectory, action_name)

    def move_to_joint_pose(self, joint_positions_deg, duration_sec=3):
        """
        Move robot to specified joint angles.
        joint_positions_deg: [j1, j2, j3, j4, j5, j6] in degrees
        """
        if not self.traj_client.wait_for_server(timeout_sec=5.0):
            self.get_logger().error("Trajectory action server not available")
            return False

        # Convert degrees to radians (ROS2 uses radians)
        joint_positions_rad = [math.radians(deg) for deg in joint_positions_deg]

        joint_names = ["joint1", "joint2", "joint3", "joint4", "joint5", "joint6"]
        goal_msg = FollowJointTrajectory.Goal()
        goal_msg.trajectory.joint_names = joint_names

        point = JointTrajectoryPoint()
        point.positions = joint_positions_rad
        point.time_from_start.sec = duration_sec
        goal_msg.trajectory.points.append(point)

        self.get_logger().info(
            f"Moving to joint pose (deg): {joint_positions_deg}"
        )

        send_goal_future = self.traj_client.send_goal_async(goal_msg)
        rclpy.spin_until_future_complete(self, send_goal_future)

        goal_handle = send_goal_future.result()
        if not goal_handle.accepted:
            self.get_logger().error("Goal rejected")
            return False

        get_result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, get_result_future)
        self.get_logger().info("Reached target pose")
        return True


def main(args=None):
    rclpy.init(args=args)
    node = DobotOverheadPose()

    # Overhead Pose: TCP points straight down, away from all singularities
    # J2 + J3 + J5 = 90° ensures TCP faces -Z (vertical down)
    # J5 = 45° avoids wrist singularity
    # J3 = 90° avoids elbow singularity
    # OVERHEAD_POSE_DEG = [0, -30, 90, 0, 60, 0]
    OVERHEAD_POSE_DEG = [0, -10, 1, 10, 10, 0]

    print("\n========== Moving to Overhead Pose ==========")
    node.move_to_joint_pose(OVERHEAD_POSE_DEG, duration_sec=4)
    print("========== Done ==========\n")

    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()