"""Wirebonder magazine transfers between base pockets and device slots.

Three transfers -- the SAME flow (pick from src, place at dst, hub at both ends),
differing only in src/dst:

    1. base   -> slot A      (load a magazine onto a device)
    2. slot B -> slot C       (move a magazine between slots)
    3. slot D -> base         (return a magazine to the base)

Slot naming: A=H_L, B=G_L, C=G_R, D=H_R (the four magazine positions; see
tools/spawn_device_markers.py / SLOT_OFFSET).

Each transfer may run on ONE device or ACROSS two devices: the wirebonders are
the same model but there can be several units, so a slot is addressed as
(device, slot_letter). Its world pose = device instance pose (DEVICES) composed
with the constant model-frame slot offset (SLOT_OFFSET). The slot is then looked
up in base_link via TF, so the AGV must be parked facing that device.

There is ONE generic transfer(node, src, dst); a BT/FSM mission node would call
it with whatever Locations. The number keys 1/2/3 are a dev stand-in trigger.

Slot offsets come from tools/spawn_device_markers.py (rail centre + 5 mm gap behind ->
magazine centre, in the model frame).

Device poses come from the AprilTag vision node (no precise AGV parking): start
wirebonder_vision_node.py FIRST (system python -- it needs cv2), then this script
in the .venv. On startup the arm goes to the hub, then servos to CAPTURE_FLANGE (a
separate close-to-tag pose) and reads every device's pose ONCE from
/vision/device_pose, returns to the hub, and offers the 1/2/3 menu -- so any
transfer reuses that one read (the arm may do 2 or 3 without 1). The hub stays
transfer-safe; the capture pose is decoupled. Press 'c' to re-capture after re-park.

Run (sim up, AGV parked roughly facing a device):
    source /opt/ros/humble/setup.bash
    source ~/dobot_ws/install/setup.bash
    cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
    python3 vision/wirebonder_vision_node.py                 # terminal A (system cv2)
    ~/dobot_ws/.venv/bin/python3 sequences/wirebonder_pick_place.py   # terminal B (.venv)

    # --no-vision: skip the vision node; use the hardcoded DEVICES placeholder
    # (requires precise parking). For arm-only testing without cv2/vision:
    ~/dobot_ws/.venv/bin/python3 sequences/wirebonder_pick_place.py --no-vision
"""

import math
import os
import sys
import time
import threading
from collections import namedtuple

import numpy as np
import rclpy
from rclpy.executors import MultiThreadedExecutor
from geometry_msgs.msg import PoseStamped
from std_msgs.msg import Int32

# sequences/ is one level below the package root; add the root (for cr7_pnp) and
# comms/ (for mcs_protocol) so both import when this file is run standalone.
_PKG_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, _PKG_ROOT)
sys.path.insert(0, os.path.join(_PKG_ROOT, 'comms'))
from cr7_pnp import (  # noqa: E402
    HubPickPlace, pose_at, quat_mul, quat_about_z,
    DOWN, GRIPPER_OPEN, GRIPPER_CLOSE, GRASP_TCP_ABOVE, GRASP_LATERAL_M,
    POCKET_X, POCKET_Y, POCKET_SURFACE_Z, BOX_SIZE,
)
import mcs_protocol as proto  # noqa: E402

# --- device model: slot magazine-centre offsets in the MODEL frame -------------
# Constant per wirebonder model. Each = the rail centre (from the collision STLs)
# shifted +y behind the rail by rail_half(0.0075) + mag_half(0.0405) + gap(0.005)
# = 0.053.
# Slot naming: A=H_L, B=G_L, C=G_R, D=H_R.
SLOT_OFFSET = {
    'A': (-0.348, -0.059, 0.896),   # H_L  (left, lower)
    'B': (-0.348, -0.059, 1.281),   # G_L  (left, upper)
    'C': (+0.348, -0.059, 1.281),   # G_R  (right, upper)
    'D': (+0.348, -0.059, 0.896),   # H_R  (right, lower)
}

# Wirebonder instances: name -> world (odom) pose (x, y, z, yaw_rad). Same model,
# several units; add a line per unit spawned in the world.
# NOTE: these are PLACEHOLDERS -- with vision, refresh_device_pose() overwrites the
# entry from /vision/device_pose. Under --no-vision the placeholder itself composes
# the waypoints, so it MUST equal OLD_DEVICE_POSE (the anchor): then SLOT_WORLD is
# reproduced EXACTLY, run after run -- fully deterministic for sim iteration.
# (The earlier "true" placeholder (2.35, 0.5, 0, 0) left the anchor bias
# UNcancelled: every waypoint shifted ~(-9,-31,-7) mm + a yaw-lever term.)
DEVICES = {
    'wb1': (2.359, 0.531, 0.007, 0.017),   # = OLD_DEVICE_POSE
}

# --- locations ----------------------------------------------------------------
# kind='base' -> ref is a constant base_link xyz (base pocket, rigid to the arm).
# kind='slot' -> ref is (device_name, slot_letter), resolved via DEVICES x SLOT.
# model/link are the Gazebo magazine names used when this location is a PICK src.
Location = namedtuple('Location', 'name kind ref yaw model link')

BOX_HALF_Z = BOX_SIZE[2] / 2.0
PICK_YAW = math.pi        # jaw azimuth at a base pocket (TUNE IN SIM)
SLOT_YAW = math.pi        # jaw azimuth at a slot, added to the device yaw (TUNE)
MAG_LINK = 'box_link'     # magazine link name (Gazebo)


def base_loc(pocket_y=POCKET_Y[3], model='box_l2c'):
    # box_l2c is on this pocket (observed). The TCP lands on the OUTER side of the box,
    # not on it -- a lateral grasp offset (GRASP_LATERAL_M) pushing the wrong way, not
    # the pocket index. That's the thing to fix next, separately.
    z = POCKET_SURFACE_Z + BOX_HALF_Z
    return Location('base', 'base', (POCKET_X, pocket_y, z), PICK_YAW, model, MAG_LINK)


def slot_loc(device, letter, model=None):
    return Location(f'{device}:{letter}', 'slot', (device, letter), SLOT_YAW,
                    model or f'mag_{device}_{letter}', MAG_LINK)


# The three transfers as (src, dst) Locations. Default to a single device (wb1);
# for a cross-device transfer point src/dst at different devices, e.g.
# slot_loc('wb2', 'C'). EDIT pocket / model placeholders to the real sim names.
# Gazebo magazine names (cr.world): base pocket=box_l2c, slot B=box_l2a, slot D=box_l2b.
# model only matters on the PICK src (the box grasped); the place dst reuses it.
SEQUENCES = {
    '1': (base_loc(),                            slot_loc('wb1', 'A')),  # box_l2c base  -> slot A
    '2': (slot_loc('wb1', 'B', model='box_l2a'), slot_loc('wb1', 'C')),  # box_l2a slotB -> slot C
    '3': (slot_loc('wb1', 'D', model='box_l2b'),
          base_loc(pocket_y=POCKET_Y[1])),                              # box_l2b slotD -> base pocket (y=0.059)
}

