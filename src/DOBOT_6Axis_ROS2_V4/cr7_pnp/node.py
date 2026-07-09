"""CR7 pick-and-place ROS2 node: IK, collision, motion primitives.

Merges the runtime parts of three former scripts into one reusable node, with
NO dependency on test_w_gripper / cbirrt_pick_place / constrained_cbirrt /
reachability_map (their needed code is copied into this package):

  CR7Node           -- ROS2 plumbing (joint states, gripper, link attacher,
                       trajectory action, joint-space RRT). The MoveIt IK /
                       validity service clients are dropped: they were overridden
                       by the pinocchio engine below and went unused.
  CBiRRTPickPlace   -- pinocchio IK/collision (ReachabilityModel) + CBiRRT and
                       Cartesian-servo primitives (ConstrainedPlanner): IK,
                       collision check, linear_servo, rotate_j6, move_single_joint,
                       move_constrained, shelf collision boards.
  HubPickPlace      -- hub routing, carried-box collision phantom, forward-path
                       recording + reverse-replay, spoke planning/pre-flight.

A sequence script (shelf, device A/B/C/D, ...) instantiates HubPickPlace and
composes these primitives; it does NOT subclass per sequence. Linear-only
sequences simply skip the CBiRRT spoke helpers and use linear_servo.
"""

import math, os
import time

import numpy as np
import pinocchio as pin
import rclpy
from rclpy.node import Node
from rclpy.callback_groups import ReentrantCallbackGroup
from rclpy.action import ActionClient
from rclpy.duration import Duration

from sensor_msgs.msg import JointState
from control_msgs.action import FollowJointTrajectory
from trajectory_msgs.msg import JointTrajectoryPoint
from geometry_msgs.msg import PoseStamped
from linkattacher_msgs.srv import AttachLink, DetachLink
from tf2_ros import Buffer, TransformListener
import tf2_geometry_msgs  # noqa: F401  (registers PoseStamped with tf2)

from .model import ReachabilityModel
from .cbirrt import ConstrainedPlanner
from .geometry import (
    XACRO_PATH, COMBINED_XACRO, quat_to_R, pose_at, DOWN,
    SHELF_WORLD_XY, SHELF_BOARD_TOPS, SHELF_FOOTPRINT, SHELF_BOARD_THICK,
    BOX_SIZE, BOX_IN_LINK6_XYZ, MAGAZINE_LINK, GRASP_LATERAL_M,
)


class RRTNode:
    def __init__(self, joints):
        self.joints = np.array(joints)
        self.parent = None


