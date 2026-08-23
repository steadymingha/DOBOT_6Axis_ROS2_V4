#!/usr/bin/env python3
"""AGV park sweep for the low-board (0.72 m) pick: for each candidate park y,
teleport the AGV and pre-flight ONLY the centred pre-grasp IK (the exact check
that failed at the default park). No arm motion. Prints PASS/FAIL per park."""
import math
import os
import sys
import threading
import time

import numpy as np
import rclpy
from gazebo_msgs.srv import SetEntityState
from rclpy.executors import MultiThreadedExecutor

sys.path.insert(0, os.path.expanduser("~/dobot_ws/diffusionpolicy"))
import isaac_collect as ic  # noqa: E402
spp = ic.spp
from cr7_pnp import geometry as G  # noqa: E402

LOW_BOARD_TOP = 0.72
IDX = 0                      # inner-most box (station anchor x = default park)
PARK_YS = (0.008, -0.05, -0.10, -0.15, -0.20, -0.25, -0.30)


def pregrasp_ik(node, idx, place_ref):
    """Replicates shelf_pick_to_hub's centred pre-grasp construction (lines
    178-197) and returns the ik_nearest result for it."""
    box_world, _ = spp.shelf_box(node, idx)
    box_ps = node.transform_world_pose(*box_world, spp.DOWN)
    insert_dir = node.transform_world_vector([0.0, 1.0, 0.0])
    row_dir = node.transform_world_vector([1.0, 0.0, 0.0])
    if box_ps is None or insert_dir is None or row_dir is None:
        return None, "TF unavailable"
    insert_dir = insert_dir / (np.linalg.norm(insert_dir) or 1.0)
    box = np.array([box_ps.pose.position.x, box_ps.pose.position.y,
                    box_ps.pose.position.z])
    node.update_shelf_collision(spp.shelf_pose_of(node))
    phi = math.atan2(row_dir[1], row_dir[0]) + spp.GRASP_YAW_OFFSET
    grasp_quat = spp.quat_mul(spp.quat_about_z(phi), spp.DOWN)
    xyz = box - insert_dir * spp.PREGRASP_BACK + np.array(
        [0, 0, spp.INSERT_TCP_ABOVE])
    q = node.ik_nearest(spp.pose_at(xyz, grasp_quat), place_ref)
    return q, None


def main():
    rclpy.init()
    node = spp.HubPickPlace()
    node.setup_planner()
    node.shelf_pose = None
    spp.pockets.subscribe_models(node)
    node._set_state_cli = node.create_client(SetEntityState,
                                             "/gazebo/set_entity_state")
    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2.0)
    if not node._set_state_cli.wait_for_service(timeout_sec=10.0):
        print("no set_entity_state service")
        return 1

    G.SHELF_TIER_TOPS[1] = LOW_BOARD_TOP
    if not ic.reset_boxes(node):
        print("box reset failed")
        return 1
    if not spp.bringup(node):
        return 1

    anchor_x = float(np.mean([G.shelf_box_center(spp.TIER, i)[0]
                              for i in (0, 1, 3, 5)])) + ic.AGV_X_OFFSET
    for y in PARK_YS:
        if not ic.move_agv(node, anchor_x, y):
            print(f"SWEEP y={y:+.3f}: AGV teleport failed")
            continue
        place_ref = spp.compute_place_ref(node, spp.PLACE_ORDER_Y[3])
        if place_ref is None:
            print(f"SWEEP y={y:+.3f}: pocket seed IK failed (pocket side)")
            continue
        q, err = pregrasp_ik(node, IDX, place_ref)
        print(f"SWEEP y={y:+.3f}: "
              f"{'PASS' if q is not None else 'FAIL (' + (err or 'pregrasp IK') + ')'}")

    G.SHELF_TIER_TOPS[1] = 1.22
    ic.reset_boxes(node)
    node.update_shelf_collision()
    node.destroy_node()
    rclpy.shutdown()
    return 0


if __name__ == "__main__":
    sys.exit(main())
