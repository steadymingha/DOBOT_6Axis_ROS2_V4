"""Hub-and-spoke shelf->base sequence (pick four tier-1 shelf boxes into pockets).

Every motion routes through a tool-down HUB waypoint (hub<->shelf, hub<->pocket)
so the arm never crosses shelf->pocket directly -- that direct carry stalls when
the shelf grasp and the pocket place fall in different elbow/wrist families.

Return is guaranteed two ways: spokes are pre-flighted under a box-attached
collision model with NO motion (infeasible -> abort before moving), and the
forward joint waypoints are recorded and replayed in reverse to come back to the
hub (a path just executed is executable backwards, so the return cannot fail for
"no IK").

This file is the SEQUENCE + main only; the reusable motion node and primitives
live in cr7_pnp. The SPACE trigger is a development stand-in for the AMR/MCS
state signal -- the sequence functions (pick_place_one_box, ...) are
trigger-agnostic so a BT/FSM mission node can call them directly later.

Box positions are ArUco-relative: the tier tag is read once up front (the LOCATE
step, locate_shelf) from /vision/shelf_pose (detection + shelf tag config in
vision/tag_vision.py, run by vision/tag_vision_node.py) into node.shelf_pose =
(x, y, yaw), and every box centre / collision board composes with it -- so a
re-parked AGV or moved shelf needs no code edit. The capture viewpoint is AUTO-AIMED
at the tag (no hand-jog; SHELF_CAPTURE_JOINTS is only a fallback). --no-vision falls
back to the SHELF_WORLD_POSE spawn default. Same capture pattern as
wirebonder_pick_place.

Run (sim already up). SYSTEM python, not the .venv -- the venv's numpy 2.x
segfaults the ROS pinocchio build:
    source /opt/ros/humble/setup.bash
    source ~/dobot_ws/install/setup.bash
    cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
    python3 vision/tag_vision_node.py            # terminal A (system cv2)
    /usr/bin/python3 sequences/shelf_pick_place.py      # terminal B
    # add --no-vision to skip the ArUco read (uses the SHELF_WORLD_POSE default)
"""

import math
import os
import sys
import time
import threading

import numpy as np
import rclpy
from rclpy.duration import Duration
from rclpy.executors import MultiThreadedExecutor
from geometry_msgs.msg import PoseStamped

# sequences/ is one level below the package root; add the root so cr7_pnp imports
# when this file is run standalone (python3 sequences/shelf_pick_place.py).
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from cr7_pnp import (  # noqa: E402
    HubPickPlace, pose_at, quat_mul, quat_about_z, wait_for_spacebar,
    DOWN, GRASP_YAW_OFFSET, GRIPPER_YAW_TWIST,
    GRIPPER_OPEN, GRIPPER_CLOSE,
    INSERT_TCP_ABOVE, GRASP_TCP_ABOVE, PREGRASP_BACK,
    POCKET_X, POCKET_SURFACE_Z, POCKET_HOVER,
    SHELF_BOX_LINK, BOX_SIZE, GRASP_LATERAL_M,
    SHELF_BOX_XS, SHELF_WORLD_POSE, shelf_box_center, shelf_box_model,
    shelf_tag_world, quat_to_R,
)
from vision import pocket_vision as pockets  # noqa: E402

# Tool-down HUB waypoint (carried-object TCP, tool pointing DOWN). Solving IK
# toward the pocket branch keeps the hub in the SAME elbow/wrist family as both
# spokes, so hub<->shelf and hub<->pocket stay short and never flip the elbow.
# TUNE IN SIM: raise z (or pull x toward the pocket ~0.37) if the hub IK fails or
# the carried box collides at the hub.
HUB_TCP = (0.33, 0.0, 0.32)
# Hub joints (J1..J6 rad): -0.45320, 0.34299, -1.88375, 0.0, -1.52609, -0.43032 (deg: -26, +20, -108, 0, -87, -25),

# Pocket centres in base_link y, in the order boxes are placed (-y to +y).
# Shared with the occupancy check so indices mean the same everywhere.
PLACE_ORDER_Y = list(pockets.POCKET_ORDER_Y)

# Tier-1 shelf boxes, derived from the SAME layout constants the collision stock
# phantoms use (SHELF_TIER_TOPS / SHELF_BOX_XS in cr7_pnp/geometry.py), so tier
# height and box names live in ONE place and always match cr.world. Box i goes to
# pocket PLACE_ORDER_Y[N-1-i] (fill from the high-y end; reversed 2026-07-15).
# The box WORLD centres compose the model-frame layout
# with the LIVE shelf pose (ArUco read, node.shelf_pose) via shelf_box() below --
# the same seam update_shelf_collision uses -- so a re-parked AGV / moved shelf
# needs no code edit.
TIER = 1
N_BOXES = len(SHELF_BOX_XS)


def shelf_pose_of(node):
    """Live tier-ArUco shelf pose (x, y, yaw) if captured, else the cr.world spawn
    default (--no-vision, or before a successful capture)."""
    return getattr(node, 'shelf_pose', None) or SHELF_WORLD_POSE


def shelf_box(node, idx):
    """(world_xyz, Gazebo model name) for shelf box `idx`, composed with the live
    shelf pose. Replaces the old static SHELF_BOXES table."""
    return (shelf_box_center(TIER, idx, shelf_pose_of(node)),
            shelf_box_model(TIER, idx))

