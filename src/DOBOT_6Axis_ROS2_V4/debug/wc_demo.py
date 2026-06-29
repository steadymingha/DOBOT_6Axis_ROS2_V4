#!/usr/bin/env python3
# Joint-space test runner.
# Pick one of WC1..WC8 by setting WC_ID below (or pass it as argv[1])
# and the robot will move to that joint configuration in Gazebo.
# All joint values in the table below are specified in DEGREES.

import math
import os
import sys

import rclpy
from rclpy.action import ActionClient
from rclpy.node import Node

from control_msgs.action import FollowJointTrajectory
from moveit_msgs.srv import GetPositionFK
from sensor_msgs.msg import JointState
from trajectory_msgs.msg import JointTrajectoryPoint

# Reference frame for end-effector pose output (user coordinate).
# base_link = robot's own base; matches the IK frame used elsewhere.
USER_FRAME = "base_link"
# Link that represents the end-effector tip for FK.
# Use "Link6" for the flange; change to a gripper tip link if you have one.
TARGET_LINK = "Link6"

# Predefined joint poses, units: degrees, order [J1, J2, J3, J4, J5, J6]
WORK_CASES = {
    "WC1": ("Max -Y reach (side cantilever toward equipment)", [0,   -70,  -20, 0, 0, 0]),
    "WC2": ("Max -Y reach (side cantilever toward equipment)", [-45,   -70,  -20, 0, 0, 0]),
    "WC3": ("Max -Y reach (side cantilever toward equipment)", [-101,   -70,  -20, 0, 0, 0]),
    # "WC1": ("Max -Y reach (side cantilever toward equipment)", [-90,   0,  -20, 0, -90, 0]),
    # "WC2": ("Max +X reach (front cantilever)",                 [  0,   0,  -20, 0, -90, 0]),
    # "WC3": ("Max -X reach (rear, J1 limit)",                   [-101,  0,  -20, 0, -90, 0]),
    # "WC4": ("Lowest Z (deepest down)",                         [-45, -60, -140, 0, -90, 0]),
    # "WC5": ("Highest Z (highest up)",                          [  0,  60, -140, 0, -20, 0]),
    # "WC6": ("Tucked pose (transport/home candidate)",          [  0,   0, -140, 0, -90, 0]),
    # "WC7": ("Mid-work pose (above equipment)",                 [-90, -30,  -90, 0, -90, 0]),
    # "WC8": ("Max -Y side reach",                               [-90,  30,  -20, 0, -90, 0]),
}

JOINT_NAMES = ["joint1", "joint2", "joint3", "joint4", "joint5", "joint6"]


def quat_to_rpy_deg(q):
    # ZYX intrinsic Euler angles, returned in degrees.
    sinr_cosp = 2.0 * (q.w * q.x + q.y * q.z)
    cosr_cosp = 1.0 - 2.0 * (q.x * q.x + q.y * q.y)
    roll = math.atan2(sinr_cosp, cosr_cosp)
    sinp = max(-1.0, min(1.0, 2.0 * (q.w * q.y - q.z * q.x)))
    pitch = math.asin(sinp)
    siny_cosp = 2.0 * (q.w * q.z + q.x * q.y)
    cosy_cosp = 1.0 - 2.0 * (q.y * q.y + q.z * q.z)
    yaw = math.atan2(siny_cosp, cosy_cosp)
    return math.degrees(roll), math.degrees(pitch), math.degrees(yaw)


