#!/usr/bin/env python3
"""CBiRRT pick-and-place rehearsal between two TAUGHT points on the REAL CR7.

No gripper is fitted yet, so nothing is actually grasped: the cycle mimics
sequences/wirebonder_pick_place.py's guarded place -- approach, descend until
contact, hold 3 s (as if the jaws closed), lift, carry, descend, hold 3 s (as if
they opened), lift -- with the grasp/release replaced by a dwell.

The two work points are TAUGHT, not computed: jog the arm with tools/jog_real.py
(or jog_action.py) to a HOVER pose a few cm above the object, then

    ... cbirrt_p1p2_test.py --teach p1        # pick hover
    ... cbirrt_p1p2_test.py --teach p2        # place hover

which stores the six joint angles in test/points.json. Teaching JOINT angles
(not a Cartesian pose) is what makes the missing gripper harmless: no IK runs,
so no tool-length assumption enters, and the taught configs are reproduced
exactly. The descend is a pure vertical translation, which is identical for
every point rigidly attached to the flange -- tool offset cannot bias it either.

ONE CYCLE
    approach p1   empty   CBiRRT if the tool already points the right way,
                          otherwise a free joint-space RRT (see approach())
    descend p1            until contact, hold 3 s, retract
    carry p1 -> p2 LOADED strictly CBiRRT, tool attitude held -- this is the leg
                          where a part would be in the jaws
    descend p2            until contact, hold 3 s, retract
    return p1     empty   as the approach

Teach p1 and p2 with the SAME wrist attitude (straight down is the usual
choice). The carry holds the tool attitude fixed, so two points that point the
tool differently have no constrained path between them at all -- --run checks
this before it moves anything.

WHAT COUNTS AS CONTACT
    1. The controller's own joint-torque collision detection (SetCollisionLevel),
       read back as robot_mode == 11 (ROBOT_MODE_COLLISION) or CollisionStates
       != 0 in the 30004 real-time frame. This is the hard, firmware-level stop.
    2. A softer joint-torque step (m_actual against a delayed baseline), which
       normally trips first and stops the stream before the firmware has to.
       TCP_force is NOT used: this controller reports it as all zeros (measured).

HOW THE DESCEND IS DRIVEN
    Straight to /dobot_bringup_ros2/srv/ServoJ at 33 Hz, NOT through the
    FollowJointTrajectory action. The action server (dobot_moveit/
    action_move_server.py) registers no cancel callback, so a goal in flight
    cannot be stopped -- and stopping on contact is the entire point. Dropping
    the ServoJ stream is itself a clean stop: ServoJ means "reach this target in
    t and hold", so the robot simply holds the last target it was given.
    Everything else (the approach and the carry) goes through the normal
    execute_path action, which resamples onto the same 30 ms grid.

BRING-UP (see docs/real_robot_jetson_bringup.md 8.3 / 8.4 -- both terminals)
    terminal A:  ros2 launch cr_robot_ros2 dobot_bringup_ros2.launch.py
    terminal B:  ros2 launch dobot_moveit dobot_joint.launch.py

RUN (inside the ros2_dobot container)
    source /opt/ros/humble/setup.bash
    source /root/dobot_ws/install/setup.bash        # xacro needs the workspace
    export DOBOT_TYPE=cr7
    cd /root/dobot_ws/src/DOBOT_6Axis_ROS2_V4
    P=/root/dobot_ws/.venv/bin/python3              # the venv that has pinocchio

    $P test/cbirrt_p1p2_test.py --monitor           # live feedback, moves nothing
    $P test/cbirrt_p1p2_test.py --teach-surface z-  # tool resting on the table
    $P test/cbirrt_p1p2_test.py --teach-surface y-  # tool against the -y wall
    $P test/cbirrt_p1p2_test.py --teach p1          # jog there first
    $P test/cbirrt_p1p2_test.py --teach p2
    $P test/cbirrt_p1p2_test.py --show
    $P test/cbirrt_p1p2_test.py --dry               # plan only, no motion
    $P test/cbirrt_p1p2_test.py --run               # the real cycle
    $P test/cbirrt_p1p2_test.py --run --cycles 3

    $P test/cbirrt_p1p2_test.py --teach obs         # where the camera sees the magazine
    $P test/cbirrt_p1p2_test.py --vision            # vision hover: plan only
    $P test/cbirrt_p1p2_test.py --vision --run      # go there, turn J6. NO DESCEND

SURFACES (table and walls)
    Nothing in the URDF knows the room exists, so --teach-surface measures each
    bounding surface and puts it in the planning collision model as a half-space
    no arm link may cross. Put the TOOL against the surface, then name the
    direction from the robot base toward it, AXIS FIRST: 'z-' for the table
    underneath, 'y-' for the wall on the -y side, 'x+' for a wall on the +x
    side. (Axis first because argparse reads a bare '-z' as a flag, not as a
    value.) See measure_plane() for why it reads the collision model rather
    than the TCP.
"""

import argparse
import json
import math
import os
import socket
import struct
import sys
import threading
import time
from collections import deque

import numpy as np
import rclpy
import pinocchio as pin
from rclpy.executors import MultiThreadedExecutor
from rclpy.signals import SignalHandlerOptions

# test/ sits one level below the package root; add the root so cr7_pnp imports
# when this file is run standalone (same trick as sequences/*.py).
_PKG_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, _PKG_ROOT)

# This script only ever talks to the real controller, whose J1/J5/J6 run
# opposite to the URDF's (see JOINT_SIGN_REAL in cr7_pnp/node.py). Set before
# the node is constructed, since that is where node.py reads it.
os.environ.setdefault('CR7_REAL_ROBOT', '1')

from cr7_pnp.node import HubPickPlace                          # noqa: E402
from cr7_pnp.geometry import XACRO_PATH, COMBINED_XACRO        # noqa: E402
from dobot_msgs_v4.srv import (                                # noqa: E402
    ClearError, RobotMode, ServoJ, SetBackDistance,
    SetCollisionLevel, SetPostCollisionMode, SpeedFactor)

POINTS_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'points.json')
SURFACES_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                             'surfaces.json')

# Each surface (table, wall) is registered as a slab this THICK reaching AWAY
# from the robot, so the planner treats the whole far side as solid instead of
# as a sheet it could route around or under. Both numbers just have to outrun
# the arm's reach.
PLANE_THICK = 4.0
PLANE_EXTENT = 4.0

AXES = {'x': 0, 'y': 1, 'z': 2}

RT_PORT = 30004
RT_FRAME = 1440
# Offsets into the 1440-byte real-time struct (dobot_bringup_v4/include/
# dobot_bringup/command.h, RealTimeData_t).
OFF_LEN, OFF_ROBOT_MODE = 0, 24
OFF_Q_ACTUAL, OFF_TCP_FORCE = 432, 720
OFF_ENABLE, OFF_ERROR, OFF_COLLISION = 1026, 1029, 1038
OFF_QD_ACTUAL = 480        # joint velocities, deg/s
# The flange pose the robot itself reports (x/y/z mm + rx/ry/rz deg), i.e. the
# same numbers as GetPose(user=0, tool=0). --vision needs it for the camera
# transform, and it is a free cross-check on the model's own FK.
OFF_TOOL_VECTOR = 624
OFF_M_ACTUAL = 1120

MODE_COLLISION = 11        # ROBOT_MODE_COLLISION
MODE_ENABLE = 5            # ROBOT_MODE_ENABLE (idle)
MODE_PAUSE = 10            # ROBOT_MODE_PAUSE

SERVOJ_DT = 0.03           # Dobot's recommended ServoJ interval (33 Hz)

# Per-joint torque step that counts as contact, N*m -- taken from a 20 mm descend
# through OPEN AIR at 3 mm/s, logged tick by tick (2026-08-06):
#
#   t < 1.2 s   starting transient   J2 up to 7.27, J3 up to 12.52, J4 up to 2.57
#   t > 1.3 s   steady descent       J1 0.20, J2 1.82, J3 1.41, J4 0.65, J5/J6 ~0
#
# Nothing was touched in that run, so the transient is the arm breaking away from
# rest, not contact. Two consequences: the soft channel has to stay BLIND for the
# transient (TORQUE_BLANK_S), and these numbers sit ~2x above the steady noise
# rather than under the transient, which is what tripped the first attempts at
# 0.1 mm. J6 is excluded -- it reads a constant 0.0 on this controller.
TORQUE_TRIP = (1.5, 4.0, 3.5, 2.0, 1.5, 1e9)

# How long after a descend starts the soft channel stays blind. Covers the 1.2 s
# breakaway transient above; the controller's own collision detection is armed
# throughout, so this is not an unprotected window. At 3 mm/s it costs 4.5 mm of
# travel, well inside a 30-50 mm hover.
TORQUE_BLANK_S = 1.5


# --------------------------------------------------------------------------
# real-time feedback
# --------------------------------------------------------------------------

class RealtimeMonitor(threading.Thread):
    """Passive tap on the controller's 30004 real-time feed, ~125 Hz.

    The bringup node already holds its own connection; the controller happily
    serves additional read-only clients (verified against the live robot), so
    this neither disturbs nor depends on bringup. Read-only by construction --
    nothing is ever sent on this socket.
    """

    def __init__(self, ip, port=RT_PORT):
        super().__init__(daemon=True)
        self.ip, self.port = ip, port
        self._lock = threading.Lock()
        self._state = None
        self._stamp = 0.0
        self._running = True
        self.error = None
        self.history = deque(maxlen=400)      # (t, m_actual) ~3 s at 125 Hz

    def run(self):
        try:
            sock = socket.create_connection((self.ip, self.port), timeout=5.0)
        except OSError as e:
            self.error = f"real-time feed {self.ip}:{self.port} unreachable: {e}"
            return
        buf = b''
        with sock:
            sock.settimeout(5.0)
            while self._running:
                try:
                    chunk = sock.recv(65536)
                except OSError as e:
                    self.error = f"real-time feed read failed: {e}"
                    return
                if not chunk:
                    self.error = "real-time feed closed by the controller"
                    return
                buf += chunk
                while len(buf) >= RT_FRAME:
                    frame, buf = buf[:RT_FRAME], buf[RT_FRAME:]
                    self._store(frame)

    def _store(self, frame):
        (length,) = struct.unpack_from('<H', frame, OFF_LEN)
        if length != RT_FRAME:
            return                              # out of sync; skip this frame
        st = dict(
            robot_mode=struct.unpack_from('<Q', frame, OFF_ROBOT_MODE)[0],
            q_actual=struct.unpack_from('<6d', frame, OFF_Q_ACTUAL),
            tcp_force=struct.unpack_from('<6d', frame, OFF_TCP_FORCE),
            qd_actual=struct.unpack_from('<6d', frame, OFF_QD_ACTUAL),
            tool_vector=struct.unpack_from('<6d', frame, OFF_TOOL_VECTOR),
            m_actual=struct.unpack_from('<6d', frame, OFF_M_ACTUAL),
            enable=struct.unpack_from('<b', frame, OFF_ENABLE)[0],
            error=struct.unpack_from('<b', frame, OFF_ERROR)[0],
            collision=struct.unpack_from('<b', frame, OFF_COLLISION)[0],
        )
        now = time.time()
        with self._lock:
            self._state, self._stamp = st, now
            self.history.append((now, st['m_actual']))

    def stop(self):
        self._running = False

    def state(self):
        with self._lock:
            return self._state, self._stamp

    def wait_ready(self, timeout=5.0):
        t0 = time.time()
        while time.time() - t0 < timeout:
            if self.error:
                return False
            if self.state()[0] is not None:
                return True
            time.sleep(0.02)
        return False

    def torque_window(self, t_from, t_to):
        """m_actual samples stamped within [t_from, t_to] -> list of 6-tuples."""
        with self._lock:
            return [m for t, m in self.history if t_from <= t <= t_to]