# Every device any transfer touches -- captured ONCE up front so any of 1/2/3 can
# run (on arrival the arm may do 2 or 3 without 1 first).
ALL_DEVICES = {loc.ref[0] for s, d in SEQUENCES.values()
               for loc in (s, d) if loc.kind == 'slot'}

# --- geometry / hub (TCP metres; TUNE IN SIM) ---------------------------------
HOVER_ABOVE = 0.12        # TCP hover height above the grasp/place point
# Base place descends until the box collision-model touches the pocket surface
# (guarded_descend), not a fixed drop. This is how far PAST the nominal HOVER_ABOVE
# it may keep going before giving up -- covers pocket-to-pocket surface height
# variance. It always stops AT contact, so a generous value is safe. (TUNE IN SIM)
BASE_PLACE_OVERTRAVEL = 0.04
# Hub = standby / transfer OBJECT hover. PROVEN transfer-safe: the slot approach
# servos run straight from here without collision (base->slotA completes under
# --no-vision). DECOUPLED from the capture pose so it never moves for the camera.
HUB_TCP = (0.33, 0.0, 0.32)

# View A = the ORIGINAL capture pose (TCP position in base_link; the pose pipeline
# drives the TCP, so this is what tools/jog_tcp.py prints as "TCP base_link"). Known
# reachable and framing the tag. capture_device plans a move here from the hub.
# DECOUPLED from the hub so tuning it never breaks the transfer.
CAPTURE_FLANGE = (0.373, 0.05, 0.148)
# Orientation at view A (quat xyzw, base_link). None -> the CAPTURE_PITCH formula (the
# original capture orientation). Set only if you jog view A to a NEW orientation.
CAPTURE_QUAT = None
# View B = view A + this base-frame TRANSLATION, the SECOND stereo viewpoint. Reached
# by a pure servo that HOLDS view A's orientation. Delta to the jogged view-B spot:
# (0.562,-0.408,0.753) - (0.373,0.05,0.148). Two views from different camera POSITIONS
# triangulate the tag (motion stereo) -> fixes range a single small tag can't.
# NOTE: ~0.78 m straight servo -- if it aborts "singular" (a prior 1 m diagonal did
# at ~372 mm), either shorten it or switch view B to a joint-space move (RRT routes
# around the singularity; needs the jogged joints).
CAPTURE_BASELINE = (0.189, -0.458, 0.605)
# PREFERRED for a far view B: the joint config at the jogged view-B pose (jog_tcp
# prints "joints (rad)"). When set, view B is reached by a joint-space RRT -- smooth
# and singularity-free -- instead of the straight CAPTURE_BASELINE servo (which
# jitters/aborts through the singularity on a long diagonal). None -> use the servo.
CAPTURE_B_JOINTS =  (-0.22080, -0.67615, -1.38912, +0.09719, -1.63472, -0.59292)
# Tilt the tool by this pitch (deg) at the capture pose so the camera views the tag
# OFF fronto-parallel. With two-view triangulation the range no longer rides on this,
# so it mainly helps the single-view diagnostic print; can be reduced. TUNE.
CAPTURE_PITCH_DEG = 20.0

# Wirebonder slots are FRONT-LOADING: part C overhangs the shelf, so a top-down
# drop is blocked. Instead the magazine is slid in HORIZONTALLY (tool stays DOWN,
# box flat) -- the long fixed jaw carries the box onto the shelf behind C while
# the gripper column stops just in front of C.
C_FRONT_LOCAL_Y = -0.120   # part C front face in the device-model frame (world = device_y + this)
C_CLEARANCE = 0.005        # TCP column stops this far in FRONT of part C (TUNE IN SIM)
SLOT_INSERT = 0.15         # horizontal travel from the front hover to the seat (TUNE IN SIM)
# Nudge the slot seat target, WORLD frame, metres. The front hover is tight, so
# tune one axis at a time (start ~0.01) until the approach servo clears:
#   x = device left(-)/right(+),  y = out toward front(-)/deeper into device(+),
#   z = down(-)/up(+).  Each attempt logs the resolved target so you can correlate.
SLOT_NUDGE = (0.0, 0.0, -0.01)

# Measured TCP world coordinates per slot, captured with tools/jog_tcp.py. LATER these
# become aruco-relative: a marker on each device is the reference frame and these
# convert to offsets from it -- for now they are absolute world xyz, so the AGV
# must be parked where they were measured. Jog and fill D.
# Two slot styles:
#   mode='front' (lower H slots, part C overhangs): slide in HORIZONTALLY. Stores
#       'approach'/'seat' TCP world points (jogged).
#   mode='top'   (upper G slots, open top): descend VERTICALLY onto the box. Stores
#       the box CENTRE world coords ('box', from cr.world) -- the grasp derives the
#       TCP from it via grasp_tcp_pose (box-centred, auto lateral hang), the SAME
#       primitive as the base pick, so no hand-jogged hover and B/C are symmetric.
SLOT_WORLD = {
    'A': {'mode': 'front', 'approach': (1.996, 0.17, 1.06), 'seat': (1.996, 0.30, 1.06)},
    # Box CENTRE world coords (SETTLED, not spawn). The cr.world spawn z=1.281 sits
    # 64 mm INTO the Cube_C shelf (top at model z 1.275), so Gazebo pops the box up
    # to rest on Cube_C -> real centre = 1.275 + box_half(0.07) = 1.345.
    # C = B with x mirrored about the device centre (2.35 +/- 0.348); same y,z.
    'B': {'mode': 'top', 'box': (2.002, 0.441, 1.355)},
    'C': {'mode': 'top', 'box': (2.698, 0.441, 1.355)},
    # D mirrors A (symmetric device): same y/z approach/seat style as A, only x
    # differs. x = box_l2b spawn centre (cr.world 2.698); the jaw is squared to the
    # device after staging (pick_slot_front_j1), so the TCP centres on the box.
    # NOT the high-jogged 2.576 -- that sits ON the central tower's (Cube_B, world
    # x<=2.58, z 0.90-1.13) right face and the gripper column collides with it.
    'D': {'mode': 'front', 'approach': (2.698, 0.17, 1.04), 'seat': (2.698, 0.30, 1.04)},
}
SLOT_PLACE_DROP = 0.03      # front-load PLACE: descend from the seat to set the
                            # (already-gripped) box onto the shelf (TUNE)
# Front-load PICK descends MORE than place: place lowers a box the pads already
# hold, but pick must plunge the OPEN pads down around the box body before closing
# (pad bottom is ~38 mm above the TCP), so the same 0.03 leaves the pads catching
# only the top edge. Separate knob -- raise until the grip is full (TUNE IN SIM).
# CEILING: past ~57 mm the elbow (Link3) hits the device central tower (wb_Cube_B)
# in this staged config, so this can't just be raised freely -- 50 mm leaves ~7 mm.
SLOT_PICK_DROP = 0.05