class JointPoseTester(Node):
    def __init__(self):
        super().__init__("joint_pose_tester")
        robot_type = os.getenv("DOBOT_TYPE", "cr7")
        action_name = f"/{robot_type}_group_controller/follow_joint_trajectory"
        self.traj_client = ActionClient(self, FollowJointTrajectory, action_name)
        self.fk_client = self.create_client(GetPositionFK, "/compute_fk")

        self.latest_joint_state = None
        self.create_subscription(JointState, "/joint_states", self._on_joint_state, 10)

    def _on_joint_state(self, msg):
        self.latest_joint_state = msg

    def wait_for_joint_state(self, timeout_sec=5.0):
        start = self.get_clock().now()
        while rclpy.ok() and self.latest_joint_state is None:
            rclpy.spin_once(self, timeout_sec=0.05)
            if (self.get_clock().now() - start).nanoseconds / 1e9 > timeout_sec:
                self.get_logger().error("Timed out waiting for /joint_states")
                return False
        return True

    def current_joint_positions(self):
        msg = self.latest_joint_state
        positions = []
        for name in JOINT_NAMES:
            if name not in msg.name:
                self.get_logger().error(f"Joint '{name}' not in /joint_states")
                return None
            positions.append(msg.position[msg.name.index(name)])
        return positions

    def print_current_joints(self, tag):
        if self.latest_joint_state is None:
            return
        print(f"[{tag}] Current joint angles (deg):")
        for n, p in zip(self.latest_joint_state.name, self.latest_joint_state.position):
            if not n.startswith("joint"):
                continue
            print(f"  {n} = {p * 180.0 / math.pi:.2f} deg")

    def compute_fk(self, joint_rad, target_link=TARGET_LINK, frame_id=USER_FRAME):
        # Ask MoveIt for the forward kinematics of the given joint angles.
        # Returns a PoseStamped of target_link expressed in frame_id, or None on failure.
        if not self.fk_client.wait_for_service(timeout_sec=5.0):
            self.get_logger().error("FK service /compute_fk not available")
            return None

        req = GetPositionFK.Request()
        req.header.frame_id = frame_id
        req.fk_link_names = [target_link]
        req.robot_state.joint_state.name = list(JOINT_NAMES)
        req.robot_state.joint_state.position = list(joint_rad)

        future = self.fk_client.call_async(req)
        rclpy.spin_until_future_complete(self, future)
        res = future.result()
        if res is None or res.error_code.val != 1:
            code = res.error_code.val if res else "N/A"
            self.get_logger().error(f"FK Failed (Code: {code})")
            return None
        return res.pose_stamped[0]

    def print_fk(self, joint_rad, tag):
        pose_stamped = self.compute_fk(joint_rad)
        if pose_stamped is None:
            return
        p = pose_stamped.pose.position
        o = pose_stamped.pose.orientation
        r, pi, y = quat_to_rpy_deg(o)
        print(f"[{tag}] End-effector pose ({TARGET_LINK}) in '{pose_stamped.header.frame_id}' frame:")
        print(f"  Position (m):      X={p.x:+.4f}  Y={p.y:+.4f}  Z={p.z:+.4f}")
        print(f"  Orientation (deg): R={r:+.2f}  P={pi:+.2f}  Y={y:+.2f}")
        print(f"  Quaternion:        x={o.x:+.4f} y={o.y:+.4f} z={o.z:+.4f} w={o.w:+.4f}")

    def move_to_joints(self, target_rad, duration_sec=3.0):
        if not self.traj_client.wait_for_server(timeout_sec=5.0):
            self.get_logger().error("Trajectory action server not available")
            return False
        if not self.wait_for_joint_state():
            return False

        q0_list = self.current_joint_positions()
        if q0_list is None:
            return False

        # Quintic polynomial interpolation from current to target with zero
        # boundary velocity and acceleration.
        T = float(duration_sec)
        N = 60

        goal = FollowJointTrajectory.Goal()
        goal.trajectory.joint_names = JOINT_NAMES

        for i in range(1, N + 1):
            t = T * i / N
            point = JointTrajectoryPoint()
            for j in range(6):
                q0 = q0_list[j]
                q1 = target_rad[j]
                a0 = q0
                a3 = 10.0 * (q1 - q0) / T**3
                a4 = -15.0 * (q1 - q0) / T**4
                a5 = 6.0 * (q1 - q0) / T**5
                pos = a0 + a3 * t**3 + a4 * t**4 + a5 * t**5
                vel = 3.0 * a3 * t**2 + 4.0 * a4 * t**3 + 5.0 * a5 * t**4
                acc = 6.0 * a3 * t + 12.0 * a4 * t**2 + 20.0 * a5 * t**3
                point.positions.append(pos)
                point.velocities.append(vel)
                point.accelerations.append(acc)
            point.time_from_start.sec = int(t)
            point.time_from_start.nanosec = int((t - int(t)) * 1e9)
            goal.trajectory.points.append(point)

        send_goal_future = self.traj_client.send_goal_async(goal)
        rclpy.spin_until_future_complete(self, send_goal_future)
        goal_handle = send_goal_future.result()
        if not goal_handle or not goal_handle.accepted:
            self.get_logger().error("Goal was rejected by the action server")
            return False

        get_result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, get_result_future)
        self.get_logger().info("Movement complete.")
        return True


def main(args=None):
    rclpy.init(args=args)

    # ============================================================
    # Select which work case to execute here.
    # Override at runtime: python3 wc_demo.py WC4
    # ============================================================
    WC_ID = "WC3"

    if len(sys.argv) >= 2:
        WC_ID = sys.argv[1]

    if WC_ID not in WORK_CASES:
        print(f"Unknown work case: {WC_ID}")
        print("Available cases:")
        for cid, (desc, _) in WORK_CASES.items():
            print(f"  {cid} - {desc}")
        rclpy.shutdown()
        return

    description, deg_list = WORK_CASES[WC_ID]
    node = JointPoseTester()

    if not node.wait_for_joint_state():
        node.destroy_node()
        rclpy.shutdown()
        return

    print(f"\n[{WC_ID}] {description}")
    print(f"Target (deg): {deg_list}")

    node.print_current_joints("BEFORE")

    target_rad = [d * math.pi / 180.0 for d in deg_list]

    # Predicted end-effector pose for the target joints, computed via MoveIt FK.
    print()
    node.print_fk(target_rad, "PREDICTED")

    node.move_to_joints(target_rad)

    # Drain a few spins so the latest joint state reflects the move.
    for _ in range(20):
        rclpy.spin_once(node, timeout_sec=0.05)
    node.print_current_joints("AFTER")

    # Actual end-effector pose using the joints reported by /joint_states.
    actual_rad = node.current_joint_positions()
    if actual_rad is not None:
        print()
        node.print_fk(actual_rad, "ACTUAL")

    print("\nDone.")
    node.destroy_node()
    rclpy.shutdown()


if __name__ == "__main__":
    main()
