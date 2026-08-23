#!/usr/bin/env python3
"""Policy rollout evaluation in Isaac Sim: the trained diffusion policy drives
the arm (instead of the scripted expert) and success is scored with the same
gates the collector used (box_in_pocket + shelf_undisturbed).

Two-process design: this script (SYSTEM python, ROS) runs the sim episode and
executes actions; policy inference runs in policy_server.py (robodiff env,
GPU) over a localhost socket.

Run (Isaac sim already up, policy_server.py already listening):
    source /opt/ros/humble/setup.bash && source ~/dobot_ws/install/setup.bash
    /usr/bin/python3 diffusionpolicy/isaac_eval.py --episodes 20

Per episode: full shelf reset -> random tier-1 box -> AGV parks at the
station anchor + the collector's noise distribution -> policy control loop
(obs at CONTROL_HZ -> T_a-step absolute-pose chunks -> per-step seeded DLS IK
with collision gating -> one FollowJointTrajectory goal per chunk).
Grasp follows the data convention: gripper-close command near the box
triggers ATTACHLINK, open triggers detach.
"""
import argparse
import json
import math
import pathlib
import pickle
import random
import socket
import struct
import sys
import threading
import time

import numpy as np
import rclpy
from control_msgs.action import FollowJointTrajectory
from gazebo_msgs.srv import SetEntityState
from rclpy.executors import MultiThreadedExecutor
from std_srvs.srv import SetBool
from trajectory_msgs.msg import JointTrajectoryPoint

sys.path.insert(0, str(pathlib.Path(__file__).parent))
import isaac_collect as ic  # noqa: E402  (also inserts the sequences path)
import shelf_pick_place as spp  # noqa: E402
from cr7_pnp import GRIPPER_OPEN, shelf_box_center  # noqa: E402

CONTROL_DT = 1.0 / ic.CONTROL_HZ
# Gripper-command hysteresis from the dataset range [~0.002 close, 0.015 open]
GRIP_CLOSE_T = 0.008
GRIP_OPEN_T = 0.012
ATTACH_RANGE = 0.09          # eef-to-box distance allowing an attach (m);
                             # grasp center sits ~53 mm off the TCP, 0.12 let
                             # an 11 cm misaligned grasp through
STUCK_WINDOW = 100           # steps without eef movement -> fail early
STUCK_EPS = 0.01             # m
MAX_STRIKES = 6              # consecutive gated/failed chunks before giving up
# Receding-horizon overlap: predict while the current chunk is still executing
# and splice the next goal at its end, so the arm never pauses (the expert's
# final descend is continuous; inference pauses freeze the To=2 velocity cue
# and the policy hovers). The obs->splice latency is compensated by dropping
# the leading steps of each predicted chunk that will already have elapsed at
# splice time, measured per chunk (paper's latency handling for abs actions).
PRED_LEAD = 0.30             # s before chunk end to capture obs + predict
FIXED_SKIP = None            # set by --fixed-skip (A/B vs measured skip)


# ------------------------------------------------------------ policy client
class PolicyClient:
    def __init__(self, port):
        self.port = port
        self.sock = socket.create_connection(('127.0.0.1', port))
        meta = self._rt({'cmd': 'meta'})
        self.n_obs_steps = meta['n_obs_steps']
        self.n_action_steps = meta['n_action_steps']
        # obs frame spacing (ticks) the ckpt was trained with (1 = adjacent)
        self.obs_stride = int(meta.get('obs_stride', 1))

    def reconnect(self):
        try:
            self.sock.close()
        except Exception:
            pass
        self.sock = socket.create_connection(('127.0.0.1', self.port))

    def _rt(self, obj):
        b = pickle.dumps(obj, protocol=4)
        self.sock.sendall(struct.pack('>I', len(b)) + b)
        hdr = self.sock.recv(4, socket.MSG_WAITALL)
        n = struct.unpack('>I', hdr)[0]
        buf = b''
        while len(buf) < n:
            buf += self.sock.recv(min(1 << 20, n - len(buf)))
        return pickle.loads(buf)

    def act(self, obs):
        return self._rt({'cmd': 'act', 'obs': obs})['action']