# Seq-3 (slot D) front-load pick: the long tuck to the D approach defeats the free
# RRT, but a jogged staging config brings the arm right in FRONT of D where a
# straight servo reaches the box. Full jogged config per slot (tools/jog_tcp.py
# "joints (rad)"): J6 at the seq-1 jaw azimuth, TCP on the slot's x plane. Reached
# in two moves -- J1 swing first (to stage's J1, ABSOLUTE so an aborted attempt
# retried from a non-hub pose doesn't compound), then the remaining joints --
# and UNDONE by the reverse replay. Slots not listed use the normal front-load pick.
# STAGE_JOINTS = {'D': (-4.05202, -0.33221, -2.24554, +1.01709, -1.56560, +2.28482)}
STAGE_JOINTS = {'D': (-3.7, -0.33221, -2.24554, +1.01709, -1.56560, +2.28482)}

# --- vision re-anchoring ------------------------------------------------------
# SLOT_WORLD holds the hand-jogged TCP waypoints as ABSOLUTE odom coords, captured
# with the device at OLD_DEVICE_POSE. Re-anchor them ONCE into the device MODEL
# frame, so at runtime they compose with the LIVE device pose (DEVICES[device],
# refreshed from /vision/device_pose) -- the AGV may park anywhere and the device
# may sit at any yaw. ponytail: device assumed upright, so only (x,y,z,yaw) matters.
# CRITICAL: this must be what VISION reported for the device when SLOT_WORLD was
# jogged, NOT the true world pose. The sim's two-view read has a fixed deterministic
# bias (~+9,+31,+7 mm, +1 deg vs the true 2.35,0.5,0,0; spread 0/0/0). Setting OLD to
# that biased read makes the SAME bias appear at both jog-reference and runtime, so it
# CANCELS and the waypoints reproduce SLOT_WORLD exactly. (Using the true pose leaves
# the bias uncancelled -> the ~31 mm shift that clips the tight front-load approach.)
OLD_DEVICE_POSE = (2.359, 0.531, 0.007, 0.017)

# Capture sanity gates (TUNE). A clean single vision node republishes ONE pose, so the
# median spread is ~0; anything large means a stale 2nd publisher or an unstable solve.
# The anchor bounds catch a constant-but-wrong solve (spread can't). AGV parks near the
# same spot, so a good capture lands close to OLD_DEVICE_POSE; z barely varies on a flat
# floor, so a big z offset is the strongest garbage signal.
CAPTURE_SPREAD_MAX_MM = 15.0
CAPTURE_POS_TOL_MM = 300.0
CAPTURE_YAW_TOL_DEG = 30.0


def fail(node, code, detail):
    """Record a failure for the MCS report AND log it. `code` = ErrorCode (the category
    MCS acts on); `detail` = the exact human message (cause/location/metrics). Both land
    on node.last_error / node.last_error_detail, read at the [REPORT] seam. Returns False
    so callers can `return fail(...)`."""
    node.last_error = code
    node.last_error_detail = detail
    node.get_logger().error(detail)
    return False


def _T_odom_model(x, y, z, yaw):
    c, s = math.cos(yaw), math.sin(yaw)
    return np.array([[c, -s, 0, x], [s, c, 0, y], [0, 0, 1, z], [0, 0, 0, 1]], float)


def _to_model(p_odom, dev_pose):
    """odom point -> device model frame, for a device at dev_pose=(x,y,z,yaw)."""
    return (np.linalg.inv(_T_odom_model(*dev_pose)) @ np.append(p_odom, 1.0))[:3]


def _to_odom(p_model, dev_pose):
    """device model point -> odom, for a device at dev_pose=(x,y,z,yaw)."""
    return (_T_odom_model(*dev_pose) @ np.append(p_model, 1.0))[:3]


# Measured TCP waypoints expressed in the device model frame (constant per model).
SLOT_LOCAL = {
    L: ({} if wp is None else
        {k: _to_model(np.array(v, float), OLD_DEVICE_POSE)
         for k, v in wp.items() if k != 'mode'})
    for L, wp in SLOT_WORLD.items()
}


def quat_yaw(o):
    """Yaw (z-rotation) from a quaternion message field."""
    return math.atan2(2.0 * (o.w * o.z + o.x * o.y),
                      1.0 - 2.0 * (o.y * o.y + o.z * o.z))


def refresh_device_pose(node, device, n=15, timeout=6.0):
    """Update DEVICES[device] from /vision/device_pose (odom). Collects up to `n`
    DISTINCT frames (dedup by stamp) and takes the per-axis MEDIAN -- robust to the
    single-tag jitter (and to a minority of ambiguity flips, since median ignores
    outliers). Returns True, or False if no pose arrives -- the caller must abort
    rather than fall back to the stale placeholder."""
    samples, seen = [], set()
    node._vision_pose = None
    t0 = time.time()
    while len(samples) < n and time.time() - t0 < timeout:
        ps = node._vision_pose
        if ps is not None:
            key = (ps.header.stamp.sec, ps.header.stamp.nanosec)
            if key not in seen:
                seen.add(key)
                p, o = ps.pose.position, ps.pose.orientation
                samples.append((p.x, p.y, p.z, quat_yaw(o)))
        time.sleep(0.02)
    if not samples:
        fail(node, proto.ErrorCode.TAG_NOT_DETECTED,
             f"[vision] no /vision/device_pose in {timeout}s -- is "
             f"wirebonder_vision_node.py running and the tag in FOV?")
        return False
    arr = np.array(samples)
    med = np.median(arr, axis=0)            # ponytail: yaw ~0 here, no wrap handling
    spread = arr[:, :3].std(axis=0) * 1000.0
    # Gate 1 (spread): clean single node republishes ONE pose -> spread ~0. Large spread
    # = a stale 2nd vision node poisoning /vision/device_pose, or an unstable solve.
    if spread.max() > CAPTURE_SPREAD_MAX_MM:
        fail(node, proto.ErrorCode.CAPTURE_SPREAD_HIGH,
             f"[vision] {device} REJECTED: spread {spread[0]:.0f}/{spread[1]:.0f}/"
             f"{spread[2]:.0f} mm > {CAPTURE_SPREAD_MAX_MM:.0f} -- stale 2nd vision node? "
             f"check 'ros2 topic info /vision/device_pose' (Publisher count should be 1)")
        return False
    # Gate 2 (plausibility): a constant-but-wrong solve passes gate 1 (spread ~0). AGV
    # parks near the same spot, so a good capture stays near the anchor; meters/90-deg
    # off = bad triangulation.
    off = np.abs(med[:3] - np.array(OLD_DEVICE_POSE[:3])) * 1000.0
    dyaw = math.degrees(abs(med[3] - OLD_DEVICE_POSE[3]))
    if off.max() > CAPTURE_POS_TOL_MM or dyaw > CAPTURE_YAW_TOL_DEG:
        fail(node, proto.ErrorCode.CAPTURE_IMPLAUSIBLE,
             f"[vision] {device} REJECTED: pose {off[0]:.0f}/{off[1]:.0f}/{off[2]:.0f} mm "
             f"dyaw={dyaw:.0f} deg off anchor (tol {CAPTURE_POS_TOL_MM:.0f} mm/"
             f"{CAPTURE_YAW_TOL_DEG:.0f} deg) -- bad triangulation")
        return False
    node.last_error = proto.ErrorCode.OK
    DEVICES[device] = tuple(med)
    node.get_logger().info(
        f"[vision] {device} <- median of {len(samples)} frames: x={med[0]:.3f} "
        f"y={med[1]:.3f} z={med[2]:.3f} yaw={med[3]:+.3f}  "
        f"(xyz spread {spread[0]:.0f}/{spread[1]:.0f}/{spread[2]:.0f} mm)")
    # Per-capture waypoint drift: how far THIS capture moves the anchored slot-D
    # seat off SLOT_WORLD. The tag sits on the LEFT of the device (model x -0.348)
    # ~0.7 m from slot D, so a capture yaw error is amplified ~12 mm/deg at D
    # (only ~2 mm/deg at A). If these numbers jump between runs, that's why the
    # D grasp misses/overshoots.
    dd = med - np.array(OLD_DEVICE_POSE)
    shift = (_to_odom(SLOT_LOCAL['D']['seat'], tuple(med))
             - np.array(SLOT_WORLD['D']['seat']))
    node.get_logger().info(
        f"[vision] {device} vs anchor: dxyz=({dd[0]*1000:+.0f},{dd[1]*1000:+.0f},"
        f"{dd[2]*1000:+.0f}) mm dyaw={math.degrees(dd[3]):+.2f} deg -> "
        f"slot-D seat drifts ({shift[0]*1000:+.0f},{shift[1]*1000:+.0f},"
        f"{shift[2]*1000:+.0f}) mm off SLOT_WORLD")
    return True