class ContactDetector:
    """Two-channel contact test, evaluated once per ServoJ tick.

    hard  -- the controller tripped its own collision detection. Authoritative,
             but it only fires once the arm has already pushed hard enough to
             trip, so it is the backstop, not the target.
    soft  -- joint torque stepped away from where it was `lag` seconds ago.
             Over a 5 mm/s descent gravity torque drifts slowly and smoothly,
             so a delayed baseline tracks it out and leaves contact as a step.
             Requires `hold` consecutive positives so a single noisy frame
             cannot stop the descend.
    """

    def __init__(self, mon, thresholds=TORQUE_TRIP, lag=0.35, win=0.25,
                 hold=2, soft=True, blank_s=TORQUE_BLANK_S):
        self.mon = mon
        self.thresholds = np.array(thresholds, dtype=float)
        self.lag, self.win, self.hold = lag, win, hold
        self.soft = soft
        self.blank_s = blank_s
        self._streak = 0
        self._t0 = 0.0
        self.margin = np.zeros(6)          # last |delta| per joint, for logging

    def reset(self):
        """Call as the descend starts: the blanking window runs from here."""
        self._streak = 0
        self._t0 = time.time()
        self.margin = np.zeros(6)

    def blanked(self):
        return self.soft and (time.time() - self._t0) < self.blank_s

    def check(self):
        """Return None when clear, else a short string naming what tripped."""
        st, stamp = self.mon.state()
        if st is None:
            return "no real-time feedback"
        if time.time() - stamp > 0.5:
            return f"real-time feed stale ({time.time() - stamp:.1f}s)"
        if st['robot_mode'] == MODE_COLLISION or st['collision']:
            return f"controller collision trip (mode={st['robot_mode']}, " \
                   f"CollisionStates={st['collision']})"
        if st['error']:
            return f"controller alarm (ErrorStatus={st['error']})"
        if not self.soft:
            return None
        if (time.time() - self._t0) < self.blank_s:
            # Breakaway transient: torque swings by ~12 N*m here with nothing
            # touched. Judging it would trip every descend in the first
            # millimetre, which is exactly what happened before this existed.
            return None

        now = time.time()
        base = self.mon.torque_window(now - self.lag - self.win, now - self.lag)
        recent = self.mon.torque_window(now - 0.06, now)
        if len(base) < 5 or len(recent) < 2:
            return None                    # not enough history yet (start of move)
        delta = np.abs(np.median(np.array(recent), axis=0)
                       - np.median(np.array(base), axis=0))
        self.margin = delta
        if np.any(delta > self.thresholds):
            self._streak += 1
            if self._streak >= self.hold:
                j = int(np.argmax(delta - self.thresholds))
                return (f"joint torque step J{j + 1} {delta[j]:.2f} N*m "
                        f"> {self.thresholds[j]:.2f}")
        else:
            self._streak = 0
        return None


# --------------------------------------------------------------------------
# dashboard services
# --------------------------------------------------------------------------

class Dashboard:
    """Thin wrapper over the bringup's dashboard service proxies.

    The 29999 dashboard socket is single-client and bringup owns it, so these
    settings have to travel as ROS services rather than a second socket (unlike
    the 30004 feed above, which is multi-client).
    """

    def __init__(self, node):
        self.node = node
        self.cli = {}
        for name, typ in (('SetCollisionLevel', SetCollisionLevel),
                          ('SetPostCollisionMode', SetPostCollisionMode),
                          ('SetBackDistance', SetBackDistance),
                          ('ClearError', ClearError),
                          ('RobotMode', RobotMode),
                          ('SpeedFactor', SpeedFactor),
                          ('ServoJ', ServoJ)):
            self.cli[name] = node.create_client(
                typ, f'/dobot_bringup_ros2/srv/{name}',
                callback_group=node.cb_group)

    def call(self, name, timeout=5.0, **kwargs):
        """Blocking service call; returns the response or None."""
        cli = self.cli[name]
        if not cli.wait_for_service(timeout_sec=timeout):
            self.node.get_logger().error(f"[dash] {name} service unavailable")
            return None
        req = cli.srv_type.Request()
        for k, v in kwargs.items():
            setattr(req, k, v)
        fut = cli.call_async(req)
        t0 = time.time()
        while rclpy.ok() and not fut.done():
            if time.time() - t0 > timeout:
                self.node.get_logger().error(f"[dash] {name} timed out")
                return None
            time.sleep(0.005)
        return fut.result()

    def servoj(self, q_rad, t=SERVOJ_DT):
        """Fire-and-forget ServoJ, degrees, in the CONTROLLER's joint convention.

        The conversion MUST happen here. node.py applies it inside execute_path /
        execute_trajectory, but the descend deliberately bypasses those (they go
        through an action that cannot be cancelled, and stopping on contact is
        the whole point). Sending URDF-convention angles straight to ServoJ
        commands J1/J5/J6 mirrored -- up to 370 deg of error on J1 -- and the arm
        lunges for it until the controller protective-stops. That is exactly what
        happened on the first --run attempt.
        """
        req = ServoJ.Request()
        deg = [math.degrees(v) for v in np.asarray(q_rad) * self.node.joint_sign]
        req.a, req.b, req.c, req.d, req.e, req.f = (float(v) for v in deg)
        req.param_value = [f"t={t}"]
        self.cli['ServoJ'].call_async(req)

    def arm_touchoff(self, level, back_distance=0.0):
        """Configure the controller's collision touch-off (place_command_guide.md).

        No SetPayload call: nothing is carried (no gripper, no object), so the
        controller's factory payload model is already the correct one.
        """
        ok = True
        for name, kw in (('SetCollisionLevel', dict(level=int(level))),
                         ('SetPostCollisionMode', dict(mode=1)),     # 1 = pause
                         ('SetBackDistance', dict(distance=float(back_distance)))):
            res = self.call(name, **kw)
            if res is None or getattr(res, 'res', -1) != 0:
                self.node.get_logger().error(
                    f"[dash] {name}{kw} -> {getattr(res, 'res', 'no response')}")
                ok = False
            else:
                self.node.get_logger().info(f"[dash] {name}{kw} ok")
        return ok

    def clear_error(self):
        res = self.call('ClearError')
        self.node.get_logger().info(f"[dash] ClearError -> {getattr(res, 'res', None)}")
        return res is not None


# --------------------------------------------------------------------------
# taught points
# --------------------------------------------------------------------------

def own_like_dir(path):
    """Give a just-written file the owner of its directory.

    This script runs inside the container as root, but the workspace is the
    HOST user's checkout: anything written lands root-owned, and the operator
    then needs sudo to edit or delete their own taught points and measurement
    logs. Matching the directory keeps the repo the user's. No-op when the
    owner already matches or when we lack the privilege.
    """
    try:
        want = os.stat(os.path.dirname(os.path.abspath(path)))
        have = os.stat(path)
        if (have.st_uid, have.st_gid) != (want.st_uid, want.st_gid):
            os.chown(path, want.st_uid, want.st_gid)
        os.chmod(path, 0o644)
    except OSError as e:
        print(f"  (note: could not match ownership on {path}: {e})")


def load_points():
    if not os.path.exists(POINTS_FILE):
        return {}
    with open(POINTS_FILE) as f:
        return json.load(f)


def level_config(node, q):
    """Nearest config with the tool EXACTLY vertical, at the SAME TCP position.

    Why this exists: the carry is CBiRRT holding the tool attitude of wherever it
    STARTS. Teaching by hand lands a degree or two off vertical, so the arm then
    holds THAT tilt for the whole carry -- the tool never points down, it points
    wherever p1 happened to point. And a p2 taught a couple of degrees away is
    not on that manifold at all, so the plan ends somewhere else and the taught
    pose is reached by an unplanned final hop: the visible "re-align" at the
    place, and ~8 mm of landing error (measured).

    Jogging a wrist to a tenth of a degree is miserable; correcting it
    numerically is exact. Only the TILT is corrected -- the rotation is the
    minimal one taking the tool axis to straight down, so yaw about the tool
    axis is left where it was and J6 does not jump.

    The correction is LOCAL -- a damped-least-squares nudge seeded at the taught
    config, the same servo cbirrt.linear_path uses. Re-solving IK globally is
    what NOT to do: it returns every branch that reaches the pose and the
    "nearest" of those was measured 210 deg away on p1, i.e. a different arm
    posture entirely. The taught posture was chosen to clear the fixtures, so
    swapping branches to gain a degree of tilt is a bad trade.

    Returns (joints, moved_rad, tcp_err_m), or (None, None, None) if the servo
    does not converge or lands in collision.
    """
    m = node.ik_model
    model, data, fid = m.model, m.data, m.frame_id
    vidx = [model.idx_vs[model.getJointId(f'joint{i}')] for i in range(1, 7)]
    from cr7_pnp.geometry import TCP_OFFSET_M

    tcp0, R = m.fk_tcp(m.pin_q(list(q)))
    tz = R[:, 2]
    down = np.array([0.0, 0.0, -1.0])
    v = np.cross(tz, down)
    s = float(np.linalg.norm(v))
    c = float(np.dot(tz, down))
    if s < 1e-9:
        return list(q), 0.0, 0.0                    # already vertical
    vx = np.array([[0.0, -v[2], v[1]], [v[2], 0.0, -v[0]], [-v[1], v[0], 0.0]])
    R_new = (np.eye(3) + vx + vx @ vx * ((1.0 - c) / s ** 2)) @ R   # Rodrigues
    # Hold the TCP where it was taught. With the tool vertical the TCP sits
    # TCP_OFFSET_M straight below the frame origin, so the origin has to rise
    # by that much along +z.
    oMdes = pin.SE3(R_new, tcp0 - TCP_OFFSET_M * R_new[:, 2])

    qc = np.array(q, dtype=float)
    for _ in range(200):
        qp = m.pin_q(list(qc))
        pin.forwardKinematics(model, data, qp)
        pin.updateFramePlacement(model, data, fid)
        err = pin.log6(data.oMf[fid].actInv(oMdes)).vector
        if np.linalg.norm(err) < 1e-9:
            break
        J = pin.computeFrameJacobian(model, data, qp, fid, pin.LOCAL)
        J = -np.dot(pin.Jlog6(data.oMf[fid].actInv(oMdes).inverse()), J)[:, vidx]
        qc = qc + 0.5 * (-J.T @ np.linalg.solve(J @ J.T + 1e-8 * np.eye(6), err))
    else:
        if np.linalg.norm(err) > 1e-6:
            return None, None, None
    if not node.is_state_valid(list(qc)):
        return None, None, None
    new_tcp, _ = m.fk_tcp(m.pin_q(list(qc)))
    return (list(qc),
            float(np.linalg.norm(qc - np.array(q))),
            float(np.linalg.norm(new_tcp - tcp0)))


def save_point(node, name, tilt_tol=2.0, level=False):
    pts = load_points()
    q = node.current_joints.tolist()
    if level:
        before = axis_angle(tool_axis(node, q), np.array([0.0, 0.0, -1.0]))
        lev, moved, tcp_err = level_config(node, q)
        if lev is None:
            print("  !! --level: no valid IK branch with the tool exactly "
                  "vertical at this position. Saving the pose as jogged.")
        else:
            after = axis_angle(tool_axis(node, lev), np.array([0.0, 0.0, -1.0]))
            print(f"  --level: tilt {before:.2f} -> {after:.2f} deg   "
                  f"joints moved {math.degrees(moved):.2f} deg   "
                  f"TCP held to {tcp_err*1000:.2f} mm")
            q = lev
    pos, R = node.ik_model.fk_tcp(node.ik_model.pin_q(q))
    quat = pin.Quaternion(R)
    valid = node.is_state_valid(q)
    pts[name] = dict(
        joints=[float(v) for v in q],
        joints_deg=[round(math.degrees(v), 3) for v in q],
        tcp_xyz=[round(float(v), 5) for v in pos],
        tool_z=[round(float(v), 5) for v in R[:, 2]],
        quat_xyzw=[float(quat.x), float(quat.y), float(quat.z), float(quat.w)],
        collision_free=bool(valid),
        taught_at=time.strftime('%Y-%m-%d %H:%M:%S'),
    )
    with open(POINTS_FILE, 'w') as f:
        json.dump(pts, f, indent=2)
    own_like_dir(POINTS_FILE)
    print(f"\nsaved {name} -> {POINTS_FILE}")
    describe(name, pts[name])
    if not valid:
        print("  !! WARNING: this config is IN COLLISION in the planning model "
              "(self, or past a taught surface). CBiRRT will refuse to plan "
              "to or from it.")
    for other, p in pts.items():
        if other == name:
            continue
        if np.allclose(p['joints'], q, atol=1e-4):
            print(f"  !! WARNING: identical to {other}. The arm did not move "
                  f"between the two teaches, so there is nothing to carry. "
                  f"Jog to the other spot, then teach it again.")
            continue
        if {'home', 'obs'} & {name, other}:
            # home is only ever a park and obs only ever a viewpoint; no carry
            # runs to or from either, so their tool attitude is nobody's business.
            continue
        # The carry holds the tool attitude, so what matters is not how vertical
        # each point is on its own but the angle BETWEEN them. Report it now,
        # while the arm is still there and a small wrist tweak is cheap.
        d = tilt_between(pts[name], p)
        print(f"  tool attitude vs {other}: {d:.2f} deg "
              f"({'OK' if d <= tilt_tol else 'TOO FAR -- the carry will refuse'})")
        if d > tilt_tol:
            print(f"     Aim for under ~0.5 deg off straight-down at BOTH "
                  f"points; then this difference takes care of itself.")
    return True


def describe(name, p):
    tilt = math.degrees(math.acos(max(-1.0, min(1.0, -p['tool_z'][2]))))
    print(f"  {name}: J(deg) " + " ".join(f"{v:+8.2f}" for v in p['joints_deg']))
    print(f"        TCP(m)  x={p['tcp_xyz'][0]:+.4f} y={p['tcp_xyz'][1]:+.4f} "
          f"z={p['tcp_xyz'][2]:+.4f}   tool tilt from straight-down: {tilt:5.1f} deg")
    print(f"        taught {p['taught_at']}   collision_free={p['collision_free']}")


# --------------------------------------------------------------------------
# bounding surfaces (table, walls)
# --------------------------------------------------------------------------
#
# A surface is a HALF-SPACE, named by the direction from the robot base toward
# it: '-z' is the table under the arm, '-y' the wall on the -y side, and so on.
# `at` is the coordinate of its face along that axis, in base_link; everything
# beyond it is solid. All three are measured and enforced the same way, so the
# table is simply the '-z' surface.

