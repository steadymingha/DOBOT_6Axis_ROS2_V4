#!/usr/bin/env python3
"""Scripted expert dataset collection for diffusion policy -- Isaac Sim edition.

Drives the PROVEN hub-and-spoke shelf mission (sequences/shelf_pick_place.py
building blocks: pre-flighted spokes, guarded place) as the expert, fully
automated -- no SPACE trigger, no ArUco/tag capture step (the shelf sits at the
exact SHELF_WORLD_POSE spawn in sim, so vision adds nothing to the dataset).

Per episode (= one pick-place of ONE random tier-1 box into one pocket):
  - the AGV is teleported to a random park near the target box (that variation
    IS the data diversity; on a pre-flight failure it re-parks once and retries)
  - obs are recorded at CONTROL_HZ: canonical fixed cam + d405 wrist cam
    (image dataset) and eef pose/gripper/target-box pose in the BASE frame
    (lowdim dataset) -- both stored in the SAME .npz, so lowdim training now
    and image-hybrid later read the same files
  - action[t] = absolute BASE-frame eef pose (+ gripper) at t+1 -- the executed
    expert trajectory as an absolute-pose action stream (matches deploy: the
    policy outputs absolute eef targets for the Jacobian servo)
  - only SUCCESSFUL episodes are saved (box verified seated in its pocket)

When all pockets are filled the boxes are teleported back to their shelf spawns
(/gazebo/set_entity_state, served by isaac/isaac_sim.py) and collection
continues -- the "base full -> restart" requirement.

Run (Isaac sim already up via run_mpo700_cr7_isaac.sh). SYSTEM python -- the
.venv numpy 2.x segfaults the ROS pinocchio build:
    source /opt/ros/humble/setup.bash && source ~/dobot_ws/install/setup.bash
    /usr/bin/python3 diffusionpolicy/isaac_collect.py [--episodes N]
"""

import math
import os
import pathlib
import random
import sys
import threading
import time

import cv2
import numpy as np
import rclpy
from cv_bridge import CvBridge
from gazebo_msgs.srv import SetEntityState
from rclpy.duration import Duration
from rclpy.executors import MultiThreadedExecutor
from sensor_msgs.msg import Image

WS = os.path.expanduser("~/dobot_ws")
sys.path.insert(0, os.path.join(WS, "src/DOBOT_6Axis_ROS2_V4/sequences"))
import shelf_pick_place as spp  # noqa: E402  (inserts the package root itself)
from cr7_pnp import (  # noqa: E402
    BOX_SIZE, GRIPPER_OPEN, POCKET_X, POCKET_SURFACE_Z,
    SHELF_BOX_XS, shelf_box_center,
)

CONTROL_HZ = 10                # obs/action rate; MUST match deploy
# Stored image size, both cams. Stored LARGE on purpose: the training dataloader
# can crop/downscale, but nothing can restore detail lost at collection time
# (84x84 smeared the boxes/pockets beyond recognition, checked 2026-07-24).
IMG_HW = (240, 320)            # (H, W)
OUT_DIR = pathlib.Path(os.path.join(WS, "diffusionpolicy/data/isaac_shelf/episodes"))
SNAP_DIR = OUT_DIR.parent / "snapshots"   # per-episode QC stills (carry / place)
ROBOT_NAME = "cr7_on_mpo700"
N_POCKETS = len(spp.PLACE_ORDER_Y)

# AGV park randomization, STATION based: boxes are grouped into clusters of up
# to N_POCKETS adjacent boxes (sorted by shelf x), and the AGV parks at the
# cluster centre + the PROVEN spawn-park offset -- the stock spawn (0.683,
# 0.008) is exactly inner-4-cluster centre 0.8 - 0.117, where all four boxes
# are grasp-verified. Per-episode noise around that anchor keeps the data
# diversity without leaving the verified reach envelope.
AGV_X_OFFSET = -0.117
AGV_Y = 0.008
AGV_X_NOISE = 0.05
AGV_Y_NOISE = 0.03

