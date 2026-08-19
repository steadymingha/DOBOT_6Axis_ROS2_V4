"""Contact-stop descend + the real-robot safety checks around it.

Moved verbatim from test/cbirrt_p1p2_test.py (2026-08-17, plan 4.2) so the
sequences can use what --vision --run already proved on the real arm:

    ContactDetector      hard (controller collision trip) + soft (torque step) channels
    Dashboard            speed / collision level / ClearError / ServoJ via bringup services
    guarded_descend      ServoJ stream straight down, stopped the tick contact is seen
    wait_until_still     "stopped" judged on qd_actual off the 30004 feed
    joint_gap_deg        target-vs-actual guard (URDF->controller sign bug catcher)
    arrived              did the arm really get there (a protective stop looks like
                         success to execute_path)
    recover_after_contact, warn_descend_speed

Only the descend bypasses the trajectory action (it cannot be cancelled and a
contact stop IS a cancel); everything else still goes through node.execute_path.
The feed (`mon`) is a cr7_pnp.robot_feed.RobotFeed -- ONE per process.
"""
import math
import time

import numpy as np
import rclpy

from dobot_msgs_v4.srv import (
    ClearError, RobotMode, ServoJ, SetBackDistance,
    SetCollisionLevel, SetPostCollisionMode, SpeedFactor)

from .robot_feed import MODE_COLLISION, MODE_PAUSE

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