# The AGV stays parked for this sequence (spawn-and-pick), so a placed box just
# settles into its pocket by gravity. Set True if the AGV will DRIVE with boxes
# aboard -- a loose box slides in the pocket (friction can't beat planar_move).
MAGAZINE_ATTACH = False

# (The fixed release height PLACE_TCP_ABOVE_HUB was retired 2026-07-15: the
# place is GUARDED now -- the box phantom is the contact sensor and the box
# seats at the pocket's true height, wirebonder place_base style.)

# Fixed-jaw azimuth at the hub (standby) and the pocket (place), as a yaw about
# the tool axis (tool stays pointing down). math.pi flips the fixed jaw 180 deg
# from the bare-DOWN direction; flip to -math.pi if the wrist twists the wrong
# way in sim. The lateral place offset follows this automatically (derived from
# the place orientation via FK), so the box stays centred either way.
PLACE_YAW = math.pi


def pocket_center_xyz(pocket_y):
    """Hover TCP directly above a pocket centre (no jaw offset)."""
    return np.array([POCKET_X, pocket_y, POCKET_SURFACE_Z + POCKET_HOVER])


# IK-seed reference height, DECOUPLED from POCKET_HOVER: when the hover rose
# to 0.38 (clear boxes in neighbouring pockets during the carry) the centred
# seed pose at that height stopped yielding collision-free IK in the full
# world (hub bring-up failed, "collision-free=0"). The seed only defines the
# pocket elbow/wrist FAMILY, so it stays at the proven 0.18-era height.
POCKET_REF_HOVER = 0.18


def pocket_ref_xyz(pocket_y):
    """Family-seed TCP above a pocket centre at the proven reference height."""
    return np.array([POCKET_X, pocket_y, POCKET_SURFACE_Z + POCKET_REF_HOVER])


def pocket_hover_xyz(pocket_y, place_jaw_x):
    """Hover TCP so the carried box -- which hangs GRASP_LATERAL_M off the tool
    axis along the jaw direction -- lands CENTRED over the pocket."""
    return pocket_center_xyz(pocket_y) - GRASP_LATERAL_M * np.asarray(place_jaw_x)


def place_quat():
    """Place orientation: down + PLACE_YAW about base z (constant)."""
    return quat_mul(quat_about_z(PLACE_YAW), DOWN)


def _abort_to_hub(node, done, reason):
    """Forward-side failure (before the box is grasped): retrace the executed
    segments in reverse so the arm ends back at the hub instead of stranded."""
    node.get_logger().error(f"[cycle] {reason}; retracing to hub")
    path = node.join([node.rev(seg) for seg in reversed(done)])
    if path:
        node.execute_path(path, speed=0.6)
    return False


def compute_place_ref(node, pocket_y):
    """Pocket-family seed (elbow-down) for goal-branch selection, on the
    bare-gripper model at the pocket centre. Returns config or None."""
    node.detach_box_collision()
    return node.compute_ik_ordered(pose_at(pocket_ref_xyz(pocket_y), place_quat()))


