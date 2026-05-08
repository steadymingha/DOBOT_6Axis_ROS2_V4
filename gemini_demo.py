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
        
        # 1. IK (역기구학) 서비스 클라이언트 (가제보/RViz에 켜진 MoveIt 뇌에 "관절 각도 계산해줘" 요청)
        self.ik_client = self.create_client(GetPositionIK, '/compute_ik')
        
        # 2. 액션 클라이언트 (가제보에 켜진 로봇 근육에 "이 각도로 움직여!" 명령)
        robot_type = os.getenv("DOBOT_TYPE", "cr7")
        action_name = f'/{robot_type}_group_controller/follow_joint_trajectory'
        self.traj_client = ActionClient(self, FollowJointTrajectory, action_name)

    def move_to_coordinate(self, x, y, z):
        self.get_logger().info("Waiting for MoveIt IK service (/compute_ik)...")
        self.ik_client.wait_for_service()

        # 목표 좌표 설정 (X, Y, Z 미터 단위)
        req = GetPositionIK.Request()
        robot_type = os.getenv("DOBOT_TYPE", "cr7")
        req.ik_request.group_name = f"{robot_type}_group"  # CR7의 경우 'cr7_group'
        
        pose = PoseStamped()
        pose.header.frame_id = "base_link"
        pose.pose.position.x = float(x)
        pose.pose.position.y = float(y)
        pose.pose.position.z = float(z)
        
        # 손끝(End-effector)이 바닥을 향하도록 임의의 쿼터니언 방향 설정
        pose.pose.orientation.x = 0.0
        pose.pose.orientation.y = 1.0
        pose.pose.orientation.z = 0.0
        pose.pose.orientation.w = 0.0
        
        req.ik_request.pose_stamped = pose
        req.ik_request.timeout.sec = 2

        # 1단계: MoveIt에 관절 각도 계산 요청
        self.get_logger().info(f"Target [X:{x}, Y:{y}, Z:{z}] - Calculating IK...")
        future = self.ik_client.call_async(req)
        rclpy.spin_until_future_complete(self, future)
        
        response = future.result()
        if response.error_code.val != 1:  # 1은 SUCCESS를 의미
            self.get_logger().error(f"Cannot reach target! (MoveIt Error Code: {response.error_code.val})")
            return

        # MoveIt이 계산해준 6개의 관절 각도 빼오기
        joint_names = ["joint1", "joint2", "joint3", "joint4", "joint5", "joint6"]
        target_positions = []
        
        for name in joint_names:
            idx = response.solution.joint_state.name.index(name)
            target_positions.append(response.solution.joint_state.position[idx])

        # 2단계: 가제보 로봇에게 이동 명령(Trajectory) 발송
        self.get_logger().info("Path found! Sending command to Gazebo...")
        self.traj_client.wait_for_server()
        
        goal_msg = FollowJointTrajectory.Goal()
        goal_msg.trajectory.joint_names = joint_names
        
        point = JointTrajectoryPoint()
        point.positions = target_positions
        point.time_from_start.sec = 3  # 3초에 걸쳐서 스르륵 부드럽게 이동
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
    
    # 🌟 로봇이 이동할 목표 X, Y, Z 좌표 (단위: m)
    node.move_to_coordinate(0.3, 0.1, 0.3)
    
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()