def slot_world(device, letter):
    """Magazine-centre world (odom) pose for a slot: device instance pose composed
    with the constant model-frame offset. Returns (xyz, device_yaw)."""
    ox, oy, oz = SLOT_OFFSET[letter]
    dx, dy, dz, dyaw = DEVICES[device]
    c, s = math.cos(dyaw), math.sin(dyaw)
    return (dx + c * ox - s * oy, dy + s * ox + c * oy, dz + oz), dyaw


def resolve(node, loc):
    """Box-centre xyz and grasp quaternion for a location, in base_link.
    Returns (center_np, quat_xyzw) or (None, None) if the world TF is missing."""
    if loc.kind == 'base':
        return np.asarray(loc.ref, dtype=float), quat_mul(quat_about_z(loc.yaw), DOWN)
    (wx, wy, wz), dyaw = slot_world(*loc.ref)
    quat = quat_mul(quat_about_z(dyaw + loc.yaw), DOWN)   # device yaw + slot yaw
    ps = node.transform_world_pose(wx, wy, wz, quat)
    if ps is None:
        fail(node, proto.ErrorCode.TF_UNAVAILABLE,
             f"[{loc.name}] world TF unavailable; park the AGV")
        return None, None
    c = np.array([ps.pose.position.x, ps.pose.position.y, ps.pose.position.z])
    o = ps.pose.orientation
    return c, (o.x, o.y, o.z, o.w)


def grasp_tcp_pose(node, center_xyz, quat):
    """PoseStamped whose TCP grasps/places a box CENTRED at center_xyz: above by
    GRASP_TCP_ABOVE and shifted off the tool axis by the gripper lateral hang so
    the box (not the flange) is centred. Returns pose or None."""
    nominal = np.asarray(center_xyz) + np.array([0.0, 0.0, GRASP_TCP_ABOVE])
    ref = node.compute_ik_ordered(pose_at(nominal, quat))
    if ref is None:
        return None
    jaw_x = node.gripper_x_in_base_fk(ref)
    if jaw_x is None:
        return None
    return pose_at(nominal - GRASP_LATERAL_M * jaw_x, quat)


def _offset(pose, dvec):
    """Same pose translated by a base_link 3-vector (orientation unchanged)."""
    p, o = pose.pose.position, pose.pose.orientation
    return pose_at([p.x + dvec[0], p.y + dvec[1], p.z + dvec[2]],
                   (o.x, o.y, o.z, o.w))


def slot_flange_seat(node, loc):
    """Flange (TCP) seat pose + unit insert direction in base_link for a front-
    loading slot. Tool stays DOWN; the TCP column is parked C_CLEARANCE in FRONT of
    part C (world y = device_y + C_FRONT_LOCAL_Y - C_CLEARANCE), and the long fixed
    jaw hangs the box GRASP_LATERAL_M behind it onto the shelf. x/z come from the
    magazine centre (TCP sits GRASP_TCP_ABOVE above the box centre).
    SLOT_YAW must orient the jaw along +insert so the box hangs toward the shelf.
    Returns (pose, dir_np) or (None, None) if the world TF is missing."""
    (sx, _, sz), dyaw = slot_world(*loc.ref)
    dy = DEVICES[loc.ref[0]][1]
    fy = dy + C_FRONT_LOCAL_Y - C_CLEARANCE          # TCP column world y (front of C)
    quat = quat_mul(quat_about_z(dyaw + loc.yaw), DOWN)
    nx, ny, nz = SLOT_NUDGE
    tx, ty, tz = sx + nx, fy + ny, sz + GRASP_TCP_ABOVE + nz
    node.get_logger().info(
        f"[{loc.name}] slot seat target (world): "
        f"x={tx:.3f} y={ty:.3f} z={tz:.3f}  nudge={SLOT_NUDGE}")
    ps = node.transform_world_pose(tx, ty, tz, quat)
    idir = node.transform_world_vector([-math.sin(dyaw), math.cos(dyaw), 0.0])  # device +y
    if ps is None or idir is None:
        fail(node, proto.ErrorCode.TF_UNAVAILABLE,
             f"[{loc.name}] world TF unavailable; park the AGV")
        return None, None
    o = ps.pose.orientation
    seat = pose_at([ps.pose.position.x, ps.pose.position.y, ps.pose.position.z],
                   (o.x, o.y, o.z, o.w))
    return seat, np.asarray(idir, dtype=float)


def base_hover_delta(loc):
    """Base-frame translation from the hub TCP to the hover above a base pocket.
    The hub is seeded from base_loc, so it shares the base-pocket jaw azimuth and
    the hub<->base move is a PURE translation: the lateral grasp offset is the same
    at both ends and cancels. Lets hub<->base run on a deterministic, collision-
    gated linear servo instead of a free RRT (slot side reorients, so it stays RRT)."""
    hover_center = np.asarray(loc.ref, dtype=float) + np.array(
        [0.0, 0.0, GRASP_TCP_ABOVE + HOVER_ABOVE])
    return hover_center - np.array(HUB_TCP)


