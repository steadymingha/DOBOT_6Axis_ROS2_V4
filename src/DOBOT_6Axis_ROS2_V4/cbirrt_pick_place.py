"""Segmented pick-and-place: the motion is split into 6 path-planning segments,
each using the planner best suited to it. The grasp orientation (gripper pointing
straight down) is held through every segment after the grasp.

Segments:
  1. Approach        -> free joint-space RRT to a pre-grasp pose above the object
                        (CR7RRTPlanner.move_to_pose). No constraint needed yet.
  2. Descend->grasp  -> after the in-gap J6 twist, a horizontal jaw-align servo
                        (the grasp centre between the pads sits ~53 mm off the
                        flange axis toward the fixed jaw), then a vertical
                        straight-line Cartesian servo down onto the box
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
import math
import time
import threading

import numpy as np
import pinocchio as pin
import rclpy
from rclpy.duration import Duration
from rclpy.executors import MultiThreadedExecutor
from geometry_msgs.msg import PoseStamped
from trajectory_msgs.msg import JointTrajectoryPoint
from control_msgs.action import FollowJointTrajectory
from tf2_ros import Buffer, TransformListener
import tf2_geometry_msgs  # noqa: F401  (registers PoseStamped with tf2)

from test_w_gripper import CR7RRTPlanner
from constrained_cbirrt import ConstrainedPlanner
from reachability_map import ReachabilityModel

# Arm-only model for the orientation constraint / Cartesian servo (FK + Jacobian).
XACRO_PATH = os.path.expanduser(
    '~/dobot_ws/install/cra_description/share/cra_description/urdf/cr7_robot.xacro')

# Combined robot (arm + cube platform + MPO-700 AGV + gripper) for collision
# checking: lets us catch the arm hitting the cube/AGV, not just itself. Source
# the ROS workspace before running so xacro can resolve the package includes.
COMBINED_XACRO = os.path.expanduser(
    '~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/cra_description/urdf/cr7_on_mpo700.urdf.xacro')

# Shelf collision model. The Gazebo 'shelf' world model is not part of the robot
# URDF, so we add its horizontal boards as thin boxes to the planning collision
# model and place them at the AGV's current pose via TF each cycle. Boards (not a
# solid block) so the GAPS between tiers stay open -- the straight insert servo
# (which is also collision-checked) must be able to pass through the opening to
# reach the box, while the boards still stop the arm hitting the structure.
SHELF_WORLD_XY = (0.8, 0.5)             # shelf model origin in world (x, y)
SHELF_BOARD_TOPS = (0.40, 0.90, 1.40, 1.90)  # board top heights (world z), 4-tier
SHELF_FOOTPRINT = (2.0, 0.30)           # board size (x, y) in metres
SHELF_BOARD_THICK = 0.018               # board thickness (z), real shelf board


def quat_to_R(x, y, z, w):
    """Unit quaternion (xyzw) -> 3x3 rotation matrix."""
    n = math.sqrt(x * x + y * y + z * z + w * w) or 1.0
    x, y, z, w = x / n, y / n, z / n, w / n
    return np.array([
        [1 - 2 * (y * y + z * z), 2 * (x * y - z * w), 2 * (x * z + y * w)],
        [2 * (x * y + z * w), 1 - 2 * (x * x + z * z), 2 * (y * z - x * w)],
        [2 * (x * z - y * w), 2 * (y * z + x * w), 1 - 2 * (x * x + y * y)],
    ])


def quat_about_z(angle):
    """Quaternion (xyzw) for a rotation `angle` (rad) about the z-axis."""
    return (0.0, 0.0, math.sin(angle / 2.0), math.cos(angle / 2.0))


def quat_mul(q1, q2):
    """Hamilton product q1 (x) q2, both (xyzw); applies q1 after q2."""
    x1, y1, z1, w1 = q1
    x2, y2, z2, w2 = q2
    return (
        w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2,
        w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2,
        w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2,
        w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2,
    )


def wait_for_spacebar():
    """Block until the user presses SPACE (-> 'go') or q/Esc/Ctrl-C (-> 'quit').

    Reads one keypress in raw terminal mode. Isolated here so it can later be
    swapped for a ROS topic/service trigger (the AGV will signal over TCP/IP)
    without touching the pick-and-place logic."""
    import sys
    import termios
    import tty
    fd = sys.stdin.fileno()
    old = termios.tcgetattr(fd)
    try:
        tty.setraw(fd)
        while True:
            ch = sys.stdin.read(1)
            if ch == ' ':
                return 'go'
            if ch in ('q', '\x1b', '\x03'):   # q, Esc, Ctrl-C
                return 'quit'
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, old)


class CBiRRTPickPlace(CR7RRTPlanner):
    """CR7 node + helpers to run CBiRRT-planned, orientation-constrained motions."""

    def setup_planner(self):
        # Attach the box to Link6, not gripper_base_link: gripper_attach_joint is
        # FIXED, so the URDF->SDF conversion lumps gripper_base_link into Link6
        # and the Gazebo model has no link by that name -- ATTACHLINK then fails
        # with "Failed to find link". Link6 exists (child of revolute joint6) and
        # is rigid with the gripper, so the attachment is equivalent.
        self.gripper_link = 'Link6'

        # Open every joint to the URDF hardware limit (+-6.27 rad). The base-class
        # values (J1 -101..10, J2 -70..60, J4 0..120, J5 -120..120, J3/J6 +-pi)
        # are conservative SOFTWARE clamps that block otherwise-valid elbow/wrist
        # branches -- e.g. the ELBOW-DOWN shelf grasp (J4 negative) that lives in
        # the pocket's family, and the wide J1 needed because the shelf sits
        # roughly opposite the magazine. Self/scene collision is still enforced by
        # the pinocchio model below, so widening to hardware cannot fold the arm
        # into itself or the AGV/cube undetected. Done here (not in test_w_gripper)
        # so that file stays unmodified.
        HW = 6.27   # URDF hardware joint limit, radians (~359 deg)
        self.joint_limits = [(-HW, HW)] * len(self.joint_limits)

        self.cbirrt = ConstrainedPlanner(xacro_path=XACRO_PATH)
        # pinocchio collision model of the WHOLE robot (arm + cube + AGV +
        # gripper). All validity checks below go through this instead of MoveIt's
        # /check_state_validity, so loosened joint limits cannot fold the arm
        # into itself or into the cube/AGV undetected.
        self.collision = ReachabilityModel(
            xacro_path=COMBINED_XACRO, lock_non_arm=True, arm_pairs_only=True,
            xacro_mappings={'use_gazebo': 'false'})
        self.collision.set_joint_limits(self.joint_limits)
        n_active = self.collision.pair_stats[2]
        self.get_logger().info(
            f"[collision] pinocchio model ready: {n_active} active pairs "
            f"(arm+cube+AGV+gripper)")

        # IK model: the SAME base_link-rooted arm model and the SAME
        # inverse_kinematics solver the reachability map uses, so "reachable on
        # the map" == "IK solvable here". (MoveIt /compute_ik is not used -- its
        # KDL solver disagrees with the map.) Targets are the gripper TCP, matching
        # the map's convention (Link6 + TCP_OFFSET_M along the tool axis).
        self.ik_model = ReachabilityModel(xacro_path=XACRO_PATH)
        self.ik_model.set_joint_limits(self.joint_limits)

        # Add the (world-fixed) shelf to the collision model so the RRT avoids it.
        self._add_shelf_boards()

        # TF: the shelf boxes are fixed in the Gazebo world; the AGV may be driven
        # anywhere before triggering, so the box pose in base_link must be looked
        # up at runtime. There is no 'world' frame in the TF tree -- the AGV spawns
        # at the world origin, so 'odom' is the world proxy.
        self.world_frame = 'odom'
        self.tf_buffer = Buffer()
        self.tf_listener = TransformListener(self.tf_buffer, self)

    def transform_world_pose(self, x, y, z, quat_xyzw, timeout=3.0):
        """Transform a pose given in the world (== odom) frame into base_link via
        the live TF tree. Returns a PoseStamped in base_link, or None if the
        transform is unavailable (so the caller can ask the user to reposition)."""
        ps = PoseStamped()
        ps.header.frame_id = self.world_frame
        ps.header.stamp = rclpy.time.Time().to_msg()   # 0 == latest available
        ps.pose.position.x, ps.pose.position.y, ps.pose.position.z = (
            float(x), float(y), float(z))
        (ps.pose.orientation.x, ps.pose.orientation.y,
         ps.pose.orientation.z, ps.pose.orientation.w) = (float(v) for v in quat_xyzw)
        try:
            return self.tf_buffer.transform(
                ps, 'base_link', timeout=Duration(seconds=timeout))
        except Exception as e:
            self.get_logger().error(
                f"[TF] {self.world_frame}->base_link lookup failed: {e}")
            return None

    def transform_world_vector(self, vec3, timeout=3.0):
        """Rotate a direction vector from world (== odom) into base_link (rotation
        only, no translation). Used to find the shelf row direction in base_link
        so the gripper yaw can be aligned to it. Returns np.array or None."""
        try:
            tf = self.tf_buffer.lookup_transform(
                'base_link', self.world_frame, rclpy.time.Time(),
                timeout=Duration(seconds=timeout))
        except Exception as e:
            self.get_logger().error(f"[TF] vector lookup failed: {e}")
            return None
        q = tf.transform.rotation
        R = quat_to_R(q.x, q.y, q.z, q.w)
        return R @ np.asarray(vec3, dtype=float)

    def is_state_valid(self, joint_positions):
        """Collision check via the pinocchio whole-robot model (overrides the
        MoveIt /check_state_validity check in the base class). Returns True if the
        6-arm-joint config is free of self-, cube/AGV- and shelf-collisions."""
        return self.collision.is_collision_free(list(joint_positions))

    def _add_shelf_boards(self):
        """Add the 3 shelf boards as thin boxes to the collision model, paired
        against every movable arm/gripper link. They start parked far away and are
        positioned per-cycle by update_shelf_collision(). Gaps between boards stay
        open so the insert servo can reach into the shelf."""
        import coal
        geom = self.collision.geom
        objs = geom.geometryObjects
        arm_links = [i for i in range(len(objs)) if objs[i].parentJoint != 0]
        far = pin.SE3(np.eye(3), np.array([0.0, 0.0, -100.0]))
        sx, sy = SHELF_FOOTPRINT
        self.shelf_geoms = []
        for ztop in SHELF_BOARD_TOPS:
            box = coal.Box(sx, sy, SHELF_BOARD_THICK)
            go = pin.GeometryObject(f"shelf_board_{int(ztop * 100)}", 0, far, box)
            idx = geom.addGeometryObject(go)
            for i in arm_links:
                geom.addCollisionPair(pin.CollisionPair(i, idx))
            self.shelf_geoms.append((idx, ztop))
        self.collision.geom_data = geom.createData()
        self.get_logger().info(
            f"[collision] added {len(self.shelf_geoms)} shelf boards "
            f"(open gaps, avoided by the planner)")

    def update_shelf_collision(self):
        """Place the shelf boards in the model-root (mpo_base_link) frame from the
        live TF, so collision checks use the shelf at the AGV's current pose. Call
        once per cycle (the AGV is stationary during a cycle). Returns False if the
        TF is unavailable (shelf then stays parked away = not enforced)."""
        try:
            tf = self.tf_buffer.lookup_transform(
                'mpo_base_link', self.world_frame, rclpy.time.Time(),
                timeout=Duration(seconds=3.0))
        except Exception as e:
            self.get_logger().warn(f"[shelf] TF unavailable, not enforced: {e}")
            return False
        t, r = tf.transform.translation, tf.transform.rotation
        T_root_world = pin.SE3(quat_to_R(r.x, r.y, r.z, r.w),
                               np.array([t.x, t.y, t.z]))
        sx, sy = SHELF_WORLD_XY
        for idx, ztop in self.shelf_geoms:
            board_world = pin.SE3(np.eye(3),
                                  np.array([sx, sy, ztop - SHELF_BOARD_THICK / 2]))
            self.collision.geom.geometryObjects[idx].placement = (
                T_root_world * board_world)
        self.collision.geom_data = self.collision.geom.createData()
        return True

    def get_ik(self, target_pose: PoseStamped, max_retries=200):
        """Override the base-class IK so move_to_pose/plan_rrt also get the
        nearest, within-limit AND collision-free IK branch (compute_ik_ordered),
        instead of the first within-limit solution which may be self/cube/AGV
        colliding under the loosened limits."""
        return self.compute_ik_ordered(target_pose, max_retries=max_retries)

    def compute_ik_ordered(self, target_pose: PoseStamped, max_retries=200,
                           want_candidates=12, near_attempts=80, near_sigma=0.25,
                           return_all=False):
        """IK returning joints in joint1..joint6 order, within limits (or None).

        Uses the reachability map's OWN inverse_kinematics (self.ik_model) so a
        pose that the map calls reachable is solvable here too -- MoveIt's KDL
        solver disagrees with the map and is not used. target_pose.position is the
        desired gripper TCP (Link6 + TCP_OFFSET along the tool axis), matching the
        map's convention. We seed near the current pose first (nearest IK branch
        -> short motion), then random restarts, gate every solution by the
        combined self/cube/AGV collision model, and return the nearest one.
        With return_all=True, returns the whole candidate list sorted by joint
        distance, so a caller can apply its own branch criterion. NOTE: when the
        start faces away from the goal (shelf behind the base vs pocket in
        front), near-seeding can make EVERY candidate the shoulder-flipped
        branch -- do not rely on candidate diversity to pick a J1 direction;
        the carry computes its J1 swing from the TCP azimuth instead."""
        cur = np.array(self.current_joints) if self.current_joints is not None else None
        lo = np.array([l[0] for l in self.joint_limits])
        hi = np.array([l[1] for l in self.joint_limits])
        p = target_pose.pose.position
        o = target_pose.pose.orientation
        pos = np.array([p.x, p.y, p.z])
        R = quat_to_R(o.x, o.y, o.z, o.w)
        qidx = self.ik_model.q_index
        candidates = []
        n_ik = n_col = 0                  # diagnostics: where do candidates die?
        for attempt in range(max_retries):
            if cur is not None and attempt < near_attempts:
                seed = np.clip(cur + np.random.normal(0, near_sigma, len(cur)), lo, hi)
                seed = self.ik_model.pin_q(seed)
            else:
                seed = self.ik_model.random_config(np.random.default_rng())
            qfull = self.ik_model.inverse_kinematics(pos, R, [seed])
            if qfull is None:
                continue
            n_ik += 1
            q = [float(qfull[i]) for i in qidx]
            # inverse_kinematics already enforces joint limits + arm self-collision;
            # add the cube/AGV collision gate the map does not include.
            if not self.is_state_valid(q):
                continue
            n_col += 1
            candidates.append(q)
            if len(candidates) >= want_candidates:
                break
        if not candidates:
            # Break down the failure: no IK convergence at all (pose truly
            # unreachable / bad orientation per the map's solver), or every
            # solution hit the cube/AGV.
            self.get_logger().error(
                f"[IK] failed after {max_retries} retries: map IK ok={n_ik}, "
                f"collision-free(cube/AGV)={n_col} "
                f"(0 ik -> unreachable per map; ik>0,col=0 -> hits cube/AGV)")
            return None
        if cur is not None:
            candidates.sort(key=lambda q: np.linalg.norm(np.array(q) - cur))
            self.get_logger().info(
                f"[IK] {len(candidates)} candidates; nearest dist="
                f"{np.linalg.norm(np.array(candidates[0]) - cur):.3f}")
        else:
            self.get_logger().info(f"[IK] {len(candidates)} candidates")
        if return_all:
            return candidates
        return candidates[0]

    def _twist_pose_180(self, pose: PoseStamped) -> PoseStamped:
        """Same pose rotated 180 deg about its OWN tool z-axis. A parallel jaw is
        symmetric under this flip (the box footprint is identical), so it is an
        equally valid place orientation -- and its tool z (down) is unchanged, so
        it stays on the same tilt-only constraint manifold."""
        q = pose.pose.orientation
        R2 = quat_to_R(q.x, q.y, q.z, q.w) @ np.diag([-1.0, -1.0, 1.0])
        quat = pin.Quaternion(R2)
        quat.normalize()
        p = pose.pose.position
        return pose_at((p.x, p.y, p.z), (quat.x, quat.y, quat.z, quat.w))

    def move_constrained(self, target_pose: PoseStamped, speed=0.6, time_limit=45.0,
                         yaw_free=True):
        """Move to target_pose holding the tool pointing DOWN (the tilt-only CBiRRT
        constraint keeps tool z fixed the whole way; yaw about the tool axis is
        free). Blocks until execution ends.

        The carry constraint is yaw-free, so the GOAL must be too: solving a single
        fixed-yaw IK pinned J6 and made it wind ~180 deg (grasp yaw + in-gap twist
        vs PLACE_YAW=0), which is what stalled the planner. With yaw_free we take
        the goal config NEAREST the current pose across both box-symmetric place
        orientations (yaw, yaw+180), so J6 does not wind and only the unavoidable
        J1/elbow reconfiguration (~the azimuth gap) remains for CBiRRT to bridge."""
        while self.current_joints is None:
            self.get_logger().info("Waiting for /joint_states...")
            time.sleep(0.5)

        start_q = self.current_joints.tolist()
        start = np.array(start_q)
        poses = [target_pose]
        if yaw_free:
            poses.append(self._twist_pose_180(target_pose))
        cands = []
        for p in poses:
            c = self.compute_ik_ordered(p, return_all=True)
            if c:
                cands.extend(c)
        if not cands:
            self.get_logger().error("[CBiRRT] goal IK failed")
            return False
        goal_q = min(cands, key=lambda q: np.linalg.norm(np.array(q) - start))
        gq = np.array(goal_q)
        self.get_logger().info(
            f"[CBiRRT] goal: nearest of {len(cands)} cand(s), "
            f"joint dist={np.linalg.norm(gq - start):.2f} rad")
        # Diagnostic: WHERE is the gap? J1-dominated -> azimuth/deployment; J6 ->
        # yaw winding; J2-J5 -> elbow/wrist flip (often a collision-forced branch).
        self.get_logger().info(
            "[CBiRRT] start(deg)= " + ",".join(f"{math.degrees(v):+.0f}" for v in start))
        self.get_logger().info(
            "[CBiRRT] goal (deg)= " + ",".join(f"{math.degrees(v):+.0f}" for v in gq))
        self.get_logger().info(
            "[CBiRRT] per-joint gap(deg)= " + ",".join(
                f"J{i+1} {math.degrees(g - s):+.0f}" for i, (g, s) in enumerate(zip(gq, start))))

        # Tilt reference = tool z (down); identical for a pose and its 180-twin, so
        # set_reference from the target is correct regardless of which goal won.
        q = target_pose.pose.orientation
        self.cbirrt.set_reference((q.x, q.y, q.z, q.w))
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

    def move_to_pose_ref(self, target_pose: PoseStamped, ref_q):
        """Like move_to_pose, but choose the goal IK branch NEAREST ref_q (not the
        branch nearest the current pose), then free-RRT to it. Used for the shelf
        pre-grasp so the box is grasped in the SAME elbow/wrist family as the
        pocket place (ref_q = the pocket config). The constrained carry then never
        has to flip the elbow, which is what stalled it. Blocks."""
        while self.current_joints is None:
            self.get_logger().info("Waiting for /joint_states...")
            time.sleep(0.5)
        cands = self.compute_ik_ordered(target_pose, return_all=True)
        if not cands:
            self.get_logger().error("[pick] pre-grasp IK failed")
            return False
        ref = np.array(ref_q)
        goal = min(cands, key=lambda q: np.linalg.norm(np.array(q) - ref))
        self.get_logger().info(
            f"[pick] pre-grasp branch nearest pocket (of {len(cands)}): "
            f"J3={math.degrees(goal[2]):+.0f} J5={math.degrees(goal[4]):+.0f} deg")
        path = self.plan_rrt(self.current_joints, goal)
        if not path:
            self.get_logger().error("[pick] pre-grasp RRT failed")
            return False
        return self.execute_trajectory(path)

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

    def linear_servo(self, delta, speed=0.4, label="move"):
        """Move the EE straight by the 3-vector `delta` (metres, base_link frame),
        holding its current orientation, via a pinocchio Cartesian-Jacobian servo
        seeded at the current pose (no IK branch jump). Stops just before any joint
        reaches its limit, a collision occurs, or the servo goes singular. Blocks.
        A short move is treated as a hard FAILURE: if the servo cannot travel the
        full requested distance we return False (no partial execution) so the
        caller aborts the cycle instead of grasping/placing from the wrong spot.
        The stop cause (singular/limit/collision) is logged for diagnosis."""
        want = float(np.linalg.norm(delta))
        while self.current_joints is None:
            self.get_logger().info("Waiting for /joint_states...")
            time.sleep(0.5)
        start_q = self.current_joints.tolist()
        path, reached, reason = self.cbirrt.linear_path(
            start_q, delta, self.is_state_valid, self.joint_limits)
        if reached < want - 1e-3:
            self.get_logger().error(
                f"[servo] {label}: stopped at {reached*1000:.0f} mm of requested "
                f"{want*1000:.0f} mm -> {reason}; aborting (no partial execution)")
            return False
        self.get_logger().info(f"[servo] {label} {reached*1000:.0f} mm, {len(path)} waypoints")
        return self.execute_path(path, speed=speed)

    def vertical_servo(self, dz, speed=0.4):
        """Straight vertical move by dz metres along base z (sign = direction),
        holding orientation. Thin wrapper over linear_servo."""
        return self.linear_servo([0.0, 0.0, dz], speed=speed,
                                 label=("up" if dz >= 0 else "down"))

    def log_gripper_box_clearance(self, box, row_dir, insert_dir, label="clearance"):
        """Diagnostic: for the gripper/wrist links, compute each collision box's
        lowest corner Z in base_link via live TF and compare it to the box TOP.
        A negative gap means that link's collision dips below the box top (i.e. it
        is what pushes the box). `box` is the box centre in base_link.

        Also checks JAW ALIGNMENT: the finger opens along the gripper X axis
        (finger_joint axis -X), so for a clean grasp gripper-X must line up with the
        box SHORT/graspable axis (row_dir, |half|=0.0405 m), i.e. ~0 deg to row_dir
        and ~90 deg to the long axis (insert_dir, |half|=0.118 m). If gripper-X is
        ~90 deg off (aligned to insert_dir) the jaws straddle the LONG side and
        clip the box. Pure logging, never fails the cycle."""
        # Per-link collision primitives copied from the URDF (link frame):
        # (origin xyz, full size). Mirrors gripper_base_link / gripper_finger_link.
        prims = {
            'gripper_base_link': [
                ((0.0801, 0.0, 0.110), (0.2016, 0.0912, 0.010)),  # fixed-jaw top beam
                ((0.17205, 0.0, 0.080), (0.0177, 0.0912, 0.050)),  # fixed-jaw column+pad
                ((0.00505, 0.0, 0.12105), (0.1315, 0.0912, 0.0121)), # top plate
                ((0.0, 0.0, 0.13355), (0.082, 0.082, 0.013)),     # mount boss (cyl AABB)
            ],
            'gripper_finger_link': [
                ((0.0687, 0.0, 0.0916), (0.027, 0.070, 0.071)),
            ],
        }
        box_top = box[2] + 0.07   # box half-height 0.14/2
        self.get_logger().info(
            f"[{label}] box centre(base)=({box[0]:+.3f},{box[1]:+.3f},{box[2]:+.3f}) "
            f"top_z={box_top:+.3f}")
        for link, boxes in prims.items():
            try:
                tf = self.tf_buffer.lookup_transform(
                    'base_link', link, rclpy.time.Time(),
                    timeout=Duration(seconds=1.0))
            except Exception as e:
                self.get_logger().warn(f"[{label}] TF base_link<-{link} failed: {e}")
                continue
            t = tf.transform.translation
            q = tf.transform.rotation
            # quaternion (x,y,z,w) -> rotation matrix
            x, y, z, w = q.x, q.y, q.z, q.w
            R = np.array([
                [1-2*(y*y+z*z), 2*(x*y-z*w),   2*(x*z+y*w)],
                [2*(x*y+z*w),   1-2*(x*x+z*z), 2*(y*z-x*w)],
                [2*(x*z-y*w),   2*(y*z+x*w),   1-2*(x*x+y*y)]])
            p = np.array([t.x, t.y, t.z])
            link_min_z = math.inf
            for origin, size in boxes:
                o = np.array(origin); h = np.array(size) / 2.0
                # 8 corners in link frame -> base_link, track min Z
                for sx in (-1, 1):
                    for sy in (-1, 1):
                        for sz in (-1, 1):
                            corner = o + h * np.array([sx, sy, sz])
                            wz = (R @ corner + p)[2]
                            link_min_z = min(link_min_z, wz)
            gap = link_min_z - box_top
            flag = "  <-- BELOW box top (hits box!)" if gap < 0 else ""
            self.get_logger().info(
                f"[{label}] {link:20s} lowest collision z={link_min_z:+.3f} "
                f"gap_above_box={gap*1000:+.0f} mm{flag}")

        # --- Jaw alignment vs box axes ---
        try:
            tf = self.tf_buffer.lookup_transform(
                'base_link', 'gripper_base_link', rclpy.time.Time(),
                timeout=Duration(seconds=1.0))
            q = tf.transform.rotation
            x, y, z, w = q.x, q.y, q.z, q.w
            R = np.array([
                [1-2*(y*y+z*z), 2*(x*y-z*w),   2*(x*z+y*w)],
                [2*(x*y+z*w),   1-2*(x*x+z*z), 2*(y*z-x*w)],
                [2*(x*z-y*w),   2*(y*z+x*w),   1-2*(x*x+y*y)]])
            jaw_axis = R[:, 0]   # gripper X = finger open/close direction
            short = np.asarray(row_dir, float); short /= (np.linalg.norm(short) or 1)
            long_ = np.asarray(insert_dir, float); long_ /= (np.linalg.norm(long_) or 1)
            ang_short = math.degrees(math.acos(np.clip(abs(jaw_axis @ short), 0, 1)))
            ang_long = math.degrees(math.acos(np.clip(abs(jaw_axis @ long_), 0, 1)))
            verdict = ("OK (jaw across short/graspable side)" if ang_short < 30
                       else "MISALIGNED ~90deg (jaw across LONG side -> clips box!)"
                       if ang_short > 60 else "PARTIAL")
            self.get_logger().info(
                f"[{label}] jaw-open axis vs box: {ang_short:.0f} deg to SHORT(graspable), "
                f"{ang_long:.0f} deg to LONG -> {verdict}")
        except Exception as e:
            self.get_logger().warn(f"[{label}] jaw-alignment check failed: {e}")

    def gripper_x_in_base(self, timeout=3.0):
        """Live gripper +X axis (fixed-jaw direction) expressed in base_link,
        projected onto the horizontal plane and normalised. Looked up via TF so
        it is correct whichever way rotate_j6 actually twisted. Returns a unit
        np.array, or None if the TF is unavailable or the axis is (near-)vertical
        (gripper not pointing down -- no meaningful lateral direction)."""
        try:
            tf = self.tf_buffer.lookup_transform(
                'base_link', 'gripper_base_link', rclpy.time.Time(),
                timeout=Duration(seconds=timeout))
        except Exception as e:
            self.get_logger().error(f"[TF] gripper_base_link lookup failed: {e}")
            return None
        q = tf.transform.rotation
        jaw_x = quat_to_R(q.x, q.y, q.z, q.w)[:, 0]
        jaw_x[2] = 0.0                      # keep the lateral shift horizontal
        n = np.linalg.norm(jaw_x)
        if n < 0.5:
            self.get_logger().error("[TF] gripper X is near-vertical; not pointing down?")
            return None
        return jaw_x / n

    def rotate_j6(self, angle, speed=0.8, label="yaw"):
        """Twist the gripper yaw by rotating J6 (wrist roll) in place by `angle`
        rad -- a pure single-joint move, NO IK. The other five joints are held, so
        the TCP position stays put and only the jaw azimuth changes. Used once the
        jaw is inside the shelf gap. J6 has +-360 deg of travel so a 90 deg twist
        never hits a limit; the only thing to avoid is the jaw sweeping into the
        shelf, so we collision-check the target and, if it collides, try the
        opposite direction (-angle) instead. Blocks. Returns False only if BOTH
        directions collide."""
        while self.current_joints is None:
            self.get_logger().info("Waiting for /joint_states...")
            time.sleep(0.5)
        start_q = self.current_joints.tolist()
        for a in (angle, -angle):
            target_q = list(start_q)
            target_q[5] = start_q[5] + a
            if self.is_state_valid(target_q):
                self.get_logger().info(f"[{label}] J6 {math.degrees(a):+.0f} deg")
                return self.execute_path([start_q, target_q], speed=speed)
            self.get_logger().warn(
                f"[{label}] J6 {math.degrees(a):+.0f} deg collides; trying other way")
        self.get_logger().error(f"[{label}] J6 twist collides both ways; aborting")
        return False

    def move_single_joint(self, idx, target, speed=0.5, label="joint", n_checks=24):
        """Move ONE joint to an absolute `target` (rad), holding the other five.
        Validity-checks `n_checks` interpolated configs along the sweep (unlike
        rotate_j6, which only checks the endpoint -- a long J1 swing passes near
        the shelf/cube, so the path itself must be clear). Blocks.

        Used for the carry: with the gripper pointing straight DOWN, a J1 move
        (base z) or a J6 move (tool axis) preserves the down orientation EXACTLY,
        so the big J1/J6 part of the shelf->pocket reconfiguration can be done by
        deterministic single-joint moves and the constrained CBiRRT only has to
        close the remaining small J2..J5 gap."""
        while self.current_joints is None:
            self.get_logger().info("Waiting for /joint_states...")
            time.sleep(0.5)
        start_q = self.current_joints.tolist()
        delta = float(target) - start_q[idx]
        if abs(delta) < 1e-3:
            return True
        for i in range(1, n_checks + 1):
            q = list(start_q)
            q[idx] = start_q[idx] + delta * i / n_checks
            if not self.is_state_valid(q):
                self.get_logger().error(
                    f"[{label}] J{idx + 1} sweep collides at "
                    f"{math.degrees(start_q[idx] + delta * i / n_checks):+.0f} deg "
                    f"({i}/{n_checks}); aborting")
                return False
        target_q = list(start_q)
        target_q[idx] = float(target)
        self.get_logger().info(f"[{label}] J{idx + 1} {math.degrees(delta):+.0f} deg")
        return self.execute_path([start_q, target_q], speed=speed)


# ----------------------------------------------------------------------------
# Workspace constants for the shelf-to-base sequence. base_link metres unless
# noted. Many heights/clearances are first guesses to be tuned in the simulator.
# ----------------------------------------------------------------------------

# Gripper jaw geometry, MEASURED from the Blender meshes (base.dae/finger.dae),
# in the gripper_base_link frame (flange face at z=0.1401, mounted flipped on
# Link6, fixed jaw on +X). The grasp centre between the pads is ~123 mm off the
# flange/tool-z axis (longer jaw), so the flange must NOT be centred over the box -- see the
# jaw-align step (2c) in the cycle.
#   finger joint axis -X: q > 0 opens;  pad gap = JAW_GAP_AT_ZERO + q
JAW_FIXED_PAD_X = 0.1632     # fixed pad inner face (gripper x), measured from base.dae pad_fixed (Xmin)
JAW_MOVING_PAD_X0 = 0.0822   # moving pad inner face at q=0, measured from finger.dae pad_moving (Xmax)
JAW_GAP_AT_ZERO = JAW_FIXED_PAD_X - JAW_MOVING_PAD_X0   # 81.0 mm (== BOX_SHORT -> close q=0)
PAD_BOTTOM_BELOW_FLANGE = 0.0821   # pad lower edge, metres below the flange face

BOX_SHORT = 0.081            # box graspable width (short side)

# Clearance between the fixed pad and the box face during the descend; closing
# then pushes the box this far sideways until it rests against the fixed pad.
FIXED_PAD_CLEARANCE = 0.003  # TUNE IN SIM (smaller = less push at close)

GRIPPER_OPEN = [0.03]        # gap 111 mm; after jaw-align the moving pad still
                             # clears the box face by ~19 mm on the descend, and
                             # the shorter close sweep hits the box at ~12 mm/s
                             # instead of ~32 mm/s (0.07), which the contact
                             # solver tolerates much better
CLOSE_SQUEEZE = 0.00 #0.002        # close this much past the box width: real pad pressure,
                             # so friction holds the box even if the link attacher
                             # misses (gap == box width has ZERO grip force)
GRIPPER_CLOSE = [BOX_SHORT - JAW_GAP_AT_ZERO - CLOSE_SQUEEZE]   # 0.0 (gap == box width at q=0)
                             # (the old -0.036 was for the pre-refit URDF with the
                             # opposite joint axis; here it would crush 44 mm in)

# Shelf pick target (Task 5): WORLD frame (from cr.world). Picked the box on the
# 2nd shelf board (top z=0.90) because its centre (0.97) is the closest to the arm
# base height (world z=0.690) -- the higher tier-3 box (1.47) is near the reach
# edge. LEFT box = smaller world x. First cut does this single box; the other
# three (box_l1b, box_l2a/b) are for the later 4-box extension.
SHELF_BOX_WORLD = (0.7095, 0.5, 0.97)   # box_l1a centre (board top 0.90 + 0.07)
SHELF_BOX_MODEL = 'box_l1a'             # Gazebo model name (for the link attacher)
SHELF_BOX_LINK = 'box_link'

# Base magazine pockets (Task 6): CONSTANT in base_link (rigid to the arm base),
# so no TF needed. 0.236 m along base_link x, 0.081 m along y, 11.8 cm y-pitch.
POCKET_X = 0.3705
POCKET_Y = [0.177, 0.059, -0.059, -0.177]
POCKET_SURFACE_Z = -0.05                # rear-half top surface height in base_link

# Orientation. DOWN = gripper straight down (known-good). The grasp yaw is built
# at runtime from the shelf row direction (world x) so the jaw aligns with the
# inter-magazine gap; GRASP_YAW_OFFSET absorbs the fixed Link6->jaw azimuth and
# is the main thing to tune in sim. PLACE_YAW orients the box in the pocket.
DOWN = (0.707, 0.707, 0.0, 0.0)
GRASP_YAW_OFFSET = 0.0   # rad, TUNE IN SIM
PLACE_YAW = 0.0          # rad about base z for the place orientation, TUNE IN SIM
# In-gap wrist twist (step 2b): pure J6 roll, no IK. +90 deg by default; flip to
# -90 (negate) if the jaw twists the wrong way in sim.
GRIPPER_YAW_TWIST = math.radians(90)   # rad, J6 in-place rotation; sign TUNE IN SIM

# Heights / clearances. These are TCP heights (Link6 + 0.12005 along tool z),
# matching the IK-target convention -- keep them low so Link6 stays inside the
# arm's reach. NOTE: 0.12005 is the OnRobot 2FG7 value; the Blender gripper's
# pads bottom out only 0.0821 m below the flange, i.e. 38 mm ABOVE the TCP. So
# the pad wrap depth below the box top is (0.07 = box half height):
#   wrap = 0.07 - (0.12005 - PAD_BOTTOM_BELOW_FLANGE) - GRASP_TCP_ABOVE
#        = 0.032 - GRASP_TCP_ABOVE
# (TUNE IN SIM: raise GRASP_TCP_ABOVE to descend less.)
GRASP_TCP_ABOVE = 0.015    # TCP above box centre at grasp -> pads wrap ~17 mm
                           # (0.005/27 mm clipped the box and ejected it)
INSERT_TCP_ABOVE = 0.105   # TCP above box centre while travelling inside the gap.
                           # ABSOLUTE (relative to the BOX, not to GRASP_TCP_ABOVE)
                           # so tuning the grasp depth does not shift the insert
                           # corridor: it is tight against both the shelf gap and
                           # the arm's reach (10 mm higher already went singular at
                           # the end of the 250 mm insert). The descend/ascend
                           # distance is derived: INSERT_TCP_ABOVE - GRASP_TCP_ABOVE.
PREGRASP_BACK = 0.25       # start this far in front of the shelf (along -insertion);
                           # keep it large enough that the pre-grasp (RRT goal) is
                           # OUTSIDE the shelf, so the RRT never routes through the
                           # shelf -- only the straight insert servo enters it
POCKET_HOVER = 0.18        # tip height above the pocket surface to hover before placing
PLACE_TCP_ABOVE = 0.08     # TCP above the pocket surface at release. The box
                           # bottom hangs 0.075 below the TCP (centre = TCP -
                           # GRASP_TCP_ABOVE, half height 0.07), so 0.08 drops
                           # the box from 5 mm; 0.06 would press it 15 mm into
                           # the surface while still attached.
# Folded standby pose: used both at startup and after each cycle, so the arm
# stays tucked low until a trigger moves it out to the shelf. J3 kept at -105
# (not -115): past ~-112 the gripper folds into the magazine (cube rear block)
# and the pose self-collides, which would make it an invalid RRT start.
STANDBY_POSE_DEG = [-8, -39, -105, 0, 0, 0]


def pose_at(xyz, quat):
    """PoseStamped in base_link at position xyz with orientation quat (xyzw)."""
    p = PoseStamped()
    p.header.frame_id = "base_link"
    p.pose.position.x, p.pose.position.y, p.pose.position.z = (
        float(xyz[0]), float(xyz[1]), float(xyz[2]))
    (p.pose.orientation.x, p.pose.orientation.y,
     p.pose.orientation.z, p.pose.orientation.w) = (float(v) for v in quat)
    return p


def shelf_to_base_cycle(node, box_world, pocket_y):
    """Run one shelf-to-base pick-and-place. Returns True on success, False (with
    a clear log) on any IK/plan/servo failure so the caller can ask the user to
    reposition the AGV and trigger again. Steps follow the agreed sequence."""
    # Resolve the box pose and the shelf axes in base_link via live TF.
    box_ps = node.transform_world_pose(*box_world, DOWN)
    insert_dir = node.transform_world_vector([0.0, 1.0, 0.0])   # world +y = into shelf
    row_dir = node.transform_world_vector([1.0, 0.0, 0.0])      # world +x = magazine row
    if box_ps is None or insert_dir is None or row_dir is None:
        node.get_logger().error("[cycle] TF unavailable; reposition and retry")
        return False
    insert_dir = insert_dir / (np.linalg.norm(insert_dir) or 1.0)
    box = np.array([box_ps.pose.position.x, box_ps.pose.position.y,
                    box_ps.pose.position.z])

    # Position the shelf in the collision model at the AGV's current pose so the
    # RRT routes around it (the AGV is stationary for the rest of the cycle).
    node.update_shelf_collision()

    # Grasp orientation: down + yaw that aligns the jaw to the magazine row.
    phi = math.atan2(row_dir[1], row_dir[0]) + GRASP_YAW_OFFSET
    grasp_quat = quat_mul(quat_about_z(phi), DOWN)
    place_quat = quat_mul(quat_about_z(PLACE_YAW), DOWN)

    pregrasp_xyz = box - insert_dir * PREGRASP_BACK + np.array([0, 0, INSERT_TCP_ABOVE])
    descend_dist = INSERT_TCP_ABOVE - GRASP_TCP_ABOVE   # gap height -> grasp height
    pocket_hover_xyz = np.array([POCKET_X, pocket_y-0.05, POCKET_SURFACE_Z + POCKET_HOVER]) # pocket no.4
    # pocket_hover_xyz = np.array([POCKET_X, pocket_y-0.17, POCKET_SURFACE_Z + POCKET_HOVER]) # pocket no.3

    # Show exactly what we are aiming at (box/pre-grasp in base_link + the shelf
    # axes), so an IK failure can be read against the arm's actual reach.
    node.get_logger().info(
        f"[cycle] box(base_link)=({box[0]:+.3f},{box[1]:+.3f},{box[2]:+.3f}) "
        f"horiz_dist={math.hypot(box[0], box[1]):.3f} m | "
        f"pre-grasp=({pregrasp_xyz[0]:+.3f},{pregrasp_xyz[1]:+.3f},{pregrasp_xyz[2]:+.3f}) | "
        f"insert_dir=({insert_dir[0]:+.2f},{insert_dir[1]:+.2f},{insert_dir[2]:+.2f}) "
        f"row_dir=({row_dir[0]:+.2f},{row_dir[1]:+.2f},{row_dir[2]:+.2f}) yaw={math.degrees(phi):.0f}deg")

    # Pocket place config, computed UP FRONT so the pick can grasp in the same
    # elbow/wrist family. The high shelf (elbow-up) and low pocket (elbow-down)
    # are opposite branches; grasping in the pocket's branch keeps the constrained
    # carry within one family (no elbow flip for CBiRRT to bridge). Seeded from the
    # current (standby, J3~-105) pose so this is the elbow-down pocket branch.
    place_ref = node.compute_ik_ordered(pose_at(pocket_hover_xyz, place_quat))
    if place_ref is None:
        node.get_logger().error("[cycle] pocket place IK failed (pre-check)"); return False
    node.get_logger().info(
        f"[cycle] pocket place branch: J3={math.degrees(place_ref[2]):+.0f} "
        f"J5={math.degrees(place_ref[4]):+.0f} deg (pick will match it)")

    # 1. RRT to the pre-grasp pose in front of the shelf box, in the pocket's
    # elbow/wrist family (move_to_pose_ref picks the branch nearest place_ref).
    print("\n===== [1/10] RRT -> pre-grasp in front of shelf =====")
    node.control_gripper(GRIPPER_OPEN)
    if not node.move_to_pose_ref(pose_at(pregrasp_xyz, grasp_quat), place_ref):
        node.get_logger().error("[cycle] step 1 pre-grasp failed"); return False

    # 2. Linear insert into the gap (over the box), fixed jaw entering the gap.
    print("===== [2/10] Linear insert into shelf gap =====")
    if not node.linear_servo(insert_dir * PREGRASP_BACK, label="insert"):
        node.get_logger().error("[cycle] step 2 insert failed"); return False

    # 2b. Twist the gripper yaw via J6 (no IK) now that the jaw is in the gap.
    print("===== [2b/10] Twist gripper yaw (J6) =====")
    if not node.rotate_j6(GRIPPER_YAW_TWIST, label="yaw-twist"):
        node.get_logger().error("[cycle] step 2b yaw twist failed"); return False

    # 2c. Jaw-align: the grasp centre between the pads is ~53 mm toward the fixed
    # jaw (+gripper X) from the flange axis, so shift the flange AWAY from the
    # fixed-jaw side until the fixed pad is FIXED_PAD_CLEARANCE from the box
    # face. Done AFTER the twist, along the live (TF) jaw axis, so it stays
    # correct even when rotate_j6 had to flip the twist direction.
    print("===== [2c/10] Jaw-align (fixed pad to box face) =====")
    time.sleep(0.3)   # let TF catch up with the finished J6 move
    jaw_x = node.gripper_x_in_base()
    if jaw_x is None:
        node.get_logger().error("[cycle] step 2c jaw axis unavailable"); return False
    lateral = JAW_FIXED_PAD_X - FIXED_PAD_CLEARANCE - BOX_SHORT / 2.0   # ~46 mm
    if not node.linear_servo(-lateral * jaw_x, label="jaw-align"):
        node.get_logger().error("[cycle] step 2c jaw align failed"); return False

    # 3. Linear descend onto the box.
    print("===== [3/10] Linear descend onto box =====")
    # Diagnostic: which gripper/wrist link (if any) sits below the box top now,
    # and is the jaw azimuth aligned to the box's graspable (short) side?
    node.log_gripper_box_clearance(box, row_dir, insert_dir, label="pre-descend")
    if not node.linear_servo([0.0, 0.0, -descend_dist+0.01], label="descend"):
        node.get_logger().error("[cycle] step 3 descend failed"); return False

    # 4. Close gripper + attach. The attach result is the load-bearing part:
    # without the attacher joint the squeeze-only grip slips during the carry,
    # so a failed attach aborts the cycle instead of dropping the box later.
    print("===== [4/10] Grip + attach =====")
    node.control_gripper(GRIPPER_CLOSE)
    node.object_model, node.object_link = SHELF_BOX_MODEL, SHELF_BOX_LINK
    if not node.attach_box():
        node.get_logger().error(
            "[cycle] step 4 ATTACHLINK failed (check model/link names and the "
            "link-attacher plugin); releasing and aborting")
        node.control_gripper(GRIPPER_OPEN)
        return False
    time.sleep(0.5)

    # 5. Linear ascend back to gap height.
    print("===== [5/10] Linear ascend =====")
    if not node.linear_servo([0.0, 0.0, descend_dist], label="ascend"):
        node.get_logger().error("[cycle] step 5 ascend failed"); return False

    # 6. Linear retreat out of the shelf.
    print("===== [6/10] Linear retreat out of shelf =====")
    if not node.linear_servo(-insert_dir * PREGRASP_BACK, label="retreat"):
        node.get_logger().error("[cycle] step 6 retreat failed"); return False

    # 7. Carry to hover above the base pocket, gripper held DOWN the whole way.
    # Carry on the tilt-only (tool-down) CBiRRT manifold straight to the hover
    # pose. The earlier 5-rad stall was NOT the azimuth/elbow gap (that is only
    # ~126 deg, which CBiRRT bridges easily) but J6 winding from a fixed-yaw goal:
    # the grasp yaw + in-gap twist left J6 ~180 deg from a PLACE_YAW=0 goal.
    # move_constrained(yaw_free=True) now picks the goal NEAREST the current pose
    # across both box-symmetric place yaws, so the wrist does not unwind and the
    # tool stays pointing down the whole way (verified: 206 deg -> 126 deg carry).
    print("===== [7/10] Carry (gripper held down) -> hover above pocket =====")
    hover_pose = pose_at(pocket_hover_xyz, place_quat)
    if not node.move_constrained(hover_pose):
        node.get_logger().error("[cycle] step 7 carry to pocket failed"); return False

    # 8. Linear descend toward the pocket surface.
    print("===== [8/10] Linear descend into pocket =====")
    if not node.linear_servo([0.0, 0.0, PLACE_TCP_ABOVE - POCKET_HOVER+0.01],
                             label="place-descend"):
        node.get_logger().error("[cycle] step 8 place descend failed"); return False

    # 9. Open gripper + detach.
    print("===== [9/10] Release + detach =====")
    node.detach_box()
    node.control_gripper(GRIPPER_OPEN)
    time.sleep(0.5)

    # 10. Retreat up a little (no fold -- folding the arm destabilised the AGV).
    print("===== [10/10] Retreat =====")
    node.vertical_servo(0.10)
    print("========== Cycle done ==========\n")
    return True


def main(args=None):
    rclpy.init(args=args)
    node = CBiRRTPickPlace()
    node.setup_planner()

    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2)  # wait for joint states

    # No startup fold: the arm stays at its spawn pose (folding the arm was
    # destabilising the AGV). The first trigger plans straight from here.

    print("\n" + "=" * 60)
    print(" Shelf-to-base pick & place ready.")
    print(" Drive the AGV near the shelf, then press SPACE to run one cycle.")
    print(" (q / Esc / Ctrl-C to quit)")
    print("=" * 60)

    try:
        while rclpy.ok():
            if wait_for_spacebar() == 'quit':
                break
            ok = shelf_to_base_cycle(node, SHELF_BOX_WORLD, POCKET_Y[0])
            if ok:
                print("\n>>> Cycle SUCCEEDED. Press SPACE for another, or q to quit.")
            else:
                print("\n>>> Cycle FAILED (no IK at this AGV position).")
                print("    The arm faces the AGV's REAR (magazine side). Park so the")
                print("    AGV's rear faces the shelf, then drive until the logged")
                print("    box(base_link) is roughly (+0.40~0.50, ~0.00) -- i.e. the")
                print("    box in FRONT of the arm, not off to the -Y side. Then SPACE.")
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()