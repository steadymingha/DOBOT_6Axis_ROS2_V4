"""Segmented pick-and-place: the motion is split into 6 path-planning segments,
each using the planner best suited to it. The grasp orientation (gripper pointing
straight down) is held through every segment after the grasp.

Segments:
  1. Approach        -> free joint-space RRT to a pre-grasp pose above the object
                        (CR7RRTPlanner.move_to_pose). No constraint needed yet.
  2. Descend->grasp  -> vertical straight-line Cartesian servo down onto the box
                        (pinocchio Jacobian, no RRT), then close gripper + attach.
  3. Lift            -> vertical straight-line Cartesian servo up to carry height.
  4. Carry           -> CBiRRT at constant carry height to above the place marker,
                        holding the tilt (gripper stays down, yaw free).
  5. Descend->place  -> vertical straight-line Cartesian servo down, then detach.
  6. Wait            -> small vertical retreat, then move to the overhead pose.

The vertical segments (2, 3, 5) use a Cartesian-Jacobian servo instead of RRT:
they are simple straight lines with no need to route around obstacles, so a
deterministic servo is faster and smoother than a sampling planner. Only the
horizontal carry (4) may need to route through joint space, so it uses CBiRRT.

Reuses CR7RRTPlanner from test_w_gripper.py for IK (/compute_ik), collision
checking (/check_state_validity), the gripper controller and the link attacher.
test_w_gripper.py itself is NOT modified. The CBiRRT/servo Jacobian comes from
pinocchio (see constrained_cbirrt.py).

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

    def vertical_servo(self, dz, speed=0.4):
        """Move the EE straight along world z by dz metres (sign = direction),
        holding its current orientation, via a pinocchio Cartesian-Jacobian servo
        seeded at the current pose (no IK branch jump). Automatically stops just
        before any joint reaches its limit or a collision occurs. Blocks."""
        direction = "up" if dz >= 0 else "down"
        while self.current_joints is None:
            self.get_logger().info("Waiting for /joint_states...")
            time.sleep(0.5)
        start_q = self.current_joints.tolist()
        path, reached = self.cbirrt.lift_path(start_q, dz, self.is_state_valid, self.joint_limits)
        if reached <= 1e-3 or len(path) < 2:
            self.get_logger().error(f"[servo] could not move {direction} (reached {reached*1000:.0f} mm)")
            return False
        if reached < abs(dz) - 1e-3:
            self.get_logger().warn(
                f"[servo] stopped at {reached*1000:.0f} mm of requested {abs(dz)*1000:.0f} mm "
                f"(joint limit / collision)")
        self.get_logger().info(f"[servo] straight {direction} {reached*1000:.0f} mm, {len(path)} waypoints")
        return self.execute_path(path, speed=speed)


def main(args=None):
    rclpy.init(args=args)
    node = CBiRRTPickPlace()
    node.setup_planner()

    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2)  # wait for joint states

    GRIPPER_OPEN = [0.04]    # axis -X: positive = open; retracted ~4 cm past box-grip position
    GRIPPER_CLOSE = [-0.01]  # axis -X: negative = close; just past zero for a firm grip on the box

    # Workspace constants.
    OBJECT_XY = (0.4, 0.0)     # pick_box location
    MARKER_XY = (0.2, 0.35)    # place_marker location
    Z_PREGRASP = 0.30          # approach height above the object
    Z_GRASP = 0.24             # EE height that grasps the box
    Z_CARRY = 0.30             # lift / transport height (above-marker is only
                               # IK-reachable up to ~0.30 with the down orientation)
    Z_PLACE = 0.25             # EE height to release over the marker
    # Gripper pointing straight DOWN (local z -> world -Z); held by the constraint.
    DOWN = (0.707, 0.707, 0.0, 0.0)

    def pose_at(xy, z):
        p = PoseStamped()
        p.header.frame_id = "base_link"
        p.pose.position.x, p.pose.position.y, p.pose.position.z = xy[0], xy[1], z
        p.pose.orientation.x, p.pose.orientation.y, p.pose.orientation.z, p.pose.orientation.w = DOWN
        return p

    def fail(msg):
        node.get_logger().error(msg)
        node.destroy_node(); rclpy.shutdown()

    # --- Segment 1: approach a pre-grasp pose above the object (free RRT) ---
    print("\n===== [1/6] Approach (free RRT) =====")
    if not node.move_to_pose(pose_at(OBJECT_XY, Z_PREGRASP)):
        return fail("Segment 1 (approach) failed")
    time.sleep(1.0)
    node.control_gripper(GRIPPER_OPEN)
    time.sleep(0.5)

    # --- Segment 2: vertical descend onto the box, then grasp (Cartesian servo) ---
    print("\n===== [2/6] Vertical descend -> grasp =====")
    if not node.vertical_servo(Z_GRASP - Z_PREGRASP):
        return fail("Segment 2 (descend) failed")
    time.sleep(0.5)
    node.control_gripper(GRIPPER_CLOSE)
    node.attach_box()

    # --- Segment 3: vertical lift to carry height (Cartesian servo) ---
    print("\n===== [3/6] Vertical lift =====")
    if not node.vertical_servo(Z_CARRY - Z_GRASP):
        return fail("Segment 3 (lift) failed")
    time.sleep(0.5)

    # --- Segment 4: carry at constant height to above the marker (CBiRRT) ---
    print("\n===== [4/6] Carry to above marker (CBiRRT, tilt held) =====")
    if not node.move_constrained(pose_at(MARKER_XY, Z_CARRY), time_limit=90.0):
        return fail("Segment 4 (carry) failed")
    time.sleep(0.5)

    # --- Segment 5: vertical descend over the marker, then release (Cartesian servo) ---
    print("\n===== [5/6] Vertical descend -> place =====")
    if not node.vertical_servo(Z_PLACE - Z_CARRY):
        return fail("Segment 5 (place descend) failed")
    time.sleep(0.5)
    node.detach_box()
    node.control_gripper(GRIPPER_OPEN)
    time.sleep(0.5)

    # --- Segment 6: retreat up a little, then go to the overhead/waiting pose ---
    print("\n===== [6/6] Retreat + overhead pose =====")
    node.vertical_servo(0.08)
    OVERHEAD_POSE_DEG = [0, -10, 1, 10, 10, 0]
    node.move_to_joint_pose(OVERHEAD_POSE_DEG, duration_sec=4)
    print("========== Done ==========\n")

    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()