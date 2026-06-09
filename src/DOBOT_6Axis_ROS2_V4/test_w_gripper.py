import rclpy
from rclpy.node import Node
from rclpy.callback_groups import ReentrantCallbackGroup
from rclpy.executors import MultiThreadedExecutor
from rclpy.action import ActionClient

import math, os
import random
import numpy as np
import time

# ROS2 Messages & Services
from sensor_msgs.msg import JointState
from moveit_msgs.srv import GetPositionIK, GetStateValidity
from control_msgs.action import FollowJointTrajectory
from trajectory_msgs.msg import JointTrajectoryPoint
from geometry_msgs.msg import PoseStamped
from moveit_msgs.msg import RobotState
from linkattacher_msgs.srv import AttachLink, DetachLink

class RRTNode:
    def __init__(self, joints):
        self.joints = np.array(joints)
        self.parent = None

class CR7RRTPlanner(Node):
    def __init__(self):
        super().__init__('cr7_rrt_planner')

        # Callback group setup (for synchronous service calls)
        self.cb_group = ReentrantCallbackGroup()

        # 1. State monitoring (Subscriber)
        self.current_joints = None
        self.joint_names = []
        self.sub_joint_states = self.create_subscription(
            JointState, '/joint_states', self.joint_state_callback, 10, callback_group=self.cb_group)

        # 2. IK service (Client)
        self.ik_client = self.create_client(
            GetPositionIK, '/compute_ik', callback_group=self.cb_group)

        # 3. Collision check service (Client)
        self.validity_client = self.create_client(
            GetStateValidity, '/check_state_validity', callback_group=self.cb_group)

        # 4. Action client (Action)
        self.traj_action_client = ActionClient(
            self, FollowJointTrajectory, '/cr7_group_controller/follow_joint_trajectory', callback_group=self.cb_group)
        
        # 5. gripper
        self.gripper_client = ActionClient(self, FollowJointTrajectory, '/gripper_controller/follow_joint_trajectory')

        # 6. link attacher (grasp fix): attach/detach the box to the gripper
        self.attach_client = self.create_client(AttachLink, '/ATTACHLINK', callback_group=self.cb_group)
        self.detach_client = self.create_client(DetachLink, '/DETACHLINK', callback_group=self.cb_group)
        # Robot/box names as known to Gazebo
        self.robot_model   = 'cr7_on_mpo700'
        self.gripper_link  = 'gripper_base_link'
        self.object_model  = 'pick_box'
        self.object_link   = 'box_link'



        # Robot default settings (adjust to match the environment)
        self.group_name = "cr7_group" # Planning Group name configured in MoveIt
        self.ik_timeout = 0.05

        self.joint_limits = [
            (math.radians(-101), math.radians(10)),  # Joint 1
            (math.radians(-70), math.radians(60)),   # Joint 2
            (-math.pi, math.pi),                     # Joint 3
            (math.radians(0), math.radians(120)),#(-math.pi, math.pi),                     # Joint 4
            (math.radians(-120), math.radians(120)),#(-math.pi, math.pi),                     # Joint 5
            (-math.pi, math.pi)                      # Joint 6
        ]

        self.get_logger().info("CR7 RRT Planner Node Initialized. Waiting for joint states...")

    def joint_state_callback(self, msg):
        """ Extract only the 6-axis arm joints from /joint_states in the correct order """
        # Exact names and order of the robot arm joints
        target_names = ['joint1', 'joint2', 'joint3', 'joint4', 'joint5', 'joint6']

        # Check that all 6 joints are present in the message
        if all(name in msg.name for name in target_names):
            filtered_names = []
            filtered_positions = []

            # Sort and extract joints 1 through 6 in order
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
        req.link1_name  = self.gripper_link
        req.model2_name = self.object_model
        req.link2_name  = self.object_link
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
        req.link1_name  = self.gripper_link
        req.model2_name = self.object_model
        req.link2_name  = self.object_link
        future = self.detach_client.call_async(req)
        while rclpy.ok() and not future.done():
            time.sleep(0.01)
        self.get_logger().info(f"Detach: {future.result().message}")
        return future.result().success

    # --- [Service call wrapper functions] ---


    def get_ik(self, target_pose: PoseStamped, max_retries=100):
        """Retry until the target pose is converted into joint angles within the limits"""
        req = GetPositionIK.Request()
        req.ik_request.group_name = self.group_name
        req.ik_request.pose_stamped = target_pose
        req.ik_request.timeout.sec = 1

        for attempt in range(max_retries):
            # 1. Set the seed (hint)
            seed_state = RobotState()
            seed_state.joint_state.name = self.joint_names

            if attempt == 0:
                # First attempt: find a solution close to the current robot pose
                seed_state.joint_state.position = self.current_joints.tolist()
            else:
                # If it failed, generate a 'random seed (hint)' within our configured limits and retry!
                random_seed = [random.uniform(limit[0], limit[1]) for limit in self.joint_limits]
                seed_state.joint_state.position = random_seed

            req.ik_request.robot_state = seed_state

            # 2. Request IK computation
            future = self.ik_client.call_async(req)
            while rclpy.ok() and not future.done():
                time.sleep(0.01)

            res = future.result()

            # 3. Validate the result
            if res.error_code.val == res.error_code.SUCCESS:
                ik_solution = list(res.solution.joint_state.position)

                # Verify the computed solution is really within our limits
                if self.is_within_limits(ik_solution, self.joint_limits):
                    self.get_logger().info(f"[Attempt {attempt+1}] Found a valid IK target within the limits!")
                    return ik_solution
                else:
                    self.get_logger().warn(f"[Attempt {attempt+1}] Found an IK solution but it is outside the limits. Discarding and retrying.")
            else:
                self.get_logger().warn(f"[Attempt {attempt+1}] No solution could be found with this seed.")

        self.get_logger().error(f"Failed to find an IK solution within the limits after {max_retries} retries.")
        return None

    def is_within_limits(self, joints, limits):
        """Check whether the joint angles are within the configured limits"""
        for j_val, limit in zip(joints, limits):
            if not (limit[0] <= j_val <= limit[1]):
                return False
        return True

    def is_state_valid(self, joint_positions):
        """Check if the given joint configuration is collision-free."""
        if not self.validity_client.wait_for_service(timeout_sec=2.0):
            self.get_logger().error("[Timeout] Service /check_state_validity is not available!")
            return False

        req = GetStateValidity.Request()
        req.group_name = self.group_name

        robot_state = RobotState()
        # [CRITICAL FIX 3] Tell MoveIt this is a partial (diff) state update
        robot_state.is_diff = True
        robot_state.joint_state.header.frame_id = "base_link" # Re-check this matches the actual robot base
        robot_state.joint_state.header.stamp = self.get_clock().now().to_msg()

        safe_length = len(joint_positions)
        robot_state.joint_state.name = self.joint_names[:safe_length]
        robot_state.joint_state.position = joint_positions

        req.robot_state = robot_state

        # # --- [DEBUG LOGS] Inspect the data right before sending ---
        # self.get_logger().info(f"--- [DEBUG] Validity Request ---")
        # self.get_logger().info(f"Group: {req.group_name}")
        # self.get_logger().info(f"Joint Names ({len(robot_state.joint_state.name)}): {robot_state.joint_state.name}")
        # self.get_logger().info(f"Joint Positions ({len(robot_state.joint_state.position)}): {robot_state.joint_state.position}")
        # self.get_logger().info(f"--------------------------------")

        future = self.validity_client.call_async(req)

        timeout = 5.0
        start_time = time.time()

        while rclpy.ok() and not future.done():
            if time.time() - start_time > timeout:
                self.get_logger().error("[Timeout] MoveIt ignored the request. Check the MoveIt/RViz terminal!")
                return False
            time.sleep(0.005)

        res = future.result()
        if res is None:
            self.get_logger().error("Validity check returned None.")
            return False

        return res.valid

    

    def plan_rrt(self, start_joints, goal_joints, max_iter=6000, step_size=0.5):
        """Simple RRT path planning with continuous collision checking and joint limits."""
        self.get_logger().info("Starting RRT Planning...")

        # Pre-check: Ensure start and goal states are valid
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
            # Print progress every 500 iterations
            if i % 500 == 0 and i > 0:
                self.get_logger().info(f"RRT exploring.. ({i}/{max_iter}) | current tree nodes: {len(tree)}")

            # 1. Random Sample (now uses self.joint_limits declared in __init__)
            if random.random() < 0.1:
                q_rand = goal_node.joints
            else:
                q_rand = np.array([random.uniform(limit[0], limit[1]) for limit in self.joint_limits])

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

            # 4. Joint Limit Check (check whether the newly extended branch crossed the limit)
            if not self.is_within_limits(q_new_joints.tolist(), self.joint_limits):
                continue

            # 5. Continuous Collision Check (interpolated check between nodes - prevents tunneling through obstacles!)
            is_collision_free = True
            check_resolution = 0.1  # Check at 0.1 radian intervals
            num_checks = max(1, int(actual_distance / check_resolution))

            for step in range(1, num_checks + 1):
                interp_ratio = step / num_checks
                interp_joints = nearest_node.joints + (q_new_joints - nearest_node.joints) * interp_ratio

                # Discard the branch if any interpolated midpoint is in collision
                if not self.is_state_valid(interp_joints.tolist()):
                    is_collision_free = False
                    break

            # 6. Add to the tree only if all midpoints were safe
            if is_collision_free:
                new_node = RRTNode(q_new_joints)
                new_node.parent = nearest_node
                tree.append(new_node)

                # Goal Reached Check
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
        return path[::-1] # Reverse order

    # --- [Trajectory execution] ---

    def execute_trajectory(self, path):
        """Send the planned path to the action server and wait until execution completes"""
        self.get_logger().info("Executing Trajectory...")

        # # Save every waypoint's joint angles (deg) to a txt file: one node per line, J1..J6.
        # filename = f"rrt_path_{time.strftime('%Y%m%d')}.txt"
        # with open(filename, "a") as f:
        #     for joints in path:
        #         degs = [math.degrees(j) for j in joints]
        #         f.write(", ".join(f"{d:.4f}" for d in degs) + "\n")
        # self.get_logger().info(f"Saved {len(path)} path nodes (deg) to {filename}")

        goal_msg = FollowJointTrajectory.Goal()
        goal_msg.trajectory.joint_names = self.joint_names

        time_from_start = 0.0
        for joints in path:
            point = JointTrajectoryPoint()
            point.positions = joints
            time_from_start += 0.5 # 0.5 s interval between each waypoint (refine with smoothing if needed)
            point.time_from_start.sec = int(time_from_start)
            point.time_from_start.nanosec = int((time_from_start % 1) * 1e9)
            goal_msg.trajectory.points.append(point)

        self.traj_action_client.wait_for_server()
        send_goal_future = self.traj_action_client.send_goal_async(goal_msg)
        while rclpy.ok() and not send_goal_future.done():
            time.sleep(0.01)
        self.get_logger().info("Trajectory Sent!")
        # Number of waypoints (nodes) in the final executed trajectory
        self.get_logger().info(f"Final executed trajectory node count: {len(path)}")

        goal_handle = send_goal_future.result()
        if not goal_handle.accepted:
            self.get_logger().error("Trajectory goal rejected")
            return False

        # Wait until the controller finishes executing the trajectory
        get_result_future = goal_handle.get_result_async()
        while rclpy.ok() and not get_result_future.done():
            time.sleep(0.01)
        self.get_logger().info("Trajectory execution finished.")
        return True

    # --- [Main driver function] ---

    def move_to_pose(self, target_pose: PoseStamped):
        # 1. Wait until joint states arrive
        while self.current_joints is None:
            self.get_logger().info("Waiting for /joint_states...")
            time.sleep(1.0)

        # 2. Compute the goal joints via IK
        goal_joints = self.get_ik(target_pose)
        if not goal_joints:
            return False

        # 3. RRT planning
        path = self.plan_rrt(self.current_joints, goal_joints)
        if not path:
            return False

        # 4. Execute the trajectory and wait until it finishes
        return self.execute_trajectory(path)

    def move_to_joint_pose(self, joint_positions_deg, duration_sec=3):
        """Move the arm directly to the given joint angles (degrees) and wait for completion"""
        joint_positions_rad = [math.radians(deg) for deg in joint_positions_deg]

        goal_msg = FollowJointTrajectory.Goal()
        goal_msg.trajectory.joint_names = self.joint_names
        point = JointTrajectoryPoint()
        point.positions = joint_positions_rad
        point.time_from_start.sec = duration_sec
        goal_msg.trajectory.points.append(point)

        self.get_logger().info(f"Moving to joint pose (deg): {joint_positions_deg}")

        self.traj_action_client.wait_for_server()
        send_goal_future = self.traj_action_client.send_goal_async(goal_msg)
        while rclpy.ok() and not send_goal_future.done():
            time.sleep(0.01)

        goal_handle = send_goal_future.result()
        if not goal_handle.accepted:
            self.get_logger().error("Joint pose goal rejected")
            return False

        get_result_future = goal_handle.get_result_async()
        while rclpy.ok() and not get_result_future.done():
            time.sleep(0.01)
        self.get_logger().info("Reached target pose")
        return True

    def move_to_coordinate(self, x, y, z, target):
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
            pose.pose.orientation.x = target.pose.orientation.x
            pose.pose.orientation.y = target.pose.orientation.y
            pose.pose.orientation.z = target.pose.orientation.z
            pose.pose.orientation.w = target.pose.orientation.w
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

            self.traj_action_client.wait_for_server()
            goal_msg = FollowJointTrajectory.Goal()
            goal_msg.trajectory.joint_names = joint_names
            point = JointTrajectoryPoint()
            point.positions = target_positions
            point.time_from_start.sec = 3
            goal_msg.trajectory.points.append(point)
            
            send_goal_future = self.traj_action_client.send_goal_async(goal_msg)
            rclpy.spin_until_future_complete(self, send_goal_future)
            
            goal_handle = send_goal_future.result()
            if not goal_handle.accepted:
                return False
                
            get_result_future = goal_handle.get_result_async()
            rclpy.spin_until_future_complete(self, get_result_future)
            return True

