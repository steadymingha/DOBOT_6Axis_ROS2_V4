#!/usr/bin/env python3
"""Keyboard jogger for the REAL robot -- drives the controller's own MoveJog
primitive via the bringup dashboard services (/dobot_bringup_ros2/srv/*).

Needs only terminal A (dobot_bringup_ros2.launch.py) -- no MoveIt/action
server, no pinocchio, no sim.

MoveJog is a continuous "move until stopped" command. Safety model:
  - each jog keypress (re)sets a deadline HOLD_S in the future
  - a watchdog thread stops the robot the instant the deadline passes
  - releasing the key -> OS key-repeat stops -> deadline expires -> auto-stop
  - SPACE / q / any exception -> immediate StopMoveJog, always, via try/finally

Run (bringup already up in another terminal):
    python3 tools/jog_real.py                  # 5% speed
    python3 tools/jog_real.py --speed 10 --enable --joint

Keys:
    w/s a/d r/f : X / Y / Z          (JOINT mode: J1 / J2 / J3)
    u/j i/k o/l : Rx / Ry / Rz        (JOINT mode: J4 / J5 / J6)
    SPACE stop   m TCP<->JOINT   -/+ speed   p position   e enable   x disable   c clear error   q quit
"""
import argparse
import os
import sys
import termios
import threading
import time
import tty

import rclpy
from rclpy.node import Node
from rclpy.signals import SignalHandlerOptions
from gamepad import Gamepad
from dobot_msgs_v4.srv import (
    EnableRobot, DisableRobot, ClearError, MoveJog, StopMoveJog,
    SpeedFactor, GetAngle, GetPose, RobotMode, GetErrorID,
)

HOLD_S = 0.25          # how long a jog keeps moving after the last matching keypress
WATCHDOG_PERIOD_S = 0.05
NS = '/dobot_bringup_ros2/srv/'

TCP_KEYS = {
    'w': 'X+', 's': 'X-', 'a': 'Y+', 'd': 'Y-', 'r': 'Z+', 'f': 'Z-',
    'u': 'Rx+', 'j': 'Rx-', 'i': 'Ry+', 'k': 'Ry-', 'o': 'Rz+', 'l': 'Rz-',
}
JOINT_KEYS = {
    'w': 'J1+', 's': 'J1-', 'a': 'J2+', 'd': 'J2-', 'r': 'J3+', 'f': 'J3-',
    'u': 'J4+', 'j': 'J4-', 'i': 'J5+', 'k': 'J5-', 'o': 'J6+', 'l': 'J6-',
}

# empirically confirmed on the connected "Dual PSX Adaptor" pad (js0):
# axis 0/1 = left stick X/Y, axis 2/3 = right stick X/Y, both full analog.
# button 4 = L1 (verified 2026-08-04). No distinct D-pad axis on this pad
# (shares 0/1 with the left stick), so Rx/Ry aren't reachable from the
# gamepad -- keyboard covers those.
PAD_DEADMAN_BUTTON = 4
PAD_STOP_BUTTON = 5   # guessed as R1 (mirrors button 4); unverified -- keyboard SPACE is the real stop
# axis0/1 swapped (X<->Y) and signs tuned per on-robot feel check, 2026-08-04.
PAD_AXES_TCP = {0: 'Y', 1: 'X', 2: 'Z', 3: 'Rz'}
PAD_AXES_JOINT = {0: 'J2', 1: 'J1', 2: 'J3', 3: 'J4'}
PAD_AXIS_SIGN = {0: -1, 1: -1, 2: -1, 3: 1}
PAD_PERIOD_S = 0.05


class Jogger(Node):
    def __init__(self, speed):
        super().__init__('jog_real')
        self.cli_enable = self.create_client(EnableRobot, NS + 'EnableRobot')
        self.cli_disable = self.create_client(DisableRobot, NS + 'DisableRobot')
        self.cli_clear = self.create_client(ClearError, NS + 'ClearError')
        self.cli_movejog = self.create_client(MoveJog, NS + 'MoveJog')
        self.cli_stopjog = self.create_client(StopMoveJog, NS + 'StopMoveJog')
        self.cli_speed = self.create_client(SpeedFactor, NS + 'SpeedFactor')
        self.cli_angle = self.create_client(GetAngle, NS + 'GetAngle')
        self.cli_pose = self.create_client(GetPose, NS + 'GetPose')
        self.cli_mode = self.create_client(RobotMode, NS + 'RobotMode')
        self.cli_error = self.create_client(GetErrorID, NS + 'GetErrorID')
        for cli in (self.cli_enable, self.cli_movejog):
            while not cli.wait_for_service(timeout_sec=1.0):
                self.get_logger().info(f'waiting for {cli.srv_name} ...')

        self.lock = threading.Lock()
        self.active_axis = None
        self.deadline = 0.0
        self.joint_mode = False
        self.speed = speed

    def _call(self, cli, req):
        fut = cli.call_async(req)
        rclpy.spin_until_future_complete(self, fut, timeout_sec=5.0)
        return fut.result()

    def jog(self, axis):
        with self.lock:
            if self.active_axis != axis:
                if self.active_axis is not None:
                    self._call(self.cli_stopjog, StopMoveJog.Request())
                req = MoveJog.Request()
                req.axis_id = axis
                if not (axis[1:2].isdigit()):  # Cartesian axis (X+/Y-/Rz+/...), not J1..J6
                    # coordtype is required for Cartesian jog -- 0 (joint default) errors with -6.
                    # 1 = user coordinate system; using user=0 (the default frame).
                    req.param_value = ['coordtype=1', 'user=0']
                r = self._call(self.cli_movejog, req)
                self.active_axis = axis
                print(f'[jog] {axis} MoveJog->res={r.res}')
            self.deadline = time.time() + HOLD_S

    def stop(self):
        with self.lock:
            if self.active_axis is not None:
                r = self._call(self.cli_stopjog, StopMoveJog.Request())
                self.active_axis = None
                print(f'[jog] stopped StopMoveJog->res={r.res}')

    def watchdog_tick(self):
        with self.lock:
            if self.active_axis is not None and time.time() > self.deadline:
                r = self._call(self.cli_stopjog, StopMoveJog.Request())
                self.active_axis = None
                print(f'[jog] auto-stop (key released) StopMoveJog->res={r.res}')

    def print_status(self):
        m = self._call(self.cli_mode, RobotMode.Request())
        e = self._call(self.cli_error, GetErrorID.Request())
        print(f'RobotMode -> {m.robot_return}   GetErrorID -> {e.robot_return}')

    def enable(self):
        r = self._call(self.cli_enable, EnableRobot.Request())
        print(f'EnableRobot -> res={r.res}')
        time.sleep(1.0)  # real hardware takes a moment to release brakes
        self.print_status()

    def disable(self):
        r = self._call(self.cli_disable, DisableRobot.Request())
        print(f'DisableRobot -> res={r.res} {r.robot_return}')

    def clear_error(self):
        r = self._call(self.cli_clear, ClearError.Request())
        print(f'ClearError -> res={r.res}')

    def set_speed(self, pct):
        pct = max(1, min(100, pct))
        r = self._call(self.cli_speed, SpeedFactor.Request(ratio=pct))
        self.speed = pct
        print(f'SpeedFactor({pct}) -> res={r.res}')

    def print_position(self):
        a = self._call(self.cli_angle, GetAngle.Request())
        p = self._call(self.cli_pose, GetPose.Request(user=0, tool=0))
        print(f'angle: {a.robot_return}\npose:  {p.robot_return}')
        self.print_status()