# Verified AGV park anchor (x_offset, y) per pick-board height (--board-top).
# Each board reuses ONE calibrated relative geometry across all stations, the
# way the 2-tier -0.117 was hand-verified. The low board is NOT just a lower y:
# the 2-tier x_offset -0.117 gives only 2/4 approach-spoke passes there;
# park_calibration_sweep.py (2026-07-28, no motion) found (-0.155, -0.160)
# passes 4/4 inner + 8/10 whole row (the 2 residual = center inner boxes a/b,
# RRT sampling-flaky, absorbed by the collector's retry+noise). See
# diffusionpolicy/lowboard_test/.
BOARD_ANCHOR = {1.22: (AGV_X_OFFSET, AGV_Y), 0.72: (-0.155, -0.160)}

# Re-park attempts per box before skipping. The low board's feasible park region
# is narrow and its planner is sampling-flaky, but its failures are CHEAP (no
# motion: approach-spoke / IK-branch pre-flight aborts before the arm moves), so
# each extra attempt just re-rolls the noise + RRT seed at ~no cost and converts
# most would-be skips into saves. The 2-tier region is broad, so 2 suffices.
BOARD_ATTEMPTS = {1.22: 2, 0.72: 5}


# ---------------------------------------------------------------- recorder
class Recorder:
    """10 Hz background recorder on the motion node's executor: two camera subs
    + a timer sampling eef FK / gripper cmd / target-box pose (BASE frame)."""

    def __init__(self, node):
        self.node = node
        self.bridge = CvBridge()
        self.latest = {"agent": None, "wrist": None}
        self.grip_cmd = float(GRIPPER_OPEN[0])
        self.active = False
        self.buf = None
        self.target_model = None
        self.raw_agent = None   # full-res canonical frame for QC snapshots
        node.create_subscription(
            Image, "/camera/canonical/image_raw",
            lambda m: self._img_cb("agent", m), 1)
        node.create_subscription(
            Image, "/camera/d405/color/image_raw",
            lambda m: self._img_cb("wrist", m), 1)
        # Track the commanded gripper value by wrapping control_gripper.
        orig = node.control_gripper
        def wrapped(cmd, *a, **kw):
            self.grip_cmd = float(cmd[0]) if hasattr(cmd, "__len__") else float(cmd)
            return orig(cmd, *a, **kw)
        node.control_gripper = wrapped
        node.create_timer(1.0 / CONTROL_HZ, self._tick)

    def _img_cb(self, key, msg):
        img = self.bridge.imgmsg_to_cv2(msg, "rgb8")
        if key == "agent":
            self.raw_agent = img
        self.latest[key] = cv2.resize(img, (IMG_HW[1], IMG_HW[0]))

    def _eef_pose(self):
        q = self.node.current_joints
        if q is None:
            return None
        pos, R = self.node.ik_model.fk_tcp(self.node.ik_model.pin_q(q.tolist()))
        return np.concatenate([pos, spp._R_to_quat(R)]).astype(np.float32)

    def _T_base_odom(self):
        """base_link<-odom 4x4 with a SHORT timeout (this runs inside the 10 Hz
        tick; spp._lookup_T's 3 s block would stall the callback group)."""
        try:
            tf = self.node.tf_buffer.lookup_transform(
                "base_link", "odom", rclpy.time.Time(),
                timeout=Duration(seconds=0.2))
        except Exception:
            return None
        t, q = tf.transform.translation, tf.transform.rotation
        return spp._make_T(spp.quat_to_R(q.x, q.y, q.z, q.w), [t.x, t.y, t.z])

    def _box_pose_base(self):
        """Target box pose in base_link (world model_states composed with TF).
        Falls back to the previous sample on a TF/model_states hiccup."""
        ms = getattr(self.node, "_model_states", None)
        if ms is None or self.target_model not in ms.name:
            return None
        p = ms.pose[ms.name.index(self.target_model)]
        T = self._T_base_odom()
        if T is None:
            return None
        pos = T @ np.array([p.position.x, p.position.y, p.position.z, 1.0])
        o = p.orientation
        R = T[:3, :3] @ spp.quat_to_R(o.x, o.y, o.z, o.w)
        return np.concatenate([pos[:3], spp._R_to_quat(R)]).astype(np.float32)

    def _tick(self):
        if not self.active:
            return
        eef = self._eef_pose()
        if (eef is None or self.latest["agent"] is None
                or self.latest["wrist"] is None):
            return
        obj = self._box_pose_base()
        if obj is None:
            obj = self.buf["object"][-1] if self.buf["object"] else np.zeros(7, np.float32)
        self.buf["agent"].append(self.latest["agent"].copy())
        self.buf["wrist"].append(self.latest["wrist"].copy())
        self.buf["eef"].append(eef)
        self.buf["grip"].append([self.grip_cmd])
        self.buf["object"].append(obj)

    def start(self, target_model):
        self.buf = {k: [] for k in ("agent", "wrist", "eef", "grip", "object")}
        self.target_model = target_model
        self.active = True

    def stop(self):
        self.active = False
        time.sleep(2.0 / CONTROL_HZ)   # let an in-flight tick finish
        b = self.buf
        T = min(len(v) for v in b.values())   # trim any tick caught mid-append
        if T < 5:
            return None
        b = {k: v[:T] for k, v in b.items()}
        eef = np.asarray(b["eef"], np.float32)
        grip = np.asarray(b["grip"], np.float32)
        # action[t] = executed absolute eef pose + gripper at t+1 (last repeated)
        nxt = np.concatenate([eef[1:], eef[-1:]])
        gnx = np.concatenate([grip[1:], grip[-1:]])
        return dict(
            agentview_image=np.asarray(b["agent"], np.uint8),          # (T,H,W,3)
            robot0_eye_in_hand_image=np.asarray(b["wrist"], np.uint8),  # (T,H,W,3)
            robot_eef_pose=eef,                                        # (T,7)
            gripper=grip,                                              # (T,1)
            object=np.asarray(b["object"], np.float32),                # (T,7)
            action=np.concatenate([nxt, gnx], axis=1),                 # (T,8)
        )