class CR7Node(Node):
    """ROS2 plumbing: joint states, gripper, link attacher, trajectory action,
    joint-space RRT. IK and collision are provided by the subclass (pinocchio),
    so the MoveIt /compute_ik and /check_state_validity clients are not created."""

    def __init__(self):
        super().__init__('cr7_rrt_planner')
        self.cb_group = ReentrantCallbackGroup()

        # State monitoring (Subscriber)
        self.current_joints = None
        self.joint_names = []
        self.sub_joint_states = self.create_subscription(
            JointState, '/joint_states', self.joint_state_callback, 10,
            callback_group=self.cb_group)

        # Trajectory action client
        self.traj_action_client = ActionClient(
            self, FollowJointTrajectory,
            '/cr7_group_controller/follow_joint_trajectory',
            callback_group=self.cb_group)

        # Gripper
        self.gripper_client = ActionClient(
            self, FollowJointTrajectory, '/gripper_controller/follow_joint_trajectory')

        # Link attacher (grasp fix): attach/detach the box to the gripper
        self.attach_client = self.create_client(
            AttachLink, '/ATTACHLINK', callback_group=self.cb_group)
        self.detach_client = self.create_client(
            DetachLink, '/DETACHLINK', callback_group=self.cb_group)
        # Robot/box names as known to Gazebo
        self.robot_model = 'cr7_on_mpo700'
        self.gripper_link = 'gripper_base_link'
        self.object_model = 'pick_box'
        self.object_link = 'box_link'

        self.joint_limits = [
            (math.radians(-101), math.radians(10)),  # Joint 1
            (math.radians(-70), math.radians(60)),   # Joint 2
            (-math.pi, math.pi),                     # Joint 3
            (math.radians(0), math.radians(120)),    # Joint 4
            (math.radians(-120), math.radians(120)), # Joint 5
            (-math.pi, math.pi)                      # Joint 6
        ]

        self.get_logger().info("CR7 RRT Planner Node Initialized. Waiting for joint states...")

    def joint_state_callback(self, msg):
        """Extract only the 6-axis arm joints from /joint_states in the correct order."""
        target_names = ['joint1', 'joint2', 'joint3', 'joint4', 'joint5', 'joint6']
        if all(name in msg.name for name in target_names):
            filtered_names = []
            filtered_positions = []
            for name in target_names:
                idx = msg.name.index(name)
                filtered_names.append(name)
                filtered_positions.append(msg.position[idx])
            self.joint_names = filtered_names
            self.current_joints = np.array(filtered_positions)

    def control_gripper(self, positions):
        if not self.gripper_client.wait_for_server(timeout_sec=5.0):
            self.get_logger().error("Gripper action server not available")
            return False

        goal_msg = FollowJointTrajectory.Goal()
        goal_msg.trajectory.joint_names = ['gripper_finger_joint']
        point = JointTrajectoryPoint()
        point.positions = [float(positions[0])]
        point.velocities = [0.0]
        point.time_from_start.sec = 2
        goal_msg.trajectory.points.append(point)

        send_goal_future = self.gripper_client.send_goal_async(goal_msg)
        while rclpy.ok() and not send_goal_future.done():
            time.sleep(0.01)

        goal_handle = send_goal_future.result()
        if not goal_handle.accepted:
            self.get_logger().error("Gripper goal rejected")
            return False

        get_result_future = goal_handle.get_result_async()
        while rclpy.ok() and not get_result_future.done():
            time.sleep(0.01)
        self.get_logger().info(f"Gripper moved to positions: {positions}")
        return True

    def attach_box(self):
        """Fix the box to the gripper (grasp) via the link attacher service."""
        if not self.attach_client.wait_for_service(timeout_sec=5.0):
            self.get_logger().error("ATTACHLINK service not available")
            return False
        req = AttachLink.Request()
        req.model1_name = self.robot_model
        req.link1_name = self.gripper_link
        req.model2_name = self.object_model
        req.link2_name = self.object_link
        future = self.attach_client.call_async(req)
        while rclpy.ok() and not future.done():
            time.sleep(0.01)
        self.get_logger().info(f"Attach: {future.result().message}")
        return future.result().success

    def detach_box(self):
        """Release the box from the gripper via the link attacher service."""
        if not self.detach_client.wait_for_service(timeout_sec=5.0):
            self.get_logger().error("DETACHLINK service not available")
            return False
        req = DetachLink.Request()
        req.model1_name = self.robot_model
        req.link1_name = self.gripper_link
        req.model2_name = self.object_model
        req.link2_name = self.object_link
        future = self.detach_client.call_async(req)
        while rclpy.ok() and not future.done():
            time.sleep(0.01)
        self.get_logger().info(f"Detach: {future.result().message}")
        return future.result().success

    def is_within_limits(self, joints, limits):
        """Check whether the joint angles are within the configured limits."""
        for j_val, limit in zip(joints, limits):
            if not (limit[0] <= j_val <= limit[1]):
                return False
        return True

    def plan_rrt(self, start_joints, goal_joints, max_iter=6000, step_size=0.5):
        """Simple RRT path planning with continuous collision checking and joint limits."""
        self.get_logger().info("Starting RRT Planning...")

        self.get_logger().info("Checking if start state is valid...")
        if not self.is_state_valid(start_joints.tolist()):
            self.get_logger().error("Planning failed: Start state is in collision!")
            return None

        self.get_logger().info("Checking if goal state is valid...")
        if not self.is_state_valid(goal_joints):
            self.get_logger().error("Planning failed: Goal state is in collision!")
            return None

        self.get_logger().info("Start and goal states are valid. Beginning tree expansion...")

        tree = [RRTNode(start_joints)]
        goal_node = RRTNode(goal_joints)

        for i in range(max_iter):
            if i % 500 == 0 and i > 0:
                self.get_logger().info(
                    f"RRT exploring.. ({i}/{max_iter}) | current tree nodes: {len(tree)}")

            # 1. Random Sample
            if np.random.rand() < 0.1:
                q_rand = goal_node.joints
            else:
                q_rand = np.array([np.random.uniform(limit[0], limit[1])
                                   for limit in self.joint_limits])

            # 2. Nearest Node
            nearest_node = min(tree, key=lambda node: np.linalg.norm(node.joints - q_rand))

            # 3. Steer
            direction = q_rand - nearest_node.joints
            distance = np.linalg.norm(direction)
            if distance > step_size:
                q_new_joints = nearest_node.joints + (direction / distance) * step_size
                actual_distance = step_size
            else:
                q_new_joints = q_rand
                actual_distance = distance

            # 4. Joint Limit Check
            if not self.is_within_limits(q_new_joints.tolist(), self.joint_limits):
                continue

            # 5. Continuous Collision Check (interpolated between nodes)
            is_collision_free = True
            check_resolution = 0.1
            num_checks = max(1, int(actual_distance / check_resolution))
            for step in range(1, num_checks + 1):
                interp_ratio = step / num_checks
                interp_joints = nearest_node.joints + (q_new_joints - nearest_node.joints) * interp_ratio
                if not self.is_state_valid(interp_joints.tolist()):
                    is_collision_free = False
                    break

            # 6. Add to the tree only if all midpoints were safe
            if is_collision_free:
                new_node = RRTNode(q_new_joints)
                new_node.parent = nearest_node
                tree.append(new_node)
                if np.linalg.norm(new_node.joints - goal_node.joints) < step_size:
                    self.get_logger().info(f"Path found in {i} iterations!")
                    goal_node.parent = new_node
                    tree.append(goal_node)
                    return self.extract_path(goal_node)

        self.get_logger().warn("RRT Planning Failed: Max iterations reached.")
        return None

    def extract_path(self, end_node):
        path = []
        curr = end_node
        while curr is not None:
            path.append(curr.joints.tolist())
            curr = curr.parent
        return path[::-1]

    def _wait_settled(self, target, tol=0.02, timeout=4.0):
        """Block until the REAL joints reach `target` (max abs error < tol rad).
        The trajectory controller returns the action result on the command
        SCHEDULE (goal position tolerances are off in ros2_controllers.yaml), while
        the Gazebo joints trail the interpolated command by up to ~2 s -- so a
        caller reading current_joints right after "finished" gets mid-motion
        values. Called at the end of every executor so completion means ARRIVED."""
        target = np.array(target, dtype=float)
        t0 = time.time()
        while rclpy.ok() and time.time() - t0 < timeout:
            if (self.current_joints is not None and
                    np.abs(self.current_joints - target).max() < tol):
                return True
            time.sleep(0.02)
        err = (np.abs(self.current_joints - target).max()
               if self.current_joints is not None else float('nan'))
        self.get_logger().warn(
            f"[settle] joints not at the last waypoint after {timeout}s "
            f"(max err {err:.3f} rad); continuing")
        return False

    def execute_trajectory(self, path):
        """Send the planned path to the action server and wait until execution completes."""
        # Record joint waypoints while capturing so a forward motion (incl. RRT /
        # go_to_config moves, which route here rather than execute_path) can be
        # replayed in REVERSE -- retracing the proven path instead of re-planning a
        # fresh RRT that (unaware of the base box) may sweep through it.
        if getattr(self, '_recording', False) and path:
            if (self._recorded and
                    np.linalg.norm(np.array(self._recorded[-1]) - np.array(path[0])) < 1e-6):
                self._recorded.extend([list(map(float, p)) for p in path[1:]])
            else:
                self._recorded.extend([list(map(float, p)) for p in path])
        self.get_logger().info("Executing Trajectory...")
        goal_msg = FollowJointTrajectory.Goal()
        goal_msg.trajectory.joint_names = self.joint_names

        time_from_start = 0.0
        for joints in path:
            point = JointTrajectoryPoint()
            point.positions = joints
            time_from_start += 0.5  # 0.5 s interval between each waypoint
            point.time_from_start.sec = int(time_from_start)
            point.time_from_start.nanosec = int((time_from_start % 1) * 1e9)
            goal_msg.trajectory.points.append(point)

        self.traj_action_client.wait_for_server()
        send_goal_future = self.traj_action_client.send_goal_async(goal_msg)
        while rclpy.ok() and not send_goal_future.done():
            time.sleep(0.01)
        self.get_logger().info("Trajectory Sent!")
        self.get_logger().info(f"Final executed trajectory node count: {len(path)}")

        goal_handle = send_goal_future.result()
        if not goal_handle.accepted:
            self.get_logger().error("Trajectory goal rejected")
            return False

        get_result_future = goal_handle.get_result_async()
        while rclpy.ok() and not get_result_future.done():
            time.sleep(0.01)
        self._wait_settled(path[-1])
        self.get_logger().info("Trajectory execution finished.")
        return True