def parse_dir(spec):
    """'z-' -> ('z', -1). Either order works.

    Write the axis first ('z-', 'y-') on the command line: argparse reads a
    bare '-z' as a flag rather than as this option's value. The sign-first
    spelling still parses, for '--teach-surface=-z' and for stored files.
    """
    s = spec.strip().lower()
    if len(s) == 2:
        if s[0] in AXES and s[1] in '+-':
            return s[0], (1 if s[1] == '+' else -1)
        if s[0] in '+-' and s[1] in AXES:
            return s[1], (1 if s[0] == '+' else -1)
    raise ValueError(f"direction must be one of x- x+ y- y+ z- z+, got {spec!r}")


def dir_name(axis, sign):
    return f"{'+' if sign > 0 else '-'}{axis}"


def span_of(bounds, axis):
    """(lo, hi) of a surface along `axis`, unbounded sides filled with the slab.

    A bound is what makes a stepped workspace expressible: the base support and
    the lower shelf are both '-z' surfaces at different heights, told apart only
    by where each one starts and stops in y.
    """
    lo, hi = (bounds or {}).get(axis, (None, None))
    return (-PLANE_EXTENT / 2 if lo is None else float(lo),
            PLANE_EXTENT / 2 if hi is None else float(hi))


def box_of(axis, sign, at, bounds):
    """(size, centre) of the slab: PLANE_THICK along `axis` reaching to `sign`,
    and the bounded span along the other two."""
    size, centre = np.zeros(3), np.zeros(3)
    for ax, k in AXES.items():
        if ax == axis:
            size[k] = PLANE_THICK
            centre[k] = at + sign * PLANE_THICK / 2.0
        else:
            lo, hi = span_of(bounds, ax)
            size[k] = max(hi - lo, 1e-3)
            centre[k] = 0.5 * (lo + hi)
    return size, centre


def add_surface(node, name, axis, sign, at, bounds=None):
    """Register a surface in the planning collision model.

    Paired against every movable arm link, the way node.py registers the shelf
    boards. The arm-only model is rooted at base_link, so this placement is
    already in base_link -- unlike the shelf there is no AGV pose to compose
    with. Returns the geometry index, for move_surface().
    """
    import coal
    geom = node.collision.geom
    objs = geom.geometryObjects
    # The node is HubPickPlace, whose setup_planner parks a 'carried_box'
    # phantom on Link6 (its collision pairs stay OFF unless attach_box_collision
    # is called). It is not fitted on this robot, so it must not be fenced by
    # the taught table/walls either -- pairing it here would reject plans for a
    # box that is not there.
    arm_links = [i for i in range(len(objs))
                 if objs[i].parentJoint != 0 and objs[i].name != 'carried_box']
    size, centre = box_of(axis, sign, at, bounds)
    go = pin.GeometryObject(f"surface_{name}", 0, pin.SE3(np.eye(3), centre),
                            coal.Box(*size))
    idx = geom.addGeometryObject(go)
    for i in arm_links:
        geom.addCollisionPair(pin.CollisionPair(i, idx))
    node.collision.geom_data = geom.createData()
    return idx


def move_surface(node, idx, axis, sign, at, bounds=None):
    """Slide a registered surface to a new `at`. Its size does not depend on
    `at`, so only the placement moves -- which is what the bisection needs."""
    _, centre = box_of(axis, sign, at, bounds)
    node.collision.geom.geometryObjects[idx].placement = pin.SE3(np.eye(3), centre)
    node.collision.geom_data = node.collision.geom.createData()


def measure_surface(node, axis, sign, bounds=None, tol=2e-4):
    """Measure where the arm's collision geometry reaches furthest toward `sign`.

    Put the tool against the surface, then run this: it bisects the half-space
    inward from far away, and the deepest position that still leaves the current
    config collision-free has its face exactly on the arm's extreme point --
    which is where you just put it, on the table or against the wall.

    Measuring with the very model that will later enforce the plane is what
    makes this right, and it is why no tool dimension is needed. The modelled
    gripper is not the dummy gripper actually fitted, but both are rigid on
    Link6, so the offset between them cancels -- PROVIDED the wrist attitude
    when measuring matches the attitude when working. Reading the TCP instead
    would be wrong by a whole tool length.

    With `bounds` set, only the part of the arm inside them counts -- which is
    how the lower shelf gets measured without the arm over the base support
    interfering, and vice versa.

    Returns (at, touching_geometry_names) or (None, reason).
    """
    q = node.current_joints.tolist()
    # u grows as the solid region grows toward the arm, whichever way it faces,
    # so one bisection covers all six directions.
    def at_of(u):
        return -sign * u

    idx = add_surface(node, '_probe', axis, sign, at_of(-10.0), bounds)
    if not node.is_state_valid(q):
        return None, "the arm is already in self-collision at this config"
    lo, hi = -3.0, 3.0              # lo: clear of the arm. hi: engulfs the arm.
    move_surface(node, idx, axis, sign, at_of(hi), bounds)
    if node.is_state_valid(q):
        return None, ("bracket failed: the slab never reaches the arm -- with "
                      "bounds, that means no part of the arm is inside them")
    while hi - lo > tol:
        mid = 0.5 * (lo + hi)
        move_surface(node, idx, axis, sign, at_of(mid), bounds)
        if node.is_state_valid(q):
            lo = mid
        else:
            hi = mid
    move_surface(node, idx, axis, sign, at_of(hi), bounds)   # just into contact
    touching = [b if a.startswith('surface_') else a
                for a, b in node.collision.colliding_pairs(q)]
    move_surface(node, idx, axis, sign, at_of(lo), bounds)
    return at_of(lo), touching


def load_surfaces():
    if not os.path.exists(SURFACES_FILE):
        return {}
    with open(SURFACES_FILE) as f:
        return json.load(f)


def parse_bounds(bound_args):
    """[['y','-0.34','inf'], ...] -> {'y': (-0.34, None)}. inf/none = open."""
    def edge(v):
        s = str(v).strip().lower()
        return None if s in ('inf', '+inf', '-inf', 'none', '*', '') else float(s)
    out = {}
    for ax, lo, hi in bound_args or []:
        ax = ax.strip().lower()
        if ax not in AXES:
            raise ValueError(f"--bound axis must be x, y or z, got {ax!r}")
        out[ax] = (edge(lo), edge(hi))
    return out


def save_surface(name, entry):
    surfaces = load_surfaces()
    surfaces[name] = entry
    with open(SURFACES_FILE, 'w') as f:
        json.dump(surfaces, f, indent=2)
    own_like_dir(SURFACES_FILE)
    print(f"\nsaved '{name}' -> {SURFACES_FILE}")
    print("  Every plan from now on refuses to put any arm link past it.")
    print("  Re-teach p1/p2 if they were taught beyond it.")


def teach_surface(node, a, spec):
    """--teach-surface DIR: record the surface the tool is currently touching."""
    axis, sign = parse_dir(spec)
    name = a.name or dir_name(axis, sign)
    bounds = parse_bounds(a.bound)
    print(f"\nmeasuring '{name}': the surface facing {dir_name(axis, sign)}"
          + (f", bounded to {bounds}" if bounds else "") + " ...")
    at, info = measure_surface(node, axis, sign, bounds)
    if at is None:
        print(f"  measurement failed: {info}")
        return False
    print(f"  the arm reaches {axis} = {at:+.4f} m (base_link) at this pose")
    print(f"  the part that touches first: {', '.join(sorted(set(info)))}")
    if not any('Link6' in n or 'gripper' in n.lower() for n in info):
        # If the tool were really against the surface while some other link
        # measured further, that link would be INSIDE it -- impossible. So this
        # is not a reading of the surface, and saving it would register a plane
        # in the wrong place for every later plan.
        print("  !! that is NOT the flange/tool: at this pose something else on")
        print("     the arm sticks out further, so this is not the surface.")
        print("     Re-pose until the tool is the part touching it, then")
        print("     measure again. NOT saved.")
        print("     (--yes overrides, if you really mean this position.)")
        if not a.yes:
            return False
    at += sign * a.surface_offset       # positive offset = away from the robot
    if a.surface_offset:
        way = "outward" if a.surface_offset > 0 else "toward the robot"
        print(f"  applying --surface-offset {a.surface_offset:+.4f} m "
              f"({way}) -> {at:+.4f} m")
    save_surface(name, dict(
        axis=axis, sign=sign, at=float(at), bounds=bounds,
        measured_at_joints_deg=[round(math.degrees(v), 3)
                                for v in node.current_joints],
        touched=sorted(set(info)),
        taught_at=time.strftime('%Y-%m-%d %H:%M:%S')))
    return True


def set_surface(a, spec):
    """--set-surface DIR --at V: define a surface from a NUMBER, not the arm.

    For anything the tool cannot or should not be driven against: a wall that
    is not built yet, a distance taken off a drawing, or the vertical step
    between two levels. No robot needed.
    """
    axis, sign = parse_dir(spec)
    name = a.name or dir_name(axis, sign)
    bounds = parse_bounds(a.bound)
    save_surface(name, dict(
        axis=axis, sign=sign, at=float(a.at), bounds=bounds,
        touched=['(given, not measured)'],
        taught_at=time.strftime('%Y-%m-%d %H:%M:%S')))
    return True


def describe_surface(name, s):
    face = f"{s['axis']} = {s['at']:+.4f} m, solid on the " \
           f"{'+' if s['sign'] > 0 else '-'} side"
    limits = ", ".join(
        f"{ax} in [{'-inf' if lo is None else f'{lo:+.3f}'}, "
        f"{'+inf' if hi is None else f'{hi:+.3f}'}]"
        for ax, (lo, hi) in sorted((s.get('bounds') or {}).items()))
    print(f"  '{name}': {face}" + (f"   bounded {limits}" if limits else ""))
    print(f"        from {', '.join(s['touched'])} at {s['taught_at']}")


def register_surfaces(node, a):
    """Put the taught surfaces into the model before anything is planned."""
    if a.no_surfaces:
        print("[collision] --no-surfaces: the taught table/walls are NOT "
              "registered. Nothing stops a plan from driving into them.")
        return
    surfaces = load_surfaces()
    if not surfaces:
        print("[collision] no surfaces taught (run --teach-surface z- for the "
              "table). Nothing but the arm itself is in the model -- only the "
              "descend is protected, by contact detection.")
        return
    for name, s in sorted(surfaces.items()):
        add_surface(node, name, s['axis'], s['sign'], s['at'], s.get('bounds'))
        print(f"[collision] surface '{name}' registered: {s['axis']} = "
              f"{s['at']:+.4f} m (from {s['taught_at']})")


def tilt_between(p1, p2):
    """Angle between the two taught tool axes, degrees."""
    return axis_angle(np.array(p1['tool_z'], dtype=float),
                      np.array(p2['tool_z'], dtype=float))


def axis_angle(a, b):
    c = float(np.dot(a, b) / (np.linalg.norm(a) * np.linalg.norm(b)))
    return math.degrees(math.acos(max(-1.0, min(1.0, c))))


def tool_axis(node, q):
    """Tool (approach) axis in base_link for a joint config."""
    _, R = node.ik_model.fk_tcp(node.ik_model.pin_q(list(q)))
    return R[:, 2]


def flange_in_base(node, q):
    """Flange position in base_link -- directly comparable with the robot's own
    GetPose(user=0, tool=0).

    NOT tcp_xyz(): that reports in the MODEL ROOT, which sits 30 mm below
    base_link, and it adds TCP_OFFSET_M for a gripper this robot does not have
    fitted. Both are harmless inside the planner, where only differences matter,
    but they make the printed numbers look 150 mm wrong next to the pendant's.
    """
    m = node.ik_model
    qp = m.pin_q(list(q))
    pin.forwardKinematics(m.model, m.data, qp)
    pin.updateFramePlacements(m.model, m.data)
    base = m.data.oMf[m.model.getFrameId('base_link')]
    return base.actInv(m.data.oMf[m.frame_id]).translation


def tilt_gap(node, q_a, q_b):
    """Angle between the tool axes of two joint configs, degrees."""
    return axis_angle(tool_axis(node, q_a), tool_axis(node, q_b))


# --------------------------------------------------------------------------
# motion primitives
# --------------------------------------------------------------------------