def shelf_pick_to_hub(node, box_world, box_model, stock_key, place_ref,
                      preflight_only=False):
    """Pick the shelf box and return to the hub holding it (box-attached model ON
    at exit). Pre-flight validates the approach spoke + grasp servos AND the
    twisted hub return (box + box-vs-stock model) with NO motion; a forward-side
    failure retraces to the hub. Returns True on success.
    preflight_only=True returns after the (no-motion) pre-flight passes, without
    executing -- used by the park-calibration sweep to test reach feasibility.
    The fixed-jaw lateral offset is baked into the approach, so the grasp is just
    approach -> J6 twist -> descend (no separate jaw-align).

    stock_key: (tier, i) of the TARGET box's resting-stock phantom. It stays ON
    for the insert pre-flights and the P1 spoke planning (pregrasp is 250 mm in
    front, insert/twist ride 73 mm above the box top, so they clear it -- and
    keeping it on stops the spoke from cutting through the target's volume,
    which brushed it, measured). It is parked only where the jaws legitimately
    meet the box: per-candidate for the descend pre-flight, then for the return
    sweep + execution once a branch is locked in."""
    box_ps = node.transform_world_pose(*box_world, DOWN)
    insert_dir = node.transform_world_vector([0.0, 1.0, 0.0])   # world +y = into shelf
    row_dir = node.transform_world_vector([1.0, 0.0, 0.0])      # world +x = magazine row
    if box_ps is None or insert_dir is None or row_dir is None:
        node.get_logger().error("[pick] TF unavailable; reposition and retry")
        return False
    insert_dir = insert_dir / (np.linalg.norm(insert_dir) or 1.0)
    box = np.array([box_ps.pose.position.x, box_ps.pose.position.y,
                    box_ps.pose.position.z])
    node.update_shelf_collision(shelf_pose_of(node))

    phi = math.atan2(row_dir[1], row_dir[0]) + GRASP_YAW_OFFSET
    grasp_quat = quat_mul(quat_about_z(phi), DOWN)
    pregrasp0_xyz = box - insert_dir * PREGRASP_BACK + np.array([0, 0, INSERT_TCP_ABOVE])
    descend_dist = INSERT_TCP_ABOVE - GRASP_TCP_ABOVE

    # ---- PRE-FLIGHT (no motion) ----
    q0 = node.ik_nearest(pose_at(pregrasp0_xyz, grasp_quat), place_ref)
    if q0 is None:
        node.get_logger().error("[pre-flight] centred pre-grasp IK failed")
        return False
    q0_tw = list(q0)
    q0_tw[5] += GRIPPER_YAW_TWIST
    jaw_x = node.gripper_x_in_base_fk(q0_tw)
    if jaw_x is None:
        node.get_logger().error("[pre-flight] gripper FK unavailable for jaw offset")
        return False
    pregrasp_xyz = pregrasp0_xyz - GRASP_LATERAL_M * jaw_x
    pregrasp_pose = pose_at(pregrasp_xyz, grasp_quat)

    # The place_ref-nearest IK branch is not always servo-able: it can drive the
    # straight-line insert through a wrist singularity even from a well-within-
    # reach pose (measured: box d stalled 4 mm short at sigma_min=1e-4). The pose
    # is fine, the BRANCH is not -- so vet candidates in place_ref order and take
    # the first whose insert + descend servos pre-flight clean. The target's own
    # phantom stays SOLID for the insert check (it rides 73 mm above the box) and
    # is parked only for the descend check, where the jaws wrap the box.
    cands = node.compute_ik_ordered(pose_at(pregrasp_xyz, grasp_quat),
                                    return_all=True)
    if not cands:
        node.get_logger().error("[pre-flight] pre-grasp IK failed")
        return False
    ref = np.array(place_ref)
    cands.sort(key=lambda q: np.linalg.norm(np.array(q) - ref))
    q_goal = None
    for k, q in enumerate(cands):
        # Candidate rejections log at INFO: trying the next branch is normal
        # flow, and the ERROR stream is forwarded to the MCS. The real abort
        # (ALL candidates rejected) is the ERROR below.
        after_insert = node.preflight_linear(q, insert_dir * PREGRASP_BACK,
                                             f"insert(cand {k})", severity='info')
        if after_insert is None:
            continue
        after_twist = list(after_insert)
        after_twist[5] += GRIPPER_YAW_TWIST
        node.set_shelf_stock_absent(*stock_key)
        ok = node.preflight_linear(after_twist, [0.0, 0.0, -descend_dist + 0.01],
                                   f"descend(cand {k})", severity='info') is not None
        node.set_shelf_stock_absent(*stock_key, absent=False)  # solid for P1
        if not ok:
            continue
        q_goal = q
        if k:
            node.get_logger().info(f"[pre-flight] branch candidate {k} passes "
                                   f"the grasp servos (0..{k - 1} rejected)")
        break
    if q_goal is None:
        node.get_logger().error("[pre-flight] no IK branch passes insert+descend; "
                                "abort (no motion)")
        return False

    node.attach_box_collision()
    if not node.is_state_valid(node.hub_q):
        node.detach_box_collision()
        node.get_logger().error("[pre-flight] hub collides with the carried box; "
                                "raise HUB_TCP")
        return False
    P1 = node.plan_spoke(node.hub_q, pregrasp_pose, place_ref, goal_q=q_goal,
                         label="P1 hub->pregrasp")
    node.detach_box_collision()
    if P1 is None:
        node.get_logger().error("[pre-flight] approach spoke infeasible; abort (no motion)")
        return False
    # Park the target's phantom for the rest of the cycle: the return sweep
    # models the box IN the gripper, and the executed descend wraps the jaws
    # around the box. On failure the caller restores it (box still on shelf).
    node.set_shelf_stock_absent(*stock_key)

    # Twisted hub return. Replaying P1 with J6 offset sweeps a volume the
    # untwisted plan never validated (measured: knocked neighbours over), so
    # validate it HERE, box phantom + box-vs-stock pairs on and the target's
    # stock absent (the box is in the gripper on the way back). If the replay
    # collides, pre-plan a replacement spoke under the same model -- still NO
    # motion, so a failure aborts before the arm ever moves. The executed twist
    # is driven to exactly +GRIPPER_YAW_TWIST below, matching this validation.
    node.attach_box_collision()
    node.set_box_stock_collision(True)
    spoke_back = node.offset_j6(node.rev(P1), GRIPPER_YAW_TWIST)
    if any(not node.is_state_valid(q) for q in spoke_back):
        node.get_logger().warn("[pre-flight] twisted replay collides with resting "
                               "stock; pre-planning the hub return")
        hub_tw = list(node.hub_q)
        hub_tw[5] += GRIPPER_YAW_TWIST
        node.cbirrt.set_reference(DOWN)
        spoke_back = (node.cbirrt.plan(list(spoke_back[0]), hub_tw,
                                       node.is_state_valid, node.joint_limits,
                                       time_limit=10.0)
                      if node.is_state_valid(hub_tw) else None)
    node.set_box_stock_collision(False)
    node.detach_box_collision()
    if not spoke_back:
        node.get_logger().error("[pre-flight] twisted hub return infeasible; "
                                "abort (no motion)")
        return False

    if preflight_only:
        # Restore the target phantom (line ~264 parked it for the pick that is
        # not going to happen) so a sweep's next box sees the shelf intact.
        node.set_shelf_stock_absent(*stock_key, absent=False)
        return True

    # ---- EXECUTE shelf side (each forward segment captured for the return) ----
    done = []

    node.control_gripper(GRIPPER_OPEN)
    if not node.execute_path(P1, speed=0.6):
        return _abort_to_hub(node, done, "approach spoke failed")
    done.append(P1)

    ok, insert_path = node.capture(
        lambda: node.linear_servo(insert_dir * PREGRASP_BACK, label="insert"))
    if not ok:
        return _abort_to_hub(node, done, "insert failed")
    done.append(insert_path)
    q_ins = node.current_joints.tolist()

    # Exactly +GRIPPER_YAW_TWIST (not rotate_j6, which may flip sign): the
    # twisted return above was validated for THIS sign, and move_single_joint
    # validity-checks the whole J6 sweep, not just the endpoint.
    if not node.move_single_joint(5, q_ins[5] + GRIPPER_YAW_TWIST,
                                  label="yaw-twist"):
        return _abort_to_hub(node, done, "yaw twist failed")
    q_tw = node.current_joints.tolist()
    twist_delta = q_tw[5] - q_ins[5]
    done.append([q_ins, q_tw])

    ok, descend_path = node.capture(
        lambda: node.linear_servo([0.0, 0.0, -descend_dist + 0.007], label="descend"))
    if not ok:
        return _abort_to_hub(node, done, "descend failed")
    done.append(descend_path)

    node.control_gripper(GRIPPER_CLOSE)
    # Resolve the actual Gazebo model at the box position (no hardcoded names);
    # the layout-derived name is only the fallback when /gazebo/model_states is
    # not up (gazebo_ros_state plugin missing / real robot).
    node.object_model = pockets.model_at(node, box_world) or box_model
    node.object_link = SHELF_BOX_LINK
    # Grasp-offset diagnostic (sim-only, from the model_states cache): where the
    # box ACTUALLY is at grasp time vs the layout target. A consistent offset
    # here = set SHELF_BOX_X_NUDGE (geometry.py) to the printed model-frame dx;
    # a big offset only on later cycles = earlier cycles are disturbing the box.
    ms = getattr(node, '_model_states', None)
    if ms is not None and node.object_model in ms.name:
        p = ms.pose[ms.name.index(node.object_model)].position
        d = (p.x - box_world[0], p.y - box_world[1], p.z - box_world[2])
        node.get_logger().info(
            f"[grasp-offset] {node.object_model}: actual - target = "
            f"({d[0]*1000:+.0f}, {d[1]*1000:+.0f}, {d[2]*1000:+.0f}) mm (world)")
    if not node.attach_box():
        node.control_gripper(GRIPPER_OPEN)
        return _abort_to_hub(node, done, "ATTACHLINK failed")
    node.attach_box_collision()
    time.sleep(0.5)

    # Return holding the twist (the un-twist waits for the hub, where there is
    # open space). Ascend + retreat are straight, box-safe moves: the box slides
    # back out exactly the way it came in.
    out_path = node.join([
        node.rev(descend_path),                              # ascend (lift box)
        node.offset_j6(node.rev(insert_path), twist_delta),  # retreat, twist held
    ])
    if not node.execute_path(out_path, speed=0.6):
        node.get_logger().error("[pick] shelf pull-out failed")
        return False

    # Spoke back to the hub: pre-validated (or pre-planned) in the pre-flight
    # under the twisted, box-attached, box-vs-stock model, so no runtime
    # re-planning is ever needed here.
    if not node.execute_path(spoke_back, speed=0.6):
        node.get_logger().error("[pick] shelf return failed")
        return False
    # At the hub now (J6 still twisted). Un-twist here, in the open, to drop the
    # picked azimuth and land exactly on hub_q for the pocket spoke.
    if not node.move_single_joint(5, node.hub_q[5], label="untwist-at-hub"):
        node.get_logger().error("[pick] hub un-twist failed")
        return False
    return True