# ------------------------------------------------------------ sim helpers
def set_entity_state(node, name, xyz, yaw=0.0, timeout=5.0):
    req = SetEntityState.Request()
    req.state.name = name
    p, o = req.state.pose.position, req.state.pose.orientation
    p.x, p.y, p.z = float(xyz[0]), float(xyz[1]), float(xyz[2])
    o.z, o.w = math.sin(yaw / 2.0), math.cos(yaw / 2.0)
    fut = node._set_state_cli.call_async(req)
    t0 = time.time()
    while not fut.done() and time.time() - t0 < timeout:
        time.sleep(0.02)
    return bool(fut.done() and fut.result() is not None and fut.result().success)


def model_pos(node, name):
    ms = getattr(node, "_model_states", None)
    if ms is None or name not in ms.name:
        return None
    p = ms.pose[ms.name.index(name)].position
    return np.array([p.x, p.y, p.z])


def move_agv(node, x, y):
    """Teleport the AGV and wait for the sim state (model_states cache is 2 Hz)
    and TF to catch up, then re-anchor the shelf collision to the new base."""
    if not set_entity_state(node, ROBOT_NAME, (x, y, 0.0)):
        node.get_logger().error("[agv] set_entity_state failed")
        return False
    t0 = time.time()
    while time.time() - t0 < 5.0:
        p = model_pos(node, ROBOT_NAME)
        if p is not None and abs(p[0] - x) < 0.03 and abs(p[1] - y) < 0.03:
            break
        time.sleep(0.1)
    else:
        node.get_logger().error("[agv] teleport not reflected in model_states")
        return False
    time.sleep(0.7)  # let the odom TF (50 Hz, from the 2 Hz pose cache) settle
    return node.update_shelf_collision()


def reset_boxes(node):
    """Teleport every shelf box back to its spawn (clears the pockets)."""
    ok = True
    for tier in (1, 2):
        for i in range(len(SHELF_BOX_XS)):
            ok &= set_entity_state(node, spp.shelf_box_model(tier, i),
                                   shelf_box_center(tier, i), yaw=1.5708)
    time.sleep(1.5)  # settle + let the 2 Hz model_states cache refresh
    return ok