def carry(node, goal_q, label, speed, time_limit, dry=False, from_q=None,
          drift_tol=0.02):
    """CBiRRT from the current config to goal_q, holding the tool tilt.

    The tilt reference is taken from the START pose, so the whole path keeps the
    tool pointing the way it points there -- the same guarantee that keeps a
    grasped part level in the wirebonder sequence.

    Because ConstrainedPlanner.plan() PROJECTS its goal onto that manifold, a
    goal whose tilt differs from the start is silently replaced by a nearby
    config that is on it -- the arm would then stop somewhere that is not the
    taught point. So the projection is checked here and the move refuses rather
    than landing short.

    from_q overrides the start config: --dry uses it to plan each leg from where
    the previous leg would have ENDED, so the p1 -> p2 carry is really checked
    rather than being re-planned from wherever the arm happens to be parked.

    drift_tol is how far, in joint-space radians, the projected goal may sit from
    the taught one. Measured on this arm: ~2 deg of tool-attitude mismatch is
    ~0.02 rad of drift and ~5 mm of landing error.
    """
    start_q = list(from_q) if from_q is not None else node.current_joints.tolist()
    _, R = node.ik_model.fk_tcp(node.ik_model.pin_q(start_q))
    quat = pin.Quaternion(R)
    node.cbirrt.set_reference((quat.x, quat.y, quat.z, quat.w))

    projected = node.cbirrt._project(list(goal_q))
    if projected is None:
        print(f"  [{label}] goal does not project onto the tilt manifold; refusing")
        return False
    drift = float(np.linalg.norm(np.array(projected) - np.array(goal_q)))
    if drift > drift_tol:                  # rad, ~1.1 deg summed over 6 joints
        print(f"  [{label}] taught goal is {drift:.3f} rad off the tilt manifold "
              f"of the current pose -- the arm would land somewhere else. "
              f"Re-teach p1/p2 with the same tool tilt. Refusing.")
        return False
    if not node.is_state_valid(list(goal_q)):
        print(f"  [{label}] goal config is in collision; refusing")
        return False

    print(f"  [{label}] planning (CBiRRT, tilt held) ...")
    path = node.cbirrt.plan(start_q, list(goal_q), node.is_state_valid,
                            node.joint_limits, time_limit=time_limit)
    if not path:
        print(f"  [{label}] CBiRRT found no path in {time_limit:.0f}s")
        return False
    if np.linalg.norm(np.array(path[0]) - np.array(start_q)) > 1e-3:
        path = [start_q] + path
    # plan() returns the PROJECTED goal; land on the taught config exactly.
    path.append(list(goal_q))
    print(f"  [{label}] path {len(path)} waypoints, {path_length(path):.2f} rad")
    if dry:
        print(f"  [{label}] --dry: not executing")
        return True
    return node.execute_path(path, speed=speed)


def retrace(node, mon, a, path, label="retrace", j6_offset=0.0):
    """Replay the recorded outbound path in reverse -- node.replay_reverse().

    The recording and the reverse-replay are the node's own (HubPickPlace.
    capture / replay_reverse): the same machinery the sim factory sequence uses
    to come back from a pick without re-solving IK. Planning the return instead
    is what sent the arm FORWARD into the magazine (2026-08-11, e-stopped):
    CBiRRT holds the tool tilt, but its path is a random tree and neither the
    magazine nor the shelf is in any collision model.

    What is added here is the check the module does not make: that the arm is
    still where the recording ENDS. Replaying from somewhere else is not a
    retrace. The vertical lift out of the slot happens before this, in
    station().
    """
    if len(path) < 2:
        print(f"  [{label}] nothing recorded to retrace")
        return True
    if j6_offset:
        # Same trick as the sequence's offset_j6(rev(insert_path), twist_delta):
        # J6 turns about the tool axis, so shifting every waypoint by the twist
        # follows the identical line with the wrist left where the grasp put it.
        path = node.offset_j6(path, j6_offset)
    q_now = node.current_joints.tolist()
    gap = max(abs(math.degrees(x - y)) for x, y in zip(q_now, path[-1]))
    print(f"  [{label}] replaying {len(path)} waypoints backwards "
          f"({path_length(path):.2f} rad); arm is {gap:.1f} deg from the "
          f"recorded end")
    if gap > 15.0:
        print(f"  [{label}] REFUSING: the arm is not where the recording ends. "
              f"Replaying from somewhere else is not a retrace. Move it clear "
              f"by hand instead.")
        return False
    if a.dry:
        print(f"  [{label}] --dry: not executing")
        return True
    # Append where the arm actually IS before reversing, so the replay starts
    # from here instead of jumping. replay_reverse() sends the reversed list
    # with its first waypoint at t=0: in the sim sequence that is harmless
    # (it is called straight after the forward motion, gap ~0), but the descend
    # and the lift leave a few degrees of gap, and a few degrees demanded in
    # zero seconds is a following error -- the controller called it a collision
    # and protective-stopped (2026-08-11, mode 11).
    return node.replay_reverse(list(path) + [q_now], speed=a.speed)


def approach(node, goal_q, label, speed, time_limit, dry=False, from_q=None,
             tol_deg=2.0, drift_tol=0.02):
    """Reach goal_q with NOTHING carried.

    Prefers the tilt-held CBiRRT path -- the same motion the loaded carry uses,
    and the one that keeps the tool pointing where it already points. That is
    only possible when the arm ALREADY holds the goal's tool attitude, which is
    the normal case right after teaching (teaching leaves the arm at p2, and p1
    and p2 share an attitude by construction).

    From a parked or home pose the tool usually points somewhere else entirely,
    and NO tilt-held path can reorient it -- the constraint forbids exactly that.
    Since nothing is carried, reorienting is safe, so this falls back to the
    unconstrained joint-space RRT. It says so first: that RRT is a random tree,
    so the path it returns can swing wide, and the object and the table are not
    in the collision model.
    """
    start_q = list(from_q) if from_q is not None else node.current_joints.tolist()
    gap = tilt_gap(node, start_q, goal_q)
    if gap <= tol_deg:
        return carry(node, goal_q, label, speed, time_limit, dry=dry,
                     from_q=start_q, drift_tol=drift_tol)

    print(f"  [{label}] the tool currently points {gap:.0f} deg away from its "
          f"attitude at the goal, so no tilt-held path exists.")
    print(f"  [{label}] falling back to a FREE joint-space RRT (nothing is "
          f"carried). It may swing wide -- watch it, estop ready.")
    if not node.is_state_valid(list(goal_q)):
        print(f"  [{label}] goal config is in collision; refusing")
        return False
    path = node.plan_rrt(np.array(start_q, dtype=float), list(goal_q))
    if not path:
        print(f"  [{label}] free RRT found no path")
        return False
    print(f"  [{label}] path {len(path)} waypoints, {path_length(path):.2f} rad")
    if dry:
        print(f"  [{label}] --dry: not executing")
        return True
    return node.execute_path(path, speed=speed)


def path_length(path):
    a = np.array(path, dtype=float)
    return float(np.sum(np.linalg.norm(np.diff(a, axis=0), axis=1)))


def wait_until_still(mon, tol_dps=0.6, hold=0.4, timeout=8.0):
    """Block until every joint velocity has been under tol_dps for `hold`.

    `_wait_settled` in node.py only checks POSITION against the last waypoint,
    which passes while the arm is still creeping toward it -- and it gives up
    after 4 s with a warning either way. Contact detection needs the arm
    genuinely stopped, so this watches qd_actual instead.
    """
    t0 = time.time()
    still_since = None
    while time.time() - t0 < timeout:
        st, _ = mon.state()
        if st is not None:
            if max(abs(v) for v in st['qd_actual']) < tol_dps:
                still_since = still_since or time.time()
                if time.time() - still_since >= hold:
                    return True
            else:
                still_since = None
        time.sleep(0.02)
    return False


def joint_gap_deg(mon, q_urdf, joint_sign):
    """Worst-joint gap, degrees, between a URDF-convention target and where the
    arm actually is. Returns None if the live feed has nothing yet.

    Compares in the CONTROLLER's convention, straight off the robot's own
    q_actual, so it is independent of anything the model believes."""
    st, _ = mon.state()
    if st is None:
        return None
    want = np.degrees(np.asarray(q_urdf) * joint_sign)
    have = np.asarray(st['q_actual'])
    return float(np.max(np.abs((want - have + 180.0) % 360.0 - 180.0)))


def guarded_descend(node, dash, det, max_drop, speed_mps, label, dry=False,
                    from_q=None, max_jump_deg=5.0):
    """Descend straight down, stopping the moment contact is detected.

    Returns the metres actually descended, or None if the descend could not
    start. The geometric path comes from the same Jacobian servo the rest of the
    stack uses (cbirrt.linear_path), so it is joint-limit- and self-collision-
    gated; what linear_path cannot know is the object, which is not modelled --
    that is what the contact channels are for.
    """
    start_q = list(from_q) if from_q is not None else node.current_joints.tolist()
    step = 0.003                            # linear_path's own sub-step, metres
    path, reachable, reason = node.cbirrt.linear_path(
        start_q, np.array([0.0, 0.0, -abs(max_drop)]),
        node.is_state_valid, node.joint_limits, step=step)
    if reachable < 1e-4:
        print(f"  [{label}] cannot descend at all -> {reason}")
        return None
    if reachable < abs(max_drop) - 1e-3:
        print(f"  [{label}] geometry limits the descend to {reachable * 1000:.0f} mm "
              f"of {abs(max_drop) * 1000:.0f} mm requested -> {reason}")
    print(f"  [{label}] descending up to {reachable * 1000:.0f} mm "
          f"at {speed_mps * 1000:.0f} mm/s, watching for contact ...")
    if dry:
        print(f"  [{label}] --dry: not executing")
        return 0.0

    # Distance along the descend for each waypoint linear_path returned.
    dists = [min(step * i, reachable) for i in range(len(path))]
    # The torque baseline is meaningless while the arm is still decelerating out
    # of the previous move: joint torque in motion differs from torque at rest by
    # far more than a contact does, so the very first tick reads as a huge step.
    # Measured: a descend started straight after an approach tripped on J3 at
    # 8.08 N*m having commanded 0.5 mm -- and the arm was still drifting UPWARD
    # 3.8 mm at the time. Wait for a real stop, then let the baseline window
    # refill with at-rest samples before the first tick is judged.
    if not wait_until_still(det.mon):
        print(f"  [{label}] the arm never came to rest; not descending")
        return None
    time.sleep(det.lag + det.win)

    per_tick = speed_mps * SERVOJ_DT
    det.reset()

    travelled = 0.0
    hit = None
    t_next = time.time()
    last_log = 0.0
    while travelled < reachable - 1e-6:
        hit = det.check()
        if hit:
            break
        travelled = min(travelled + per_tick, reachable)
        target = interp_path(path, dists, travelled)
        # Never hand the arm a target it is nowhere near. At 3 mm/s a tick moves
        # a joint by hundredths of a degree, so anything past a few degrees is
        # not a descend -- it is a framing or convention bug, and the arm would
        # answer it by lunging. Cheap to check, and it is the guard that would
        # have caught the missing URDF->controller flip in servoj().
        gap = joint_gap_deg(det.mon, target, node.joint_sign)
        if gap is not None and gap > max_jump_deg:
            print(f"  [{label}] ABORT: next target is {gap:.1f} deg from where "
                  f"the arm actually is (limit {max_jump_deg:.1f}). Not sending "
                  f"it. This is a bug, not a contact -- check the joint "
                  f"convention before running again.")
            return None
        dash.servoj(target)
        if travelled - last_log >= 0.005:   # progress line every 5 mm
            last_log = travelled
            print(f"      {travelled * 1000:5.1f} mm   "
                  + ("torque detection blind (start-up transient)"
                     if det.blanked() else
                     "torque delta " + " ".join(f"{v:4.1f}" for v in det.margin)
                     + "  vs trip "
                     + " ".join(f"{v:4.1f}" for v in det.thresholds[:5]) + " ..."))
        t_next += SERVOJ_DT
        time.sleep(max(0.0, t_next - time.time()))

    # One last look: contact can land in the final tick.
    hit = hit or det.check()
    time.sleep(0.3)                         # let the arm settle onto the target

    # Report and retract on what the arm ACTUALLY did, not on what was commanded.
    # The two differ whenever the controller stops the arm itself (its collision
    # pause) or refuses the stream outright (not enabled, control not handed to
    # TCP mode) -- and retracting by a commanded distance the arm never
    # descended would drive it that far ABOVE where it started.
    actual = float(node.tcp_xyz(start_q)[2] - node.tcp_xyz()[2])
    if travelled < 1e-6:
        print(f"  [{label}] did not move: {hit or 'no reason reported'}")
        return None
    if hit:
        print(f"  [{label}] CONTACT after {actual * 1000:.1f} mm "
              f"({travelled * 1000:.1f} mm commanded) -> {hit}")
    else:
        print(f"  [{label}] reached {actual * 1000:.1f} mm with NO contact "
              f"(object lower than the {abs(max_drop) * 1000:.0f} mm budget, "
              f"or the trip thresholds are too high)")
    if actual < 0.5 * travelled - 1e-3:
        print(f"  [{label}] !! the arm followed only {actual * 1000:.1f} of the "
              f"{travelled * 1000:.1f} mm commanded. Retracting by the measured "
              f"amount. If it never moved, check that the robot is enabled and "
              f"that the pendant is in TCP/IP secondary-development mode.")
    return max(0.0, actual)


def interp_path(path, dists, d):
    """Joint config at descend distance `d`, linearly between waypoints."""
    if d <= dists[0]:
        return path[0]
    if d >= dists[-1]:
        return path[-1]
    i = int(np.searchsorted(dists, d))
    lo, hi = dists[i - 1], dists[i]
    a = 0.0 if hi <= lo else (d - lo) / (hi - lo)
    return [p + (q - p) * a for p, q in zip(path[i - 1], path[i])]