class CBiRRTPickPlace(CR7Node):
    """CR7 node + helpers to run CBiRRT-planned, orientation-constrained motions."""

    def setup_planner(self):
        # Attach the box to Link6, not gripper_base_link: gripper_attach_joint is
        # FIXED, so the URDF->SDF conversion lumps gripper_base_link into Link6
        # and the Gazebo model has no link by that name -- ATTACHLINK then fails
        # with "Failed to find link". Link6 exists (child of revolute joint6) and
        # is rigid with the gripper, so the attachment is equivalent.
        self.gripper_link = 'Link6'

        # Open every joint to the URDF hardware limit (+-6.27 rad). The base-class
        # values are conservative SOFTWARE clamps that block otherwise-valid
        # elbow/wrist branches. Self/scene collision is still enforced by the
        # pinocchio model below, so widening to hardware cannot fold the arm into
        # itself or the AGV/cube undetected.
        HW = 6.27   # URDF hardware joint limit, radians (~359 deg)
        self.joint_limits = [(-HW, HW)] * len(self.joint_limits)

        self.cbirrt = ConstrainedPlanner(xacro_path=XACRO_PATH)
        # pinocchio collision model of the WHOLE robot (arm + cube + AGV +
        # gripper). All validity checks below go through this.
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
        # the map" == "IK solvable here". Targets are the gripper TCP (Link6 +
        # TCP_OFFSET_M along the tool axis).
        self.ik_model = ReachabilityModel(xacro_path=XACRO_PATH)
        self.ik_model.set_joint_limits(self.joint_limits)

        # Add the (world-fixed) shelf to the collision model so the RRT avoids it.
        self._add_shelf_boards()

        # TF: the shelf boxes are fixed in the Gazebo world; the AGV may be driven
        # anywhere before triggering. There is no 'world' frame in the TF tree --
        # the AGV spawns at the world origin, so 'odom' is the world proxy.
        self.world_frame = 'odom'
        self.tf_buffer = Buffer()
        self.tf_listener = TransformListener(self.tf_buffer, self)

    def transform_world_pose(self, x, y, z, quat_xyzw, timeout=3.0):
        """Transform a pose given in the world (== odom) frame into base_link via
        the live TF tree. Returns a PoseStamped in base_link, or None."""
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
        only, no translation). Returns np.array or None."""
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
        """Collision check via the pinocchio whole-robot model. Returns True if the
        6-arm-joint config is free of self-, cube/AGV- and shelf-collisions."""
        return self.collision.is_collision_free(list(joint_positions))

    def _add_shelf_boards(self):
        """Add the shelf boards as thin boxes to the collision model, paired
        against every movable arm/gripper link. They start parked far away and are
        positioned per-cycle by update_shelf_collision()."""
        import coal
        geom = self.collision.geom
        objs = geom.geometryObjects
        # Exclude the carried-box phantom, as in add_wirebonder_meshes: its pairs
        # must toggle with attach/detach_box_collision, not stay always-on.
        box_idx = getattr(self, '_box_geom_idx', None)
        arm_links = [i for i in range(len(objs))
                     if objs[i].parentJoint != 0 and i != box_idx]
        far = pin.SE3(np.eye(3), np.array([0.0, 0.0, -100.0]))
        sx, sy = SHELF_FOOTPRINT
        self.shelf_geoms = []
        for ztop in SHELF_BOARD_TOPS:
            box = coal.Box(sx, sy, SHELF_BOARD_THICK)
            go = pin.GeometryObject(f"shelf_board_{int(ztop * 100)}", 0, far, box)
            idx = geom.addGeometryObject(go)
            for i in arm_links:
                geom.addCollisionPair(pin.CollisionPair(i, idx))
            if box_idx is not None:
                cp = pin.CollisionPair(box_idx, idx)
                self._box_pairs.append(cp)
                if self._box_attached_model:
                    geom.addCollisionPair(cp)
            self.shelf_geoms.append((idx, ztop))
        self.collision.geom_data = geom.createData()
        self.get_logger().info(
            f"[collision] added {len(self.shelf_geoms)} shelf boards "
            f"(open gaps, avoided by the planner)")

    def update_shelf_collision(self):
        """Place the shelf boards in the model-root (mpo_base_link) frame from the
        live TF. Call once per cycle. Returns False if the TF is unavailable."""
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

    def add_wirebonder_meshes(self, stl_dir):
        """Load the wirebonder per-part collision STLs into the collision model,
        paired against every movable arm/gripper link, so the RRT avoids the device
        BODY. The parts are authored to keep the slot recesses OPEN, so a front-load
        insert into a slot stays valid -- only the bulk is blocked. Parked far until
        update_wirebonder_collision() places them at the live device pose. Call once
        after setup_planner(); no-op-safe to call again (re-adds)."""
        import glob
        import coal
        geom = self.collision.geom
        objs = geom.geometryObjects
        # Movable arm links EXCLUDING the carried-box phantom: pairing the phantom
        # here makes (carried_box, wb_*) ALWAYS-ON -- attach/detach_box_collision
        # only toggles _box_pairs, built before these geoms existed -- so an EMPTY
        # gripper near the device false-collides on the phantom. Route the phantom
        # pairs through _box_pairs instead, so they toggle with the carry state.
        box_idx = getattr(self, '_box_geom_idx', None)
        arm_links = [i for i in range(len(objs))
                     if objs[i].parentJoint != 0 and i != box_idx]
        far = pin.SE3(np.eye(3), np.array([0.0, 0.0, -100.0]))
        loader = coal.MeshLoader()
        self.wirebonder_geoms = []
        for f in sorted(glob.glob(os.path.join(stl_dir, '*.stl'))):
            mesh = loader.load(f, np.array([1.0, 1.0, 1.0]))
            name = 'wb_' + os.path.splitext(os.path.basename(f))[0]
            go = pin.GeometryObject(name, 0, far, mesh)
            idx = geom.addGeometryObject(go)
            for i in arm_links:
                geom.addCollisionPair(pin.CollisionPair(i, idx))
            if box_idx is not None:
                cp = pin.CollisionPair(box_idx, idx)
                self._box_pairs.append(cp)
                if self._box_attached_model:
                    geom.addCollisionPair(cp)
            self.wirebonder_geoms.append(idx)
        self.collision.geom_data = geom.createData()
        self.get_logger().info(
            f"[collision] added wirebonder body ({len(self.wirebonder_geoms)} parts, "
            f"slot recesses open)")

    def update_wirebonder_collision(self, device_pose):
        """Place the wirebonder collision parts in the model root (mpo_base_link)
        from the live device world pose (x, y, z, yaw in odom). The STL verts are
        baked in the device MODEL frame, so mapping model->world(device)->root puts
        the body where the arm sees it. Call once the device pose is known (after the
        vision capture / before a transfer). Returns False if the TF is unavailable."""
        try:
            tf = self.tf_buffer.lookup_transform(
                'mpo_base_link', self.world_frame, rclpy.time.Time(),
                timeout=Duration(seconds=3.0))
        except Exception as e:
            self.get_logger().warn(f"[wirebonder] TF unavailable, not enforced: {e}")
            return False
        t, r = tf.transform.translation, tf.transform.rotation
        T_root_world = pin.SE3(quat_to_R(r.x, r.y, r.z, r.w),
                               np.array([t.x, t.y, t.z]))
        x, y, z, yaw = device_pose
        c, s = math.cos(yaw), math.sin(yaw)
        T_world_dev = pin.SE3(np.array([[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]]),
                              np.array([x, y, z]))
        for idx in self.wirebonder_geoms:
            self.collision.geom.geometryObjects[idx].placement = (
                T_root_world * T_world_dev)
        self.collision.geom_data = self.collision.geom.createData()
        return True

    def get_ik(self, target_pose: PoseStamped, max_retries=200):
        """IK via the pinocchio reachability model (nearest, within-limit,
        collision-free branch). Used by plan_rrt-based movers."""
        return self.compute_ik_ordered(target_pose, max_retries=max_retries)

    def compute_ik_ordered(self, target_pose: PoseStamped, max_retries=200,
                           want_candidates=12, near_attempts=80, near_sigma=0.25,
                           return_all=False):
        """IK returning joints in joint1..joint6 order, within limits (or None).

        Uses the reachability map's OWN inverse_kinematics (self.ik_model) so a
        pose the map calls reachable is solvable here too. target_pose.position is
        the desired gripper TCP. We seed near the current pose first (nearest IK
        branch -> short motion), then random restarts, gate every solution by the
        combined self/cube/AGV collision model, and return the nearest one. With
        return_all=True, returns the whole candidate list sorted by joint distance."""
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
            if not self.is_state_valid(q):
                continue
            n_col += 1
            candidates.append(q)
            if len(candidates) >= want_candidates:
                break
        if not candidates:
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
        symmetric under this flip, so it is an equally valid place orientation --
        and its tool z (down) is unchanged, so it stays on the tilt-only manifold."""
        q = pose.pose.orientation
        R2 = quat_to_R(q.x, q.y, q.z, q.w) @ np.diag([-1.0, -1.0, 1.0])
        quat = pin.Quaternion(R2)
        quat.normalize()
        p = pose.pose.position
        return pose_at((p.x, p.y, p.z), (quat.x, quat.y, quat.z, quat.w))

    def move_constrained(self, target_pose: PoseStamped, speed=0.6, time_limit=45.0,
                         yaw_free=True):
        """Move to target_pose holding the tool pointing DOWN (tilt-only CBiRRT
        constraint; yaw about the tool axis free). Blocks until execution ends.

        With yaw_free we take the goal config NEAREST the current pose across both
        box-symmetric place orientations (yaw, yaw+180), so J6 does not wind and
        only the unavoidable J1/elbow reconfiguration remains for CBiRRT."""
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
        self.get_logger().info(
            "[CBiRRT] start(deg)= " + ",".join(f"{math.degrees(v):+.0f}" for v in start))
        self.get_logger().info(
            "[CBiRRT] goal (deg)= " + ",".join(f"{math.degrees(v):+.0f}" for v in gq))
        self.get_logger().info(
            "[CBiRRT] per-joint gap(deg)= " + ",".join(
                f"J{i+1} {math.degrees(g - s):+.0f}" for i, (g, s) in enumerate(zip(gq, start))))

        # Tilt reference = tool z (down); identical for a pose and its 180-twin.
        q = target_pose.pose.orientation
        self.cbirrt.set_reference((q.x, q.y, q.z, q.w))
        self.get_logger().info("[CBiRRT] planning (orientation held)...")
        path = self.cbirrt.plan(start_q, goal_q, self.is_state_valid, self.joint_limits,
                                time_limit=time_limit)
        if not path:
            self.get_logger().error("[CBiRRT] planning failed")
            return False

        if np.linalg.norm(np.array(path[0]) - np.array(start_q)) > 1e-3:
            path = [start_q] + path
        self.get_logger().info(f"[CBiRRT] path: {len(path)} waypoints")
        return self.execute_path(path, speed=speed)

    def move_to_pose_ref(self, target_pose: PoseStamped, ref_q):
        """Like move_to_pose, but choose the goal IK branch NEAREST ref_q (not the
        branch nearest the current pose), then free-RRT to it. Blocks."""
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
        self._wait_settled(path[-1])
        self.get_logger().info("[execute_path] done")
        return True

    def linear_servo(self, delta, speed=0.4, label="move"):
        """Move the EE straight by the 3-vector `delta` (metres, base_link frame),
        holding its current orientation, via a pinocchio Cartesian-Jacobian servo
        seeded at the current pose. A short move is a hard FAILURE (return False),
        so the caller aborts the cycle instead of grasping/placing from the wrong
        spot. The stop cause (singular/limit/collision) is logged."""
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
            bad = getattr(self.cbirrt, 'last_invalid_q', None)
            if reason == "collision" and bad is not None:
                self.get_logger().error(
                    f"[servo] {label}: colliding pairs: "
                    f"{self.collision.colliding_pairs(bad)}")
            return False
        self.get_logger().info(f"[servo] {label} {reached*1000:.0f} mm, {len(path)} waypoints")
        return self.execute_path(path, speed=speed)

    def guarded_descend(self, max_drop, label="descend", speed=0.1):
        """Descend straight down until the carried-box collision model meets a
        surface -- the SIM analog of the real robot's joint-torque touch-off (see
        place_command_guide.md). Commands an over-travel drop (max_drop m, tool-
        down) with the box phantom ON and, UNLIKE linear_servo, executes the
        linear_path only UP TO where it reports the collision -- so the box seats
        at each surface's TRUE height, absorbing per-pocket height and carried-box
        offset variance that a single fixed drop can't. The box-vs-surface pair is
        the same one that used to false-reject place IK; here it is the sensor.
        Requires the phantom attached. Returns the metres actually descended."""
        while self.current_joints is None:
            self.get_logger().info("Waiting for /joint_states...")
            time.sleep(0.5)
        path, reached, reason = self.cbirrt.linear_path(
            self.current_joints.tolist(), np.array([0.0, 0.0, -abs(max_drop)]),
            self.is_state_valid, self.joint_limits)
        if len(path) > 1:
            self.execute_path(path, speed=speed)
        self.get_logger().info(
            f"[guarded] {label}: descended {reached*1000:.0f} of {abs(max_drop)*1000:.0f} mm max, "
            f"stop={reason} -> {'CONTACT (seated)' if reason == 'collision' else 'NO CONTACT'}")
        bad = getattr(self.cbirrt, 'last_invalid_q', None)
        if reason == "collision" and bad is not None:
            self.get_logger().info(
                f"[guarded] {label}: stopped by pairs: {self.collision.colliding_pairs(bad)}")
        return reached

    def gripper_x_in_base(self, timeout=3.0):
        """Live gripper +X axis (fixed-jaw direction) in base_link, projected
        horizontal and normalised. Looked up via TF. Returns a unit np.array or None."""
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
        rad -- a pure single-joint move, NO IK. If the target collides, try the
        opposite direction (-angle). Returns False only if BOTH directions collide."""
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
        Validity-checks n_checks interpolated configs along the sweep. Blocks.

        With the gripper pointing straight DOWN, a J1 move (base z) or a J6 move
        (tool axis) preserves the down orientation EXACTLY."""
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


