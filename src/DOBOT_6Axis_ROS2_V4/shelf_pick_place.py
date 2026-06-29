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

Run (sim already up):
    source /opt/ros/humble/setup.bash
    source ~/dobot_ws/install/setup.bash
    cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
    ~/dobot_ws/.venv/bin/python3 shelf_pick_place.py
"""

import math
import time
import threading

import numpy as np
import rclpy
from rclpy.executors import MultiThreadedExecutor

from cr7_pnp import (
    HubPickPlace, pose_at, quat_mul, quat_about_z, wait_for_spacebar,
    DOWN, GRASP_YAW_OFFSET, GRIPPER_YAW_TWIST,
    GRIPPER_OPEN, GRIPPER_CLOSE,
    INSERT_TCP_ABOVE, GRASP_TCP_ABOVE, PREGRASP_BACK,
    POCKET_X, POCKET_SURFACE_Z, POCKET_HOVER, PLACE_TCP_ABOVE,
    SHELF_BOX_LINK, BOX_SIZE, GRASP_LATERAL_M,
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

# Tier-1 shelf boxes (world xyz, Gazebo model name). Box i goes to pocket
# PLACE_ORDER_Y[i] (1:1). Edit the order/mapping here if needed.
SHELF_BOXES = [
    ((0.7095, 0.5, 0.97), 'box_l1a'),
    ((0.8905, 0.5, 0.97), 'box_l1b'),
    ((0.5285, 0.5, 0.97), 'box_l1c'),
    ((1.0715, 0.5, 0.97), 'box_l1d'),
]

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


def shelf_pick_to_hub(node, box_world, box_model, place_ref):
    """Pick the shelf box and return to the hub holding it (box-attached model ON
    at exit). Pre-flight validates the approach spoke + grasp servos with NO
    motion; a forward-side failure retraces to the hub. Returns True on success.
    The fixed-jaw lateral offset is baked into the approach, so the grasp is just
    approach -> J6 twist -> descend (no separate jaw-align)."""
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

    node.attach_box_collision()
    if not node.is_state_valid(node.hub_q):
        node.detach_box_collision()
        node.get_logger().error("[pre-flight] hub collides with the carried box; "
                                "raise HUB_TCP")
        return False
    P1 = node.plan_spoke(node.hub_q, pregrasp_pose, place_ref, label="P1 hub->pregrasp")
    node.detach_box_collision()
    if P1 is None:
        node.get_logger().error("[pre-flight] approach spoke infeasible; abort (no motion)")
        return False
    after_insert = node.preflight_linear(P1[-1], insert_dir * PREGRASP_BACK, "insert")
    if after_insert is None:
        node.get_logger().error("[pre-flight] insert unreachable; abort (no motion)")
        return False
    after_twist = list(after_insert)
    after_twist[5] += GRIPPER_YAW_TWIST
    if node.preflight_linear(after_twist, [0.0, 0.0, -descend_dist + 0.01],
                             "descend") is None:
        node.get_logger().error("[pre-flight] descend unreachable; abort (no motion)")
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

    if not node.rotate_j6(GRIPPER_YAW_TWIST, label="yaw-twist"):
        return _abort_to_hub(node, done, "yaw twist failed")
    q_tw = node.current_joints.tolist()
    twist_delta = q_tw[5] - q_ins[5]   # actual J6 change (rotate_j6 may flip sign)
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

    # Return holding the twist the WHOLE way to the hub: ascend, retreat, then
    # reverse the (box-validated) approach spoke -- all with J6 offset so the
    # gripper stays in its picked azimuth. The longer gripper sweeps into the
    # neighbouring shelf boxes if it un-twists right next to the shelf, so the
    # un-twist is deferred to the hub, where there is open space.
    return_path = node.join([
        node.rev(descend_path),                              # ascend (lift box)
        node.offset_j6(node.rev(insert_path), twist_delta),  # retreat, twist held
        node.offset_j6(node.rev(P1), twist_delta),           # spoke to hub, twist held
    ])
    bad = sum(0 if node.is_state_valid(q) else 1 for q in return_path)
    if bad:
        node.get_logger().warn(
            f"[return] {bad}/{len(return_path)} return waypoints collide under the "
            f"twist-held box model; proceeding (straight pull-out is box-safe; the "
            f"spoke was box-validated in the untwisted azimuth)")
    if not node.execute_path(return_path, speed=0.6):
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

    # Hand the box from the gripper to the magazine: detach from the gripper, then
    # fix it to the AGV so it rides along when the base drives (a loose box slides
    # in the pocket even at low speed -- friction can't beat the planar_move step).
    node.detach_box()
    if not node.attach_box_to_magazine():
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

    if not shelf_pick_to_hub(node, box_world, box_model, place_ref):
        return False
    return pocket_place_from_hub(node, pocket_y, place_ref, place_jaw_x,
                                 label=box_model)


def main(args=None):
    rclpy.init(args=args)
    node = HubPickPlace()
    node.setup_planner()

    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2)  # wait for joint states

    if not node.init_hub(pose_at(pocket_center_xyz(PLACE_ORDER_Y[0]), place_quat()),
                         HUB_TCP, GRASP_LATERAL_M):
        node.get_logger().error("Hub bring-up failed; adjust HUB_TCP and retry")
        node.destroy_node()
        rclpy.shutdown()
        return

    if not node.go_to_hub():
        node.get_logger().error("Could not reach the hub from the spawn pose")
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