def pocket_place_from_hub(node, pocket_y, place_ref, place_jaw_x, label):
    """From the hub holding the box: carry to the pocket, GUARDED-place it
    (leave it), and return to the hub by reverse-replay. Assumes the
    box-attached collision model is ON at entry; it is OFF on exit (box left
    in the pocket). Returns True/False.

    GUARDED place (ported 2026-07-15 from wirebonder place_base; the v1
    restore had regressed to a fixed 80 mm drop that released the box 5 mm
    INTO the pocket floor every cycle -- the release log said so each time --
    and the press reaction ratcheted the AGV base downward, see level_base):
    descend with the box phantom ON as the contact sensor and STOP the instant
    the box meets the pocket surface, so each pocket seats at its true height
    with no press."""
    hover_pose = pose_at(pocket_hover_xyz(pocket_y, place_jaw_x), place_quat())
    P2 = node.plan_spoke(node.hub_q, hover_pose, place_ref, label=label)
    if P2 is None:
        node.get_logger().error(f"[{label}] carry spoke infeasible")
        return False

    node._start_recording()
    if not node.execute_path(P2, speed=0.6):
        node.get_logger().error(f"[{label}] carry spoke exec failed")
        return False
    forward = node._stop_recording()

    # Guarded descend: phantom stays ON (it IS the sensor). Nominal gap from
    # the hover to box-bottom contact is POCKET_HOVER - GRASP_TCP_ABOVE -
    # BOX_H/2 (~95 mm); the generous max is safe because it always stops AT
    # contact (same reasoning as wirebonder BASE_PLACE_OVERTRAVEL).
    drop = node.guarded_descend(POCKET_HOVER, label=f"{label} place-descend")
    node.detach_box_collision()

    # Detach from the gripper; optionally fix the box to the AGV so it rides
    # along when the base drives (MAGAZINE_ATTACH -- off while the AGV is parked).
    node.detach_box()
    if MAGAZINE_ATTACH and not node.attach_box_to_magazine():
        node.get_logger().warn(f"[{label}] magazine attach failed; box left loose")
    node.control_gripper(GRIPPER_OPEN)
    time.sleep(0.5)

    # Back out: undo the measured drop, then retrace the recorded carry spoke.
    if drop > 1e-3 and not node.linear_servo([0.0, 0.0, drop],
                                             label=f"{label} place-ascend"):
        node.get_logger().error(f"[{label}] place-ascend failed")
        return False
    if not node.replay_reverse(forward):
        node.get_logger().error(f"[{label}] return to hub failed")
        return False
    return True


