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
    python3 wirebonder_vision_node.py                 # terminal A (system cv2)
    ~/dobot_ws/.venv/bin/python3 wirebonder_pick_place.py   # terminal B (.venv)

    # --no-vision: skip the vision node; use the hardcoded DEVICES placeholder
    # (requires precise parking). For arm-only testing without cv2/vision:
    ~/dobot_ws/.venv/bin/python3 wirebonder_pick_place.py --no-vision
"""

import math
import sys
import time
import threading
from collections import namedtuple

import numpy as np
import rclpy
from rclpy.executors import MultiThreadedExecutor
from geometry_msgs.msg import PoseStamped

from cr7_pnp import (
    HubPickPlace, pose_at, quat_mul, quat_about_z,
    DOWN, GRIPPER_OPEN, GRIPPER_CLOSE, GRASP_TCP_ABOVE, GRASP_LATERAL_M,
    POCKET_X, POCKET_Y, POCKET_SURFACE_Z, BOX_SIZE,
)

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
# NOTE: these are PLACEHOLDERS -- refresh_device_pose() overwrites the entry from
# the AprilTag vision node (/vision/device_pose) before each transfer, so the real
# device location need not match this and the AGV need not park precisely.
DEVICES = {
    'wb1': (2.35, 0.5, 0.0, 0.0),
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
    # Default = the +Y outer pocket (POCKET_Y[0]), where box_l2c is staged in cr.world.
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
    '3': (slot_loc('wb1', 'D', model='box_l2b'), base_loc()),            # box_l2b slotD -> base
}

# Every device any transfer touches -- captured ONCE up front so any of 1/2/3 can
# run (on arrival the arm may do 2 or 3 without 1 first).
ALL_DEVICES = {loc.ref[0] for s, d in SEQUENCES.values()
               for loc in (s, d) if loc.kind == 'slot'}

# --- geometry / hub (TCP metres; TUNE IN SIM) ---------------------------------
HOVER_ABOVE = 0.12        # TCP hover height above the grasp/place point
# Hub = standby / transfer OBJECT hover. PROVEN transfer-safe: the slot approach
# servos run straight from here without collision (base->slotA completes under
# --no-vision). DECOUPLED from the capture pose so it never moves for the camera.
HUB_TCP = (0.33, 0.0, 0.32)

# Capture pose (absolute FLANGE, base_link): close to the tag for an accurate read.
# capture_device moves here from the hub, captures, then returns. DECOUPLED from the
# hub so tuning it never breaks the transfer. TUNE x/y/z for framing (image viewer).
CAPTURE_FLANGE = (0.373, 0.05, 0.148)
# Tilt the tool by this pitch (deg) at the capture pose so the camera views the tag
# OFF fronto-parallel -- this separates the two planar-pose solutions and kills the
# ambiguity flip (fronto-parallel is the worst case). TUNE: increase if it still
# flips; flip the sign (or ask to switch axis) if the tag leaves the frame.
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
#   mode='front' (lower H slots, part C overhangs): slide in HORIZONTALLY.
#       approach = TCP in front of the slot;  seat = TCP over the magazine spot.
#   mode='top'   (upper G slots, open top): hover above + descend VERTICALLY.
#       hover = TCP above the box; descend by SLOT_TOPDOWN_DROP to grasp/place.
SLOT_WORLD = {
    'A': {'mode': 'front', 'approach': (1.996, 0.17, 1.032), 'seat': (1.996, 0.28, 1.032)},
    'B': {'mode': 'top', 'hover': (2.006, 0.324, 1.440)},
    'C': {'mode': 'top', 'hover': (2.705, 0.324, 1.440)},
    'D': None,
}
SLOT_PLACE_DROP = 0.03      # front-load: descend from the seat onto the shelf (TUNE)
SLOT_TOPDOWN_DROP = 0.10    # top-load: descend from the hover to grasp/place (TUNE)

# --- vision re-anchoring ------------------------------------------------------
# SLOT_WORLD holds the hand-jogged TCP waypoints as ABSOLUTE odom coords, captured
# with the device at OLD_DEVICE_POSE. Re-anchor them ONCE into the device MODEL
# frame, so at runtime they compose with the LIVE device pose (DEVICES[device],
# refreshed from /vision/device_pose) -- the AGV may park anywhere and the device
# may sit at any yaw. ponytail: device assumed upright, so only (x,y,z,yaw) matters.
OLD_DEVICE_POSE = (2.35, 0.5, 0.0, 0.0)


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
        node.get_logger().error(
            f"[vision] no /vision/device_pose in {timeout}s -- is "
            f"wirebonder_vision_node.py running and the tag in FOV?")
        return False
    arr = np.array(samples)
    med = np.median(arr, axis=0)            # ponytail: yaw ~0 here, no wrap handling
    spread = arr[:, :3].std(axis=0) * 1000.0
    DEVICES[device] = tuple(med)
    node.get_logger().info(
        f"[vision] {device} <- median of {len(samples)} frames: x={med[0]:.3f} "
        f"y={med[1]:.3f} z={med[2]:.3f} yaw={med[3]:+.3f}  "
        f"(xyz spread {spread[0]:.0f}/{spread[1]:.0f}/{spread[2]:.0f} mm)")
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
        node.get_logger().error(f"[{loc.name}] world TF unavailable; park the AGV")
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
        node.get_logger().error(f"[{loc.name}] world TF unavailable; park the AGV")
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
        node.get_logger().error(f"[{label}] RRT move failed")
        return False
    return True


def slot_target(node, loc, key):
    """TCP pose in base_link for a slot waypoint ('approach'/'seat'/'hover'), from
    the measured world coords (SLOT_WORLD). Only the position is used downstream
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
        node.get_logger().error(f"[{loc.name}] world TF unavailable; park the AGV")
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


