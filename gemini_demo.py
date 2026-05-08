#!/usr/bin/env python3
import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
import os
import time

from geometry_msgs.msg import PoseStamped
from moveit_msgs.srv import GetPositionIK
from control_msgs.action import FollowJointTrajectory
from trajectory_msgs.msg import JointTrajectoryPoint

class DobotSimController(Node):
    def __init__(self):
        super().__init__('dobot_sim_controller')
        self.ik_client = self.create_client(GetPositionIK, '/compute_ik')
        robot_type = os.getenv("DOBOT_TYPE", "cr7")
        action_name = f'/{robot_type}_group_controller/follow_joint_trajectory'
        self.traj_client = ActionClient(self, FollowJointTrajectory, action_name)

    def move_to_coordinate(self, x, y, z):
        if not self.ik_client.wait_for_service(timeout_sec=5.0):
            return False

        req = GetPositionIK.Request()
        req.ik_request.group_name = f"{os.getenv('DOBOT_TYPE', 'cr7')}_group"
        pose = PoseStamped()
        pose.header.frame_id = "base_link"
        pose.pose.position.x = float(x)
        pose.pose.position.y = float(y)
        pose.pose.position.z = float(z)
        # Pointing downwards
        pose.pose.orientation.y = 1.0
        pose.pose.orientation.w = 0.0
        req.ik_request.pose_stamped = pose
        req.ik_request.timeout.sec = 2

        self.get_logger().info(f"Target [X:{x}, Y:{y}, Z:{z}] - Calculating IK...")
        future = self.ik_client.call_async(req)
        rclpy.spin_until_future_complete(self, future)
        
        response = future.result()
        if response.error_code.val != 1:
            self.get_logger().error(f"IK Failed (Code: {response.error_code.val})")
            return False

        joint_names = ["joint1", "joint2", "joint3", "joint4", "joint5", "joint6"]
        target_positions = [response.solution.joint_state.position[response.solution.joint_state.name.index(name)] for name in joint_names]

        self.traj_client.wait_for_server()
        goal_msg = FollowJointTrajectory.Goal()
        goal_msg.trajectory.joint_names = joint_names
        point = JointTrajectoryPoint()
        point.positions = target_positions
        point.time_from_start.sec = 3
        goal_msg.trajectory.points.append(point)
        
        send_goal_future = self.traj_client.send_goal_async(goal_msg)
        rclpy.spin_until_future_complete(self, send_goal_future)
        
        goal_handle = send_goal_future.result()
        if not goal_handle.accepted:
            return False
            
        get_result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, get_result_future)
        return True

def main(args=None):
    rclpy.init(args=args)
    node = DobotSimController()
    
    # --- Step 1: Move to picking point ---
    print("\n[STEP 1] Moving to Target...")
    # 40cm forward, 30cm high
    node.move_to_coordinate(0.4, 0.0, 0.3)
    
    # --- Step 2: Wait ---
    print("[WAIT] Picking up object (2 seconds)...")
    time.sleep(2.0)
    
    # --- Step 3: Return to safe standby position (Home) ---
    print("[STEP 2] Returning to Home...")
    # Recommended Home Position: 20cm forward, 50cm high
    # DO NOT use (0,0,0) - it causes self-collision
    node.move_to_coordinate(0.2, 0.0, 0.5)
    
    print("\nAll tasks completed successfully!")
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()