def pick_place_one_box(node, idx):
    """Pick shelf box `idx` and place it into the next free pocket from the
    HIGH-y end (default PLACE_ORDER_Y[N-1-idx]), starting and ending at the
    hub. Both spokes are pre-flighted with NO motion, so an unreachable box or
    pocket aborts before the arm moves. Returns True/False."""
    box_world, box_model = shelf_box(node, idx)
    # Dynamic pocket: ONE wrist-bend look at the base per command, then place
    # into the next FREE pocket. Fill order REVERSED (2026-07-15): from the
    # HIGH-y end downward (box 0 -> pocket 3, then 2, 1, 0), scanning step=-1.
    # None = vision not configured (--no-vision) -> the fixed mirrored order; a
    # FAILED look reads all-UNKNOWN and aborts here (never place blind).
    n_pockets = len(PLACE_ORDER_Y)
    prefer = n_pockets - 1 - idx
    occ = pockets.check_pockets(node)
    if occ is None:
        pocket = prefer
    else:
        pocket = pockets.next_free(occ, start=prefer, step=-1)
        if pocket is None:
            node.get_logger().error(
                f"[cycle] no usable pocket (occupancy {occ}; all -1 = the "
                f"look/read failed, see [pockets] errors above)")
            return False
        if pocket != prefer:
            node.get_logger().info(
                f"[cycle] pocket {prefer} occupied; placing into pocket {pocket}")
    pocket_y = PLACE_ORDER_Y[pocket]

    place_ref = compute_place_ref(node, pocket_y)
    if place_ref is None:
        node.get_logger().error("[cycle] pocket family seed IK failed")
        return False
    place_jaw_x = node.gripper_x_in_base_fk(place_ref)
    if place_jaw_x is None:
        node.get_logger().error("[cycle] gripper FK unavailable for place offset")
        return False

    # Pre-flight the pocket carry spoke (box phantom on, no motion).
    node.attach_box_collision()
    feasible = node.plan_spoke(
        node.hub_q, pose_at(pocket_hover_xyz(pocket_y, place_jaw_x), place_quat()),
        place_ref, label=f"pre pocket(y={pocket_y:+.3f})") is not None
    node.detach_box_collision()
    if not feasible:
        node.get_logger().error("[cycle] pocket unreachable; abort (no motion)")
        return False

    # The target's own stock phantom is parked INSIDE shelf_pick_to_hub, right
    # before the descend pre-flight (approach + insert are validated against it).
    # On any pick failure the box is still on the shelf: restore its phantom.
    if not shelf_pick_to_hub(node, box_world, box_model, (TIER, idx), place_ref):
        node.set_shelf_stock_absent(TIER, idx, absent=False)  # box still on shelf
        return False
    return pocket_place_from_hub(node, pocket_y, place_ref, place_jaw_x,
                                 label=box_model)


# --- vision: live shelf pose from the tier ArUco -------------------------------
# Detection + shelf tag config live in vision/tag_vision.py, run by
# vision/tag_vision_node.py; it publishes T_odom_shelfmodel on /vision/shelf_pose. This
# sequence only CONSUMES it -- capture_shelf() medians a burst into node.shelf_pose
# = (x, y, yaw), the SAME (x, y, yaw) seam shelf_box_center / update_shelf_collision
# already compose with. Mirrors wirebonder_pick_place capture_device /
# refresh_device_pose.

# Capture viewpoint. Default (None) is AUTO-AIM: capture_shelf computes a pose that
# frames the tier-TIER tag for the D405 from the tag's NOMINAL world position
# (shelf_tag_world) + the live camera extrinsic (no hand-jog). Set this to a jogged
# 6-joint config (tools/jog_tcp.py) ONLY as a fallback if auto-aim can't reach a
# viewpoint in your cell -- then it's used verbatim (wirebonder's CAPTURE_A_JOINTS
# style).
SHELF_CAPTURE_JOINTS = None

