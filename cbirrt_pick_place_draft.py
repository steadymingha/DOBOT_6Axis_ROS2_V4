"""Pick-and-place demo that mirrors test_w_gripper.py's main() sequence, but
replaces the joint-space RRT with CBiRRT for every motion performed while the
grasp orientation must be held.

Phases:
  1. Approach the grasp pose      -> free joint-space RRT (reused from
     CR7RRTPlanner.move_to_pose); this is what establishes the grasp tilt.
  2. Open gripper.
  3. Descend onto the box         -> CBiRRT (orientation held).
  4. Close gripper + attach box.
  5. Lift the box                 -> CBiRRT (orientation held).
  6. Carry to the place location  -> CBiRRT (orientation held; the box keeps
     the exact pose it had when grasped).
  7. Detach + open gripper.
  8. Move to the overhead/waiting pose -> direct joint move.

Reuses CR7RRTPlanner from test_w_gripper.py for IK (/compute_ik), collision
checking (/check_state_validity), the gripper controller and the link
attacher. test_w_gripper.py itself is NOT modified.

The CBiRRT constraint Jacobian comes from pinocchio (see constrained_cbirrt.py).

Run (sim already up):
    source /opt/ros/humble/setup.bash
    source ~/dobot_ws/install/setup.bash
    cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
    .venv/bin/python3 cbirrt_pick_place.py
"""

import os
import time
import random
import threading

import numpy as np
import rclpy
from rclpy.executors import MultiThreadedExecutor
from geometry_msgs.msg import PoseStamped
from trajectory_msgs.msg import JointTrajectoryPoint
from control_msgs.action import FollowJointTrajectory
from moveit_msgs.msg import RobotState
from moveit_msgs.srv import GetPositionIK

from test_w_gripper import CR7RRTPlanner
from constrained_cbirrt import ConstrainedPlanner

XACRO_PATH = os.path.expanduser(
    '~/dobot_ws/install/cra_description/share/cra_description/urdf/cr7_robot.xacro')