# Off-shelf stash for randomized occupancy: far floor spot outside every camera
# FOV (base cam looks +y at the shelf; the stash sits far behind it).
def stash_xyz(j):
    return (6.0 + 0.4 * j, -5.5, 0.07)


def randomize_occupancy(node):
    """Pass-start shelf dressing: the PICK tier starts FULL (the pass itself
    then shows every state from full down to empty as boxes are delivered);
    non-pick-tier boxes are absent with p=0.5 for visual variety -- teleported
    to the stash AND their resting-stock phantoms parked
    (set_shelf_stock_absent), so vision, physics and the collision model agree.
    Covers ALL (tier, i), which also clears stale absent flags from the
    previous pass's picks. Returns the set of absent model names."""
    absent = set()
    for tier in (1, 2):
        for i in range(len(SHELF_BOX_XS)):
            gone = (tier != spp.TIER) and random.random() < 0.5
            node.set_shelf_stock_absent(tier, i, absent=gone)
            if gone:
                m = spp.shelf_box_model(tier, i)
                set_entity_state(node, m, stash_xyz(len(absent)))
                absent.add(m)
    time.sleep(0.7)
    return absent


def snap(rec, path):
    """QC still: full-res canonical frame -> PNG (the headless-run eyeball check:
    is the box carried upright, did the place land clean)."""
    if rec.raw_agent is not None:
        cv2.imwrite(str(path), cv2.cvtColor(rec.raw_agent, cv2.COLOR_RGB2BGR))


def shelf_undisturbed(node, exclude):
    """Contamination gate: every shelf box NOT in `exclude` must still sit at its
    spawn pose (2 cm xy / 3 cm z). A brushed neighbour or a toppled box poisons
    the episode AND the rest of the cycle -- the caller discards and resets."""
    bad = []
    for tier in (1, 2):
        for i in range(len(SHELF_BOX_XS)):
            m = spp.shelf_box_model(tier, i)
            if m in exclude:
                continue
            p = model_pos(node, m)
            if p is None:
                continue
            d = p - np.array(shelf_box_center(tier, i))
            if abs(d[0]) > 0.02 or abs(d[1]) > 0.02 or abs(d[2]) > 0.03:
                bad.append(f"{m}({d[0]*1000:+.0f},{d[1]*1000:+.0f},{d[2]*1000:+.0f}mm)")
    if bad:
        print(f"[check] shelf DISTURBED: {', '.join(bad)}")
    return not bad


def box_in_pocket(node, box_model, pocket_y):
    """Success check: box centre within tolerance of the pocket centre (world)."""
    T = spp._lookup_T(node, "odom", "base_link")
    p = model_pos(node, box_model)
    if T is None or p is None:
        return False
    want = T @ np.array([POCKET_X, pocket_y,
                         POCKET_SURFACE_Z + BOX_SIZE[2] / 2.0, 1.0])
    err = p - want[:3]
    ok = abs(err[0]) < 0.06 and abs(err[1]) < 0.06 and abs(err[2]) < 0.05
    if not ok:
        node.get_logger().error(
            f"[check] {box_model} off pocket by ({err[0]*1000:+.0f}, "
            f"{err[1]*1000:+.0f}, {err[2]*1000:+.0f}) mm")
    return ok


