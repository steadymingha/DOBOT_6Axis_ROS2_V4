#!/usr/bin/env python3
"""Simultaneous multi-joint jogger for the REAL robot -- drives action_move_server
(FollowJointTrajectory -> ServoJ) instead of MoveJog, so the gamepad's 4 stick axes
can move 4 joints AT THE SAME TIME (MoveJog can't: the dashboard protocol only jogs
one axis per command, see tools/jog_real.py).

Needs terminal A (dobot_bringup_ros2.launch.py) AND terminal B
(dobot_joint.launch.py, DOBOT_TYPE=cr7) up.

Mechanism: unlike MoveJog's start/stop-until-told model, this sends small absolute
joint-target updates (FollowJointTrajectory, single point, time_from_start=TICK_S)
at a fixed rate while a key/stick is active. The controller (ServoJ) moves to that
target and then just holds -- so when nothing is held, no goals are sent and the
robot naturally stays put. No watchdog needed for the "stop" side; a deadman still
gates the gamepad so a stuck stick can't drive it.

Run:
    python3 tools/jog_action.py                  # 10 deg/s
    python3 tools/jog_action.py --speed 20 --enable

Keys (single joint, same layout as jog_real.py JOINT mode):
    w/s a/d r/f : J1 / J2 / J3     u/j i/k o/l : J4 / J5 / J6
    SPACE stop   -/+ speed   p position   e enable   x disable   c clear error   q quit

Gamepad: hold L1 (button 4) + move sticks -- left stick -> J2/J1, right stick ->
J3/J4, proportional to deflection, all four can move at once.
"""
import argparse
import math
import os
import sys
import termios
import threading
import time
import tty

import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
from rclpy.signals import SignalHandlerOptions
from control_msgs.action import FollowJointTrajectory
from trajectory_msgs.msg import JointTrajectoryPoint
from sensor_msgs.msg import JointState
from gamepad import Gamepad
from dobot_msgs_v4.srv import EnableRobot, DisableRobot, ClearError, GetAngle, GetPose, RobotMode, GetErrorID

NS = '/dobot_bringup_ros2/srv/'
TICK_S = 0.2            # matches action_move_server's internal ServoJ pacing (t=0.2, sleep 0.18)
HOLD_S = 0.25            # keyboard: how long a key stays "active" after the last matching press
JOINT_NAMES = ['joint1', 'joint2', 'joint3', 'joint4', 'joint5', 'joint6']

# key -> (joint index 0-5, sign)
KEY_JOINT = {
    'w': (0, +1), 's': (0, -1), 'a': (1, +1), 'd': (1, -1), 'r': (2, +1), 'f': (2, -1),
    'u': (3, +1), 'j': (3, -1), 'i': (4, +1), 'k': (4, -1), 'o': (5, +1), 'l': (5, -1),
}
# gamepad axis -> (joint index 0-5, sign) -- reuse the feel tuned in jog_real.py
PAD_DEADMAN_BUTTON = 4
PAD_JOINT = {0: (1, -1), 1: (0, -1), 2: (2, -1), 3: (3, 1)}  # axis -> (joint idx, sign)
PAD_DEADZONE = 0.15
# How far the commanded target may run ahead of where the arm actually is,
# in ticks of commanded motion. Without a bound the target integrates forever
# while the arm is not following -- which is exactly what happens when the
# controller trips into COLLISION/ERROR and rejects every goal. The gap grows
# for as long as the stick is held, and the instant the error clears the arm
# lunges the whole way in one go. Two ticks is enough lead to stay smooth.
MAX_LEAD_TICKS = 2