# ------------------------------------------------------------ chunk executor
def stack_obs(buf, To, stride):
    """Last To frames spaced `stride` ticks apart (newest last). Early in an
    episode indices clamp to the oldest frame -- same edge-repeat the training
    pad uses."""
    return [buf[max(-1 - (To - 1 - i) * stride, -len(buf))] for i in range(To)]


def chunk_to_joints(node, actions, q_seed):
    """Per-step seeded DLS IK for a (Ta,8) absolute-pose chunk. Returns
    (joint_paths, reason): reason None on success, else 'ik'/'collision'."""
    qs = []
    seed = list(q_seed)
    for a in actions:
        quat = np.asarray(a[3:7], float)
        n = np.linalg.norm(quat)
        if n < 0.5:                       # degenerate network output
            return qs, 'ik'
        R = spp.quat_to_R(*(quat / n))    # regressed quats are not unit-norm
        qp = node.ik_model.inverse_kinematics(
            np.asarray(a[:3], float), R, seeds=[node.ik_model.pin_q(seed)])
        if qp is None:
            cur = node.ik_model.fk_tcp(node.ik_model.pin_q(seed))[0]
            print(f'[ik] fail: target {np.round(a[:3], 3)} '
                  f'({np.linalg.norm(np.asarray(a[:3]) - cur):.3f} m from eef), '
                  f'|q|={n:.3f}')
            return qs, 'ik'
        q6 = [float(qp[i]) for i in node.ik_model.q_index]
        if not node.is_state_valid(q6):
            print(f'[gate] collision at target {np.round(a[:3], 3)}: '
                  f'{collision_pairs_of(node, q6)}')
            return qs, 'collision'
        qs.append(q6)
        seed = q6
    return qs, None


def collision_pairs_of(node, q6):
    try:
        return node.collision.colliding_pairs(q6)
    except Exception:
        return '?'


def send_chunk(node, qs):
    """Send one FollowJointTrajectory goal (CONTROL_DT per waypoint) WITHOUT
    waiting for the result. Returns the accepted goal handle or None. Sending
    the next goal as the previous ends preempt-splices them into one
    continuous motion."""
    goal = FollowJointTrajectory.Goal()
    goal.trajectory.joint_names = node.joint_names
    for i, q in enumerate(qs):
        pt = JointTrajectoryPoint()
        pt.positions = [float(v) for v in q]
        t = (i + 1) * CONTROL_DT
        pt.time_from_start.sec = int(t)
        pt.time_from_start.nanosec = int((t % 1) * 1e9)
        goal.trajectory.points.append(pt)
    fut = node.traj_action_client.send_goal_async(goal)
    t0 = time.time()
    while not fut.done():
        if time.time() - t0 > 5.0:
            print('[chunk] goal send timeout')
            return None
        time.sleep(0.002)
    gh = fut.result()
    if not gh or not gh.accepted:
        print('[chunk] goal rejected')
        return None
    return gh


def execute_chunk(node, qs):
    """One FollowJointTrajectory goal, CONTROL_DT per waypoint (the policy's
    time base), wait for the result. No settle wait: the next chunk's IK is
    seeded from wherever the arm actually is."""
    goal = FollowJointTrajectory.Goal()
    goal.trajectory.joint_names = node.joint_names
    for i, q in enumerate(qs):
        pt = JointTrajectoryPoint()
        pt.positions = [float(v) for v in q]
        t = (i + 1) * CONTROL_DT
        pt.time_from_start.sec = int(t)
        pt.time_from_start.nanosec = int((t % 1) * 1e9)
        goal.trajectory.points.append(pt)
    fut = node.traj_action_client.send_goal_async(goal)
    t0 = time.time()
    while not fut.done():
        if time.time() - t0 > 5.0:
            print('[chunk] goal send timeout')
            return False
        time.sleep(0.005)
    gh = fut.result()
    if not gh or not gh.accepted:
        print('[chunk] goal rejected')
        return False
    rf = gh.get_result_async()
    deadline = time.time() + len(qs) * CONTROL_DT + 3.0
    while not rf.done():
        if time.time() > deadline:
            print('[chunk] result timeout')
            return False
        time.sleep(0.005)
    r = rf.result()
    if r.result.error_code != 0:
        print(f'[chunk] controller error {r.result.error_code} '
              f'({r.result.error_string})')
        return False
    return True