# Auto-aim: camera standoff in FRONT of the tag (m, along the tag normal, world -y).
# RAISE if the tag over/underfills the frame or the viewpoint is unreachable; the
# READ itself is range-exact, so this only has to put the tag in the FOV. TUNE.
SHELF_CAPTURE_STANDOFF = 0.30

# Capture sanity gates (mirror wirebonder). Spread catches a stale 2nd vision node;
# the anchor tolerance catches a constant-but-wrong solve -- the AGV parks near the
# same spot, so a good read stays near SHELF_WORLD_POSE.
CAPTURE_SPREAD_MAX_MM = 15.0
CAPTURE_POS_TOL_MM = 400.0
CAPTURE_YAW_TOL_DEG = 30.0


def quat_yaw(o):
    """Yaw (z-rotation) from a quaternion message field."""
    return math.atan2(2.0 * (o.w * o.z + o.x * o.y),
                      1.0 - 2.0 * (o.y * o.y + o.z * o.z))


def refresh_shelf_pose(node, n=15, timeout=6.0):
    """Median a burst of /vision/shelf_pose into node.shelf_pose = (x, y, yaw).
    Collects up to `n` DISTINCT frames (dedup by stamp) and takes the per-axis
    median (robust to single-tag jitter). Returns True, or False on no pose / a
    failed sanity gate -- the caller then keeps the SHELF_WORLD_POSE default or
    aborts rather than trusting a bad read."""
    samples, seen = [], set()
    node._shelf_vision_pose = None
    t0 = time.time()
    while len(samples) < n and time.time() - t0 < timeout:
        ps = node._shelf_vision_pose
        if ps is not None:
            key = (ps.header.stamp.sec, ps.header.stamp.nanosec)
            if key not in seen:
                seen.add(key)
                p, o = ps.pose.position, ps.pose.orientation
                samples.append((p.x, p.y, quat_yaw(o)))
        time.sleep(0.02)
    if not samples:
        node.get_logger().error(
            f"[vision] no /vision/shelf_pose in {timeout}s -- is the vision node "
            f"running and the tier-{TIER} tag (id {2 + TIER - 1}) in FOV?")
        return False
    arr = np.array(samples)
    med = np.median(arr, axis=0)            # ponytail: yaw ~0 here, no wrap handling
    spread = arr[:, :2].std(axis=0) * 1000.0
    if spread.max() > CAPTURE_SPREAD_MAX_MM:
        node.get_logger().error(
            f"[vision] shelf REJECTED: spread {spread[0]:.0f}/{spread[1]:.0f} mm > "
            f"{CAPTURE_SPREAD_MAX_MM:.0f} -- stale 2nd vision node? "
            f"(ros2 topic info /vision/shelf_pose: Publisher count should be 1)")
        return False
    off = np.abs(med[:2] - np.array(SHELF_WORLD_POSE[:2])) * 1000.0
    dyaw = math.degrees(abs(med[2] - SHELF_WORLD_POSE[2]))
    if off.max() > CAPTURE_POS_TOL_MM or dyaw > CAPTURE_YAW_TOL_DEG:
        node.get_logger().error(
            f"[vision] shelf REJECTED: pose {off[0]:.0f}/{off[1]:.0f} mm "
            f"dyaw={dyaw:.0f} deg off anchor (tol {CAPTURE_POS_TOL_MM:.0f} mm/"
            f"{CAPTURE_YAW_TOL_DEG:.0f} deg) -- bad read")
        return False
    node.shelf_pose = tuple(med)
    node.get_logger().info(
        f"[vision] shelf <- median of {len(samples)} frames: x={med[0]:.3f} "
        f"y={med[1]:.3f} yaw={med[2]:+.3f}  (xy spread "
        f"{spread[0]:.0f}/{spread[1]:.0f} mm)")
    return True


# --- auto-aim SE3 helpers (local so the sequence needs no cv2/vision import) -----
def _make_T(R, t):
    T = np.eye(4)
    T[:3, :3] = R
    T[:3, 3] = np.asarray(t, dtype=float)
    return T


def _inv_T(T):
    R, t = T[:3, :3], T[:3, 3]
    Ti = np.eye(4)
    Ti[:3, :3] = R.T
    Ti[:3, 3] = -R.T @ t
    return Ti


def _R_to_quat(m):
    """Rotation matrix -> quaternion (x, y, z, w)."""
    tr = m[0, 0] + m[1, 1] + m[2, 2]
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        w, x = 0.25 * s, (m[2, 1] - m[1, 2]) / s
        y, z = (m[0, 2] - m[2, 0]) / s, (m[1, 0] - m[0, 1]) / s
    elif m[0, 0] > m[1, 1] and m[0, 0] > m[2, 2]:
        s = math.sqrt(1.0 + m[0, 0] - m[1, 1] - m[2, 2]) * 2
        w, x = (m[2, 1] - m[1, 2]) / s, 0.25 * s
        y, z = (m[0, 1] + m[1, 0]) / s, (m[0, 2] + m[2, 0]) / s
    elif m[1, 1] > m[2, 2]:
        s = math.sqrt(1.0 + m[1, 1] - m[0, 0] - m[2, 2]) * 2
        w, x = (m[0, 2] - m[2, 0]) / s, (m[0, 1] + m[1, 0]) / s
        y, z = 0.25 * s, (m[1, 2] + m[2, 1]) / s
    else:
        s = math.sqrt(1.0 + m[2, 2] - m[0, 0] - m[1, 1]) * 2
        w, x = (m[1, 0] - m[0, 1]) / s, (m[0, 2] + m[2, 0]) / s
        y, z = (m[1, 2] + m[2, 1]) / s, 0.25 * s
    return (x, y, z, w)