class ActionJogger(Node):
    def __init__(self, speed_deg_s):
        super().__init__('jog_action')
        dobot_type = os.getenv('DOBOT_TYPE')
        if not dobot_type:
            raise SystemExit('DOBOT_TYPE env var not set (export DOBOT_TYPE=cr7)')
        self.action_name = f'/{dobot_type}_group_controller/follow_joint_trajectory'
        self.client = ActionClient(self, FollowJointTrajectory, self.action_name)
        self.cli_enable = self.create_client(EnableRobot, NS + 'EnableRobot')
        self.cli_disable = self.create_client(DisableRobot, NS + 'DisableRobot')
        self.cli_clear = self.create_client(ClearError, NS + 'ClearError')
        self.cli_angle = self.create_client(GetAngle, NS + 'GetAngle')
        self.cli_pose = self.create_client(GetPose, NS + 'GetPose')
        self.cli_mode = self.create_client(RobotMode, NS + 'RobotMode')
        self.cli_error = self.create_client(GetErrorID, NS + 'GetErrorID')

        self.lock = threading.Lock()
        self.target = None      # commanded position (integrated)
        self.actual = None      # where the arm actually is, live
        self.create_subscription(JointState, '/joint_states', self._on_js, 10)

        self.speed_deg_s = speed_deg_s
        self.kb_axis = None       # (joint idx, sign) currently active from keyboard
        self.kb_deadline = 0.0

    def _on_js(self, msg):
        order = [msg.name.index(n) for n in JOINT_NAMES]
        # Rebind rather than mutate: tick() reads this without the lock, and a
        # whole-list swap is atomic where an in-place edit would not be.
        self.actual = [msg.position[i] for i in order]
        if self.target is None:
            self.target = list(self.actual)

    def _call(self, cli, req):
        # a background thread runs rclpy.spin(self) for the whole process lifetime
        # (needed for the action client's goal/result callbacks) -- don't spin here
        # too, or two threads touch the executor at once (that's what deadlocked
        # jog_real.py's shutdown before). Just wait for the future to resolve.
        fut = cli.call_async(req)
        while not fut.done():
            time.sleep(0.01)
        return fut.result()

    def step_rad(self):
        return math.radians(self.speed_deg_s) * TICK_S

    def kb_press(self, joint_idx, sign):
        with self.lock:
            self.kb_axis = (joint_idx, sign)
            self.kb_deadline = time.time() + HOLD_S

    def kb_stop(self):
        with self.lock:
            self.kb_axis = None

    def tick(self, pad):
        with self.lock:
            if self.kb_axis is not None and time.time() > self.kb_deadline:
                self.kb_axis = None
            delta = [0.0] * 6
            if self.kb_axis is not None:
                idx, sign = self.kb_axis
                delta[idx] += sign * self.step_rad()
            if pad is not None and pad.button(PAD_DEADMAN_BUTTON):
                for axis, (idx, sign) in PAD_JOINT.items():
                    v = pad.axis(axis, PAD_DEADZONE)
                    if v:
                        delta[idx] += sign * v * self.step_rad()
            actual = self.actual
            if not any(delta):
                # Nothing held: start the next push from where the arm really
                # is, so settling error and any rejected goals do not accumulate.
                if actual is not None:
                    self.target = list(actual)
                return
            self.target = [t + d for t, d in zip(self.target, delta)]
            if actual is not None:
                lead = max(math.radians(2.0), MAX_LEAD_TICKS * self.step_rad())
                self.target = [min(max(t, a - lead), a + lead)
                               for t, a in zip(self.target, actual)]
            goal = FollowJointTrajectory.Goal()
            goal.trajectory.joint_names = JOINT_NAMES
            point = JointTrajectoryPoint()
            point.positions = list(self.target)
            point.time_from_start.sec = 0
            point.time_from_start.nanosec = int(TICK_S * 1e9)
            goal.trajectory.points = [point]
            moved = [JOINT_NAMES[i] for i, d in enumerate(delta) if d]
            fut = self.client.send_goal_async(goal)
            fut.add_done_callback(lambda f, moved=moved: self._on_goal_response(f, moved))

    def _on_goal_response(self, fut, moved):
        handle = fut.result()
        if not handle.accepted:
            print(f'[jog] goal REJECTED ({moved})')

    def print_status(self):
        m = self._call(self.cli_mode, RobotMode.Request())
        e = self._call(self.cli_error, GetErrorID.Request())
        print(f'RobotMode -> {m.robot_return}   GetErrorID -> {e.robot_return}')

    def enable(self):
        r = self._call(self.cli_enable, EnableRobot.Request())
        print(f'EnableRobot -> res={r.res}')
        time.sleep(1.0)
        self.print_status()

    def disable(self):
        r = self._call(self.cli_disable, DisableRobot.Request())
        print(f'DisableRobot -> res={r.res} {r.robot_return}')

    def clear_error(self):
        r = self._call(self.cli_clear, ClearError.Request())
        print(f'ClearError -> res={r.res}')

    def print_position(self):
        a = self._call(self.cli_angle, GetAngle.Request())
        p = self._call(self.cli_pose, GetPose.Request(user=0, tool=0))
        print(f'angle: {a.robot_return}\npose:  {p.robot_return}')
        self.print_status()


def tick_loop(node, pad, stop_event):
    while not stop_event.is_set():
        node.tick(pad)
        time.sleep(TICK_S)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--speed', type=float, default=10.0, help='joint jog speed, deg/s')
    ap.add_argument('--enable', action='store_true')
    args = ap.parse_args()

    rclpy.init(signal_handler_options=SignalHandlerOptions.NO)
    node = ActionJogger(args.speed)
    spin_thread = threading.Thread(target=rclpy.spin, args=(node,), daemon=True)
    spin_thread.start()

    print(f'waiting for {node.action_name} and /joint_states ...')
    while node.target is None:
        time.sleep(0.1)
    if not node.client.wait_for_server(timeout_sec=10.0):
        raise SystemExit('action server not available')

    if args.enable:
        node.enable()

    print(__doc__)
    print(f'speed={node.speed_deg_s} deg/s')

    pad = Gamepad()
    have_pad = pad.open()
    if have_pad:
        print(f'[gamepad] {pad.dev} connected -- hold button {PAD_DEADMAN_BUTTON} + move '
              f'sticks to jog multiple joints at once.')

    stop_event = threading.Event()
    threading.Thread(target=tick_loop, args=(node, pad if have_pad else None, stop_event),
                      daemon=True).start()

    fd = sys.stdin.fileno()
    old_settings = termios.tcgetattr(fd)
    try:
        tty.setcbreak(fd)
        while True:
            ch = sys.stdin.read(1)
            if ch in KEY_JOINT:
                node.kb_press(*KEY_JOINT[ch])
            elif ch == ' ':
                node.kb_stop()
            elif ch == '-':
                node.speed_deg_s = max(1.0, node.speed_deg_s - 5)
                print(f'speed={node.speed_deg_s} deg/s')
            elif ch == '+':
                node.speed_deg_s += 5
                print(f'speed={node.speed_deg_s} deg/s')
            elif ch == 'p':
                node.print_position()
            elif ch == 'e':
                node.enable()
            elif ch == 'x':
                node.kb_stop()
                node.disable()
            elif ch == 'c':
                node.clear_error()
            elif ch == 'q' or ch == '\x1b':
                break
    finally:
        node.kb_stop()
        stop_event.set()
        time.sleep(2 * TICK_S)
        if have_pad:
            pad.close()
        termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
        # shutdown() first so the background rclpy.spin(node) thread notices
        # ok()==False and returns, THEN destroy the node -- destroying it while
        # that thread is still spinning it is a use-after-free waiting to happen.
        rclpy.shutdown()
        spin_thread.join(timeout=2.0)
        node.destroy_node()
        print('\nbye.')


if __name__ == '__main__':
    main()
