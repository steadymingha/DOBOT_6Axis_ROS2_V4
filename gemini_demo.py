#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
import os

from geometry_msgs.msg import PoseStamped
from moveit_msgs.srv import GetPositionIK
from control_msgs.action import FollowJointTrajectory
from trajectory_msgs.msg import JointTrajectoryPoint

class DobotSimController(Node):
    def __init__(self):
        super().__init__('dobot_sim_controller')
        
        # 1. IK (Inverse Kinematics) service client (requests joint angle calculation from MoveIt running in Gazebo/RViz)
        self.ik_client = self.create_client(GetPositionIK, '/compute_ik')

        # 2. Action client (sends "move to this angle!" commands to the robot controller running in Gazebo)
        robot_type = os.getenv("DOBOT_TYPE", "cr7")
        action_name = f'/{robot_type}_group_controller/follow_joint_trajectory'
        self.traj_client = ActionClient(self, FollowJointTrajectory, action_name)

    def move_to_coordinate(self, x, y, z):
        self.get_logger().info("Waiting for MoveIt IK service (/compute_ik)...")
        self.ik_client.wait_for_service()

        # Set target coordinates (X, Y, Z in meters)
        req = GetPositionIK.Request()
        robot_type = os.getenv("DOBOT_TYPE", "cr7")
        req.ik_request.group_name = f"{robot_type}_group"  # e.g. 'cr7_group' for CR7
        
        pose = PoseStamped()
        pose.header.frame_id = "base_link"
        pose.pose.position.x = float(x)
        pose.pose.position.y = float(y)
        pose.pose.position.z = float(z)
        
        # Set quaternion orientation so the end-effector points downward
        pose.pose.orientation.x = 0.0
        pose.pose.orientation.y = 1.0
        pose.pose.orientation.z = 0.0
        pose.pose.orientation.w = 0.0
        
        req.ik_request.pose_stamped = pose
        req.ik_request.timeout.sec = 2

        # Step 1: Request joint angle calculation from MoveIt
        self.get_logger().info(f"Target [X:{x}, Y:{y}, Z:{z}] - Calculating IK...")
        future = self.ik_client.call_async(req)
        rclpy.spin_until_future_complete(self, future)
        
        response = future.result()
        if response.error_code.val != 1:  # 1 means SUCCESS
            self.get_logger().error(f"Cannot reach target! (MoveIt Error Code: {response.error_code.val})")
            return

        # Extract the 6 joint angles computed by MoveIt
        joint_names = ["joint1", "joint2", "joint3", "joint4", "joint5", "joint6"]
        target_positions = []
        
        for name in joint_names:
            idx = response.solution.joint_state.name.index(name)
            target_positions.append(response.solution.joint_state.position[idx])

        # Step 2: Send trajectory command to the Gazebo robot
        self.get_logger().info("Path found! Sending command to Gazebo...")
        self.traj_client.wait_for_server()
        
        goal_msg = FollowJointTrajectory.Goal()
        goal_msg.trajectory.joint_names = joint_names
        
        point = JointTrajectoryPoint()
        point.positions = target_positions
        point.time_from_start.sec = 3  # Smoothly reach the target over 3 seconds
        goal_msg.trajectory.points.append(point)
        
        send_goal_future = self.traj_client.send_goal_async(goal_msg)
        rclpy.spin_until_future_complete(self, send_goal_future)
        
        goal_handle = send_goal_future.result()
        if not goal_handle.accepted:
            self.get_logger().error("Gazebo rejected the movement command.")
            return
            
        self.get_logger().info("Robot is moving in Gazebo!")
        get_result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, get_result_future)
        self.get_logger().info("Movement Complete!")

def main(args=None):
    rclpy.init(args=args)
    node = DobotSimController()
    
    # Target X, Y, Z coordinates for the robot to move to (unit: m)
    node.move_to_coordinate(0.3, 0.1, 0.3)
    
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()