def recover_after_contact(node, dash, mon, timeout=5.0):
    """Clear a controller collision trip so the arm can be commanded again.

    Harmless when nothing tripped (the soft channel usually stops the stream
    before the firmware does) -- ClearError on a clean robot is a no-op.
    """
    st, _ = mon.state()
    if st and (st['robot_mode'] == MODE_COLLISION or st['collision'] or st['error']):
        print("      controller is in collision/alarm; clearing ...")
    dash.clear_error()
    t0 = time.time()
    while time.time() - t0 < timeout:
        st, _ = mon.state()
        if st and st['robot_mode'] not in (MODE_COLLISION, MODE_PAUSE) \
                and not st['collision'] and not st['error']:
            return True
        time.sleep(0.1)
    st, _ = mon.state()
    print(f"      !! robot did not leave the collision/alarm state "
          f"(mode={st['robot_mode'] if st else '?'}); stopping the cycle")
    return False


def lift(node, height, label, speed):
    """Straight-up retract by `height` metres."""
    if height < 1e-4:
        return True
    print(f"  [{label}] lifting {height * 1000:.0f} mm")
    return node.linear_servo([0.0, 0.0, abs(height)], speed=speed, label=label)


# --------------------------------------------------------------------------
# the cycle
# --------------------------------------------------------------------------

def settled(mon, a, what):
    """Every leg must end with the arm genuinely stopped before the next one
    reads its pose. The next leg takes its CBiRRT tilt reference from wherever
    the arm is; reading that mid-creep put the reference off by enough that the
    carry then refused (measured: 0.111 rad of projection drift after an ascend
    that ended 0.069 rad short)."""
    if a.dry:
        return True
    if wait_until_still(mon):
        return True
    print(f"  [{what}] the arm is still moving after 8 s; stopping")
    return False


def station(node, dash, mon, det, name, note, a, sim_q):
    """Descend to contact at the current station, dwell, and retract.

    The dwell stands in for the grasp (or the release): there is no gripper yet,
    so the arm just waits at the contact height for a.dwell seconds.
    """
    print(f"--- descend at {name} ---")
    dropped = guarded_descend(node, dash, det, a.drop, a.descend_speed,
                              f"{name} descend", dry=a.dry, from_q=sim_q)
    if dropped is None:
        return False
    if not a.dry and not recover_after_contact(node, dash, mon):
        return False

    print(f"  holding {a.dwell:.0f} s ({note}) ...")
    if not a.dry:
        time.sleep(a.dwell)

    if not a.dry and not lift(node, dropped + a.clearance, f"{name} ascend", a.speed):
        print(f"  [{name} ascend] retract failed; stopping with the arm down")
        return False
    return settled(mon, a, f"{name} ascend")


def run_cycle(node, dash, mon, det, pts, a):
    """One p1 -> p2 -> p1 transfer. Returns True when it completed.

    Only the p1 -> p2 leg is a strict CBiRRT carry: that is the leg where a part
    would be in the jaws, so that is the leg whose tool attitude must be held.
    The two empty legs go through approach(), which uses the same CBiRRT when it
    can and drops the constraint when the tool has to be reoriented.
    """
    p1, p2 = pts['p1'], pts['p2']
    # --dry bookkeeping: where the arm WOULD be, since nothing actually moves.
    # Stays None during a real run so every leg reads the live joint states.
    sim_q = None

    print("\n--- approach p1 (empty) ---")
    if not approach(node, p1['joints'], "goto p1", a.speed, a.plan_time,
                    dry=a.dry, from_q=sim_q, tol_deg=a.tilt_tol,
                    drift_tol=a.drift_tol):
        return False
    if a.dry:
        sim_q = list(p1['joints'])
    if not settled(mon, a, "goto p1"):
        return False
    if not station(node, dash, mon, det, 'p1', 'as if the jaws closed', a, sim_q):
        return False

    print("\n--- carry p1 -> p2 (CBiRRT, tool attitude held) ---")
    if not carry(node, p2['joints'], "carry p1->p2", a.speed, a.plan_time,
                 dry=a.dry, from_q=sim_q, drift_tol=a.drift_tol):
        return False
    if a.dry:
        sim_q = list(p2['joints'])
    if not settled(mon, a, "carry p1->p2"):
        return False
    if not station(node, dash, mon, det, 'p2', 'as if the jaws opened', a, sim_q):
        return False

    # Park at 'home' if one was taught, else back at p1. Ending on p1 leaves the
    # tool hovering over the part, which is a poor place to leave it between
    # runs; a taught home is somewhere clear. Repeated --cycles still work
    # because the next cycle opens with its own approach to p1.
    park = pts.get('home') or p1
    where = 'home' if 'home' in pts else 'p1'
    print(f"\n--- return to {where} (empty) ---")
    if not approach(node, park['joints'], f"return {where}", a.speed, a.plan_time,
                    dry=a.dry, from_q=sim_q, tol_deg=a.tilt_tol,
                    drift_tol=a.drift_tol):
        return False
    if a.dry:
        sim_q = list(park['joints'])
    return True


# --------------------------------------------------------------------------
# vision hover (--vision)
# --------------------------------------------------------------------------
#
# One-way trip: observe -> ask the camera where the magazine is -> stand the
# FLANGE 10 cm above its top face -> turn J6 to line the grasp up. NO DESCEND.
# The descend needs a collision level that reflects the magazine, and the
# magazine is in no collision model, so it waits for its own instruction.
#
# What this measures is the ABSOLUTE bias of the whole chain (camera ->
# transform -> IK -> controller). Everything before it was repeatability: the
# same point seen from different poses landing on itself. Only a tape measure
# against the real magazine says whether that point is where the magazine
# actually is.


def j6_target(node, q_from, deg):
    """q_from with J6 turned by `deg`. Returns (q, None) or (None, reason).

    No automatic sign flip: if +90 runs past the limit the operator gets told to
    pass --j6-deg -90 and decide. Flipping it here would silently reverse the
    grasp approach direction, which is the one thing this step is meant to set.
    """
    q = list(q_from)
    q[5] = q_from[5] + math.radians(deg)
    lo, hi = node.joint_limits[5]
    if not (lo <= q[5] <= hi):
        return None, (f"J6 would reach {math.degrees(q[5]):+.1f} deg, past the "
                      f"limit [{math.degrees(lo):+.0f}, {math.degrees(hi):+.0f}]. "
                      f"Try the other sign: --j6-deg {-deg:+.0f}")
    for i in range(1, 13):                      # sweep, not just the endpoint
        qi = list(q_from)
        qi[5] = q_from[5] + math.radians(deg) * i / 12.0
        if not node.is_state_valid(qi):
            return None, (f"the J6 sweep collides at "
                          f"{math.degrees(qi[5]):+.1f} deg "
                          f"({i}/12). Try the other sign: --j6-deg {-deg:+.0f}")
    return q, None