def sim_pause(node, flag):
    """Freeze/unfreeze the sim world (stepped eval). Blocking, short."""
    req = SetBool.Request()
    req.data = bool(flag)
    fut = node._sim_pause_cli.call_async(req)
    t0 = time.time()
    while not fut.done():
        if time.time() - t0 > 5.0:
            raise RuntimeError('sim_pause service timeout')
        time.sleep(0.002)
    return fut.result().success


def run_episode_stepped(node, rec, client, idx, park, max_steps,
                        absent=frozenset()):
    """Stepped (robomimic-semantics) rollout: the sim is FROZEN during
    inference+IK, so the policy sees zero effective latency and the arm never
    pauses in sim time. This measures the algorithm's ability on the task,
    decoupled from execution-latency engineering (the overlap path below is
    the deployment-realistic counterpart)."""
    box_model = spp.shelf_box_model(spp.TIER, idx)
    if not ic.move_agv(node, *park):
        return 'agv_fail', {}, None
    node.set_shelf_stock_absent(spp.TIER, idx, absent=True)
    rec.start(box_model)
    t0 = time.time()
    while len(rec.buf['eef']) < client.n_obs_steps:
        time.sleep(CONTROL_DT)
        if time.time() - t0 > 10.0:
            rec.stop()
            return 'no_obs', {}, None

    attached = False
    steps = 0
    strikes = 0
    reason = 'timeout'
    eef_hist = []
    try:
        while steps < max_steps:
            To, st = client.n_obs_steps, client.obs_stride
            obs = {
                'agentview_image': np.asarray(stack_obs(rec.buf['agent'], To, st), np.uint8),
                'robot0_eye_in_hand_image': np.asarray(stack_obs(rec.buf['wrist'], To, st), np.uint8),
                'robot_eef_pose': np.asarray(stack_obs(rec.buf['eef'], To, st), np.float32),
                'gripper': np.asarray(stack_obs(rec.buf['grip'], To, st), np.float32),
                'object': np.asarray(stack_obs(rec.buf['object'], To, st), np.float32),
            }
            sim_pause(node, True)         # world frozen while the policy thinks
            try:
                action = client.act(obs)
                # split the chunk at the first gripper transition so the
                # command fires at its true tick (chunk boundaries only)
                g = action[:, 7]
                cur_g = float(rec.grip_cmd)
                if cur_g > GRIP_CLOSE_T:      # currently open: split at close
                    cut = np.argmax(g < GRIP_CLOSE_T) if (g < GRIP_CLOSE_T).any() else len(g)
                else:                          # currently closed: split at open
                    cut = np.argmax(g > GRIP_OPEN_T) if (g > GRIP_OPEN_T).any() else len(g)
                grip_next = float(g[cut]) if cut < len(g) else None
                exec_part = action[:max(cut, 1)]   # >=1 step so we always move
                qs, err = chunk_to_joints(node, exec_part,
                                          node.current_joints.tolist())
                gh = send_chunk(node, qs) if qs else None
            finally:
                sim_pause(node, False)    # goal starts the instant time resumes
            if err:
                strikes += 1
                if strikes >= MAX_STRIKES:
                    reason = f'chunk_{err}'
                    break
            else:
                strikes = 0
            if qs and gh is None:
                strikes += 1
                if strikes >= MAX_STRIKES:
                    reason = 'traj_fail'
                    break
            if gh is not None:
                rf = gh.get_result_async()
                deadline = time.time() + len(qs) * CONTROL_DT + 3.0
                while not rf.done():
                    if time.time() > deadline:
                        break
                    time.sleep(0.005)
            elif not qs:
                time.sleep(CONTROL_DT)
            steps += max(len(qs), 1)

            # gripper transition scheduled at this chunk's end tick
            if grip_next is not None and cut <= len(qs):
                if attached and grip_next > GRIP_OPEN_T:
                    node.detach_box()
                    attached = False
                node.control_gripper([grip_next])
                if not attached and grip_next < GRIP_CLOSE_T:
                    box_p = ic.model_pos(node, box_model)
                    T = rec._T_base_odom()
                    eef = rec._eef_pose()
                    if box_p is not None and T is not None and eef is not None:
                        box_b = (T @ np.array([*box_p, 1.0]))[:3]
                        dist = float(np.linalg.norm(box_b - eef[:3]))
                        print(f'[grip] close cmd, box dist {dist:.3f} m '
                              f'(attach range {ATTACH_RANGE})')
                        if dist < ATTACH_RANGE:
                            node.object_model = box_model
                            ok = node.attach_box()
                            print(f'[grip] attach_box -> {ok}')
                            attached = bool(ok)

            if not attached:
                for py in spp.PLACE_ORDER_Y:
                    if ic.box_in_pocket(node, box_model, py):
                        reason = 'success'
                        break
                if reason == 'success':
                    break
            eef_now = rec._eef_pose()
            if eef_now is not None:
                eef_hist.append(eef_now[:3])
                if len(eef_hist) * client.n_action_steps > STUCK_WINDOW:
                    eef_hist.pop(0)
                    if np.ptp(np.asarray(eef_hist), axis=0).max() < STUCK_EPS:
                        reason = 'stuck'
                        break
    finally:
        try:
            sim_pause(node, False)        # never leave the world frozen
        except Exception:
            pass

    data = rec.stop()
    undisturbed = ic.shelf_undisturbed(node, set(absent) | {box_model})
    if attached:
        node.detach_box()
    node.control_gripper(GRIPPER_OPEN)
    info = {'steps': steps, 'undisturbed': bool(undisturbed)}
    if reason == 'success' and not undisturbed:
        reason = 'success_disturbed'
    return reason, info, data


