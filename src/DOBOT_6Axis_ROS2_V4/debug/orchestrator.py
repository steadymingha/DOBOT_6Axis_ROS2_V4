"""
mission_orchestrator.py — Top-level FSM Orchestrator (Abstract Skeleton)

Design Principles
- SingleThreadedExecutor + call_async/add_done_callback (No blocking calls allowed)
- This node only decides 'what to do'. 'How to do it' is handled by boundary nodes:
    MQTT bridge (MCS) / AMR bridge (Moving-Q) / vision bridge (ZeroMQ) / arm_skill_node / gripper_node
- All interfaces only refer to the contracts in the mm_interfaces package
"""

import enum
import rclpy
from rclpy.node import Node
from rclpy.executors import SingleThreadedExecutor

# from mm_interfaces.srv import SetBaseLock, GetObjectPose, ExecuteArmSkill, SetGripper
# from mm_interfaces.msg import MissionCommand, MissionResult, SystemFault


class State(enum.Enum):
    IDLE = enum.auto()
    NAV = enum.auto()          # Send destination to AMR, wait for arrival
    LOCK = enum.auto()         # SetBaseLock(lock=True) — Lock base before arm movement
    LOCATE = enum.auto()       # vision GetObjectPose — Secure magazine position/orientation
    PICK = enum.auto()         # arm_skill: approach → grasp → retreat
    PLACE = enum.auto()        # arm_skill: transport → place → retreat
    REPORT_DONE = enum.auto()  # Report result to MCS after UNLOCK
    FAULT = enum.auto()        # Unrecoverable error — Stop and wait for operator intervention


class MissionOrchestrator(Node):

    def __init__(self):
        super().__init__('mission_orchestrator')

        self.state = State.IDLE
        self.mission = None          # Current mission context (destination, magazine slot, etc.)
        self.pending_future = None   # In-progress asynchronous call handle

        # ---- Clients to boundary nodes (referencing contracts only) ----
        # self.cli_base_lock  = self.create_client(SetBaseLock, '/amr/set_base_lock')
        # self.cli_get_pose   = self.create_client(GetObjectPose, '/vision/get_object_pose')
        # self.cli_arm_skill  = self.create_client(ExecuteArmSkill, '/arm/execute_skill')
        # self.cli_gripper    = self.create_client(SetGripper, '/gripper/set')

        # ---- Input Events ----
        # self.sub_mission = self.create_subscription(MissionCommand, '/mcs/mission', self.on_mission, 10)
        # self.sub_fault   = self.create_subscription(SystemFault, '/system/fault', self.on_fault, 10)

        # ---- Output ----
        # self.pub_result = self.create_publisher(MissionResult, '/mcs/mission_result', 10)

    # =========================================================
    # Event Entry Points
    # =========================================================

    def on_mission(self, msg):
        """Receive MCS mission. Accept only in IDLE state."""
        if self.state is not State.IDLE:
            # TODO: Reject response due to busy
            return
        self.mission = msg
        self.transition(State.NAV)

    def on_fault(self, msg):
        """Immediately transition to FAULT upon receiving a fault in any state. (E-Stop is a hardware parallel path)"""
        self.transition(State.FAULT)

    # =========================================================
    # FSM Core
    # =========================================================

    def transition(self, new_state: State):
        self.get_logger().info(f'{self.state.name} -> {new_state.name}')
        self.state = new_state
        self.on_enter(new_state)

    def on_enter(self, state: State):
        """State entry action dispatch. Each handler only makes an asynchronous call and returns immediately."""
        dispatch = {
            State.NAV: self.enter_nav,
            State.LOCK: self.enter_lock,
            State.LOCATE: self.enter_locate,
            State.PICK: self.enter_pick,
            State.PLACE: self.enter_place,
            State.REPORT_DONE: self.enter_report_done,
            State.FAULT: self.enter_fault,
        }
        handler = dispatch.get(state)
        if handler:
            handler()

    def guarded(self, expected: State):
        """Guard to ignore the result if the state has already changed (e.g., fault) at the time of callback execution."""
        return self.state is expected

    # =========================================================
    # State Entry Actions + Completion Callbacks (call_async chain)
    # =========================================================

    def enter_nav(self):
        # TODO: Send destination to AMR bridge (call_async)
        # future.add_done_callback(self.on_nav_done)
        pass

    def on_nav_done(self, future):
        if not self.guarded(State.NAV):
            return
        # TODO: Check arrival success → LOCK / Failure → FAULT
        self.transition(State.LOCK)

    def enter_lock(self):
        # TODO: SetBaseLock(lock=True) call_async
        pass

    def on_lock_done(self, future):
        if not self.guarded(State.LOCK):
            return
        self.transition(State.LOCATE)

    def enter_locate(self):
        # TODO: GetObjectPose call_async (Pass target magazine ID)
        pass

    def on_locate_done(self, future):
        if not self.guarded(State.LOCATE):
            return
        # TODO: Validate pose → Save to mission context
        self.transition(State.PICK)

    def enter_pick(self):
        # TODO: Call arm_skill 'pick' (Pass locate result pose)
        #       Whether the gripper sequence is internal to arm_skill_node,
        #       or the orchestrator calls gripper separately, is decided in the skill contract
        pass

    def on_pick_done(self, future):
        if not self.guarded(State.PICK):
            return
        self.transition(State.PLACE)

    def enter_place(self):
        # TODO: Call arm_skill 'place' (Pass target port coordinates)
        pass

    def on_place_done(self, future):
        if not self.guarded(State.PLACE):
            return
        # TODO: If next magazine in mission remains, loop to LOCATE, else REPORT_DONE
        self.transition(State.REPORT_DONE)

    def enter_report_done(self):
        # TODO: SetBaseLock(lock=False) → Publish MissionResult on success → IDLE
        pass

    def enter_fault(self):
        # TODO: Invalidate pending future, request arm stop, report fault to MCS
        #       Recovery via separate reset event after operator intervention
        pass


def main():
    rclpy.init()
    node = MissionOrchestrator()
    executor = SingleThreadedExecutor()
    executor.add_node(node)
    try:
        executor.spin()
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()