class CBiRRTPickPlace(CR7RRTPlanner):
    """CR7 node + helpers to run CBiRRT-planned, orientation-constrained motions."""

    def setup_planner(self):
        self.cbirrt = ConstrainedPlanner(xacro_path=XACRO_PATH)

    def compute_ik_ordered(self, target_pose: PoseStamped, max_retries=120,
                           want_candidates=12, near_attempts=60, near_sigma=0.25):
        """IK returning joints in joint1..joint6 order, within limits (or None).

        To keep the orientation-constrained CBiRRT path short, the goal should be
        the IK branch CLOSEST to the current configuration. MoveIt returns the
        branch nearest its seed, so we first try many seeds that are small random
        perturbations of the current pose (biasing toward the local branch), then
        fall back to fully random seeds. Among all valid, within-limit solutions
        we return the one closest in joint space to the current configuration."""
        if not self.ik_client.wait_for_service(timeout_sec=5.0):
            return None
        names = ['joint1', 'joint2', 'joint3', 'joint4', 'joint5', 'joint6']
        cur = np.array(self.current_joints) if self.current_joints is not None else None
        lo = np.array([l[0] for l in self.joint_limits])
        hi = np.array([l[1] for l in self.joint_limits])
        candidates = []
        for attempt in range(max_retries):
            req = GetPositionIK.Request()
            req.ik_request.group_name = self.group_name
            req.ik_request.pose_stamped = target_pose
            req.ik_request.timeout.sec = 1
            seed = RobotState()
            seed.joint_state.name = self.joint_names
            if cur is not None and attempt < near_attempts:
                # Seed near the current pose to find the local (nearest) IK branch.
                seed.joint_state.position = np.clip(
                    cur + np.random.normal(0, near_sigma, len(cur)), lo, hi).tolist()
            else:
                seed.joint_state.position = [random.uniform(l[0], l[1]) for l in self.joint_limits]
            req.ik_request.robot_state = seed
            future = self.ik_client.call_async(req)
            while rclpy.ok() and not future.done():
                time.sleep(0.01)
            res = future.result()
            if res.error_code.val != res.error_code.SUCCESS:
                continue
            sol = res.solution.joint_state
            try:
                q = [sol.position[sol.name.index(n)] for n in names]
            except ValueError:
                continue
            if self.is_within_limits(q, self.joint_limits):
                candidates.append(q)
                if len(candidates) >= want_candidates:
                    break
        if not candidates:
            self.get_logger().error(f"[IK] failed after {max_retries} retries")
            return None
        if cur is not None:
            candidates.sort(key=lambda q: np.linalg.norm(np.array(q) - cur))
            self.get_logger().info(
                f"[IK] {len(candidates)} candidates; nearest dist="
                f"{np.linalg.norm(np.array(candidates[0]) - cur):.3f}")
        else:
            self.get_logger().info(f"[IK] {len(candidates)} candidates")
        return candidates[0]

    def move_constrained(self, target_pose: PoseStamped, speed=0.6, time_limit=45.0):
        """Move to target_pose with the end-effector orientation held fixed at the
        target (= grasp) orientation, using CBiRRT. Blocks until execution ends."""
        while self.current_joints is None:
            self.get_logger().info("Waiting for /joint_states...")
            time.sleep(0.5)

        goal_q = self.compute_ik_ordered(target_pose)
        if goal_q is None:
            self.get_logger().error("[CBiRRT] goal IK failed")
            return False

        q = target_pose.pose.orientation
        self.cbirrt.set_reference((q.x, q.y, q.z, q.w))

        start_q = self.current_joints.tolist()
        self.get_logger().info("[CBiRRT] planning (orientation held)...")
        path = self.cbirrt.plan(start_q, goal_q, self.is_state_valid, self.joint_limits,
                                time_limit=time_limit)
        if not path:
            self.get_logger().error("[CBiRRT] planning failed")
            return False

        # Prepend the true current configuration so execution starts without a jump
        # (the projected start may differ from current by a hair).
        if np.linalg.norm(np.array(path[0]) - np.array(start_q)) > 1e-3:
            path = [start_q] + path
        self.get_logger().info(f"[CBiRRT] path: {len(path)} waypoints")
        return self.execute_path(path, speed=speed)

    def execute_path(self, path, speed=0.6):
        """Send joint waypoints (joint1..joint6) as one trajectory. Time between
        waypoints is proportional to joint-space distance (speed in rad/s)."""
        goal_msg = FollowJointTrajectory.Goal()
        goal_msg.trajectory.joint_names = self.joint_names
        t = 0.0
        prev = np.array(path[0], dtype=float)
        for i, joints in enumerate(path):
            cur = np.array(joints, dtype=float)
            dt = np.linalg.norm(cur - prev) / speed
            t += max(dt, 0.05) if i > 0 else 0.0
            prev = cur
            pt = JointTrajectoryPoint()
            pt.positions = [float(v) for v in joints]
            pt.time_from_start.sec = int(t)
            pt.time_from_start.nanosec = int((t % 1) * 1e9)
            goal_msg.trajectory.points.append(pt)

        self.traj_action_client.wait_for_server()
        send_goal_future = self.traj_action_client.send_goal_async(goal_msg)
        while rclpy.ok() and not send_goal_future.done():
            time.sleep(0.01)
        goal_handle = send_goal_future.result()
        if not goal_handle.accepted:
            self.get_logger().error("[execute_path] trajectory goal rejected")
            return False
        get_result_future = goal_handle.get_result_async()
        while rclpy.ok() and not get_result_future.done():
            time.sleep(0.01)
        self.get_logger().info("[execute_path] done")
        return True

    def lift_straight(self, dz, speed=0.4):
        """Lift the EE straight up by up to dz metres, holding orientation, via a
        pinocchio Cartesian-Jacobian servo seeded at the current pose (no IK branch
        jump). Automatically stops just before any joint reaches its limit. Blocks."""
        while self.current_joints is None:
            self.get_logger().info("Waiting for /joint_states...")
            time.sleep(0.5)
        start_q = self.current_joints.tolist()
        path, reached = self.cbirrt.lift_path(start_q, dz, self.is_state_valid, self.joint_limits)
        if reached <= 1e-3 or len(path) < 2:
            self.get_logger().error(f"[lift] could not lift (reached {reached*1000:.0f} mm)")
            return False
        if reached < abs(dz) - 1e-3:
            self.get_logger().warn(
                f"[lift] stopped at {reached*1000:.0f} mm of requested {abs(dz)*1000:.0f} mm "
                f"(joint limit reached)")
        self.get_logger().info(f"[lift] straight up {reached*1000:.0f} mm, {len(path)} waypoints")
        return self.execute_path(path, speed=speed)


