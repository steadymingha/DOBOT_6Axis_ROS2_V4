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

Run (sim already up). SYSTEM python, not the .venv -- the venv's numpy 2.x
segfaults the ROS pinocchio build:
    source /opt/ros/humble/setup.bash
    source ~/dobot_ws/install/setup.bash
    cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
    /usr/bin/python3 sequences/shelf_pick_place.py
"""

import math
import os
import sys
import time
import threading

import numpy as np
import rclpy
from rclpy.executors import MultiThreadedExecutor

# sequences/ is one level below the package root; add the root so cr7_pnp imports
# when this file is run standalone (python3 sequences/shelf_pick_place.py).
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from cr7_pnp import (  # noqa: E402
    HubPickPlace, pose_at, quat_mul, quat_about_z, wait_for_spacebar,
    DOWN, GRASP_YAW_OFFSET, GRIPPER_YAW_TWIST,
    GRIPPER_OPEN, GRIPPER_CLOSE,
    INSERT_TCP_ABOVE, GRASP_TCP_ABOVE, PREGRASP_BACK,
    POCKET_X, POCKET_SURFACE_Z, POCKET_HOVER, PLACE_TCP_ABOVE,
    SHELF_BOX_LINK, BOX_SIZE, GRASP_LATERAL_M,
    SHELF_BOX_XS, shelf_box_center, shelf_box_model,
)

# Tool-down HUB waypoint (carried-object TCP, tool pointing DOWN). Solving IK
# toward the pocket branch keeps the hub in the SAME elbow/wrist family as both
# spokes, so hub<->shelf and hub<->pocket stay short and never flip the elbow.
# TUNE IN SIM: raise z (or pull x toward the pocket ~0.37) if the hub IK fails or
# the carried box collides at the hub.
HUB_TCP = (0.33, 0.0, 0.32)
# Hub joints (J1..J6 rad): -0.45320, 0.34299, -1.88375, 0.0, -1.52609, -0.43032 (deg: -26, +20, -108, 0, -87, -25),

# Pocket centres in base_link y, in the order boxes are placed (-y to +y).
PLACE_ORDER_Y = [-0.177, -0.059, 0.059, 0.177]

# Tier-1 shelf boxes (world xyz, Gazebo model name), derived from the SAME
# layout constants the collision stock phantoms use (SHELF_TIER_TOPS /
# SHELF_BOX_XS in cr7_pnp/geometry.py), so tier height and box names live in
# ONE place and always match cr.world. Box i goes to pocket PLACE_ORDER_Y[i].
TIER = 1
SHELF_BOXES = [(shelf_box_center(TIER, i), shelf_box_model(TIER, i))
               for i in range(len(SHELF_BOX_XS))]

# The AGV stays parked for this sequence (spawn-and-pick), so a placed box just
# settles into its pocket by gravity. Set True if the AGV will DRIVE with boxes
# aboard -- a loose box slides in the pocket (friction can't beat planar_move).
MAGAZINE_ATTACH = False

# Sim-only stowaway magazine the launch file spawns onto a base pocket for the
# wirebonder flow; dropped at bring-up so all four pockets are free for placing.
STOWAWAY_MODEL = 'box_l2c'

# Release TCP height above the pocket surface. RAISE this if the box is pressed
# into the pocket floor; LOWER it if it drops from too high. TUNE IN SIM.
PLACE_TCP_ABOVE_HUB = PLACE_TCP_ABOVE   # default = 0.08

# Fixed-jaw azimuth at the hub (standby) and the pocket (place), as a yaw about
# the tool axis (tool stays pointing down). math.pi flips the fixed jaw 180 deg
# from the bare-DOWN direction; flip to -math.pi if the wrist twists the wrong
# way in sim. The lateral place offset follows this automatically (derived from
# the place orientation via FK), so the box stays centred either way.
PLACE_YAW = math.pi


def pocket_center_xyz(pocket_y):
    """Hover TCP directly above a pocket centre (no jaw offset)."""
    return np.array([POCKET_X, pocket_y, POCKET_SURFACE_Z + POCKET_HOVER])


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
    return node.compute_ik_ordered(pose_at(pocket_center_xyz(pocket_y), place_quat()))


def shelf_pick_to_hub(node, box_world, box_model, stock_key, place_ref):
    """Pick the shelf box and return to the hub holding it (box-attached model ON
    at exit). Pre-flight validates the approach spoke + grasp servos AND the
    twisted hub return (box + box-vs-stock model) with NO motion; a forward-side
    failure retraces to the hub. Returns True on success.
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
    node.update_shelf_collision()

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
    node.object_model, node.object_link = box_model, SHELF_BOX_LINK
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
    """From the hub holding the box: carry to the pocket, place it (leave it), and
    return to the hub by reverse-replay. Assumes the box-attached collision model
    is ON at entry; it is OFF on exit (box left in the pocket). Returns True/False.

    The carry runs with the box model on (clears obstacles); the final descend
    drops the phantom first, since the box entering its pocket would otherwise read
    contact with the magazine as a collision."""
    hover_pose = pose_at(pocket_hover_xyz(pocket_y, place_jaw_x), place_quat())
    P2 = node.plan_spoke(node.hub_q, hover_pose, place_ref, label=label)
    if P2 is None:
        node.get_logger().error(f"[{label}] carry spoke infeasible")
        return False

    node._start_recording()
    if not node.execute_path(P2, speed=0.6):
        node.get_logger().error(f"[{label}] carry spoke exec failed")
        return False
    node.detach_box_collision()
    box_bottom = PLACE_TCP_ABOVE_HUB - GRASP_TCP_ABOVE - BOX_SIZE[2] / 2.0
    node.get_logger().info(
        f"[{label}] release: TCP {PLACE_TCP_ABOVE_HUB * 1000:.0f} mm above pocket; "
        f"box bottom {box_bottom * 1000:+.0f} mm vs surface "
        f"(raise PLACE_TCP_ABOVE_HUB if pressed in)")
    if not node.linear_servo([0.0, 0.0, PLACE_TCP_ABOVE_HUB - POCKET_HOVER + 0.02],
                             label="place-descend"):
        node.get_logger().error(f"[{label}] place-descend failed")
        return False
    forward = node._stop_recording()

    # Detach from the gripper; optionally fix the box to the AGV so it rides
    # along when the base drives (MAGAZINE_ATTACH -- off while the AGV is parked).
    node.detach_box()
    if MAGAZINE_ATTACH and not node.attach_box_to_magazine():
        node.get_logger().warn(f"[{label}] magazine attach failed; box left loose")
    node.control_gripper(GRIPPER_OPEN)
    time.sleep(0.5)

    if not node.replay_reverse(forward):
        node.get_logger().error(f"[{label}] return to hub failed")
        return False
    return True


