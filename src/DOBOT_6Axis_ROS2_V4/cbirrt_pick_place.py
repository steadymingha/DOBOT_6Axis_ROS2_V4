"""Segmented shelf-to-base pick-and-place DEMO (pre-hub version).

The motion is split into path-planning segments, each using the planner best
suited to it; the grasp orientation (gripper straight down) is held after the
grasp. This is the original single-cycle demo; the production sequence is the
hub-and-spoke version in shelf_pick_place.py. Both now share the reusable motion
node and constants in cr7_pnp -- this file is kept only as a runnable reference
of the segmented (non-hub) flow.

Segments: RRT approach -> insert -> J6 twist -> jaw-align -> descend -> grip ->
ascend -> retreat -> constrained carry -> place-descend -> release -> retreat.

Run (sim already up):
    source /opt/ros/humble/setup.bash
    source ~/dobot_ws/install/setup.bash
    cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
    .venv/bin/python3 cbirrt_pick_place.py
"""

import math
import time
import threading

import numpy as np
import rclpy
from rclpy.executors import MultiThreadedExecutor

from cr7_pnp import (
    CBiRRTPickPlace, pose_at, quat_mul, quat_about_z, wait_for_spacebar,
    DOWN, GRASP_YAW_OFFSET, PLACE_YAW, GRIPPER_YAW_TWIST,
    GRIPPER_OPEN, GRIPPER_CLOSE,
    INSERT_TCP_ABOVE, GRASP_TCP_ABOVE, PREGRASP_BACK,
    POCKET_X, POCKET_SURFACE_Z, POCKET_HOVER, PLACE_TCP_ABOVE,
    JAW_FIXED_PAD_X, FIXED_PAD_CLEARANCE, BOX_SHORT,
    SHELF_BOX_WORLD, SHELF_BOX_MODEL, SHELF_BOX_LINK, POCKET_Y,
)