def pick_slot_top(node, loc, to_hub):
    """Top-accessible slot (upper G slots, open top): hover above the box with the
    empty gripper, descend, grasp, ascend, then carry to the hub (phantom on)."""
    hover = slot_target(node, loc, 'hover')
    if hover is None:
        return False
    node.control_gripper(GRIPPER_OPEN)
    if not servo_to(node, hover, f"pick {loc.name} hover"):
        return False
    if not node.linear_servo([0.0, 0.0, -SLOT_TOPDOWN_DROP], label=f"pick {loc.name} descend"):
        return False
    node.control_gripper(GRIPPER_CLOSE)
    node.object_model, node.object_link = loc.model, loc.link
    if not node.attach_box():
        node.control_gripper(GRIPPER_OPEN)
        node.get_logger().error(f"[pick {loc.name}] ATTACHLINK failed")
        return False
    time.sleep(0.5)
    if not node.linear_servo([0.0, 0.0, SLOT_TOPDOWN_DROP], label=f"pick {loc.name} ascend"):
        return False
    node.attach_box_collision()
    return node.go_to_hub() if to_hub else True


def place_slot_top(node, loc):
    """Top-accessible slot: carry to the hover (phantom on), descend, release,
    ascend, return to the hub. Mirrors the base-pocket drop."""
    hover = slot_target(node, loc, 'hover')
    if hover is None:
        return False
    node.attach_box_collision()
    if not servo_to(node, hover, f"place {loc.name} approach"):
        return False
    node.detach_box_collision()
    if not node.linear_servo([0.0, 0.0, -SLOT_TOPDOWN_DROP], label=f"place {loc.name} descend"):
        return False
    node.detach_box()
    node.control_gripper(GRIPPER_OPEN)
    time.sleep(0.5)
    if not node.linear_servo([0.0, 0.0, SLOT_TOPDOWN_DROP], label=f"place {loc.name} ascend"):
        return False
    return node.go_to_hub()