def pick_place_one_box(node, idx):
    """Pick shelf box `idx` and place it into pocket PLACE_ORDER_Y[idx], starting
    and ending at the hub. Both spokes are pre-flighted with NO motion, so an
    unreachable box or pocket aborts before the arm moves. Returns True/False."""
    box_world, box_model = SHELF_BOXES[idx]
    pocket_y = PLACE_ORDER_Y[idx]

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


def clear_pocket_stowaway(node):
    """Delete the sim-only stowaway magazine (spawned onto a pocket by the launch
    file for the wirebonder flow) so all four pockets are free. Best-effort: the
    model may already be gone, or lying on the floor away from the pockets."""
    from gazebo_msgs.srv import DeleteEntity
    cli = node.create_client(DeleteEntity, '/delete_entity')
    if not cli.wait_for_service(timeout_sec=3.0):
        node.get_logger().warn("[bringup] /delete_entity unavailable; stowaway left")
        return
    req = DeleteEntity.Request()
    req.name = STOWAWAY_MODEL
    future = cli.call_async(req)
    if node._wait_future(future, 10.0, "delete stowaway"):
        node.get_logger().info(
            f"[bringup] {STOWAWAY_MODEL}: {future.result().status_message}")


def bringup(node):
    """One-time bring-up (also called by main.py): place the shelf collision
    (boards + resting-stock phantoms) at the anchor pose BEFORE any motion --
    the spawn->hub RRT must already know the shelf -- then compute the hub, move
    there, and drop the pocket stowaway. Returns True when ready to cycle."""
    while node.current_joints is None:
        node.get_logger().info("Waiting for /joint_states...")
        time.sleep(0.5)
    if not node.update_shelf_collision():
        node.get_logger().error("[bringup] shelf TF unavailable; is the sim up?")
        return False
    if not node.init_hub(pose_at(pocket_center_xyz(PLACE_ORDER_Y[0]), place_quat()),
                         HUB_TCP, GRASP_LATERAL_M):
        node.get_logger().error("Hub bring-up failed; adjust HUB_TCP and retry")
        return False
    if not node.go_to_hub():
        node.get_logger().error("Could not reach the hub from the spawn pose")
        return False
    clear_pocket_stowaway(node)
    return True


def main(args=None):
    rclpy.init(args=args)
    node = HubPickPlace()
    node.setup_planner()

    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2)  # wait for joint states

    if not bringup(node):
        node.destroy_node()
        rclpy.shutdown()
        return

    n = len(SHELF_BOXES)
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