def run_vision(node, mon, a):
    """--vision: obs -> vision -> hover -> J6. Returns True when it completed."""
    # zmq/msgpack are only needed here, so the import stays local: --teach and
    # --run must not start failing because the vision stack is not installed.
    import vision_target as vt
    if a.j6_deg is None:
        a.j6_deg = vt.J6_ROT_DEG

    # The dry/run switch for this mode is --run; everything reused below
    # (approach, settled) reads a.dry, so set it once here.
    a.dry = not a.run

    pts = load_points()
    if 'obs' not in pts:
        print("no observation pose taught: jog the arm so the camera sees the "
              "magazine, then run --teach obs")
        return False
    describe('obs', pts['obs'])

    dash = Dashboard(node)
    if a.run:
        if a.speed_factor:
            dash.call('SpeedFactor', ratio=int(a.speed_factor))
            print(f"[dash] SpeedFactor({a.speed_factor}%) set -- it PERSISTS on "
                  f"the controller after this script exits")
        if not dash.arm_touchoff(a.collision_level):
            print("could not configure the collision touch-off; aborting")
            return False
        dash.clear_error()
        print(f"\nabout to move the REAL robot: obs -> vision hover -> J6 "
              f"{a.j6_deg:+.0f} deg. NO DESCEND. joint speed {a.speed} rad/s, "
              f"collision level {a.collision_level}")
        if not a.yes:
            try:
                go = input("proceed? [y/N] ").strip().lower()
            except EOFError:           # piped/non-interactive: no one to ask
                go = ''
            if go not in ('y', 'yes'):
                print("aborted (pass --yes to skip this prompt)")
                return False

    # CBiRRT ONLY. approach()'s free joint-space RRT fallback is deliberately not
    # used here: the factory sequence reaches the magazine under the tilt
    # constraint, grasps, and comes back by REPLAYING THE RECORDED PATH in
    # reverse with no IK re-solve. A path that only exists because the
    # constraint was dropped cannot carry that guarantee, so this mode refuses
    # rather than falling back -- carry() is the strict planner.
    print("\n--- 1. approach the observation pose 'obs' (CBiRRT, tilt held) ---")
    # Check the tilt gap BEFORE carry() does, so the operator gets an
    # obs-specific instruction instead of carry()'s p1/p2 wording (that message
    # is correct for the pick-place rehearsal, misleading here).
    gap_obs = tilt_gap(node, node.current_joints.tolist(), pts['obs']['joints'])
    if gap_obs > a.tilt_tol:
        print(f"  the arm's tool is {gap_obs:.2f} deg away from the attitude "
              f"stored in obs (limit {a.tilt_tol:.1f}). CBiRRT holds the tool "
              f"attitude, so it cannot turn one into the other.")
        print("  obs was taught --level (exactly vertical), so level the ARM "
              "too: jog Rx/Ry until")
        print("      test/vision_target.py --tilt   reads under 0.5 deg")
        print("  (position does not matter -- only the tilt.) This mode does "
              "NOT fall back to a free RRT.")
        return False
    # Already there? Do nothing. CBiRRT grows a random tree even when start ==
    # goal, and the repeat loop (--goto obs, run, measure, run again) starts
    # every cycle exactly at obs -- so without this the arm wandered 0.56 rad
    # out and back before every single measurement.
    segs = []           # node.capture() 가 돌려주는 실행 경로 -- 복귀는 이걸 역재생
    worst_obs = max(abs(math.degrees(g - c)) for c, g
                    in zip(node.current_joints.tolist(), pts['obs']['joints']))
    if worst_obs < 0.5:
        print(f"  already at obs (worst joint {worst_obs:.3f} deg). Not moving.")
    elif not carry(node, pts['obs']['joints'], "goto obs", a.speed, a.plan_time,
                   dry=a.dry, drift_tol=a.drift_tol):
        print("  no tilt-held path to obs. This mode does NOT fall back to a "
              "free RRT (the return leg replays this path in reverse). Jog the "
              "wrist near the obs attitude and retry, or widen --drift-tol.")
        return False
    if not settled(mon, a, "goto obs"):
        return False
    # In --dry nothing moved, so plan the next leg from where the arm WOULD be.
    sim_q = list(pts['obs']['joints']) if a.dry else None
    if a.dry:
        print("  (--dry: the arm did NOT move -- the vision read below is taken "
              "from wherever it is standing right now)")

    if a.target:
        print("\n--- 1b. coarse look -> pick -> move to a close viewpoint ---")
        try:
            cands, tool0 = vt.coarse_detections(mon)
            chosen, rule = vt.pick_target(cands, a.target)
            print(f"  [coarse] '{rule}' -> ID{chosen['track_id']} "
                  f"{chosen['cls']}  P_base "
                  f"[{chosen['P_base'][0]:+.3f} {chosen['P_base'][1]:+.3f} "
                  f"{chosen['P_base'][2]:+.3f}]  (조준 전용 좌표)")
            q_view, ikv = vt.viewpoint_q(node, chosen['P_base'],
                                         node.current_joints.tolist(),
                                         view_dist=a.view_dist)
        except vt.VisionTargetError as e:
            print(f"\n관측 자세를 만들 수 없다 -- 이동하지 않는다:\n  {e}")
            return False
        print("  [coarse] viewpoint joints (deg): "
              + " ".join(f"{math.degrees(v):+8.2f}" for v in q_view)
              + f"   (yaw {ikv['viewpoint_yaw_deg']:+.1f} deg, "
                f"카메라를 전면 {ikv['view_dist_m']*1000:.0f} mm 앞으로)")
        gap_v = tilt_gap(node, node.current_joints.tolist(), q_view)
        if gap_v > a.tilt_tol:
            print(f"  REFUSING: 관측 자세와 툴 기울기가 {gap_v:.2f} deg 벌어져 "
                  f"tilt-held 경로가 없다 (한도 {a.tilt_tol:.1f})")
            return False
        if a.run:
            ok, seg = node.capture(
                lambda: carry(node, q_view, "goto viewpoint", a.speed,
                              a.plan_time, dry=False, drift_tol=a.drift_tol))
            if not ok:
                return False
            segs.append(seg)
            if not settled(mon, a, "goto viewpoint"):
                return False
            if not arrived(node, mon, q_view, "goto viewpoint"):
                return False
        else:
            print("  --dry: 관측 자세로 이동하지 않는다. 아래 스냅샷은 지금 자리에서 "
                  "읽은 것이라 게이트를 못 넘을 수 있다")

    print("\n--- 2. vision ---")
    try:
        q_hover, info = vt.acquire_hover_q(node, mon)
    except vt.VisionTargetError as e:
        print(f"\n비전 목표를 만들 수 없다 -- 이동하지 않는다:\n  {e}")
        if e.info:
            print(vt.dump_info(e.info))
        return False

    if a.target:
        info['coarse'] = dict(rule=a.target, candidates=cands,
                              chosen_track_id=chosen['track_id'],
                              viewpoint_joints=[float(v) for v in q_view], **ikv)
    hover = np.array(info['hover'], dtype=float)
    print("\n--- 3. plan ---")
    print(f"  hover (base_link, FLANGE origin): "
          f"x={hover[0]:+.4f} y={hover[1]:+.4f} z={hover[2]:+.4f} m")
    print("  target joints (deg): "
          + " ".join(f"{v:+8.2f}" for v in info['hover_joints_deg']))
    print(f"  J6 after alignment: "
          f"{info['hover_joints_deg'][5] + a.j6_deg:+.2f} deg "
          f"({a.j6_deg:+.0f} deg from the hover pose)")

    # The hover pose points the tool straight down by construction, and CBiRRT
    # HOLDS the attitude it starts with -- it cannot rotate the tool. So obs has
    # to point straight down too, or no constrained path exists and the only way
    # there would be the free RRT this mode refuses to use.
    start_q = sim_q if sim_q is not None else node.current_joints.tolist()
    gap = tilt_gap(node, start_q, q_hover)
    print(f"  tool attitude obs -> hover: {gap:.2f} deg apart "
          f"(limit {a.tilt_tol:.1f})")
    if gap > a.tilt_tol:
        print("  REFUSING: obs does not point the tool where hover needs it, so "
              "no tilt-held path exists. Re-teach it level (the arm does not "
              "move, only the stored config is snapped):")
        print("      run.sh --teach obs --level")
        return False
    if not a.run:
        print("\n--vision (dry): planned and checked, nothing moved. "
              "Add --run to execute.")
        return True

    print("\n--- 4. approach the hover pose (CBiRRT, tilt held) ---")
    # capture() records what execute_path actually drives, so the way back is a
    # replay of this leg rather than a fresh plan (see retrace()).
    ok, seg = node.capture(lambda: carry(node, q_hover, "goto hover", a.speed,
                                         a.plan_time, dry=False,
                                         drift_tol=a.drift_tol))
    if not ok:
        return False
    segs.append(seg)
    if not settled(mon, a, "goto hover"):
        return False
    if not arrived(node, mon, q_hover, "goto hover"):
        return False

    print(f"\n--- 5. J6 {a.j6_deg:+.0f} deg ---")
    q_now = node.current_joints.tolist()
    q_j6, why = j6_target(node, q_now, a.j6_deg)
    if q_j6 is None:
        print(f"  refusing to turn J6: {why}")
        print("  the arm stays at the hover pose.")
        return False
    # Same execution path as every other leg (execute_path applies the
    # URDF->controller sign flip and paces the move by a.speed). NOT
    # execute_trajectory: that one hands the controller a fixed 0.5 s per
    # waypoint, i.e. ~180 deg/s for this move.
    # move_single_joint sweeps 24 interpolated configs through the collision
    # model and then drives the two-point path -- the node's own primitive.
    # j6_target() above adds only the joint-LIMIT check, which it does not make.
    ok, _ = node.capture(
        lambda: node.move_single_joint(5, q_j6[5], speed=a.speed, label="j6"))
    if not ok:
        return False
    # The twist is NOT part of the retreat: sequences/shelf_pick_place.py comes
    # back out of the shelf with it HELD (offset_j6 on the reversed insert) and
    # un-twists at the hub, in open space. Un-twisting first would sweep the
    # jaws through 86 deg directly over the magazine.
    twist_delta = q_j6[5] - q_now[5]
    if not settled(mon, a, "j6"):
        return False
    if not arrived(node, mon, q_j6, "j6"):
        return False

    print("\n--- 6. arrived ---")
    q_end = node.current_joints.tolist()
    fl = flange_in_base(node, q_end)
    err = fl - hover
    st, _ = mon.state()
    tool = np.array(st['tool_vector'], dtype=float) if st else None
    print(f"  commanded hover : x={hover[0]:+.4f} y={hover[1]:+.4f} z={hover[2]:+.4f} m")
    print(f"  actual flange   : x={fl[0]:+.4f} y={fl[1]:+.4f} z={fl[2]:+.4f} m  "
          f"(model FK)")
    if tool is not None:
        t = tool[:3] / 1000.0
        print(f"  robot's own     : x={t[0]:+.4f} y={t[1]:+.4f} z={t[2]:+.4f} m  "
              f"(tool_vector = GetPose(user=0,tool=0))")
    print(f"  tracking error  : dx={err[0]*1000:+.1f} dy={err[1]*1000:+.1f} "
          f"dz={err[2]*1000:+.1f} mm  (|e| = {np.linalg.norm(err)*1000:.1f} mm)")
    info['arrival'] = dict(
        joints=[float(v) for v in q_end],
        joints_deg=[round(math.degrees(v), 3) for v in q_end],
        flange_in_base=[float(v) for v in fl],
        tool_vector=tool.tolist() if tool is not None else None,
        tracking_err_mm=[float(v * 1000) for v in err],
        j6_deg=float(a.j6_deg),
    )

    if not a.no_descend:
        print(f"\n--- 7. descend into the magazine (contact-stop) ---")
        print("  the magazine is NOT in any collision model -- the joint-torque "
              "channel is the only guard.")
        thresholds = TORQUE_TRIP
        if a.torque_trip is not None:
            thresholds = (a.torque_trip,) * 5 + (1e9,)
        warn_descend_speed(a.descend_speed)
        det = ContactDetector(mon, thresholds=thresholds,
                              soft=not a.no_torque_trip)
        # Same shape as sequences/shelf_pick_place.py's place side, which is the
        # pipeline this is validating:
        #     forward = capture(approach)
        #     drop = guarded_descend(...)          # contact-stop, not recorded
        #     linear_servo([0, 0, drop])           # undo EXACTLY the measured drop
        #     replay_reverse(forward)
        # Undoing exactly the drop is what puts the arm back where the recording
        # ends. station() (the p1<->p2 rehearsal) lifts by drop + --clearance
        # instead, and those extra millimetres were the 6.5 deg gap that made
        # replay_reverse command a jump and trip the controller (2026-08-11).
        dropped = guarded_descend(node, dash, det, a.drop, a.descend_speed,
                                  "magazine descend")
        if dropped is None:
            print("  the arm is left where it stopped. Lift it vertically "
                  "(--descend -N) before moving anywhere else.")
            return False
        if not recover_after_contact(node, dash, mon):
            return False
        print(f"  holding {a.dwell:.0f} s (as if the jaws closed) ...")
        time.sleep(a.dwell)
        if dropped > 1e-3 and not node.linear_servo(
                [0.0, 0.0, dropped], speed=a.speed, label="magazine ascend"):
            print("  ascend failed; the arm is left down. Lift it vertically "
                  "(--descend -N) by hand before anything else.")
            return False
        if not settled(mon, a, "magazine ascend"):
            return False
        # The ascend is a Cartesian servo undoing the measured drop, so it lands
        # near -- not exactly on -- the pre-descend config. 4 deg is the gate.
        if not arrived(node, mon, q_j6, "magazine ascend", tol_deg=4.0):
            return False
        q_after = node.current_joints.tolist()
        fl2 = flange_in_base(node, q_after)
        print(f"  descended {dropped * 1000:.1f} mm, undone exactly")
        print(f"  after the lift the flange is at z={fl2[2]:+.4f} m "
              f"(hover was z={hover[2]:+.4f})")
        info['descend'] = dict(
            drop_limit_m=float(a.drop), speed_mps=float(a.descend_speed),
            dropped_m=float(dropped),
            thresholds=[float(v) for v in thresholds[:5]],
            flange_after_lift=[float(v) for v in fl2])

    print("\n--- 8. retreat holding the twist, then un-twist at obs ---")
    # sequences/shelf_pick_place.py, pick side:
    #     out_path = join([rev(descend_path),
    #                      offset_j6(rev(insert_path), twist_delta)])
    #     execute_path(out_path); ... ; move_single_joint(5, hub_q[5])
    # Ours is the same shape with the ascend already done (the descend is a
    # ServoJ stream and cannot be recorded), so what is left to retrace is the
    # hover approach -- driven backwards with J6 still turned.
    outbound = node.join(segs)
    if not retrace(node, mon, a, outbound, label="return obs",
                   j6_offset=twist_delta):
        return False
    settled(mon, a, "return obs")
    if not a.dry:
        if outbound and not arrived(node, mon,
                                    node.offset_j6([outbound[0]], twist_delta)[0],
                                    "return obs"):
            return False
        # Un-twist here, in the open, exactly as the sequence does at the hub.
        if not node.move_single_joint(5, pts['obs']['joints'][5], speed=a.speed,
                                      label="untwist-at-obs"):
            return False
        if not settled(mon, a, "untwist"):
            return False
        if not arrived(node, mon, pts['obs']['joints'], "untwist-at-obs"):
            return False

    out = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                       time.strftime('vision_hover_%Y%m%d_%H%M%S.json'))
    vt.dump_info(info, out)
    own_like_dir(out)
    print("\n러너는 IDLE 로 돌려놨다.")
    return True


def arrived(node, mon, target_q, label, tol_deg=2.0):
    """Did the arm actually get there? Silent on success, error log on failure.

    execute_path returns True even when its _wait_settled times out -- it only
    warns and continues, which is right in sim (the controller answers on the
    command schedule) but on the real robot a PROTECTIVE STOP looks exactly the
    same: the action returns, the script carries on, and a trip reads as a
    completed cycle. It did (2026-08-11: mode 11 mid-retrace, cycle logged as
    finished). So every leg is checked against where it was told to go, and the
    controller state is read straight off the 30004 feed.

    Failures log like the sequences do (get_logger().error) and abort; success
    prints nothing.
    """
    st, _ = mon.state()
    if st and (st['robot_mode'] == MODE_COLLISION or st['collision']
               or st['error']):
        node.get_logger().error(
            f"[{label}] controller stopped the move: mode={st['robot_mode']} "
            f"collision={st['collision']} error={st['error']}")
        return False
    if node.current_joints is None:
        node.get_logger().error(f"[{label}] no joint states")
        return False
    err = max(abs(math.degrees(c - t)) for c, t
              in zip(node.current_joints.tolist(), target_q))
    if err > tol_deg:
        node.get_logger().error(
            f"[{label}] did not arrive: worst joint {err:.1f} deg from the "
            f"commanded target (limit {tol_deg:.1f})")
        return False
    return True


def warn_descend_speed(speed_mps, lag_s=0.4):
    """Say what a faster descend costs, in millimetres, instead of just 'don't'.

    The trip thresholds were measured at 3 mm/s (docs/real_robot_p1p2_test.md
    5.3). Two things change with speed: the detection lag (baseline 0.35 s + two
    ticks) turns into extra travel in proportion, and the steady-state torque
    noise floor rises, eating the margin. The first is arithmetic and is printed
    here; the second is why the printed torque deltas are worth watching.
    """
    if speed_mps <= 0.0031:
        return
    print(f"  note: {speed_mps * 1000:.0f} mm/s is above the 3 mm/s the trip "
          f"thresholds were measured at. The ~{lag_s:.1f} s detection lag now "
          f"costs ~{speed_mps * lag_s * 1000:.1f} mm of extra travel after "
          f"contact (1.2 mm at 3 mm/s), and the noise floor rises with speed -- "
          f"watch the printed torque deltas: back off if they climb past half "
          f"the trip values.")


