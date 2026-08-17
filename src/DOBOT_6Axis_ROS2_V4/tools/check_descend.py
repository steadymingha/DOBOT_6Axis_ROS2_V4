"""Check the descend path in isolation, without committing to a full --run.

Part 1 runs always and commands NOTHING: it prints the controller-vs-URDF joint
angles, plans the descend, recomputes what the first ServoJ tick would send and
compares it against the robot's own q_actual, then feeds the safety gate a
deliberately mirrored target to confirm the gate rejects it. That is the
regression check for JOINT_SIGN_REAL -- run it after touching the URDF, the
joint-sign code, the gripper / TCP offset, or after a controller firmware change.

Part 2 (--move) does a real descend, 30 mm at 3 mm/s (--mm N to change).
--profile descends 20 mm with the soft channel OFF and logs torque; that is how
the contact thresholds were derived, so re-run it if payload or speed changes.

See docs/real_robot_descend_check.md.
    /root/dobot_ws/.venv/bin/python3 tools/check_descend.py
    /root/dobot_ws/.venv/bin/python3 tools/check_descend.py --move
"""
import math, os, sys, threading, time
import numpy as np
import rclpy
from rclpy.executors import MultiThreadedExecutor
from rclpy.signals import SignalHandlerOptions

ROOT = '/root/dobot_ws/src/DOBOT_6Axis_ROS2_V4'
sys.path.insert(0, ROOT)
sys.path.insert(0, os.path.join(ROOT, 'test'))
os.environ.setdefault('CR7_REAL_ROBOT', '1')

import cbirrt_p1p2_test as T
from cr7_pnp.node import CBiRRTPickPlace

MOVE = '--move' in sys.argv
# 30 mm, because the soft channel stays blind for the first 1.5 s (4.5 mm at
# 3 mm/s) to clear the breakaway transient -- a 5 mm descend would finish before
# detection ever armed, which proves nothing about contact.
DROP = 0.030
SPEED = 0.003
for i, arg in enumerate(sys.argv):
    if arg == '--mm':
        DROP = float(sys.argv[i + 1]) / 1000.0

mon = T.RealtimeMonitor(T.robot_ip())
mon.start()
if not mon.wait_ready():
    raise SystemExit(f'no real-time feed: {mon.error}')

rclpy.init(signal_handler_options=SignalHandlerOptions.NO)
node = CBiRRTPickPlace()
node.setup_planner(combined_xacro=T.collision_model_xacro())
ex = MultiThreadedExecutor(); ex.add_node(node)
threading.Thread(target=ex.spin, daemon=True).start()
while node.current_joints is None:
    time.sleep(0.1)
T.register_surfaces(node, type('A', (), {'no_surfaces': False})())

st, _ = mon.state()
print('\n--- part 1: no motion ---')
print('robot q_actual (deg, controller) :',
      [round(v, 2) for v in st['q_actual']])
print('node.current_joints (deg, URDF)  :',
      [round(math.degrees(v), 2) for v in node.current_joints])

q = node.current_joints.tolist()
path, reach, reason = node.cbirrt.linear_path(
    q, np.array([0.0, 0.0, -DROP]), node.is_state_valid, node.joint_limits,
    step=0.003)
print(f'linear_path: {reach*1000:.1f} mm of {DROP*1000:.0f} mm, {reason}')
first = T.interp_path(path, [min(0.003 * i, reach) for i in range(len(path))],
                      min(SPEED * T.SERVOJ_DT, reach))
print('first ServoJ target would be (deg, controller) :',
      [round(math.degrees(v), 2) for v in np.asarray(first) * node.joint_sign])
gap = T.joint_gap_deg(mon, first, node.joint_sign)
print(f'gap from where the arm actually is : {gap:.3f} deg   '
      f'{"OK" if gap < 1.0 else "<<< WRONG, do not run --move"}')

mirrored = list(np.asarray(first) * -1.0)
bad = T.joint_gap_deg(mon, mirrored, node.joint_sign)
print(f'gate test, deliberately mirrored target : {bad:.1f} deg   '
      f'{"gate would ABORT (good)" if bad > 5.0 else "GATE FAILED"}')

if '--profile' in sys.argv:
    # Descend with the soft channel DISABLED (the controller's own collision
    # detection is still armed as the backstop) and log what the torque actually
    # does, so the thresholds come from data instead of from a guess. The first
    # attempt tripped on J3 at 7-8 N*m within 0.2 s of starting: that is the
    # stationary-to-moving transition, not contact, and it is what has to be
    # blanked out or thresholded around.
    DIST = 0.020
    q = node.current_joints.tolist()
    path, reach, reason = node.cbirrt.linear_path(
        q, np.array([0.0, 0.0, -DIST]), node.is_state_valid, node.joint_limits,
        step=0.003)
    print(f'\n--- profile: {reach*1000:.0f} mm at {SPEED*1000:.0f} mm/s, '
          f'soft detection OFF ---')
    dash = T.Dashboard(node)
    dash.arm_touchoff(3)
    dash.clear_error()
    det = T.ContactDetector(mon, soft=False)
    T.wait_until_still(mon)
    time.sleep(det.lag + det.win)
    dists = [min(0.003 * i, reach) for i in range(len(path))]
    per_tick = SPEED * T.SERVOJ_DT
    print(f'{"t(s)":>6} {"mm":>6}   torque delta vs {det.lag:.2f}s ago (N*m), J1..J6')
    travelled, t0, t_next, k = 0.0, time.time(), time.time(), 0
    while travelled < reach - 1e-6:
        if det.check():
            print('  controller tripped; stopping'); break
        travelled = min(travelled + per_tick, reach)
        tgt = T.interp_path(path, dists, travelled)
        gap = T.joint_gap_deg(mon, tgt, node.joint_sign)
        if gap is not None and gap > 5.0:
            print(f'  ABORT: target {gap:.1f} deg away'); break
        dash.servoj(tgt)
        k += 1
        if k % 5 == 0:
            now = time.time()
            base = mon.torque_window(now - det.lag - det.win, now - det.lag)
            recent = mon.torque_window(now - 0.06, now)
            if len(base) >= 5 and len(recent) >= 2:
                d = np.abs(np.median(np.array(recent), axis=0)
                           - np.median(np.array(base), axis=0))
                print(f'{now-t0:6.2f} {travelled*1000:6.2f}   ' +
                      ' '.join(f'{v:5.2f}' for v in d))
        t_next += T.SERVOJ_DT
        time.sleep(max(0.0, t_next - time.time()))
    time.sleep(0.3)
    T.recover_after_contact(node, dash, mon)
    T.lift(node, DIST + 0.005, 'profile ascend', 0.15)
elif not MOVE:
    print(f'\n(--move for a real {DROP*1000:.0f} mm descend, --profile to log torque over 20 mm)')
else:
    print(f'\n--- part 2: real descend, {DROP*1000:.0f} mm at {SPEED*1000:.0f} mm/s ---')
    if st['robot_mode'] != 5 or not st['enable']:
        print(f'robot not idle/enabled (mode={st["robot_mode"]}, '
              f'enable={st["enable"]}) -- clear and enable first')
    else:
        dash = T.Dashboard(node)
        dash.arm_touchoff(3)
        dash.clear_error()
        det = T.ContactDetector(mon)
        got = T.guarded_descend(node, dash, det, DROP, SPEED, 'check descend')
        print(f'result: {got if got is None else f"{got*1000:.1f} mm"}')
        if got:
            T.recover_after_contact(node, dash, mon)
            T.lift(node, got + 0.01, 'check ascend', 0.15)

mon.stop(); node.destroy_node(); rclpy.shutdown()