# ------------------------------------------------------------ expert cycle
def pick_place(node, rec, idx, pocket, snap_prefix):
    """shelf_pick_place.pick_place_one_box minus vision: explicit pocket index,
    layout-derived box pose (SHELF_WORLD_POSE default -- exact in sim).
    Saves a QC still at the two moments that can poison the dataset: back at the
    hub CARRYING the box, and back at the hub after the PLACE."""
    box_world, box_model = spp.shelf_box(node, idx)
    pocket_y = spp.PLACE_ORDER_Y[pocket]
    place_ref = spp.compute_place_ref(node, pocket_y)
    if place_ref is None:
        node.get_logger().error("[ep] pocket family seed IK failed")
        return False
    place_jaw_x = node.gripper_x_in_base_fk(place_ref)
    if place_jaw_x is None:
        return False
    node.attach_box_collision()
    feasible = node.plan_spoke(
        node.hub_q,
        spp.pose_at(spp.pocket_hover_xyz(pocket_y, place_jaw_x), spp.place_quat()),
        place_ref, label=f"pre pocket(y={pocket_y:+.3f})") is not None
    node.detach_box_collision()
    if not feasible:
        node.get_logger().error("[ep] pocket unreachable from this park")
        return False
    if not spp.shelf_pick_to_hub(node, box_world, box_model,
                                 (spp.TIER, idx), place_ref):
        node.set_shelf_stock_absent(spp.TIER, idx, absent=False)
        return False
    snap(rec, f"{snap_prefix}_carry.png")
    ok = spp.pocket_place_from_hub(node, pocket_y, place_ref, place_jaw_x,
                                   label=box_model)
    if ok:
        snap(rec, f"{snap_prefix}_place.png")
    return ok


def recover(node):
    """After a failed episode: make sure nothing is attached and the arm is at
    the hub. A hub return failure is fatal (every next episode starts there)."""
    node.detach_box_collision()
    try:
        node.detach_box()
    except Exception:
        pass
    node.control_gripper(GRIPPER_OPEN)
    if not node.go_to_hub():
        node.get_logger().fatal("[ep] cannot return to hub; stopping collection")
        return False
    return True