def _look_at_R(z_dir, up=(0.0, 0.0, -1.0)):
    """Optical-frame rotation (columns x, y, z) whose z-axis (camera forward) points
    along z_dir. Optical convention: z fwd, y down (image up = world up)."""
    z = np.asarray(z_dir, float)
    z = z / (np.linalg.norm(z) or 1.0)
    x = np.cross(up, z)
    if np.linalg.norm(x) < 1e-6:
        x = np.cross((0.0, 1.0, 0.0), z)
    x = x / (np.linalg.norm(x) or 1.0)
    return np.column_stack([x, np.cross(z, x), z])


def _pose_to_T(ps):
    o, p = ps.pose.orientation, ps.pose.position
    return _make_T(quat_to_R(o.x, o.y, o.z, o.w), [p.x, p.y, p.z])


def _lookup_T(node, target, source):
    """target<-source as a 4x4 from the live TF tree, or None."""
    try:
        tf = node.tf_buffer.lookup_transform(
            target, source, rclpy.time.Time(), timeout=Duration(seconds=3.0))
    except Exception as e:
        node.get_logger().error(f"[TF] {target}<-{source} failed: {e}")
        return None
    t, q = tf.transform.translation, tf.transform.rotation
    return _make_T(quat_to_R(q.x, q.y, q.z, q.w), [t.x, t.y, t.z])


def aim_pose_at_tag(node):
    """Compute a base_link TCP pose that frames the tier-TIER tag for the D405, from
    the tag's NOMINAL world position (shelf_tag_world) + the live camera extrinsic
    (TCP<-optical, from FK + TF). No hand-jog. Returns a PoseStamped or None.

    Aiming only needs the tag in the FOV (the READ is range-exact), so the anchor-pose
    tag position is plenty. The tag faces world -y, so the camera sits
    SHELF_CAPTURE_STANDOFF in front on the -y side, optical z looking +y at the tag."""
    tx, ty, tz = shelf_tag_world(TIER)
    cam_w = (tx, ty - SHELF_CAPTURE_STANDOFF, tz)

    # Desired optical pose in WORLD (look toward +y), then -> base_link via TF
    # (transform_world_pose rotates the quaternion too, not just the position).
    opt_ps = node.transform_world_pose(*cam_w, _R_to_quat(_look_at_R([0.0, 1.0, 0.0])))
    if opt_ps is None:
        return None
    T_base_opt_goal = _pose_to_T(opt_ps)

    # Constant extrinsic TCP<-optical, measured at the CURRENT config (both are rigid
    # to the flange): inv(T_base_tcp_now) @ T_base_optical_now.
    pos, R = node.ik_model.fk_tcp(node.ik_model.pin_q(node.current_joints.tolist()))
    T_base_opt_now = _lookup_T(node, 'base_link', 'd405_optical_frame')
    if T_base_opt_now is None:
        return None
    T_tcp_opt = _inv_T(_make_T(R, pos)) @ T_base_opt_now

    # TCP goal that lands the optical frame on the look-at pose.
    T = T_base_opt_goal @ _inv_T(T_tcp_opt)
    return pose_at(T[:3, 3], _R_to_quat(T[:3, :3]))


def capture_shelf(node):
    """Drive to the tag viewpoint, read the shelf pose ONCE (arm at rest), and return
    to the hub. The viewpoint is AUTO-AIMED at the tier tag by default
    (aim_pose_at_tag), or the jogged SHELF_CAPTURE_JOINTS if that override is set. The
    read runs only at REST (dwell + median); the outbound path is recorded and
    reverse-replayed home (box-safe), then snapped to the exact hub_q -- the SAME
    pattern as wirebonder capture_device. Returns True on a good read
    (node.shelf_pose set)."""
    if not node.go_to_hub():
        return False

    if SHELF_CAPTURE_JOINTS is not None:
        def forward():
            if not node.joint_move(list(SHELF_CAPTURE_JOINTS)):
                return False
            time.sleep(2.0)   # settle: the arm keeps moving ~2 s after "finished"
            return True
    else:
        aim = aim_pose_at_tag(node)
        if aim is None:
            node.get_logger().error(
                "[vision] auto-aim pose unavailable (TF/extrinsic missing); jog a "
                "viewpoint and set SHELF_CAPTURE_JOINTS")
            return False
        p = aim.pose.position
        node.get_logger().info(
            f"[vision] auto-aim viewpoint (base_link TCP): "
            f"x={p.x:.3f} y={p.y:.3f} z={p.z:.3f}")

        def forward():
            # Free RRT to the computed viewpoint (branch nearest the hub).
            # move_to_pose_ref gates on IK + plans a collision-free path, so an
            # unreachable/blocked viewpoint fails HERE (nothing stranded) -- then set
            # SHELF_CAPTURE_JOINTS as a fallback.
            if not node.move_to_pose_ref(aim, node.hub_q):
                return False
            time.sleep(2.0)   # settle before the read
            return True

    ok_fwd, fwd = node.capture(forward)
    if not ok_fwd:
        node.get_logger().error(
            "[vision] capture viewpoint unreachable/blocked; jog one and set "
            "SHELF_CAPTURE_JOINTS")
        node.go_to_hub()
        return False
    ok = refresh_shelf_pose(node)
    node.replay_reverse(fwd)                 # viewpoint -> hub (recorded, box-safe)
    node.go_to_hub()                         # snap residual drift to the exact hub_q
    return ok