# ------------------------------------------------------------ episode
def run_episode(node, rec, client, idx, park, max_steps, absent=frozenset()):
    """One policy rollout. Returns (result_str, info_dict)."""
    box_model = spp.shelf_box_model(spp.TIER, idx)
    if not ic.move_agv(node, *park):
        return 'agv_fail', {}, None
    # The expert drops the TARGET box's stock phantom before reaching in
    # (grasping means entering it); without this every rollout dies in
    # chunk_collision right at the pre-grasp. Other boxes stay guarded.
    node.set_shelf_stock_absent(spp.TIER, idx, absent=True)
    rec.start(box_model)
    t0 = time.time()
    while len(rec.buf['eef']) < client.n_obs_steps:
        time.sleep(CONTROL_DT)
        if time.time() - t0 > 10.0:
            rec.stop()
            return 'no_obs', {}, None

    attached = False
    steps = 0
    strikes = 0
    reason = 'timeout'
    eef_hist = []
    chunk_end = None                  # wall time the in-flight chunk finishes
    seed_q = node.current_joints.tolist()   # where the NEXT chunk will start
    pending_grip = None               # gripper value to apply at next chunk end
    grip_cmd_now = float(GRIPPER_OPEN[0])
    while steps < max_steps:
        # capture obs PRED_LEAD before the in-flight chunk ends, predict and
        # IK while the arm is still moving, splice the next goal at its end
        if chunk_end is not None:
            while time.time() < chunk_end - PRED_LEAD:
                time.sleep(0.005)
        if pending_grip is not None:
            # a close/open predicted mid-chunk: apply near its actual tick
            # (chunk end) instead of at send time (up to 0.8 s early)
            if attached and pending_grip > GRIP_OPEN_T:
                node.detach_box()
                attached = False
            node.control_gripper([pending_grip])
            grip_cmd_now = pending_grip
            pending_grip = None
        To, st = client.n_obs_steps, client.obs_stride
        t_obs = time.time()           # for the dynamic splice skip below
        obs = {
            'agentview_image': np.asarray(stack_obs(rec.buf['agent'], To, st), np.uint8),
            'robot0_eye_in_hand_image': np.asarray(stack_obs(rec.buf['wrist'], To, st), np.uint8),
            'robot_eef_pose': np.asarray(stack_obs(rec.buf['eef'], To, st), np.float32),
            'gripper': np.asarray(stack_obs(rec.buf['grip'], To, st), np.float32),
            # privileged box pose for the lowdim baseline; the server drops it
            # for image policies (filters obs to the ckpt's shape_meta keys).
            'object': np.asarray(stack_obs(rec.buf['object'], To, st), np.float32),
        }
        try:
            action = client.act(obs)
        except Exception:
            # transient socket drop: reconnect once, else fail the episode
            try:
                time.sleep(1.0)
                client.reconnect()
                action = client.act(obs)
            except Exception:
                reason = 'server_lost'
                break
        # drop the steps that will elapse between obs capture and the splice
        # (measured, not fixed: inference time varies and a fixed skip leaves
        # the gripper channel misaligned -> spurious close commands)
        if chunk_end is not None:
            if FIXED_SKIP is not None:      # A/B: reproduce the fixed-skip path
                skip = FIXED_SKIP
            else:
                splice_t = max(chunk_end, time.time())  # late inference -> now
                # round, not ceil: the nominal lead is ~0.30-0.31s and ceil's
                # +1 tick misaligns the gripper channel (A/B verified: closes
                # at 0.36m vs 0.10-0.13m with the correct 3-tick skip)
                skip = round((splice_t - t_obs) / CONTROL_DT)
            skip = min(max(skip, 0), len(action) - 1)
            if skip != 3:             # 3 = the old fixed value; log deviations
                print(f'[splice] skip={skip}', flush=True)
            action = action[skip:]
        qs, err = chunk_to_joints(node, action, seed_q)
        # A grazing waypoint is not fatal: execute the valid prefix and let the
        # policy re-predict from fresh obs (closed-loop self-correction). Only
        # persistent failure ends the episode; contamination is still caught by
        # the final shelf_undisturbed gate.
        if err:
            strikes += 1
            if strikes >= MAX_STRIKES:
                reason = f'chunk_{err}'
                break
        else:
            strikes = 0
        if qs:
            if chunk_end is not None:
                while time.time() < chunk_end - 0.03:
                    time.sleep(0.002)
            gh = send_chunk(node, qs)
            if gh is None:
                strikes += 1
                chunk_end = None
                seed_q = node.current_joints.tolist()
                if strikes >= MAX_STRIKES:
                    reason = 'traj_fail'
                    break
            else:
                chunk_end = time.time() + len(qs) * CONTROL_DT
                seed_q = qs[-1]
        else:
            time.sleep(CONTROL_DT)    # nothing executable; wait one tick
            chunk_end = None
            seed_q = node.current_joints.tolist()
        steps += max(len(qs), 1)

        if not qs:
            continue                  # nothing executed: no gripper/success step
        # gripper: apply step-0 commands now (correct tick); a transition
        # predicted deeper in the chunk is DEFERRED to the chunk-end wake so
        # close/open land near their actual ticks despite the overlap.
        g_first = float(action[0, 7])
        g_min = float(action[:len(qs), 7].min())
        g_max = float(action[:len(qs), 7].max())
        if steps % 80 == 0 or (not attached and g_min < GRIP_CLOSE_T):
            print(f'[grip] t={steps} g0={g_first:.4f} g_min={g_min:.4f} '
                  f'attached={attached}')
        if not attached and grip_cmd_now > GRIP_CLOSE_T:
            if g_first < GRIP_CLOSE_T:
                node.control_gripper([g_first])
                grip_cmd_now = g_first
            elif g_min < GRIP_CLOSE_T:
                pending_grip = g_min
        elif attached and grip_cmd_now < GRIP_OPEN_T:
            if g_first > GRIP_OPEN_T:
                node.detach_box()
                node.control_gripper([g_first])
                grip_cmd_now = g_first
                attached = False
            elif g_max > GRIP_OPEN_T:
                pending_grip = g_max
        # attach on a fresh close command near the box (data convention)
        if not attached and grip_cmd_now < GRIP_CLOSE_T:
            box_p = ic.model_pos(node, box_model)
            T = rec._T_base_odom()
            eef = rec._eef_pose()
            if box_p is not None and T is not None and eef is not None:
                box_b = (T @ np.array([*box_p, 1.0]))[:3]
                dist = float(np.linalg.norm(box_b - eef[:3]))
                print(f'[grip] close cmd, box dist {dist:.3f} m '
                      f'(attach range {ATTACH_RANGE})')
                if dist < ATTACH_RANGE:
                    node.object_model = box_model
                    ok = node.attach_box()
                    print(f'[grip] attach_box -> {ok}')
                    if ok:
                        attached = True

        # success: box seated in any pocket, gripper released
        if not attached:
            for pk, py in enumerate(spp.PLACE_ORDER_Y):
                if ic.box_in_pocket(node, box_model, py):
                    reason = 'success'
                    break
            if reason == 'success':
                break

        eef_now = rec._eef_pose()
        if eef_now is not None:
            eef_hist.append(eef_now[:3])
            if len(eef_hist) * client.n_action_steps > STUCK_WINDOW:
                eef_hist.pop(0)
                if np.ptp(np.asarray(eef_hist), axis=0).max() < STUCK_EPS:
                    reason = 'stuck'
                    break

    data = rec.stop()
    undisturbed = ic.shelf_undisturbed(node, set(absent) | {box_model})
    if attached:
        node.detach_box()
    node.control_gripper(GRIPPER_OPEN)
    # NOTE: the target box's stock phantom stays ABSENT here on purpose --
    # restoring it while the gripper is still at the shelf wraps the phantom
    # around the arm ("start state in collision", planner can never start).
    # The caller restores it AFTER recover() has moved the arm away.
    info = {'steps': steps, 'undisturbed': bool(undisturbed)}
    if reason == 'success' and not undisturbed:
        reason = 'success_disturbed'
    return reason, info, data