def pick(node, loc, to_hub=True):
    """Grasp the magazine at loc and return to the hub holding it. True/False.
    to_hub=False leaves the arm at the lifted-out pose (slot src only) so a
    slot->slot transfer goes straight to the dst without the hub detour."""
    if loc.kind == 'slot' and slot_mode(loc) == 'top':
        return pick_slot_top(node, loc, to_hub)
    center, quat = resolve(node, loc)
    if center is None:
        return False
    node.control_gripper(GRIPPER_OPEN)

    if loc.kind == 'base':
        # Tool-down: linear approach (hub shares the base azimuth), vertical descend.
        grasp = grasp_tcp_pose(node, center, quat)
        if grasp is None:
            node.get_logger().error(f"[pick {loc.name}] grasp IK unavailable")
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
        node.get_logger().error(f"[pick {loc.name}] ATTACHLINK failed")
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
        # Tool-down: linear approach + vertical drop. Phantom off (line is box-safe).
        target = grasp_tcp_pose(node, center, quat)
        if target is None:
            node.get_logger().error(f"[place {loc.name}] place IK unavailable")
            return False
        node.detach_box_collision()
        if not node.linear_servo(base_hover_delta(loc), label=f"place {loc.name} approach"):
            return False
        if not node.linear_servo([0.0, 0.0, -HOVER_ABOVE], label=f"place {loc.name} descend"):
            return False
        node.detach_box()
        node.control_gripper(GRIPPER_OPEN)
        time.sleep(0.5)
        if not node.linear_servo([0.0, 0.0, HOVER_ABOVE], label=f"place {loc.name} ascend"):
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


def transfer(node, src, dst):
    """Move a magazine from src to dst (Locations). True/False.

    slot->slot goes DIRECT (pick lifts out, then RRT straight to dst, phantom on);
    transfers touching a base pocket route via the hub, since the hub shares the
    base azimuth and the base spokes run on linear servos."""
    node.get_logger().info(f"[transfer] {src.name} -> {dst.name}")
    direct = src.kind == 'slot' and dst.kind == 'slot'
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
    """From the hub, move to the capture pose -- CAPTURE_FLANGE with a CAPTURE_PITCH
    tilt so the camera views the tag OFF fronto-parallel (breaks the planar-tag
    ambiguity) -- refresh EVERY device's pose from vision, then return to the hub.
    DECOUPLED from the hub (which stays transfer-safe). A planned move (goto) is used
    because the tilt REORIENTS the tool -- a straight servo can't change orientation.
    Run once before the menu (and on 'c'). Returns True -- transfers are gated on it."""
    if not node.go_to_hub():                                  # start from the hub
        return False
    quat = quat_mul(quat_about_y(math.radians(CAPTURE_PITCH_DEG)),
                    quat_mul(quat_about_z(PICK_YAW), DOWN))
    cap = pose_at(CAPTURE_FLANGE, quat)
    if not goto(node, cap, "capture viewpoint"):
        return False
    time.sleep(0.5)  # settle so the eye-in-hand image + TF are consistent
    ok = all(refresh_device_pose(node, d) for d in ALL_DEVICES)
    node.go_to_hub()                                          # back home to hub_q
    return ok


def main(args=None):
    rclpy.init(args=args)
    node = HubPickPlace()
    node.setup_planner()

    # --no-vision: skip the vision node and use the hardcoded DEVICES placeholder
    # (precise parking required). Default is vision-driven.
    use_vision = '--no-vision' not in sys.argv
    if use_vision:
        # Vision layer: the device pose arrives on /vision/device_pose (odom) from
        # wirebonder_vision_node.py. Cache the latest; refresh_device_pose() reads it.
        node._vision_pose = None
        node.create_subscription(PoseStamped, '/vision/device_pose',
                                 lambda m: setattr(node, '_vision_pose', m), 10)

    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2)  # wait for joint states

    # Hub seeded from the base-pocket family (always available in base_link).
    base = base_loc()
    ref = pose_at([base.ref[0], base.ref[1], base.ref[2] + GRASP_TCP_ABOVE],
                  quat_mul(quat_about_z(base.yaw), DOWN))
    if not node.init_hub(ref, HUB_TCP, GRASP_LATERAL_M):
        node.get_logger().error("Hub bring-up failed; adjust HUB_TCP and retry")
        node.destroy_node(); rclpy.shutdown(); return
    if not node.go_to_hub():   # rest at the hub (transfer standby)
        node.get_logger().error("Could not reach the hub from the spawn pose")
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