def main(args=None):
    rclpy.init(args=args)
    node = CBiRRTPickPlace()
    node.setup_planner()

    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2)  # wait for joint states

    GRIPPER_OPEN = [0.09]
    GRIPPER_CLOSE = [0.036]

    # Grasp pose (same as test_w_gripper.py). The approach uses the free RRT.
    target = PoseStamped()
    target.header.frame_id = "base_link"
    target.pose.position.x = 0.4
    target.pose.position.y = 0.0
    target.pose.position.z = 0.3
    target.pose.orientation.x = 0.707
    target.pose.orientation.y = 0.707
    target.pose.orientation.z = 0.0
    target.pose.orientation.w = 0.0

    # 1. Approach the grasp pose with the free joint-space RRT.
    if not node.move_to_pose(target):
        node.get_logger().error("Approach failed")
        node.destroy_node(); rclpy.shutdown(); return

    time.sleep(1.0)
    node.control_gripper(GRIPPER_OPEN)
    time.sleep(0.5)

    # 2. Descend onto the box, holding the grasp orientation (CBiRRT).
    target.pose.position.z = 0.24
    if not node.move_constrained(target):
        node.get_logger().error("Descend (constrained) failed")
        node.destroy_node(); rclpy.shutdown(); return
    time.sleep(0.5)

    # 3. Grasp and fix the box to the gripper.
    node.control_gripper(GRIPPER_CLOSE)
    node.attach_box()

    # 4. Lift the box straight up, holding orientation (pinocchio Cartesian servo).
    #    Requests +0.21 m; stops automatically just before joint4 hits its 0 deg limit.
    if not node.lift_straight(0.21):
        node.get_logger().error("Lift (straight-up) failed")
        node.destroy_node(); rclpy.shutdown(); return
    time.sleep(0.5)

    # 5. Carry to above the place_marker at (0.2, 0.35), holding the grasp tilt
    #    (CBiRRT). z=0.25 puts the box bottom just over the marker; detach lets it
    #    settle. (The original (0.08, 0.08) target was unreachable with the grasp
    #    orientation; the marker location is reachable.)
    target.pose.position.x = 0.2
    target.pose.position.y = 0.35
    target.pose.position.z = 0.25
    if not node.move_constrained(target, time_limit=90.0):
        node.get_logger().error("Transport (constrained) failed")
        node.destroy_node(); rclpy.shutdown(); return
    time.sleep(0.5)

    # 6. Release the box.
    node.detach_box()
    node.control_gripper(GRIPPER_OPEN)
    time.sleep(0.5)

    # 7. Move to the overhead/waiting pose.
    OVERHEAD_POSE_DEG = [0, -10, 1, 10, 10, 0]
    print("\n========== Moving to Overhead Pose ==========")
    node.move_to_joint_pose(OVERHEAD_POSE_DEG, duration_sec=4)
    print("========== Done ==========\n")

    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()