def goto(node, pose, label):
    """Free joint-space RRT to a pose (branch nearest current), then execute."""
    if not node.move_to_pose_ref(pose, node.current_joints.tolist()):
        fail(node, proto.ErrorCode.PLAN_FAILED, f"[{label}] RRT move failed")
        return False
    return True


def slot_target(node, loc, key):
    """World point for a slot waypoint ('approach'/'seat' for front, 'box' for top),
    in base_link, from SLOT_WORLD. Only the position is used downstream
    (the servo holds the current orientation); the quat just makes the pose well-
    formed. Returns a PoseStamped or None (coords not measured yet / TF missing)."""
    device, letter = loc.ref
    wp = SLOT_WORLD.get(letter)
    if wp is None:
        node.get_logger().error(f"[{loc.name}] no measured coords; jog with tools/jog_tcp.py")
        return None
    dyaw = DEVICES[device][3]
    quat = quat_mul(quat_about_z(dyaw + loc.yaw), DOWN)
    # Compose the model-frame waypoint with the LIVE (vision-refreshed) device pose.
    wx, wy, wz = _to_odom(SLOT_LOCAL[letter][key], DEVICES[device])
    node.get_logger().info(
        f"[{loc.name}] {key} target (world): x={wx:.3f} y={wy:.3f} z={wz:.3f}  "
        f"(device {DEVICES[device][0]:.3f},{DEVICES[device][1]:.3f},"
        f"{DEVICES[device][2]:.3f},yaw={DEVICES[device][3]:+.3f})")
    ps = node.transform_world_pose(wx, wy, wz, quat)
    if ps is None:
        fail(node, proto.ErrorCode.TF_UNAVAILABLE,
             f"[{loc.name}] world TF unavailable; park the AGV")
    return ps


def servo_to(node, pose, label):
    """Straight collision-gated servo from the current TCP to pose's position,
    holding orientation. Valid only when the current azimuth already matches the
    target's (every wb1 slot and the hub share it). Returns True/False."""
    cur, _ = node.ik_model.fk_tcp(node.ik_model.pin_q(node.current_joints.tolist()))
    p = pose.pose.position
    return node.linear_servo(np.array([p.x, p.y, p.z]) - cur, label=label)


def slot_mode(loc):
    """'front'/'top'/None for a slot Location (None = coords not measured)."""
    wp = SLOT_WORLD.get(loc.ref[1])
    return wp.get('mode') if wp else None


def top_grasp_pose(node, loc):
    """Box-centred top-down grasp TCP for a top slot, DERIVED (no jog): the box
    centre (SLOT_WORLD[letter]['box'], vision-re-anchored) shifted GRASP_LATERAL_M
    toward the device FRONT so the long jaw hangs the box back over the TCP (the
    same convention as the front slots), lifted GRASP_TCP_ABOVE, held at the shared
    DOWN azimuth. The lateral direction is the KNOWN device +y, NOT grasp_tcp_pose's
    IK-derived jaw_x -- at slot B that IK goes contorted (nearest dist ~2.5 rad) and
    flips the offset BEHIND the box into wb_Cube_E. Returns a base_link pose or None."""
    letter = loc.ref[1]
    if SLOT_WORLD.get(letter, {}).get('box') is None:
        return None
    device = loc.ref[0]
    bx, by, bz = _to_odom(SLOT_LOCAL[letter]['box'], DEVICES[device])   # live box centre, world
    dyaw = DEVICES[device][3]
    fx, fy = -math.sin(dyaw), math.cos(dyaw)             # device +y (front -> back) in world
    tx, ty, tz = bx - GRASP_LATERAL_M * fx, by - GRASP_LATERAL_M * fy, bz + GRASP_TCP_ABOVE
    node.get_logger().info(
        f"[{loc.name}] top grasp TCP (world): x={tx:.3f} y={ty:.3f} z={tz:.3f}  "
        f"(box {bx:.3f},{by:.3f},{bz:.3f})")
    quat = quat_mul(quat_about_z(dyaw + loc.yaw), DOWN)
    return node.transform_world_pose(tx, ty, tz, quat)


def pick_slot_top(node, loc, to_hub):
    """Top-accessible slot (upper G slots, open top): hover above the box, descend
    to the box-centred grasp, grasp, ascend, then carry to the hub (phantom on).
    Same hover->descend->grasp->ascend shape as the base pick."""
    grasp = top_grasp_pose(node, loc)
    if grasp is None:
        return False
    node.control_gripper(GRIPPER_OPEN)
    if not servo_to(node, _offset(grasp, [0.0, 0.0, HOVER_ABOVE]), f"pick {loc.name} hover"):
        return False
    # Fixed descend to the box-centred grasp (grasp TCP = box centre + GRASP_TCP_ABOVE,
    # from SLOT_WORLD 'box'). NOT guarded: the box phantom false-triggers on the front
    # rail (Cube_G_L) ~54 mm early -- a VIRTUAL planner overlap, no physical hit -- so
    # the pads never reach the box. Depth is tuned via the box centre z.
    if not node.linear_servo([0.0, 0.0, -HOVER_ABOVE], label=f"pick {loc.name} descend"):
        return False
    node.control_gripper(GRIPPER_CLOSE)
    node.object_model, node.object_link = loc.model, loc.link
    if not node.attach_box():
        node.control_gripper(GRIPPER_OPEN)
        fail(node, proto.ErrorCode.ATTACH_FAILED,
             f"[pick {loc.name}] ATTACHLINK failed")
        return False
    time.sleep(0.5)
    if not node.linear_servo([0.0, 0.0, HOVER_ABOVE], label=f"pick {loc.name} ascend"):
        return False
    node.attach_box_collision()
    return node.go_to_hub() if to_hub else True


def place_slot_top(node, loc):
    """Top-accessible slot: carry to the hover (phantom on), descend to the box-
    centred grasp height, release, ascend, return to the hub. Mirrors the base drop."""
    grasp = top_grasp_pose(node, loc)
    if grasp is None:
        return False
    node.attach_box_collision()
    if not servo_to(node, _offset(grasp, [0.0, 0.0, HOVER_ABOVE]), f"place {loc.name} approach"):
        return False
    node.detach_box_collision()
    if not node.linear_servo([0.0, 0.0, -HOVER_ABOVE], label=f"place {loc.name} descend"):
        return False
    node.detach_box()
    node.control_gripper(GRIPPER_OPEN)
    time.sleep(0.5)
    if not node.linear_servo([0.0, 0.0, HOVER_ABOVE], label=f"place {loc.name} ascend"):
        return False
    return node.go_to_hub()


