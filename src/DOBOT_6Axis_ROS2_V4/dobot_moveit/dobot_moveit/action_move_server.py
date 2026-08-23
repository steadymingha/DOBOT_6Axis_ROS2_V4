#!/usr/bin/env python3
# -*- coding: utf-8 -*-
'''
@author FTX
@date 2025 / 03 / 03
'''

import time
import rclpy
from rclpy.action import ActionServer
from rclpy.node import Node
from control_msgs.action import FollowJointTrajectory
from trajectory_msgs.msg import JointTrajectory, JointTrajectoryPoint
from dobot_msgs_v4.srv import *   # 自定义的服务接口
import os

class FollowJointTrajectoryServer(Node):

    def __init__(self):
        super().__init__('dobot_group_controller')
        name = os.getenv("DOBOT_TYPE")
        # 创建FollowJointTrajectory动作服务器
        self._action_server = ActionServer(self,FollowJointTrajectory,f'/{name}_group_controller/follow_joint_trajectory',self.execute_callback)
        self.get_logger().info("FollowJointTrajectory Action Server is ready...")
        self.EnableRobot_l = self.create_client(EnableRobot, '/dobot_bringup_ros2/srv/EnableRobot')
        self.ServoJ_l = self.create_client(ServoJ, '/dobot_bringup_ros2/srv/ServoJ')
        while not self.EnableRobot_l.wait_for_service(timeout_sec=1.0):  # 循环等待服务器端成功启动
            self.get_logger().info('service not available, waiting again...')

    async def execute_callback(self, goal_handle):
        self.get_logger().info("Received a new trajectory goal!")
        # 获取目标轨迹
        trajectory = goal_handle.request.trajectory
        self.execution_trajectory(trajectory)
        goal_handle.succeed()
        # 返回结果
        result = FollowJointTrajectory.Result()
        result.error_code = 0
        return result

    # ServoJ streaming interval. Dobot's own doc for ServoJ recommends calling it
    # at 33 Hz (30 ms) -- "issue the speed-planned points at a fixed interval t".
    # The old code called ServoJ once per incoming waypoint at a fixed 0.18s,
    # ignoring each point's time_from_start entirely: dense waypoints (fine,
    # slow moves) ran too slow, sparse ones (fast moves) forced a big joint jump
    # into a 0.2s ServoJ window. Resampling the trajectory onto a fixed DT grid
    # makes the executed speed match what the caller actually asked for.
    SERVOJ_DT = 0.03

    def execution_trajectory(self, trajectory: JointTrajectory):
        points = trajectory.points
        if not points:
            return
        times = [p.time_from_start.sec + p.time_from_start.nanosec * 1e-9 for p in points]
        positions_rad = [list(p.positions) for p in points]
        self.get_logger().info(
            f"Joint Names: {trajectory.joint_names}  points: {len(points)}  "
            f"total: {times[-1]:.2f}s")

        if len(points) == 1:
            # Single absolute target (e.g. a jog tick) -- ServoJ handles the
            # smooth move to it internally, no resampling needed.
            t = max(times[0], 0.05)
            self.ServoJ_C(*self._to_deg(positions_rad[0]), t=t)
            time.sleep(t)
            return

        dt = self.SERVOJ_DT
        total = times[-1]
        seg = 0
        t = dt
        while t < total:
            while seg < len(times) - 2 and t > times[seg + 1]:
                seg += 1
            t0, t1 = times[seg], times[seg + 1]
            p0, p1 = positions_rad[seg], positions_rad[seg + 1]
            alpha = 0.0 if t1 <= t0 else (t - t0) / (t1 - t0)
            rad = [a + (b - a) * alpha for a, b in zip(p0, p1)]
            self.ServoJ_C(*self._to_deg(rad), t=dt)
            time.sleep(dt)
            t += dt
        # land exactly on the final commanded point
        self.ServoJ_C(*self._to_deg(positions_rad[-1]), t=dt)
        time.sleep(dt)

    @staticmethod
    def _to_deg(rad_list):
        return [180.0 * v / 3.14159 for v in rad_list]

    def ServoJ_C(self, j1, j2, j3, j4, j5, j6, t=0.1):  # 运动指令
        P1 = ServoJ.Request()
        P1.a = float(j1)
        P1.b = float(j2)
        P1.c = float(j3)
        P1.d = float(j4)
        P1.e = float(j5)
        P1.f = float(j6)
        P1.param_value = [f"t={t}"]
        response = self.ServoJ_l.call_async(P1)
        # self.spin_until_future_complete(response)  # 等待响应
        # self.get_logger().info(f"{response.result()}")

def main(args=None):
    rclpy.init(args=args)
    follow_joint_trajectory_server = FollowJointTrajectoryServer()
    rclpy.spin(follow_joint_trajectory_server)
    follow_joint_trajectory_server.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()