def write_video(path, data, fps=ic.CONTROL_HZ):
    """Side-by-side (agentview | wrist) mp4 of one rollout, from the recorder
    buffers the collector already fills at CONTROL_HZ."""
    import cv2
    a, w = data['agentview_image'], data['robot0_eye_in_hand_image']
    h, wd = a.shape[1], a.shape[2] * 2
    vw = cv2.VideoWriter(str(path), cv2.VideoWriter_fourcc(*'mp4v'), fps, (wd, h))
    for f_a, f_w in zip(a, w):
        frame = np.concatenate([f_a, f_w], axis=1)
        vw.write(cv2.cvtColor(frame, cv2.COLOR_RGB2BGR))
    vw.release()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--episodes', type=int, default=20)
    ap.add_argument('--port', type=int, default=5556)
    ap.add_argument('--board-top', type=float, default=1.22)
    ap.add_argument('--max-steps', type=int, default=900)
    ap.add_argument('--seed', type=int, default=0)
    ap.add_argument('--tag', default='eval')
    ap.add_argument('--fixed-skip', type=int, default=3,
                    help='splice skip in ticks (default 3, A/B-verified best: '
                         'closes at 0.10-0.13m vs 0.36m+ with measured skip); '
                         'pass -1 for the measured-latency variant')
    ap.add_argument('--record', action='store_true',
                    help='save a (agentview|wrist) mp4 per episode')
    ap.add_argument('--stepped', action='store_true',
                    help='freeze the sim during inference (robomimic '
                         'semantics): measures the algorithm, not the latency '
                         'engineering. Requires isaac_sim.py with /sim_pause.')
    args = ap.parse_args()
    global FIXED_SKIP
    FIXED_SKIP = None if args.fixed_skip < 0 else args.fixed_skip

    from cr7_pnp import geometry as _geom
    _geom.SHELF_TIER_TOPS[1] = args.board_top
    x_offset, park_y = ic.BOARD_ANCHOR[args.board_top]
    random.seed(args.seed)

    client = PolicyClient(args.port)
    print(f'policy: To={client.n_obs_steps} Ta={client.n_action_steps}')

    rclpy.init()
    node = spp.HubPickPlace()
    node.setup_planner()
    node.shelf_pose = None
    spp.pockets.subscribe_models(node)
    node._set_state_cli = node.create_client(SetEntityState,
                                             '/gazebo/set_entity_state')
    node._sim_pause_cli = node.create_client(SetBool, '/sim_pause')
    rec = ic.Recorder(node)
    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2.0)
    if not node._set_state_cli.wait_for_service(timeout_sec=10.0):
        print('no /gazebo/set_entity_state -- is isaac_sim.py running?')
        return
    if args.stepped and not node._sim_pause_cli.wait_for_service(timeout_sec=5.0):
        print('no /sim_pause service -- restart isaac_sim.py (needs the '
              'stepped-eval patch)')
        return
    # self-heal a dirty start (a crashed run can leave the AGV at a station,
    # where hub bring-up IK is infeasible): park at the stock spawn first
    ic.move_agv(node, 0.683, 0.008)
    if not spp.bringup(node):
        # A killed run can leave the arm twisted inside a phantom ("start
        # state in collision") or in a posture that wastes the hub IK's
        # near-seeding (collision-free=0 at 600 retries). Force an unchecked
        # move back to the URDF spawn posture (all zeros) -- where every
        # clean bring-up starts from -- and retry once. Sim-only brute force.
        print('[eval] bringup failed; forcing unchecked spawn-posture move')
        node.detach_box_collision()
        node.execute_path([node.current_joints.tolist(), [0.0] * 6],
                          speed=0.3)
        if not spp.bringup(node):
            return
    while rec.latest['agent'] is None or rec.latest['wrist'] is None:
        print('waiting for camera topics...')
        time.sleep(1.0)

    n_boxes = spp.N_BOXES
    by_x = sorted(range(n_boxes), key=lambda i: ic.SHELF_BOX_XS[i])
    stations = [by_x[k:k + ic.N_POCKETS]
                for k in range(0, n_boxes, ic.N_POCKETS)]
    out = pathlib.Path(ic.OUT_DIR).parent / 'eval'
    out.mkdir(parents=True, exist_ok=True)
    log_path = out / f'{args.tag}_{time.strftime("%m%d_%H%M%S")}.jsonl'
    results = []

    for ep in range(args.episodes):
        if not ic.reset_boxes(node):
            print('box reset failed; retrying in 5 s')
            time.sleep(5.0)
            continue
        # Match the training distribution: the collector stashed each
        # non-pick-tier box with p=0.5 (an always-full tier 2 is a ~0.1%
        # state in the data and visibly degrades the policy).
        absent = ic.randomize_occupancy(node)
        idx = ep % n_boxes            # round robin -> per-box stats
        st = next(s for s in stations if idx in s)
        anchor_x = float(np.mean([shelf_box_center(spp.TIER, i)[0]
                                  for i in st])) + x_offset
        park = (anchor_x + random.uniform(-ic.AGV_X_NOISE, ic.AGV_X_NOISE),
                park_y + random.uniform(-ic.AGV_Y_NOISE, ic.AGV_Y_NOISE))
        print(f'[ep {ep}] box idx {idx}, park ({park[0]:.3f}, {park[1]:.3f})')
        run_fn = run_episode_stepped if args.stepped else run_episode
        result, info, data = run_fn(node, rec, client, idx, park,
                                    args.max_steps, absent=absent)
        print(f'[ep {ep}] -> {result} {info}')
        if args.record and data is not None:
            vdir = out / 'videos'
            vdir.mkdir(exist_ok=True)
            vpath = vdir / f'{args.tag}_ep{ep:02d}_box{idx}_{result}.mp4'
            write_video(vpath, data)
            print(f'[ep {ep}] video -> {vpath}')
        recovered = ic.recover(node)
        if not recovered:
            # rollouts can end IN collision; planner can't start from there.
            # Sim-only brute force: unchecked straight move to the hub
            # (the next episode resets every box anyway).
            print('[eval] recover: planner failed, forcing unchecked hub move')
            node.detach_box_collision()
            node.execute_path([node.current_joints.tolist(), list(node.hub_q)],
                              speed=0.3)
            recovered = ic.recover(node)
        # safe to re-arm the target box phantom only now (arm away from shelf)
        node.set_shelf_stock_absent(spp.TIER, idx, absent=False)
        rec_row = {'ep': ep, 'box_idx': idx, 'park': list(park),
                   'result': result, **info}
        results.append(rec_row)
        with open(log_path, 'a') as f:
            f.write(json.dumps(rec_row) + '\n')
        if not recovered:
            print('[eval] recover failed even after forced hub move; stopping')
            break

    n = len(results)
    succ = sum(r['result'] == 'success' for r in results)
    print(f'\n=== {succ}/{n} success ({100.0*succ/max(n,1):.0f}%) '
          f'-> {log_path}')
    by_reason = {}
    for r in results:
        by_reason[r['result']] = by_reason.get(r['result'], 0) + 1
    print('by result:', by_reason)
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