def pick_slot_front_j1(node, loc):
    """Front-load pick for a slot whose approach defeats the free RRT (seq-3 slot D).
    Swings ONLY J1 (STAGE_JOINTS[..][0]) to bring the arm in FRONT of the slot, then
    squares the jaw and STRAIGHT-servos to the jogged approach -> seat -> descend
    onto the box, grasps, and REVERSE-REPLAYS the whole outbound path back to the
    hub. No full staging config: the jog captured the approach POSITION, not a
    joint config. Always returns to the hub (this pick only feeds a base place)."""
    approach = slot_target(node, loc, 'approach')
    seat = slot_target(node, loc, 'seat')
    if approach is None or seat is None:
        return False
    node.control_gripper(GRIPPER_OPEN)
    stage = STAGE_JOINTS[loc.ref[1]]

    def forward():
        # J1 swing to face the slot (only J1 moves; the other joints hold). Recorded
        # so the reverse replay undoes it. The jaw-square + straight servo below take
        # the TCP to the jogged approach -- no full staging config is commanded.
        target = node.current_joints.copy()
        target[0] = stage[0]
        if not node.joint_move(target):
            return False
        # Square the jaw to the device insert axis: the J1 swing leaves the
        # tool yaw a few deg off, which cocks the grasp and swings the lateral
        # gripper hang off the box centre. err = signed angle from the current jaw
        # to the insert axis; J6 += err (NOT -=): the gripper is mounted FLIPPED on
        # Link6, so the tool axis is inverted and jaw azimuth tracks +J6. Residual
        # is logged -- if it grows instead of ~0, this sign is wrong.
        des = node.transform_world_vector(
            [-math.sin(DEVICES[loc.ref[0]][3]), math.cos(DEVICES[loc.ref[0]][3]), 0.0])
        cur = node.gripper_x_in_base_fk(node.current_joints.tolist())
        if des is not None and cur is not None:
            err = math.atan2(cur[0] * des[1] - cur[1] * des[0],
                             cur[0] * des[0] + cur[1] * des[1])
            if abs(err) > math.radians(1.0):
                target = node.current_joints.copy()
                target[5] += err
                if not node.joint_move(target):
                    return False
                cur2 = node.gripper_x_in_base_fk(node.current_joints.tolist())
                res = math.degrees(math.atan2(
                    cur2[0] * des[1] - cur2[1] * des[0],
                    cur2[0] * des[0] + cur2[1] * des[1]))
                node.get_logger().info(
                    f"[pick {loc.name}] jaw squared: was {math.degrees(err):+.1f} deg off, "
                    f"residual {res:+.1f} deg")
        # Seq-1 mirror: straight servos to the front hover, slide in level, then
        # plunge the open pads down AROUND the box (deeper than a place -- see
        # SLOT_PICK_DROP). The reverse replay lifts back out by the same amount.
        if not servo_to(node, approach, f"pick {loc.name} approach"):
            return False
        if not servo_to(node, seat, f"pick {loc.name} seat"):
            return False
        return node.linear_servo([0.0, 0.0, -SLOT_PICK_DROP],
                                 label=f"pick {loc.name} descend")

    ok, fwd = node.capture(forward)
    if not ok:
        return False
    node.control_gripper(GRIPPER_CLOSE)
    node.object_model, node.object_link = loc.model, loc.link
    if not node.attach_box():
        node.control_gripper(GRIPPER_OPEN)
        fail(node, proto.ErrorCode.ATTACH_FAILED,
             f"[pick {loc.name}] ATTACHLINK failed")
        return False
    time.sleep(0.5)
    node.attach_box_collision()
    # Retrace the outbound path in reverse: seat -> approach -> J1 back -> hub.
    return node.replay_reverse(fwd)


def pick(node, loc, to_hub=True):
    """Grasp the magazine at loc and return to the hub holding it. True/False.
    to_hub=False leaves the arm at the lifted-out pose (slot src only) so a
    slot->slot transfer goes straight to the dst without the hub detour."""
    if loc.kind == 'slot' and slot_mode(loc) == 'top':
        return pick_slot_top(node, loc, to_hub)
    if loc.kind == 'slot' and loc.ref[1] in STAGE_JOINTS:
        return pick_slot_front_j1(node, loc)
    center, quat = resolve(node, loc)
    if center is None:
        return False
    node.control_gripper(GRIPPER_OPEN)

    if loc.kind == 'base':
        # Tool-down: linear approach (hub shares the base azimuth), vertical descend.
        grasp = grasp_tcp_pose(node, center, quat)
        if grasp is None:
            fail(node, proto.ErrorCode.UNREACHABLE,
                 f"[pick {loc.name}] grasp IK unavailable")
            return False
        if not node.linear_servo(base_hover_delta(loc), label=f"pick {loc.name} approach"):
            return False
        if not node.linear_servo([0.0, 0.0, -HOVER_ABOVE + 0.01], label=f"pick {loc.name} descend"):
            return False
    else:
        # Front-loading slot: hover in front, slide in HORIZONTALLY to grasp.
        seat, idir = slot_flange_seat(node, loc)
        if seat is None:
            return False
        if not goto(node, _offset(seat, -SLOT_INSERT * idir), f"pick {loc.name} approach"):
            return False
        if not node.linear_servo(SLOT_INSERT * idir, label=f"pick {loc.name} insert"):
            return False

    node.control_gripper(GRIPPER_CLOSE)
    node.object_model, node.object_link = loc.model, loc.link
    if not node.attach_box():
        node.control_gripper(GRIPPER_OPEN)
        fail(node, proto.ErrorCode.ATTACH_FAILED,
             f"[pick {loc.name}] ATTACHLINK failed")
        return False
    time.sleep(0.5)

    if loc.kind == 'base':
        # Vertical pull-out (box-safe), phantom off; linear retract to the hub.
        if not node.linear_servo([0.0, 0.0, HOVER_ABOVE], label=f"pick {loc.name} ascend"):
            return False
        return node.linear_servo(-base_hover_delta(loc), label=f"pick {loc.name} retract")
    # Slot: horizontal pull-out clears the device front, then RRT to the hub with
    # the carried-box phantom on. to_hub=False (slot->slot) stays at the pulled-out
    # pose so place() RRTs straight to the dst, skipping the hub detour.
    if not node.linear_servo(-SLOT_INSERT * idir, label=f"pick {loc.name} retract"):
        return False
    node.attach_box_collision()
    return node.go_to_hub() if to_hub else True