def shelf_to_base_cycle(node, box_world, pocket_y):
    """Run one shelf-to-base pick-and-place. Returns True on success, False (with
    a clear log) on any IK/plan/servo failure so the caller can ask the user to
    reposition the AGV and trigger again. Steps follow the agreed sequence."""
    # Resolve the box pose and the shelf axes in base_link via live TF.
    box_ps = node.transform_world_pose(*box_world, DOWN)
    insert_dir = node.transform_world_vector([0.0, 1.0, 0.0])   # world +y = into shelf
    row_dir = node.transform_world_vector([1.0, 0.0, 0.0])      # world +x = magazine row
    if box_ps is None or insert_dir is None or row_dir is None:
        node.get_logger().error("[cycle] TF unavailable; reposition and retry")
        return False
    insert_dir = insert_dir / (np.linalg.norm(insert_dir) or 1.0)
    box = np.array([box_ps.pose.position.x, box_ps.pose.position.y,
                    box_ps.pose.position.z])

    # Position the shelf in the collision model at the AGV's current pose so the
    # RRT routes around it (the AGV is stationary for the rest of the cycle).
    node.update_shelf_collision()

    # Grasp orientation: down + yaw that aligns the jaw to the magazine row.
    phi = math.atan2(row_dir[1], row_dir[0]) + GRASP_YAW_OFFSET
    grasp_quat = quat_mul(quat_about_z(phi), DOWN)
    place_quat = quat_mul(quat_about_z(PLACE_YAW), DOWN)

    pregrasp_xyz = box - insert_dir * PREGRASP_BACK + np.array([0, 0, INSERT_TCP_ABOVE])
    descend_dist = INSERT_TCP_ABOVE - GRASP_TCP_ABOVE   # gap height -> grasp height
    pocket_hover_xyz = np.array([POCKET_X, pocket_y - 0.05, POCKET_SURFACE_Z + POCKET_HOVER])

    node.get_logger().info(
        f"[cycle] box(base_link)=({box[0]:+.3f},{box[1]:+.3f},{box[2]:+.3f}) "
        f"horiz_dist={math.hypot(box[0], box[1]):.3f} m | "
        f"pre-grasp=({pregrasp_xyz[0]:+.3f},{pregrasp_xyz[1]:+.3f},{pregrasp_xyz[2]:+.3f}) | "
        f"insert_dir=({insert_dir[0]:+.2f},{insert_dir[1]:+.2f},{insert_dir[2]:+.2f}) "
        f"row_dir=({row_dir[0]:+.2f},{row_dir[1]:+.2f},{row_dir[2]:+.2f}) yaw={math.degrees(phi):.0f}deg")

    # Pocket place config, computed UP FRONT so the pick can grasp in the same
    # elbow/wrist family (high shelf elbow-up vs low pocket elbow-down are
    # opposite branches; grasping in the pocket's branch keeps the constrained
    # carry within one family).
    place_ref = node.compute_ik_ordered(pose_at(pocket_hover_xyz, place_quat))
    if place_ref is None:
        node.get_logger().error("[cycle] pocket place IK failed (pre-check)"); return False
    node.get_logger().info(
        f"[cycle] pocket place branch: J3={math.degrees(place_ref[2]):+.0f} "
        f"J5={math.degrees(place_ref[4]):+.0f} deg (pick will match it)")

    # 1. RRT to the pre-grasp pose in front of the shelf box, in the pocket's
    # elbow/wrist family.
    print("\n===== [1/10] RRT -> pre-grasp in front of shelf =====")
    node.control_gripper(GRIPPER_OPEN)
    if not node.move_to_pose_ref(pose_at(pregrasp_xyz, grasp_quat), place_ref):
        node.get_logger().error("[cycle] step 1 pre-grasp failed"); return False

    # 2. Linear insert into the gap (over the box), fixed jaw entering the gap.
    print("===== [2/10] Linear insert into shelf gap =====")
    if not node.linear_servo(insert_dir * PREGRASP_BACK, label="insert"):
        node.get_logger().error("[cycle] step 2 insert failed"); return False

    # 2b. Twist the gripper yaw via J6 (no IK) now that the jaw is in the gap.
    print("===== [2b/10] Twist gripper yaw (J6) =====")
    if not node.rotate_j6(GRIPPER_YAW_TWIST, label="yaw-twist"):
        node.get_logger().error("[cycle] step 2b yaw twist failed"); return False

    # 2c. Jaw-align: the grasp centre between the pads is offset toward the fixed
    # jaw (+gripper X) from the flange axis, so shift the flange AWAY from the
    # fixed-jaw side until the fixed pad is FIXED_PAD_CLEARANCE from the box face.
    print("===== [2c/10] Jaw-align (fixed pad to box face) =====")
    time.sleep(0.3)   # let TF catch up with the finished J6 move
    jaw_x = node.gripper_x_in_base()
    if jaw_x is None:
        node.get_logger().error("[cycle] step 2c jaw axis unavailable"); return False
    lateral = JAW_FIXED_PAD_X - FIXED_PAD_CLEARANCE - BOX_SHORT / 2.0   # ~46 mm
    if not node.linear_servo(-lateral * jaw_x, label="jaw-align"):
        node.get_logger().error("[cycle] step 2c jaw align failed"); return False

    # 3. Linear descend onto the box.
    print("===== [3/10] Linear descend onto box =====")
    if not node.linear_servo([0.0, 0.0, -descend_dist + 0.01], label="descend"):
        node.get_logger().error("[cycle] step 3 descend failed"); return False

    # 4. Close gripper + attach. The attach result is the load-bearing part.
    print("===== [4/10] Grip + attach =====")
    node.control_gripper(GRIPPER_CLOSE)
    node.object_model, node.object_link = SHELF_BOX_MODEL, SHELF_BOX_LINK
    if not node.attach_box():
        node.get_logger().error(
            "[cycle] step 4 ATTACHLINK failed (check model/link names and the "
            "link-attacher plugin); releasing and aborting")
        node.control_gripper(GRIPPER_OPEN)
        return False
    time.sleep(0.5)

    # 5. Linear ascend back to gap height.
    print("===== [5/10] Linear ascend =====")
    if not node.linear_servo([0.0, 0.0, descend_dist], label="ascend"):
        node.get_logger().error("[cycle] step 5 ascend failed"); return False

    # 6. Linear retreat out of the shelf.
    print("===== [6/10] Linear retreat out of shelf =====")
    if not node.linear_servo(-insert_dir * PREGRASP_BACK, label="retreat"):
        node.get_logger().error("[cycle] step 6 retreat failed"); return False

    # 7. Carry to hover above the base pocket, gripper held DOWN the whole way.
    print("===== [7/10] Carry (gripper held down) -> hover above pocket =====")
    hover_pose = pose_at(pocket_hover_xyz, place_quat)
    if not node.move_constrained(hover_pose):
        node.get_logger().error("[cycle] step 7 carry to pocket failed"); return False

    # 8. Linear descend toward the pocket surface.
    print("===== [8/10] Linear descend into pocket =====")
    if not node.linear_servo([0.0, 0.0, PLACE_TCP_ABOVE - POCKET_HOVER + 0.01],
                             label="place-descend"):
        node.get_logger().error("[cycle] step 8 place descend failed"); return False

    # 9. Open gripper + detach.
    print("===== [9/10] Release + detach =====")
    node.detach_box()
    node.control_gripper(GRIPPER_OPEN)
    time.sleep(0.5)

    # 10. Retreat up a little (no fold -- folding the arm destabilised the AGV).
    print("===== [10/10] Retreat =====")
    node.linear_servo([0.0, 0.0, 0.10], label="up")
    print("========== Cycle done ==========\n")
    return True


def main(args=None):
    rclpy.init(args=args)
    node = CBiRRTPickPlace()
    node.setup_planner()

    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2)  # wait for joint states

    print("\n" + "=" * 60)
    print(" Shelf-to-base pick & place ready.")
    print(" Drive the AGV near the shelf, then press SPACE to run one cycle.")
    print(" (q / Esc / Ctrl-C to quit)")
    print("=" * 60)

    try:
        while rclpy.ok():
            if wait_for_spacebar() == 'quit':
                break
            ok = shelf_to_base_cycle(node, SHELF_BOX_WORLD, POCKET_Y[0])
            if ok:
                print("\n>>> Cycle SUCCEEDED. Press SPACE for another, or q to quit.")
            else:
                print("\n>>> Cycle FAILED (no IK at this AGV position).")
                print("    Park so the AGV's rear faces the shelf, then drive until the")
                print("    logged box(base_link) is roughly (+0.40~0.50, ~0.00). Then SPACE.")
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