def watchdog_loop(node, stop_event):
    while not stop_event.is_set():
        node.watchdog_tick()
        time.sleep(WATCHDOG_PERIOD_S)


def gamepad_loop(node, pad, stop_event):
    while not stop_event.is_set():
        if pad.button(PAD_STOP_BUTTON):
            node.stop()
        elif pad.button(PAD_DEADMAN_BUTTON):
            axes = PAD_AXES_JOINT if node.joint_mode else PAD_AXES_TCP
            best_n, best_v = None, 0.0
            for n in axes:
                v = pad.axis(n)
                if abs(v) > abs(best_v):
                    best_n, best_v = n, v
            if best_n is not None:
                signed_v = best_v * PAD_AXIS_SIGN.get(best_n, 1)
                node.jog(axes[best_n] + ('+' if signed_v > 0 else '-'))
        else:
            node.stop()
        time.sleep(PAD_PERIOD_S)


def read_key(fd):
    return sys.stdin.read(1)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--speed', type=int, default=5, help='initial jog speed percent (1-100)')
    ap.add_argument('--enable', action='store_true', help='enable the robot on startup')
    ap.add_argument('--joint', action='store_true', help='start in JOINT mode (default TCP)')
    args = ap.parse_args()

    # rclpy's own SIGINT handler swallows Ctrl+C instead of raising KeyboardInterrupt,
    # which skipped the try/finally stop-the-robot cleanup below. Disable it so Ctrl+C
    # behaves normally.
    rclpy.init(signal_handler_options=SignalHandlerOptions.NO)
    node = Jogger(args.speed)
    node.joint_mode = args.joint
    node.set_speed(args.speed)
    if args.enable:
        node.enable()

    print(__doc__)
    print(f"mode={'JOINT' if node.joint_mode else 'TCP'}  speed={node.speed}%")

    stop_event = threading.Event()
    wd = threading.Thread(target=watchdog_loop, args=(node, stop_event), daemon=True)
    wd.start()

    pad = Gamepad()
    if pad.open():
        print(f'[gamepad] {pad.dev} connected -- hold button {PAD_DEADMAN_BUTTON} + move a '
              f'stick to jog, button {PAD_STOP_BUTTON} to stop. enable/disable/mode stay '
              f'keyboard-only.')
        threading.Thread(target=gamepad_loop, args=(node, pad, stop_event), daemon=True).start()

    fd = sys.stdin.fileno()
    old_settings = termios.tcgetattr(fd)
    try:
        tty.setcbreak(fd)
        while True:
            ch = read_key(fd)
            keymap = JOINT_KEYS if node.joint_mode else TCP_KEYS
            if ch in keymap:
                node.jog(keymap[ch])
            elif ch == ' ':
                node.stop()
            elif ch == 'm':
                node.stop()
                node.joint_mode = not node.joint_mode
                print(f"\nmode={'JOINT' if node.joint_mode else 'TCP'}")
            elif ch == '-':
                node.set_speed(node.speed - 5)
            elif ch == '+':
                node.set_speed(node.speed + 5)
            elif ch == 'p':
                node.print_position()
            elif ch == 'e':
                node.enable()
            elif ch == 'x':
                node.stop()
                node.disable()
            elif ch == 'c':
                node.clear_error()
            elif ch == 'q' or ch == '\x1b':
                break
    finally:
        # stop the background threads FIRST -- they call rclpy.spin_until_future_complete
        # on this same node, and doing that concurrently with the stop() call below can
        # deadlock (seen in practice: process wouldn't die on Ctrl+C).
        stop_event.set()
        time.sleep(2 * max(WATCHDOG_PERIOD_S, PAD_PERIOD_S))
        node.stop()
        pad.close()
        termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
        node.destroy_node()
        rclpy.shutdown()
        print('\nbye.')


if __name__ == '__main__':
    main()