def place(node, loc):
    """Carry from the hub and release the magazine at loc, then return to the hub.
    Assumes the carried-box collision model is ON at entry. True/False."""
    if loc.kind == 'slot' and slot_mode(loc) == 'top':
        return place_slot_top(node, loc)
    if loc.kind == 'base':
        center, quat = resolve(node, loc)
        if center is None:
            return False
        # Guarded place (sim analog of the real-robot torque touch-off, see
        # place_command_guide.md). Approach to the hover with the phantom OFF (the
        # IK gate rejects it near the surface -- "ok=94, collision-free=0"), then
        # descend with the phantom ON and STOP the instant the box meets the pocket
        # surface. Each pocket seats at ITS true height and any carried-box offset
        # is absorbed -- no single fixed drop that over-descends one, short of another.
        # Phantom OFF before the IK gate: arriving from the pick the box phantom is
        # ON, but the place-grasp pose rests the box on the pocket, so with it
        # attached the IK rejects every solution ("ok=83, collision-free=0").
        node.detach_box_collision()
        target = grasp_tcp_pose(node, center, quat)
        if target is None:
            fail(node, proto.ErrorCode.UNREACHABLE,
                 f"[place {loc.name}] place IK unavailable")
            return False
        if not node.linear_servo(base_hover_delta(loc), label=f"place {loc.name} approach"):
            return False
        node.attach_box_collision()                 # box vs surface = the contact sensor
        drop = node.guarded_descend(HOVER_ABOVE + BASE_PLACE_OVERTRAVEL,
                                    label=f"place {loc.name} descend")
        node.detach_box_collision()
        node.detach_box()
        node.control_gripper(GRIPPER_OPEN)
        time.sleep(0.5)
        if drop > 1e-3 and not node.linear_servo([0.0, 0.0, drop], label=f"place {loc.name} ascend"):
            return False
        return node.linear_servo(-base_hover_delta(loc), label=f"place {loc.name} retract")

    # Front-loading slot (part C overhangs, so no top drop). Every leg is a straight
    # collision-gated servo to a MEASURED world point, so the tight front hover
    # never needs a free RRT. ponytail: pure translation, valid only because every
    # wb1 slot and the hub share the azimuth (DOWN, yaw pi).
    approach = slot_target(node, loc, 'approach')
    seat = slot_target(node, loc, 'seat')
    if approach is None or seat is None:
        return False

    # Record the forward run (hub -> front -> under part C -> down). Entry is at
    # the hub (front-load is only ever reached via the hub), so the reverse of
    # this proven path lands EXACTLY on hub_q -- deterministic, unlike go_to_hub's
    # free RRT which re-plans a fresh (often wild) path every cycle.
    def forward():
        node.attach_box_collision()
        # 1. straight servo from the hub to the front of the slot (phantom on). The
        #    endpoint is reachable+collision-free (verified by jog) but CBiRRT goal
        #    IK samples colliding branches; the servo seeds from the current joints,
        #    so it stays on the reachable branch like the jog does.
        if not servo_to(node, approach, f"place {loc.name} approach"):
            return False
        # 2. slide in under part C to the magazine spot; the box now contacts the
        #    shelf (intended), so drop the phantom for the seat servo.
        node.detach_box_collision()
        if not servo_to(node, seat, f"place {loc.name} insert"):
            return False
        # 3. set the box down on the shelf.
        return node.linear_servo([0.0, 0.0, -SLOT_PLACE_DROP],
                                 label=f"place {loc.name} descend")

    ok, fwd = node.capture(forward)
    if not ok:
        return False
    node.detach_box()
    node.control_gripper(GRIPPER_OPEN)
    time.sleep(0.5)
    # 4. retrace the proven forward path in reverse: up off the box, back out under
    #    part C, and home to the hub -- the empty gripper follows the same line.
    return node.replay_reverse(fwd)


def preflight_place(node, dst, direct):
    """Dry-run the hub->slot place legs (approach, then seat) WITHOUT moving the arm,
    so a sequence that can't reach the slot FAILS BEFORE the box is picked (instead of
    ending stranded holding it). Uses the SAME Cartesian servo the place uses
    (cbirrt.linear_path -- it only computes the path), from the CURRENT (hub) config,
    with the carried-box phantom on for the approach as the real place carries it.
    Only meaningful when the place starts from the hub: skipped for direct slot->slot
    and non-front dst. Returns True to proceed, False to refuse."""
    if direct or dst.kind != 'slot' or slot_mode(dst) != 'front':
        return True
    approach = slot_target(node, dst, 'approach')
    seat = slot_target(node, dst, 'seat')
    if approach is None or seat is None:
        return False

    def dry(from_q, pose):
        cur, _ = node.ik_model.fk_tcp(node.ik_model.pin_q(from_q))
        p = pose.pose.position
        delta = np.array([p.x, p.y, p.z]) - cur
        want = float(np.linalg.norm(delta))
        path, reached, reason = node.cbirrt.linear_path(
            from_q, delta, node.is_state_valid, node.joint_limits)
        return (reached >= want - 1e-3), (path[-1] if path else from_q), reached, want, reason

    q0 = node.current_joints.tolist()
    node.attach_box_collision()                     # approach carries the box
    ok_a, q1, ra, wa, rea = dry(q0, approach)
    node.detach_box_collision()                     # seat runs after the box is released
    if not ok_a:
        fail(node, proto.ErrorCode.UNREACHABLE,
             f"[preflight] {dst.name}: hub->approach unreachable "
             f"({ra*1000:.0f}/{wa*1000:.0f} mm -> {rea}); NOT picking the box")
        return False
    ok_s, _, rs, ws, res = dry(q1, seat)
    if not ok_s:
        fail(node, proto.ErrorCode.UNREACHABLE,
             f"[preflight] {dst.name}: approach->seat unreachable "
             f"({rs*1000:.0f}/{ws*1000:.0f} mm -> {res}); NOT picking the box")
        return False
    node.get_logger().info(f"[preflight] {dst.name}: hub->slot reachable; proceeding")
    return True


def transfer(node, src, dst):
    """Move a magazine from src to dst (Locations). True/False.

    slot->slot goes DIRECT (pick lifts out, then RRT straight to dst, phantom on);
    transfers touching a base pocket route via the hub, since the hub shares the
    base azimuth and the base spokes run on linear servos."""
    node.get_logger().info(f"[transfer] {src.name} -> {dst.name}")
    # A transfer always starts with an EMPTY gripper, but an aborted previous
    # cycle can leak the carried-box phantom ON (nothing detaches it on failure)
    # -- then every pick approach false-collides ('carried_box' vs the device).
    node.detach_box_collision()
    # Place the wirebonder body at the involved device so the RRTs avoid it.
    for loc in (src, dst):
        if loc.kind == 'slot':
            node.update_wirebonder_collision(DEVICES[loc.ref[0]])
            break
    direct = src.kind == 'slot' and dst.kind == 'slot'
    # Preflight the hub->slot place BEFORE picking, so an unreachable slot fails now
    # instead of stranding the box in the gripper.
    if not preflight_place(node, dst, direct):
        return False
    if not pick(node, src, to_hub=not direct):
        return False
    return place(node, dst)


def wait_for_key():
    """Block for one keypress; return the char ('1'..'3' select a case, q/Esc quit)."""
    import sys, termios, tty
    fd = sys.stdin.fileno()
    old = termios.tcgetattr(fd)
    try:
        tty.setraw(fd)
        ch = sys.stdin.read(1)
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, old)
    return ch