def bringup(node):
    """One-time bring-up (also called by main.py): place the shelf collision
    (boards + resting-stock phantoms) at the anchor pose BEFORE any motion --
    the spawn->hub RRT must already know the shelf -- then compute the hub, move
    there. Returns True when ready to cycle. (The box_l2c pocket stowaway and
    its bring-up delete were removed 2026-07-15 -- nothing spawns onto the
    pockets anymore.)

    Vision-agnostic: it places the collision at the SHELF_WORLD_POSE anchor. The
    live ArUco read (capture_shelf) is the CALLER's step -- main() below runs it up
    front, and main.py's locate('shelf') would run it after the AMR parks -- and
    then re-places the collision at node.shelf_pose."""
    while node.current_joints is None:
        node.get_logger().info("Waiting for /joint_states...")
        time.sleep(0.5)
    if not node.update_shelf_collision():
        node.get_logger().error("[bringup] shelf TF unavailable; is the sim up?")
        return False
    if not node.init_hub(pose_at(pocket_ref_xyz(PLACE_ORDER_Y[0]), place_quat()),
                         HUB_TCP, GRASP_LATERAL_M):
        node.get_logger().error("Hub bring-up failed; adjust HUB_TCP and retry")
        return False
    if not node.go_to_hub():
        node.get_logger().error("Could not reach the hub from the spawn pose")
        return False
    return True


def locate_shelf(node):
    """LOCATE step -- read the tier tag and re-place the shelf collision at the
    live pose. The explicit 'read the tag FIRST' step (shelf analog of wirebonder's
    capture_device): call it after bring-up, once the AMR is parked at the shelf.
    Returns True on a good read (node.shelf_pose set + collision re-placed), False
    otherwise -- the caller then aborts rather than run on a possibly-wrong default."""
    if not capture_shelf(node):
        return False
    node.update_shelf_collision(shelf_pose_of(node))
    return True


def main(args=None):
    rclpy.init(args=args)
    node = HubPickPlace()
    node.setup_planner()

    # --no-vision: skip the ArUco read and use the SHELF_WORLD_POSE spawn default
    # (precise shelf spawn required). Default is vision-driven: the shelf model pose
    # arrives on /vision/shelf_pose (odom) from tag_vision_node.py; locate_shelf()
    # reads it below.
    use_vision = '--no-vision' not in sys.argv
    if use_vision:
        node._shelf_vision_pose = None
        node.shelf_pose = None
        node.create_subscription(PoseStamped, '/vision/shelf_pose',
                                 lambda m: setattr(node, '_shelf_vision_pose', m), 10)
        # Pocket-occupancy cache (check_pockets reads it before each place).
        pockets.subscribe(node)
    # Gazebo model-name cache: the shelf pick resolves the box model via
    # model_at() in sim, vision or not (layout name stays the fallback).
    pockets.subscribe_models(node)

    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2)  # wait for joint states

    if not bringup(node):
        node.destroy_node()
        rclpy.shutdown()
        return

    # LOCATE: read the tier-1 tag ONCE up front (arm parked at the shelf), the shelf
    # analog of wirebonder's capture_device. Refuse to run on a bad read rather than
    # silently placing boxes at a possibly-wrong SHELF_WORLD_POSE default.
    if use_vision:
        print(" Reading the tier-1 shelf tag at the capture viewpoint ...")
        if not locate_shelf(node):
            print(" Shelf tag read FAILED -- restart with the vision node up and the "
                  "tag in FOV, or run with --no-vision (SHELF_WORLD_POSE default).")
            node.destroy_node()
            rclpy.shutdown()
            return

    n = N_BOXES
    print("\n" + "=" * 60)
    print(f" Hub-and-spoke pick & place ready: {n} shelf boxes -> {n} pockets.")
    print(" Each SPACE picks the next box and places it in the next pocket.")
    print(" Drive the AGV near the shelf, then press SPACE. (q / Esc to quit)")
    print("=" * 60)

    try:
        while rclpy.ok():
            if wait_for_spacebar() == 'quit':
                break
            if node.box_idx >= n:
                print(f"\n>>> All {n} boxes placed. Nothing left to do (q to quit).")
                continue
            idx = node.box_idx
            if pick_place_one_box(node, idx):
                node.box_idx += 1
                print(f"\n>>> Box {idx + 1}/{n} placed (arm at hub). "
                      f"SPACE for the next box.")
            else:
                print(f"\n>>> Box {idx + 1}/{n} FAILED (no motion if pre-flight, "
                      f"else returned to hub). Reposition the AGV and SPACE to retry.")
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