def descend_only(node, dash, mon, a):
    """--descend MM: straight down from HERE, stopping on contact. Negative = up.

    Reuses guarded_descend unchanged -- the ServoJ stream, the torque channel
    and the per-tick safety gate are the parts that were measured on this arm,
    and the magazine is in no collision model, so that torque channel is the
    ONLY thing between the jaws and the box. Nothing else moves: no vision, no
    re-planning, no automatic retract (leaving the arm down is a decision for
    the operator, who is standing there looking at it).
    """
    st, _ = mon.state()
    if st and st['robot_mode'] != MODE_ENABLE:
        print(f"  robot_mode is {st['robot_mode']}, not {MODE_ENABLE} (ENABLE); "
              f"the move would be refused")
        return False

    mm = float(a.descend)
    if abs(mm) > 150.0:
        print(f"  {mm:.0f} mm is more than this mode allows (150). If that is "
              f"really meant, do it in steps and look between them.")
        return False

    q_now = node.current_joints.tolist()
    fl0 = flange_in_base(node, q_now)
    print(f"  flange now: x={fl0[0]:+.4f} y={fl0[1]:+.4f} z={fl0[2]:+.4f} m")

    if mm < 0:
        print(f"  lifting {abs(mm):.0f} mm (no contact detection needed going up)")
        if not a.yes:
            try:
                go = input("proceed? [y/N] ").strip().lower()
            except EOFError:
                go = ''
            if go not in ('y', 'yes'):
                print("aborted (pass --yes to skip this prompt)")
                return False
        if not lift(node, abs(mm) / 1000.0, "ascend", a.speed):
            return False
        settled(mon, a, "ascend")
        fl1 = flange_in_base(node, node.current_joints.tolist())
        print(f"  flange now: x={fl1[0]:+.4f} y={fl1[1]:+.4f} z={fl1[2]:+.4f} m "
              f"({(fl1[2] - fl0[2]) * 1000:+.1f} mm)")
        return True

    thresholds = TORQUE_TRIP
    if a.torque_trip is not None:
        thresholds = (a.torque_trip,) * 5 + (1e9,)
    det = ContactDetector(mon, thresholds=thresholds, soft=not a.no_torque_trip)

    warn_descend_speed(a.descend_speed)

    if not dash.arm_touchoff(a.collision_level):
        print("  could not configure the controller touch-off; aborting")
        return False
    dash.clear_error()

    print(f"\n  about to descend up to {mm:.0f} mm at "
          f"{a.descend_speed * 1000:.0f} mm/s, collision level "
          f"{a.collision_level}"
          + ("" if a.no_torque_trip else f", torque trip {thresholds[:5]}"))
    print("  the magazine is NOT in any collision model -- the torque channel "
          "is the only guard. Hand on the e-stop.")
    if not a.yes:
        try:
            go = input("proceed? [y/N] ").strip().lower()
        except EOFError:
            go = ''
        if go not in ('y', 'yes'):
            print("aborted (pass --yes to skip this prompt)")
            return False

    dropped = guarded_descend(node, dash, det, mm / 1000.0, a.descend_speed,
                              "descend")
    if dropped is None:
        return False
    recover_after_contact(node, dash, mon)
    fl1 = flange_in_base(node, node.current_joints.tolist())
    print(f"  flange now: x={fl1[0]:+.4f} y={fl1[1]:+.4f} z={fl1[2]:+.4f} m "
          f"({(fl1[2] - fl0[2]) * 1000:+.1f} mm)")
    print(f"  descended {dropped * 1000:.1f} mm. The arm stays here -- "
          f"'--descend -{abs(mm):.0f}' lifts it back.")
    return True


def j6_only(node, mon, a):
    """--j6-only DEG: turn J6 in place, nothing else moves.

    For finding the magazine's yaw by eye: the vision chain gives no
    orientation (the runner's plane fit keeps only the intercept), so the angle
    that makes the jaws parallel to the box is measured by looking. Running the
    whole --vision cycle for that would drive back to obs, re-read the camera
    and re-plan the hover -- minutes of motion to answer a question about one
    joint. Same guards as the alignment step: joint limit, 12-point collision
    sweep, no automatic sign flip.
    """
    st, _ = mon.state()
    if st and st['robot_mode'] != MODE_ENABLE:
        print(f"  robot_mode is {st['robot_mode']}, not {MODE_ENABLE} (ENABLE); "
              f"the move would be refused")
        return False
    q_now = node.current_joints.tolist()
    q_j6, why = j6_target(node, q_now, a.j6_only)
    if q_j6 is None:
        print(f"  refusing: {why}")
        return False
    print(f"  J6 {math.degrees(q_now[5]):+.2f} -> {math.degrees(q_j6[5]):+.2f} deg "
          f"({a.j6_only:+.1f} deg), everything else held")
    fl = flange_in_base(node, q_now)
    print(f"  flange stays at x={fl[0]:+.4f} y={fl[1]:+.4f} z={fl[2]:+.4f} m "
          f"(J6 turns about the tool axis, so the origin does not move)")
    if not a.yes:
        try:
            go = input("proceed? [y/N] ").strip().lower()
        except EOFError:
            go = ''
        if go not in ('y', 'yes'):
            print("aborted (pass --yes to skip this prompt)")
            return False
    if not node.move_single_joint(5, q_j6[5], speed=a.speed, label="j6-only"):
        return False
    if not settled(mon, a, "j6-only"):
        return False
    q_end = node.current_joints.tolist()
    print(f"  now at J6 {math.degrees(q_end[5]):+.2f} deg. Cumulative from the "
          f"hover pose is what --j6-deg should be on the next --vision --run.")
    return True


def goto_point(node, mon, a):
    """--goto NAME: go back to a taught point under the SAME strict CBiRRT.

    Exists because the vision hover is measured by repetition: run, tape-measure,
    return, run again. Returning by hand or from the pendant each time invites a
    different starting attitude, and the tilt constraint is what makes the
    outbound leg reproducible. No free-RRT fallback here either -- if the tool
    points the wrong way this refuses and says so, rather than swinging wide.
    """
    pts = load_points()
    if a.goto not in pts:
        print(f"'{a.goto}' is not taught (have: {', '.join(sorted(pts)) or 'none'})")
        return False
    describe(a.goto, pts[a.goto])
    goal = list(pts[a.goto]['joints'])

    st, _ = mon.state()
    if st and st['robot_mode'] != MODE_ENABLE:
        print(f"  robot_mode is {st['robot_mode']}, not {MODE_ENABLE} (ENABLE). "
              f"Drag mode on, or a collision/alarm pending -- the move would be "
              f"refused. Clear it first.")
        return False
    if not node.is_state_valid(goal):
        print(f"  '{a.goto}' is in collision in the planning model; refusing")
        return False

    q_now = node.current_joints.tolist()
    worst = max(abs(math.degrees(b - c)) for c, b in zip(q_now, goal))
    if worst < 0.5:
        # CBiRRT grows a random tree even when start == goal: asked to "return"
        # to where it already stood it produced a 0.58 rad round trip and drove
        # it. For a recovery command that excursion is pure risk, so a no-op
        # stays a no-op.
        print(f"  already at '{a.goto}' (worst joint {worst:.3f} deg). "
              f"Not moving.")
        return True
    gap = tilt_gap(node, q_now, goal)
    print("  joints to move (deg): "
          + " ".join(f"{math.degrees(b - c):+7.2f}" for c, b in zip(q_now, goal)))
    print(f"  tool attitude now -> {a.goto}: {gap:.2f} deg apart "
          f"(limit {a.tilt_tol:.1f})")
    if gap > a.tilt_tol:
        print("  no tilt-held path exists: CBiRRT holds the tool attitude, it "
              "cannot turn one into the other. This mode does NOT fall back to "
              "a free RRT. Level the arm first (test/vision_target.py --tilt) "
              "or jog nearer that attitude.")
        return False

    if not a.yes:
        print(f"\nabout to move the REAL robot to '{a.goto}' at {a.speed} rad/s")
        try:
            go = input("proceed? [y/N] ").strip().lower()
        except EOFError:
            go = ''
        if go not in ('y', 'yes'):
            print("aborted (pass --yes to skip this prompt)")
            return False

    if not carry(node, goal, f"goto {a.goto}", a.speed, a.plan_time,
                 dry=False, drift_tol=a.drift_tol):
        return False
    if not settled(mon, a, f"goto {a.goto}"):
        return False
    q_end = node.current_joints.tolist()
    err = max(abs(math.degrees(b - c)) for c, b in zip(q_end, goal))
    print(f"  arrived: worst joint {err:.3f} deg from the taught '{a.goto}'")
    return True


def monitor_loop(mon):
    """Print live feedback until Ctrl+C. Sends nothing to the robot."""
    print("live real-time feedback (Ctrl+C to stop) -- nothing is commanded\n")
    base = None
    try:
        while True:
            st, stamp = mon.state()
            if st is None:
                time.sleep(0.2)
                continue
            m = np.array(st['m_actual'])
            if base is None:
                base = m.copy()
            print(f"\rmode {st['robot_mode']:2d}  en {st['enable']}  err {st['error']}  "
                  f"col {st['collision']}  "
                  f"J(deg) " + " ".join(f"{v:+7.1f}" for v in st['q_actual']) +
                  "  torque " + " ".join(f"{v:+6.2f}" for v in m) +
                  "  d " + " ".join(f"{v:5.2f}" for v in np.abs(m - base)) + "   ",
                  end='', flush=True)
            time.sleep(0.1)
    except KeyboardInterrupt:
        print("\n")


# --------------------------------------------------------------------------

def collision_model_xacro():
    """Which xacro to collision-check against: combined if it builds, else arm-only.

    The combined model is the sim rig -- arm on a cube platform on an MPO-700
    AGV -- and it reaches into neo_simulation2 for the AGV body, a simulation
    package a real-robot workspace need not carry. On a pedestal-mounted arm
    that AGV is not there to hit, so the arm-only model is the CORRECT one, not
    a degraded one; it still checks the arm against itself and its own gripper.

    What no model here covers either way is the WORKSPACE: the table, the
    fixture and the object are not represented anywhere. Vertical approach is
    covered by the contact detection; a free RRT leg (approach(), when the tool
    has to be reoriented) is not, which is why it warns before it runs.

    Returns None to mean "use node.py's default (combined)".
    """
    import xacro
    try:
        xacro.process_file(COMBINED_XACRO, mappings={'use_gazebo': 'false'})
        return None
    except Exception as e:
        reason = str(e).split('"')[1] if '"' in str(e) else str(e)
        print("\n[collision] the combined (arm + cube + AGV) model is unavailable:")
        print(f"     {reason[:160]}")
        print("   Using the ARM-ONLY model: self- and gripper-collision checked.")
        print("   Correct for a pedestal-mounted arm. Note that the table, the")
        print("   fixture and the object are not modelled in EITHER case -- the")
        print("   descend relies on contact detection, so watch the first cycle.\n")
        return XACRO_PATH


def robot_ip():
    cfg = os.path.join(_PKG_ROOT, 'dobot_bringup_v4', 'config', 'param.json')
    with open(cfg) as f:
        data = json.load(f)
    return data['node_info'][data['current_robot'] - 1]['ip_address']