def main(args=None):
    rclpy.init(args=args)
    node = CR7RRTPlanner()

    # Multi-threaded execution (required for synchronous service waits)
    executor = MultiThreadedExecutor()
    executor.add_node(node)

    # Run the ROS loop in a background thread
    import threading
    spin_thread = threading.Thread(target=executor.spin, daemon=True)
    spin_thread.start()

    # --- [Test execution part] ---
    time.sleep(2) # Wait for node initialization

    # Set the desired target pose (in the RViz coordinate frame)
    target = PoseStamped()
    target.header.frame_id = "base_link" # Check the robot base link name
    target.pose.position.x = 0.4#0.4
    target.pose.position.y = 0.0
    target.pose.position.z = 0.3#0.2
    # target.pose.orientation.w = 1.0 # Default rotation (quaternion)

    target.pose.orientation.x = 0.707#0.0
    target.pose.orientation.y = 0.707#1.0
    target.pose.orientation.z = 0.0
    target.pose.orientation.w = 0.0

    GRIPPER_OPEN  = [0.05]     # fingers wide open (joint upper limit, axis -X: positive = open)
    GRIPPER_CLOSE = [-0.036]   # light grip on the 81 mm box; do NOT fully close or the box gets ejected


    #  방향별 참고표 (기본 자세 (0,1,0,0) 기준)
    # yaw 각도	x	y	z	w
    # 0° (현재)	0.0	1.0	0.0	0.0
    # +45°	0.383	0.924	0.0	0.0
    # +90°	0.707	0.707	0.0	0.0
    # +180°	1.0	0.0	0.0	0.0
    # -90°	-0.707	0.707	0.0	0.0

    # Issue the move command (RRT plan + execute, blocks until the motion finishes)
    if node.move_to_pose(target):
        # Once the RRT motion is done, wait 1 s then move to the overhead (waiting) pose.
        time.sleep(1.0)
        node.control_gripper(GRIPPER_OPEN)
        time.sleep(0.5)

        # fine-tune the position to grasp the box
        target.pose.position.z = 0.24 # 0.12#0.2
        node.move_to_pose(target)
        time.sleep(0.5)

        # Grasp the box
        node.control_gripper(GRIPPER_CLOSE)
        node.attach_box()

        # Lift the box up
        target.pose.position.z = 0.45 # 0.12#0.2
        node.move_to_pose(target)
        time.sleep(0.5)

        # Move to a different location while holding the box
        target.pose.position.x = 0.08#0.4
        target.pose.position.y = 0.08
        target.pose.position.z = 0.041#0.2

        node.move_to_pose(target)
        time.sleep(0.5)

        # Release the box
        node.detach_box()

        # Move to the waiting pose
        OVERHEAD_POSE_DEG = [0, -10, 1, 10, 10, 0]
        print("\n========== Moving to Overhead Pose ==========")
        node.move_to_joint_pose(OVERHEAD_POSE_DEG, duration_sec=4)
        print("========== Done ==========\n")

    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()

# 0.08 0.08 0.004