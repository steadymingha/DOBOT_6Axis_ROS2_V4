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
        self.gripper_client = ActionClient(self, FollowJointTrajectory, '/gripper_controller/follow_joint_trajectory')

    def control_gripper(self, positions):
        if not self.gripper_client.wait_for_server(timeout_sec=5.0):
            self.get_logger().error("Gripper action server not available")
            return False

        goal_msg = FollowJointTrajectory.Goal()
        goal_msg.trajectory.joint_names = ['left_finger_joint', 'right_finger_joint']
        point = JointTrajectoryPoint()
        point.positions = [float(p) for p in positions]
        point.velocities = [0.0, 0.0]
        point.time_from_start.sec = 2
        goal_msg.trajectory.points.append(point)

        send_goal_future = self.gripper_client.send_goal_async(goal_msg)
        rclpy.spin_until_future_complete(self, send_goal_future)

        goal_handle = send_goal_future.result()
        if not goal_handle.accepted:
            self.get_logger().error("Gripper goal rejected")
            return False

        get_result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, get_result_future)
        self.get_logger().info(f"Gripper moved to positions: {positions}")
        return True

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

    # Box pick position:  (0.4, 0.0)  — red box in cr.world
    # Box place position: (0.2, 0.35) — yellow marker in cr.world
    PICK_X,  PICK_Y  = 0.4, 0.0
    PLACE_X, PLACE_Y = 0.2, 0.35
    APPROACH_Z = 0.25   # safe height for horizontal travel
    PICK_Z     = 0.07   # gripper descend height to grip the box
    CARRY_Z    = 0.35   # lift height while carrying
    GRIPPER_OPEN  = [0.06, 0.06]   # fingers wide open
    GRIPPER_CLOSE = [0.01, 0.01]   # fingers pressed against box

    print("\n========== Pick-and-Place Sequence Start ==========")

    # 1. Move to safe home position
    print("[1/9] Moving to home position...")
    node.move_to_coordinate(0.2, 0.0, 0.5)

    # 2. Open gripper before approaching
    print("[2/9] Opening gripper...")
    node.control_gripper(GRIPPER_OPEN)

    # 3. Move above pick position
    print("[3/9] Moving above pick position (0.4, 0.0)...")
    node.move_to_coordinate(PICK_X, PICK_Y, APPROACH_Z)

    # 4. Descend onto box
    print("[4/9] Descending to grasp box...")
    node.move_to_coordinate(PICK_X, PICK_Y, PICK_Z)

    # 5. Close gripper to grasp box
    print("[5/9] Closing gripper to grasp box...")
    node.control_gripper(GRIPPER_CLOSE)
    time.sleep(0.5)

    # 6. Lift box
    print("[6/9] Lifting box...")
    node.move_to_coordinate(PICK_X, PICK_Y, CARRY_Z)

    # 7. Move horizontally to place position
    print("[7/9] Moving to place position (0.2, 0.35)...")
    node.move_to_coordinate(PLACE_X, PLACE_Y, CARRY_Z)

    # 8. Descend to place height
    print("[8/9] Lowering box onto place marker...")
    node.move_to_coordinate(PLACE_X, PLACE_Y, PICK_Z)

    # 9. Open gripper to release box
    print("[9/9] Releasing box...")
    node.control_gripper(GRIPPER_OPEN)
    time.sleep(0.5)

    # Return to safe home
    print("[Done] Returning to home position...")
    node.move_to_coordinate(PLACE_X, PLACE_Y, APPROACH_Z)
    node.move_to_coordinate(0.2, 0.0, 0.5)

    print("========== Pick-and-Place Sequence Complete ==========\n")
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()