def parse_args():
    p = argparse.ArgumentParser(
        description=__doc__.split('\n')[0],
        formatter_class=argparse.RawDescriptionHelpFormatter)
    # --vision sits OUTSIDE this group: it is the one mode that takes --run /
    # --dry as its own switch rather than being one of them.
    mode = p.add_mutually_exclusive_group(required=False)
    mode.add_argument('--teach', metavar='NAME',
                      choices=('p1', 'p2', 'home', 'obs'),
                      help="store the CURRENT joint config as p1, p2, home or "
                           "obs. home is where the cycle parks when it "
                           "finishes; without one it parks at p1, hovering over "
                           "the part. obs is the observation pose --vision "
                           "looks at the magazine from (no --level needed: "
                           "nothing is carried to or from it)")
    mode.add_argument('--teach-surface', metavar='DIR',
                      help="measure the table or wall the TOOL is touching and "
                           "register it as a collision plane from then on. DIR "
                           "is the direction from the robot base toward it, "
                           "AXIS FIRST: z- the table underneath, y- the wall on "
                           "the -y side, x+ the wall on the +x side, ...")
    mode.add_argument('--set-surface', metavar='DIR',
                      help="define a surface from --at instead of measuring it: "
                           "for a wall that is not built yet, or a distance off "
                           "a drawing. Needs no robot contact.")
    mode.add_argument('--forget-surface', metavar='NAME',
                      help="delete one taught surface by NAME, or 'all' to "
                           "delete every one. Use it when a surface was taught "
                           "wrongly -- teaching 'y+' does NOT replace a stale "
                           "'y-', they are separate surfaces.")
    mode.add_argument('--descend', type=float, metavar='MM',
                      help="descend straight down from the CURRENT pose by up "
                           "to MM millimetres, stopping the moment the joint-"
                           "torque channel sees contact. Negative lifts by "
                           "|MM| instead. Nothing else moves. Use "
                           "--descend-speed 0.003 (the thresholds were "
                           "measured there) and keep the e-stop in hand: the "
                           "magazine is in no collision model.")
    mode.add_argument('--j6-only', type=float, metavar='DEG',
                      help="turn J6 in place by DEG and stop -- nothing else "
                           "moves. For eyeballing the magazine's yaw: nudge "
                           "until the jaws look parallel to the box, then pass "
                           "the total as --j6-deg (or tell it to the operator "
                           "as MAGAZINE_YAW_DEG). No vision, no re-planning.")
    mode.add_argument('--goto', metavar='NAME',
                      help="move to a taught point (obs, p1, p2, home) under the "
                           "strict tilt-held CBiRRT -- no free-RRT fallback. Made "
                           "for the vision-hover repeat loop: run, measure, "
                           "--goto obs, run again. Speed comes from --speed.")
    mode.add_argument('--show', action='store_true', help="print the taught points")
    mode.add_argument('--monitor', action='store_true',
                      help="stream live robot feedback; commands nothing")
    mode.add_argument('--run', action='store_true', help="execute the cycle")
    mode.add_argument('--dry', action='store_true',
                      help="plan and check everything, command no motion")

    p.add_argument('--vision', action='store_true',
                   help="VISION HOVER: go to the taught 'obs' pose, ask the "
                        "runner where the magazine is, stand the FLANGE "
                        "10 cm above its top face and turn J6 to line the "
                        "grasp up. NO DESCEND. Plans only unless --run is "
                        "given as well. See test/vision_target.py for the "
                        "constants and the work-box gate.")
    p.add_argument('--target', metavar='RULE', default=None,
                   help="--vision only: with SEVERAL magazines in view, look "
                        "from where you are (ungated, aiming only), pick one by "
                        "RULE, then MOVE the camera to ~--view-dist in front of "
                        "it and take the real, gated snapshot there. RULE is "
                        "nearest | left | right | id=N. Without --target the "
                        "snapshot is taken from the obs pose as before.")
    p.add_argument('--view-dist', type=float, default=None,
                   help="--target only: camera-to-magazine-front distance for "
                        "the close look, metres (default 0.20). The gate wants "
                        "0.10~0.30 and depth noise grows with distance.")
    p.add_argument('--no-descend', action='store_true',
                   help="--vision only: stop after the J6 alignment instead of "
                        "descending into the magazine. The full cycle is "
                        "detect -> hover -> J6 -> descend to contact -> lift -> "
                        "retrace to obs; this cuts it after J6.")
    p.add_argument('--j6-deg', type=float, default=None,
                   help="--vision only: J6 alignment turn, degrees (default "
                        "+90 from vision_target.J6_ROT_DEG). If it runs past "
                        "the joint limit the move is refused, not flipped")
    p.add_argument('--cycles', type=int, default=1, help="repeat the cycle N times")
    p.add_argument('--drop', type=float, default=0.06,
                   help="max descend per station, metres (default 0.06)")
    p.add_argument('--clearance', type=float, default=0.12,
                   help="extra lift above the contact point, metres (default 0.02)")
    p.add_argument('--dwell', type=float, default=3.0,
                   help="hold at contact, seconds (default 3)")
    p.add_argument('--speed', type=float, default=0.25,
                   help="approach/carry joint speed, rad/s (default 0.25)")
    p.add_argument('--descend-speed', type=float, default=0.005,
                   help="descend speed, m/s (default 0.005 = 5 mm/s)")
    p.add_argument('--plan-time', type=float, default=30.0,
                   help="CBiRRT time budget per move, seconds")
    p.add_argument('--collision-level', type=int, default=3,
                   help="controller collision sensitivity 0-5, 0=off (default 3)")
    p.add_argument('--speed-factor', type=int, default=0,
                   help="global controller speed cap in %%; 0 leaves it untouched")
    p.add_argument('--no-torque-trip', action='store_true',
                   help="disable the soft torque channel; firmware trip only")
    p.add_argument('--torque-trip', type=float, default=None,
                   help="override the soft trip threshold for J1-J5, N*m")
    p.add_argument('--name', default=None,
                   help="name for the surface being taught/set (default: its "
                        "direction). Two surfaces facing the same way -- a base "
                        "support and a lower shelf, both 'z-' -- need distinct "
                        "names to coexist.")
    p.add_argument('--at', type=float, default=None,
                   help="--set-surface only: the coordinate of the surface, m")
    p.add_argument('--bound', action='append', nargs=3, metavar=('AXIS', 'LO', 'HI'),
                   help="limit the surface along AXIS to [LO, HI] metres; "
                        "'inf' for an open side. Repeatable. e.g. "
                        "--bound y -0.34 inf keeps a surface to y >= -0.34")
    p.add_argument('--level', action='store_true',
                   help="snap the taught pose so the tool is EXACTLY vertical, "
                        "keeping the TCP position. The carry holds whatever "
                        "attitude p1 has, so a hand-jogged degree or two of "
                        "tilt is carried the whole way and p2 then needs an "
                        "unplanned final hop. Use on p1 AND p2.")
    p.add_argument('--tilt-tol', type=float, default=2.0,
                   help="how far apart p1 and p2 may point the TOOL, degrees "
                        "(default 2.0). The carry holds the tool attitude, so a "
                        "wider gap makes the arm land off the taught point: "
                        "measured on this arm, 2.27 deg cost 8 mm. Raise it only "
                        "if you can live with that error.")
    p.add_argument('--drift-tol', type=float, default=0.02,
                   help="how far, in joint-space radians, the projected goal may "
                        "sit from the taught one (default 0.02). The path plan "
                        "ends at the PROJECTED goal; the taught one is appended "
                        "as a final unplanned hop, so this bounds that hop. Goes "
                        "with --tilt-tol: 2.27 deg of tilt gap measured 0.036 rad "
                        "here, so raising one usually means raising both.")
    p.add_argument('--no-surfaces', action='store_true',
                   help="do not register the taught table/walls")
    p.add_argument('--surface-offset', type=float, default=0.0,
                   help="push the measured surface OUTWARD by this many metres "
                        "(use if the tool was pressed into it); negative pulls "
                        "it toward the robot, which is the conservative way")
    p.add_argument('--ip', default=None, help="robot IP (default: from param.json)")
    p.add_argument('--yes', action='store_true', help="skip the pre-motion prompt")
    a = p.parse_args()
    if not (a.vision or a.teach or a.teach_surface or a.set_surface
            or a.forget_surface or a.show or a.monitor or a.run or a.dry
            or a.goto or a.j6_only is not None or a.descend is not None):
        p.error("one of --teach --teach-surface --set-surface --forget-surface "
                "--show --monitor --dry --run --vision --goto --j6-only "
                "--descend is required")
    if a.j6_deg is not None and not a.vision:
        p.error("--j6-deg only means something with --vision")
    for spec in (a.teach_surface, a.set_surface):
        if spec:
            try:
                parse_dir(spec)
            except ValueError as e:
                p.error(str(e))
    try:
        parse_bounds(a.bound)
    except ValueError as e:
        p.error(str(e))
    if a.set_surface and a.at is None:
        p.error("--set-surface needs --at (the surface coordinate, metres)")
    return a


def main():
    a = parse_args()

    if a.set_surface:
        # Numbers only: no robot, no planner, nothing to connect to.
        set_surface(a, a.set_surface)
        return 0

    if a.forget_surface:
        # Deleting is a file edit: likewise nothing to connect to.
        surfaces = load_surfaces()
        if a.forget_surface.strip().lower() == 'all':
            removed, surfaces = sorted(surfaces), {}
        else:
            name = a.forget_surface
            removed = [name] if name in surfaces else []
            surfaces.pop(name, None)
        if not removed:
            print(f"nothing to forget: '{a.forget_surface}' is not taught "
                  f"(have: {', '.join(sorted(surfaces)) or 'none'})")
            return 1
        with open(SURFACES_FILE, 'w') as f:
            json.dump(surfaces, f, indent=2)
        own_like_dir(SURFACES_FILE)
        print(f"forgot {', '.join(removed)}; "
              f"{len(surfaces)} surface(s) still taught")
        return 0

    if a.show:
        pts = load_points()
        if pts:
            print(f"{POINTS_FILE}")
            for name in ('p1', 'p2', 'home'):
                if name in pts:
                    describe(name, pts[name])
            if 'p1' in pts and 'p2' in pts:
                print(f"  tool tilt difference p1 vs p2: "
                      f"{tilt_between(pts['p1'], pts['p2']):.2f} deg")
        else:
            print(f"no points taught yet ({POINTS_FILE} does not exist)")
        surfaces = load_surfaces()
        print(f"\n{SURFACES_FILE}")
        if surfaces:
            for name, s in sorted(surfaces.items()):
                describe_surface(name, s)
        else:
            print("  no surfaces taught (run --teach-surface z- for the table)")
        return 0 if pts else 1

    ip = a.ip or robot_ip()
    mon = RealtimeMonitor(ip)
    mon.start()
    if not mon.wait_ready():
        print(f"cannot read the robot's real-time feed: {mon.error or 'timed out'}")
        print("is the robot powered and reachable? (ping " + ip + ")")
        return 1

    if a.monitor:
        monitor_loop(mon)
        mon.stop()
        return 0

    st, _ = mon.state()
    print(f"robot {ip}: mode={st['robot_mode']} enable={st['enable']} "
          f"error={st['error']} collision={st['collision']}")
    if not st['enable']:
        print("robot is NOT enabled -- enable it (jog_real.py 'e', or the pendant) first")
        mon.stop()
        return 1

    # rclpy's own SIGINT handler swallows Ctrl+C, so the shutdown path below
    # never runs (docs/real_robot_jetson_bringup.md 5).
    rclpy.init(signal_handler_options=SignalHandlerOptions.NO)
    node = HubPickPlace()
    node.setup_planner(combined_xacro=collision_model_xacro())
    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()

    rc = 1
    try:
        t0 = time.time()
        while node.current_joints is None:
            if time.time() - t0 > 15.0:
                print("no /joint_states in 15 s -- is dobot_joint.launch.py up "
                      "(bringup terminal B)?")
                return 1
            time.sleep(0.1)
        q_now = node.current_joints.tolist()
        down_deg = axis_angle(tool_axis(node, q_now), np.array([0.0, 0.0, -1.0]))
        fl = flange_in_base(node, q_now)
        print("joint states live (URDF convention): " +
              " ".join(f"{math.degrees(v):+.1f}" for v in q_now))
        print(f"  tool  : {down_deg:5.1f} deg off straight-down "
              f"({'DOWN' if down_deg < 90 else 'UP -- reorient the wrist'})")
        print(f"  flange: x={fl[0]:+.3f}  y={fl[1]:+.3f}  z={fl[2]:+.3f}  m "
              f"(base_link -- compare with GetPose(user=0,tool=0))")

        if a.teach_surface:
            rc = 0 if teach_surface(node, a, a.teach_surface) else 1
            return rc

        # Surfaces go in BEFORE the taught points are validated, so "p1 is in
        # collision" also covers "p1 is below the table or past a wall".
        register_surfaces(node, a)

        if a.teach:
            save_point(node, a.teach, tilt_tol=a.tilt_tol, level=a.level)
            rc = 0
            return rc

        if a.descend is not None:
            rc = 0 if descend_only(node, Dashboard(node), mon, a) else 1
            return rc

        if a.j6_only is not None:
            rc = 0 if j6_only(node, mon, a) else 1
            return rc

        if a.goto:
            rc = 0 if goto_point(node, mon, a) else 1
            return rc

        if a.vision:
            rc = 0 if run_vision(node, mon, a) else 1
            return rc

        pts = load_points()
        missing = [n for n in ('p1', 'p2') if n not in pts]
        if missing:
            print(f"missing taught point(s): {', '.join(missing)} -- run --teach first")
            return 1
        for name in ('p1', 'p2'):
            describe(name, pts[name])
        sep = float(np.linalg.norm(np.array(pts['p1']['tcp_xyz'])
                                   - np.array(pts['p2']['tcp_xyz'])))
        print(f"\np1 -> p2 distance: {sep * 1000:.0f} mm")
        if sep < 0.02:
            print("!! p1 and p2 are the same place. Both were taught without "
                  "moving the arm in between, so the carry has nothing to do. "
                  "Jog to each spot BEFORE its --teach.")
            return 1
        dtilt = tilt_between(pts['p1'], pts['p2'])
        print(f"tool tilt difference p1 vs p2: {dtilt:.2f} deg")
        if dtilt > a.tilt_tol:
            print("!! p1 and p2 do not point the tool the same way. The carry is "
                  "CBiRRT with the tool attitude HELD, so no such path can reach "
                  "from one to the other -- a carried part would have to tip. "
                  "Re-teach both with the same wrist attitude (straight down is "
                  "the usual choice).")
            return 1

        thresholds = TORQUE_TRIP
        if a.torque_trip is not None:
            thresholds = (a.torque_trip,) * 5 + (1e9,)
        det = ContactDetector(mon, thresholds=thresholds, soft=not a.no_torque_trip)

        dash = Dashboard(node)
        if not a.dry:
            if a.speed_factor:
                dash.call('SpeedFactor', ratio=int(a.speed_factor))
                print(f"[dash] SpeedFactor({a.speed_factor}%) set -- it PERSISTS "
                      f"on the controller after this script exits")
            if not dash.arm_touchoff(a.collision_level):
                print("could not configure the collision touch-off; aborting")
                return 1
            dash.clear_error()

            print(f"\nabout to move the REAL robot: {a.cycles} cycle(s), "
                  f"carry {a.speed} rad/s, descend {a.descend_speed * 1000:.0f} mm/s, "
                  f"max drop {a.drop * 1000:.0f} mm, collision level {a.collision_level}"
                  + ("" if a.no_torque_trip else f", torque trip {thresholds[:5]}"))
            if not a.yes:
                if input("proceed? [y/N] ").strip().lower() not in ('y', 'yes'):
                    print("aborted")
                    return 0

        for i in range(a.cycles):
            print(f"\n{'=' * 62}\n cycle {i + 1}/{a.cycles}\n{'=' * 62}")
            if not run_cycle(node, dash, mon, det, pts, a):
                print(f"\ncycle {i + 1} FAILED -- stopping")
                return 1
        print(f"\n{a.cycles} cycle(s) done." if not a.dry
              else f"\n--dry: {a.cycles} cycle(s) planned and checked, nothing moved.")
        rc = 0
    except KeyboardInterrupt:
        print("\ninterrupted -- the arm holds its last ServoJ target")
    finally:
        mon.stop()
        try:
            node.destroy_node()
        except Exception:
            pass
        rclpy.shutdown()
    return rc


if __name__ == '__main__':
    sys.exit(main())