def main():
    n_episodes = 200
    if "--episodes" in sys.argv:
        n_episodes = int(sys.argv[sys.argv.index("--episodes") + 1])
    board_top = 1.22
    if "--board-top" in sys.argv:
        board_top = float(sys.argv[sys.argv.index("--board-top") + 1])
    if board_top not in BOARD_ANCHOR:
        print(f"--board-top {board_top} has no verified AGV park "
              f"(known: {sorted(BOARD_ANCHOR)})")
        return
    # Every pick target / stock phantom / servo height derives from this dict
    # at call time, so one patch moves the whole verified pipeline.
    from cr7_pnp import geometry as _geom
    _geom.SHELF_TIER_TOPS[1] = board_top
    x_offset, park_y = BOARD_ANCHOR[board_top]
    max_attempts = BOARD_ATTEMPTS[board_top]
    random.seed()

    rclpy.init()
    node = spp.HubPickPlace()
    node.setup_planner()
    node.shelf_pose = None                      # layout default (no vision)
    spp.pockets.subscribe_models(node)          # /gazebo/model_states cache
    node._set_state_cli = node.create_client(SetEntityState,
                                             "/gazebo/set_entity_state")
    rec = Recorder(node)

    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2.0)

    if not node._set_state_cli.wait_for_service(timeout_sec=10.0):
        print("no /gazebo/set_entity_state -- is isaac_sim.py (current) running?")
        return
    if not spp.bringup(node):
        return
    while rec.latest["agent"] is None or rec.latest["wrist"] is None:
        print("waiting for camera topics (/camera/canonical, /camera/d405)...")
        time.sleep(1.0)

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    SNAP_DIR.mkdir(parents=True, exist_ok=True)
    saved = int(len(list(OUT_DIR.glob("episode_*.npz"))))   # resume-friendly
    n_boxes = spp.N_BOXES
    print(f"collecting {n_episodes} episodes ({saved} already on disk), "
          f"{n_boxes} tier-1 boxes, {N_POCKETS} pockets/cycle, "
          f"board top {board_top} m (park anchor x_off={x_offset:+.3f}, "
          f"y={park_y:+.3f})")

    # Stations: chunks of up to N_POCKETS adjacent boxes (pick-order indices
    # sorted by shelf x). Each box is picked from ITS station's proven park
    # anchor (cluster centre + AGV_X_OFFSET, the verified grasp geometry).
    by_x = sorted(range(n_boxes), key=lambda i: SHELF_BOX_XS[i])
    stations = [by_x[k:k + N_POCKETS] for k in range(0, n_boxes, N_POCKETS)]

    def station_of(idx):
        return next(s for s in stations if idx in s)

    while saved < n_episodes and rclpy.ok():
        # ---- one PASS: the shelf starts full and is emptied box by box, in
        # random order across ALL stations. Every N_POCKETS successful places
        # the base is "delivered" -- the pocket boxes are stashed off-scene and
        # picking continues -- so the dataset contains every shelf state from
        # full down to nearly empty. The shelf refills only at the next pass.
        if not reset_boxes(node):
            print("box reset failed; retrying in 5 s")
            time.sleep(5.0)
            continue
        absent = randomize_occupancy(node)
        stash_n = len(absent)
        print(f"[pass] tier-1 full ({n_boxes} boxes), "
              f"{len(absent)} tier-2 boxes stashed")
        placed = set()      # boxes currently in the base pockets
        dirty = False       # a neighbour got disturbed -> reset the shelf
        for idx in random.sample(range(n_boxes), n_boxes):
            if saved >= n_episodes or dirty:
                break
            # Next free pocket from the high-y end, CONTIGUOUS like the
            # mission: tied to the pocket count, so a skipped box leaves no
            # hole. (Episodes 0-66 predate this fix and the pass structure.)
            pocket = N_POCKETS - 1 - len(placed)
            box_model = spp.shelf_box_model(spp.TIER, idx)
            st = station_of(idx)
            anchor_x = float(np.mean([shelf_box_center(spp.TIER, i)[0]
                                      for i in st])) + x_offset
            ok = False
            for attempt in range(max_attempts):   # unreachable -> re-park & retry
                agv_x = anchor_x + random.uniform(-AGV_X_NOISE, AGV_X_NOISE)
                agv_y = park_y + random.uniform(-AGV_Y_NOISE, AGV_Y_NOISE)
                if not move_agv(node, agv_x, agv_y):
                    continue
                print(f"[ep {saved}] box {box_model} -> pocket {pocket}, "
                      f"AGV ({agv_x:.3f}, {agv_y:.3f}), attempt {attempt}")
                rec.start(box_model)
                ok = pick_place(node, rec, idx, pocket,
                                SNAP_DIR / f"ep_{saved:04d}_{box_model}")
                data = rec.stop()
                if ok and data is not None and box_in_pocket(
                        node, box_model, spp.PLACE_ORDER_Y[pocket]):
                    if not shelf_undisturbed(node, placed | absent | {box_model}):
                        print(f"[ep] episode DISCARDED (shelf disturbed); "
                              f"resetting the shelf")
                        dirty = True
                        break
                    # Episode meta: makes any future rule change FILTERABLE
                    # instead of forcing a re-collection (absent from eps 0-66).
                    data.update(
                        meta_box=np.array(box_model),
                        meta_box_idx=np.int64(idx),
                        meta_pocket=np.int64(pocket),
                        meta_station=np.array(st, np.int64),
                        meta_agv_xy=np.array([agv_x, agv_y], np.float32),
                        meta_board_top=np.float32(board_top),
                    )
                    np.savez_compressed(OUT_DIR / f"episode_{saved:04d}.npz",
                                        **data)
                    placed.add(box_model)
                    saved += 1
                    print(f"[ep] saved {saved}/{n_episodes} "
                          f"(len={len(data['action'])})")
                    break
                ok = False
                if not recover(node):
                    return
            if not ok and not dirty:
                print(f"[ep] box idx {idx} skipped (both attempts failed)")
            if len(placed) == N_POCKETS:
                # Base full -> DELIVER: stash the pocket boxes off-scene (the
                # AGV "drove off and unloaded") and keep emptying the shelf.
                for m in placed:
                    set_entity_state(node, m, stash_xyz(stash_n))
                    stash_n += 1
                absent |= placed
                placed = set()
                time.sleep(0.7)
                print("[pass] base delivered (pockets cleared); shelf continues")

    print(f"done: {saved} episodes in {OUT_DIR}")
    node.destroy_node()
    rclpy.shutdown()


if __name__ == "__main__":
    main()