class HubPickPlace(CBiRRTPickPlace):
    """CBiRRTPickPlace + hub routing, box-attached collision, and reverse-replay.

    This is THE reusable pick-and-place node: every sequence (shelf, device
    A/B/C/D) instantiates it and composes its primitives. Linear-only sequences
    use linear_servo and skip the CBiRRT spoke helpers."""

    def setup_planner(self):
        super().setup_planner()
        self.hub_q = None
        self.box_idx = 0            # next object to handle (sequence-managed)
        self._recording = False
        self._recorded = []
        self._box_attached_model = False
        self._compute_gripper_x_offset()   # cache _gx_in_link6 before the phantom
        self._add_attached_box()

    # --- jaw-axis prediction (pre-flight has no live TF for the planned pose) ---

    def _compute_gripper_x_offset(self):
        """Cache the gripper +X axis (fixed-jaw direction) in the Link6 frame, so
        pre-flight can PREDICT the jaw-align direction by FK of a simulated config
        instead of needing the arm to actually sit there for a TF lookup."""
        m, d = self.collision.model, self.collision.data
        fid_l6 = m.getFrameId('Link6')
        fid_g = m.getFrameId('gripper_base_link')
        if fid_l6 >= m.nframes or fid_g >= m.nframes:
            self._gx_in_link6 = None
            self.get_logger().warn("[pre-flight] gripper frame not found; "
                                   "jaw-align will not be pre-validated")
            return
        pin.forwardKinematics(m, d, pin.neutral(m))
        pin.updateFramePlacements(m, d)
        self._gx_in_link6 = d.oMf[fid_l6].rotation.T @ d.oMf[fid_g].rotation[:, 0]

    def gripper_x_in_base_fk(self, config):
        """Predict gripper_x_in_base() for an arbitrary arm config via FK,
        projected horizontal and normalised. Returns a unit np.array, or None."""
        if getattr(self, '_gx_in_link6', None) is None:
            return None
        m, d = self.ik_model.model, self.ik_model.data
        pin.forwardKinematics(m, d, self.ik_model.pin_q(config))
        pin.updateFramePlacement(m, d, self.ik_model.frame_id)
        gx = (d.oMf[self.ik_model.frame_id].rotation @ self._gx_in_link6).copy()
        gx[2] = 0.0
        n = float(np.linalg.norm(gx))
        return gx / n if n >= 0.5 else None

    # --- carried-box collision phantom (toggled on only while carrying) ---

    def _add_attached_box(self):
        """Create the carried-box phantom, rigidly parented to Link6's joint.
        Collision pairs are NOT added here; attach/detach_box_collision() toggle
        the box vs every geometry NOT rigid to the gripper."""
        import coal
        geom = self.collision.geom
        model = self.collision.model
        frame = model.frames[model.getFrameId('Link6')]
        self._box_parent_joint = frame.parentJoint
        # The grasped box hangs GRASP_LATERAL_M off the tool axis toward the fixed
        # jaw (grasp_tcp_pose shifts the TCP by the same amount). Place the phantom
        # there too -- on-axis (the old (0,0,z)) put it ~46 mm off the real box and
        # tripped false cube/AGV collisions. _gx_in_link6 = gripper +x in Link6.
        gx = getattr(self, '_gx_in_link6', None)
        box_xyz = np.array(BOX_IN_LINK6_XYZ, dtype=float)
        if gx is not None:
            box_xyz = box_xyz + GRASP_LATERAL_M * np.asarray(gx, dtype=float)
        placement = frame.placement * pin.SE3(np.eye(3), box_xyz)
        go = pin.GeometryObject('carried_box', self._box_parent_joint,
                                placement, coal.Box(*BOX_SIZE))
        self._box_geom_idx = geom.addGeometryObject(go)
        objs = geom.geometryObjects
        self._box_pairs = [
            pin.CollisionPair(self._box_geom_idx, i)
            for i in range(len(objs))
            if i != self._box_geom_idx
            and objs[i].parentJoint != self._box_parent_joint
        ]
        self.collision.geom_data = geom.createData()

    def attach_box_collision(self):
        """Enable the carried-box phantom in the collision model."""
        if self._box_attached_model:
            return
        for cp in self._box_pairs:
            self.collision.geom.addCollisionPair(cp)
        self.collision.geom_data = self.collision.geom.createData()
        self._box_attached_model = True

    def detach_box_collision(self):
        """Disable the carried-box phantom (back to the bare-gripper model)."""
        if not self._box_attached_model:
            return
        for cp in self._box_pairs:
            self.collision.geom.removeCollisionPair(cp)
        self.collision.geom_data = self.collision.geom.createData()
        self._box_attached_model = False

    def attach_box_to_magazine(self):
        """Fix the just-placed box (self.object_model/object_link) to the AGV
        magazine link so it rides rigidly with the base when the AGV drives.
        Returns the service success flag."""
        if not self.attach_client.wait_for_service(timeout_sec=5.0):
            self.get_logger().error("ATTACHLINK service not available")
            return False
        req = AttachLink.Request()
        req.model1_name = self.robot_model
        req.link1_name = MAGAZINE_LINK
        req.model2_name = self.object_model
        req.link2_name = self.object_link
        future = self.attach_client.call_async(req)
        while rclpy.ok() and not future.done():
            time.sleep(0.01)
        res = future.result()
        self.get_logger().info(f"[magazine] attach {self.object_model}: {res.message}")
        return res.success

    # --- recording + reverse-replay ---

    def execute_path(self, path, speed=0.6):
        """Base execute_path, plus: while recording, append the executed joint
        waypoints so the forward motion can be replayed in reverse to the hub."""
        if self._recording and path:
            if (self._recorded and
                    np.linalg.norm(np.array(self._recorded[-1]) -
                                   np.array(path[0])) < 1e-6):
                self._recorded.extend([list(map(float, p)) for p in path[1:]])
            else:
                self._recorded.extend([list(map(float, p)) for p in path])
        return super().execute_path(path, speed=speed)

    def _start_recording(self):
        self._recorded = []
        self._recording = True

    def _stop_recording(self):
        self._recording = False
        return list(self._recorded)

    def replay_reverse(self, path, speed=0.6):
        """Replay a recorded forward path in reverse to retrace it to the hub."""
        if not path:
            return True
        return super().execute_path(self.rev(path), speed=speed)

    def capture(self, fn):
        """Run a motion call while recording; return (ok, executed_path)."""
        self._start_recording()
        ok = fn()
        return ok, self._stop_recording()

    @staticmethod
    def rev(path):
        """A path reversed (deep float copy)."""
        return [list(map(float, q)) for q in path[::-1]]

    @staticmethod
    def offset_j6(path, d):
        """Copy a path with J6 shifted by d rad on every waypoint. Shifting J6
        leaves the TCP position unchanged, so the arm follows the same line but
        keeps the grasped box in its picked orientation."""
        out = []
        for q in path:
            q = list(map(float, q))
            q[5] += d
            out.append(q)
        return out

    @staticmethod
    def join(parts):
        """Concatenate joint paths, dropping a duplicated junction waypoint."""
        out = []
        for p in parts:
            p = [list(map(float, q)) for q in p]
            if not p:
                continue
            if out and np.linalg.norm(np.array(out[-1]) - np.array(p[0])) < 1e-6:
                out.extend(p[1:])
            else:
                out.extend(p)
        return out

    # --- IK / spoke planning helpers ---

    def ik_nearest(self, target_pose, ref_q):
        """IK solution for target_pose whose config is NEAREST ref_q, so a goal
        can be pinned to a chosen elbow/wrist family. Returns a config or None."""
        cands = self.compute_ik_ordered(target_pose, return_all=True)
        if not cands:
            return None
        ref = np.array(ref_q)
        return min(cands, key=lambda q: np.linalg.norm(np.array(q) - ref))

    def plan_spoke(self, start_q, goal_pose, ref_q, time_limit=10.0, label="spoke"):
        """Plan a tool-down CBiRRT spoke from start_q to goal_pose, choosing the
        goal IK branch nearest ref_q. Validity uses whatever collision model is
        active (box-attached during pre-flight). Returns waypoints or None."""
        goal_q = self.ik_nearest(goal_pose, ref_q)
        if goal_q is None:
            self.get_logger().error(f"[{label}] goal IK failed")
            return None
        self.cbirrt.set_reference(DOWN)
        self.get_logger().info(
            f"[{label}] CBiRRT planning (tool-down, up to {time_limit:.0f}s)...")
        path = self.cbirrt.plan(list(start_q), list(goal_q), self.is_state_valid,
                                self.joint_limits, time_limit=time_limit)
        if not path:
            self.get_logger().error(f"[{label}] CBiRRT planning failed")
            return None
        return path

    def preflight_linear(self, start_q, delta, label):
        """Pre-flight a Cartesian servo WITHOUT moving: run the same straight-line
        solver linear_servo uses and check it reaches the full distance. Returns
        the end config (to chain the next servo) or None if it would stall short."""
        path, reached, reason = self.cbirrt.linear_path(
            list(start_q), list(delta), self.is_state_valid, self.joint_limits)
        want = float(np.linalg.norm(delta))
        if reached < want - 1e-3:
            self.get_logger().error(
                f"[pre-flight] {label} servo infeasible: reaches "
                f"{reached * 1000:.0f} of {want * 1000:.0f} mm -> {reason}")
            return None
        return path[-1]

    # --- hub bring-up ---

    def init_hub(self, ref_pose, hub_tcp, lateral_m):
        """Compute the hub config once. ref_pose: a place-family reference pose
        (e.g. above the pocket centre) used to seed the elbow/wrist branch AND to
        supply the standby orientation. hub_tcp: (x,y,z) where the CARRIED OBJECT
        should hover at standby. lateral_m: how far the carried object hangs off
        the tool axis (toward the fixed jaw). Requires the hub valid with the
        carried box attached. Returns True on success.

        # ponytail: ref_pose couples the hub to the place geometry of the calling
        # sequence; that is intentional -- each sequence passes its own reference.
        """
        while self.current_joints is None:
            self.get_logger().info("Waiting for /joint_states...")
            time.sleep(0.5)
        self.detach_box_collision()
        place_ref = self.compute_ik_ordered(ref_pose)
        if place_ref is None:
            self.get_logger().error("[hub] reference IK failed")
            return False
        self.attach_box_collision()
        # Offset the flange target by the carried-object lateral hang so the OBJECT
        # -- not the flange -- sits at the nominal hub_tcp.
        place_jaw_x = self.gripper_x_in_base_fk(place_ref)
        if place_jaw_x is None:
            self.detach_box_collision()
            self.get_logger().error("[hub] gripper FK unavailable for hub offset")
            return False
        hub_xyz = np.array(hub_tcp) - lateral_m * place_jaw_x
        o = ref_pose.pose.orientation
        ref_quat = (o.x, o.y, o.z, o.w)
        # Hub uses the reference orientation so the standby fixed-jaw direction
        # matches the place. Still tool-down -> same CBiRRT manifold.
        hub_q = self.ik_nearest(pose_at(hub_xyz, ref_quat), place_ref)
        self.detach_box_collision()
        if hub_q is None:
            self.get_logger().error("[hub] hub_tcp IK failed with the carried box "
                                    "attached; raise hub_tcp z or pull x toward place")
            return False
        self.hub_q = hub_q
        return True

    def go_to_hub(self, speed=0.6):
        """Move from the spawn pose to the hub via a free joint-space RRT (once at
        start-up; the spawn pose is not tool-down). Cycles then start at the hub."""
        while self.current_joints is None:
            self.get_logger().info("Waiting for /joint_states...")
            time.sleep(0.5)
        if np.linalg.norm(self.current_joints - np.array(self.hub_q)) < 1e-2:
            return True
        path = self.plan_rrt(self.current_joints, list(self.hub_q))
        if not path:
            self.get_logger().error("[hub] RRT to hub failed")
            return False
        return self.execute_trajectory(path)

    def go_to_config(self, joints, speed=0.6):
        """Free joint-space RRT to an arbitrary joint config, then execute. Blocks.
        Smooth and singularity-free (a long Cartesian servo jitters/aborts near a
        singularity); use for far poses reached by jogging. True/False."""
        while self.current_joints is None:
            self.get_logger().info("Waiting for /joint_states...")
            time.sleep(0.5)
        if np.linalg.norm(self.current_joints - np.array(joints)) < 1e-2:
            return True
        path = self.plan_rrt(self.current_joints, list(joints))
        if not path:
            self.get_logger().error("[go_to_config] RRT failed")
            return False
        return self.execute_trajectory(path)

    def joint_move(self, joints, max_step=0.5):
        """Direct joint-space move (like a robot MoveJ): straight-line interpolation
        from the current config to `joints`, NO RRT. Shortest joint path. Inherently
        singularity-free -- it never inverts the Jacobian (no Cartesian IK like
        linear_servo), so joints just sweep linearly. UNLIKE a real MoveJ, it does NO
        collision checking. Steps match plan_rrt's resolution so timing matches an RRT
        move. ponytail: assumes the straight joint path is collision-free -- only use
        where that's known (e.g. the capture view-B jog). True/False."""
        while self.current_joints is None:
            self.get_logger().info("Waiting for /joint_states...")
            time.sleep(0.5)
        start = np.array(self.current_joints, float)
        delta = np.array(joints, float) - start
        if np.linalg.norm(delta) < 1e-2:
            return True
        n = max(2, int(np.ceil(np.abs(delta).max() / max_step)) + 1)
        path = [list(start + delta * t) for t in np.linspace(0.0, 1.0, n)]
        return self.execute_trajectory(path)