def quat_about_y(theta):
    """Quaternion (x, y, z, w) for a rotation theta (rad) about the base-frame Y."""
    return (0.0, math.sin(theta / 2.0), 0.0, math.cos(theta / 2.0))


def capture_device(node):
    """Two-view (motion-stereo) capture. From the hub, move to view A (CAPTURE_FLANGE
    with a CAPTURE_PITCH tilt), then to view B (offset by CAPTURE_BASELINE), pinging
    /vision/capture at each so the vision node triangulates the tag across the two
    camera POSITIONS -- that recovers depth a single view can't. Then refresh EVERY
    device's pose from the triangulated /vision/device_pose and return to the hub.
    DECOUPLED from the hub (which stays transfer-safe). Planned moves (goto) are used
    because the tilt REORIENTS the tool. Run once before the menu (and on 'c').
    Returns True -- transfers are gated on it."""
    if not node.go_to_hub():                                  # start from the hub
        return False
    quat = CAPTURE_QUAT if CAPTURE_QUAT is not None else quat_mul(
        quat_about_y(math.radians(CAPTURE_PITCH_DEG)),
        quat_mul(quat_about_z(PICK_YAW), DOWN))
    node._cap_pub.publish(Int32(data=0))                     # reset the node buffer

    def forward():
        # View A: planned move to the jogged capture pose (needs the jogged orientation).
        if not goto(node, pose_at(CAPTURE_FLANGE, quat), "capture view A"):
            return False
        time.sleep(2.0)  # settle: the arm keeps moving ~2 s after "finished" (see log)
        node._cap_pub.publish(Int32(data=1))                 # grab view A
        time.sleep(0.3)
        # View B: direct MoveJ to the jogged config (shortest joint path, smooth,
        # singularity-free) if given, else the pure-translation baseline servo. Only
        # the endpoint pose matters for triangulation, so the straight path is fine.
        if CAPTURE_B_JOINTS is not None:
            # Direct MoveJ (straight joint interpolation) instead of RRT: shortest
            # joint path, no re-planned swing. Path recorded for the reverse replay.
            if not node.joint_move(CAPTURE_B_JOINTS):
                return False
        elif not node.linear_servo(np.array(CAPTURE_BASELINE, dtype=float), label="capture view B"):
            return False
        time.sleep(2.0)  # settle (as view A) so the tag is in frame at grab time
        node._cap_pub.publish(Int32(data=1))                 # grab view B
        time.sleep(0.3)
        return True

    # Record the outbound hub->A->B path; the return is a REVERSE REPLAY of it, not a
    # fresh go_to_hub RRT -- that RRT is unaware of the base box (not in the planning
    # scene) and swings through it. Retracing the proven outbound path can't hit it.
    ok_fwd, fwd = node.capture(forward)
    if not ok_fwd:
        return False
    ok = all(refresh_device_pose(node, d) for d in ALL_DEVICES)
    node.replay_reverse(fwd)                                 # B -> A -> hub (box-safe path)
    # Snap to the EXACT hub_q: base pick's approach is a pure translation FROM the hub
    # (base_hover_delta), so the arm must start at the exact hub or the pick lands
    # offset. replay_reverse only lands NEAR hub; this is a tiny, box-safe move.
    node.go_to_hub()
    return ok


def main(args=None):
    rclpy.init(args=args)
    node = HubPickPlace()
    node.setup_planner()
    # Register the wirebonder BODY so the free RRTs (front-load approach, go_to_hub)
    # route around the device instead of sweeping through it. Per-part STLs keep the
    # slot recesses open, so front-load insert stays valid. Placed per transfer.
    WB_STL_DIR = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(
        os.path.abspath(__file__)))), 'blender', 'wirebonder', 'collision')
    node.add_wirebonder_meshes(WB_STL_DIR)

    # --no-vision: skip the vision node and use the hardcoded DEVICES placeholder
    # (precise parking required). Default is vision-driven.
    use_vision = '--no-vision' not in sys.argv
    if use_vision:
        # Vision layer: the device pose arrives on /vision/device_pose (odom) from
        # wirebonder_vision_node.py. Cache the latest; refresh_device_pose() reads it.
        node._vision_pose = None
        node.create_subscription(PoseStamped, '/vision/device_pose',
                                 lambda m: setattr(node, '_vision_pose', m), 10)
        # Drives the two-view capture: data=0 resets, data=1 grabs a view.
        node._cap_pub = node.create_publisher(Int32, '/vision/capture', 10)

    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2)  # wait for joint states

    # Hub seeded from the base-pocket family (always available in base_link).
    base = base_loc()
    ref = pose_at([base.ref[0], base.ref[1], base.ref[2] + GRASP_TCP_ABOVE],
                  quat_mul(quat_about_z(base.yaw), DOWN))
    if not node.init_hub(ref, HUB_TCP, GRASP_LATERAL_M):
        fail(node, proto.ErrorCode.INIT_FAILED,
             "Hub bring-up failed; adjust HUB_TCP and retry")
        node.destroy_node(); rclpy.shutdown(); return
    if not node.go_to_hub():   # rest at the hub (transfer standby)
        fail(node, proto.ErrorCode.INIT_FAILED,
             "Could not reach the hub from the spawn pose")
        node.destroy_node(); rclpy.shutdown(); return

    # Capture the device pose ONCE up front (vision mode) so any of 1/2/3 can run.
    captured = True
    if use_vision:
        print(" Capturing device pose(s) at the capture viewpoint ...")
        captured = capture_device(node)
        if not captured:
            print(" Capture FAILED -- press 'c' to retry (vision node up + tag in FOV).")

    print("\n" + "=" * 60)
    print(f" Device pose source: {'VISION (/vision/device_pose)' if use_vision else 'HARDCODED DEVICES (--no-vision)'}")
    print(" Wirebonder transfers ready (park the AGV facing the device):")
    for k, (s, d) in SEQUENCES.items():
        print(f"   {k} : {s.name} -> {d.name}")
    print(" Press 1/2/3 to run a transfer." +
          ("  'c' = re-capture device pose." if use_vision else "") + " (q / Esc to quit)")
    print("=" * 60)

    try:
        while rclpy.ok():
            ch = wait_for_key()
            if ch in ('q', '\x1b', '\x03'):
                break
            if use_vision and ch == 'c':   # re-capture (e.g. after the AGV re-parks)
                captured = capture_device(node)
                print(f"\n>>> {'re-captured device pose' if captured else 'capture FAILED'}.")
                continue
            if ch not in SEQUENCES:
                continue
            # The device pose was captured up front; transfers reuse it. Refuse if the
            # capture failed -- never run on the stale placeholder.
            if use_vision and not captured:
                print("\n>>> no device pose; press 'c' to capture first.")
                continue
            src, dst = SEQUENCES[ch]
            ok = transfer(node, src, dst)
            print(f"\n>>> {src.name}->{dst.name} "
                  f"{'DONE (arm at hub)' if ok else 'FAILED'}. Pick 1/2/3 again.")
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
