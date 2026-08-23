#!/usr/bin/env python3
"""Low-board reach test: can the arm pick from the LOWEST shelf board (0.72 m)?

Tier-1 picks are verified at the 1.22 m board. This test moves the tier-1 pick
geometry down to the 0.72 m board (SHELF_TIER_TOPS[1] is read at call time by
every derived function: pick targets, stock phantoms, servo heights), teleports
the tier-1 boxes there, and runs the verified pick pipeline for a few boxes.
QC snapshots (carry / place) go next to this script. Restores the normal
layout at the end.
"""
import os
import sys
import threading
import time

import numpy as np
import rclpy
from gazebo_msgs.srv import SetEntityState
from rclpy.executors import MultiThreadedExecutor

sys.path.insert(0, os.path.expanduser("~/dobot_ws/diffusionpolicy"))
import isaac_collect as ic  # noqa: E402  (also inserts the sequences path)
spp = ic.spp
from cr7_pnp import geometry as G  # noqa: E402

LOW_BOARD_TOP = 0.72          # lowest board top (world z); tier-1 default 1.22
TEST_IDXS = (0, 8)            # inner-most box + outer-most station box
# Default park (y=0.008) fails at the low board: every pre-grasp IK branch
# folds the arm over the AGV deck (hits cube/AGV). Sweep 2026-07-27: y<=-0.05
# all pass; -0.15 sits mid-band with margin both ways.
AGV_Y_LOW = -0.15
OUT = os.path.dirname(os.path.abspath(__file__))


def station_anchor_x(idx):
    by_x = sorted(range(spp.N_BOXES), key=lambda i: G.SHELF_BOX_XS[i])
    stations = [by_x[k:k + ic.N_POCKETS]
                for k in range(0, spp.N_BOXES, ic.N_POCKETS)]
    st = next(s for s in stations if idx in s)
    return float(np.mean([G.shelf_box_center(spp.TIER, i)[0]
                          for i in st])) + ic.AGV_X_OFFSET


def main():
    rclpy.init()
    node = spp.HubPickPlace()
    node.setup_planner()
    node.shelf_pose = None                       # layout default (no vision)
    spp.pockets.subscribe_models(node)
    node._set_state_cli = node.create_client(SetEntityState,
                                             "/gazebo/set_entity_state")
    rec = ic.Recorder(node)                      # for full-res QC snapshots
    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2.0)
    if not node._set_state_cli.wait_for_service(timeout_sec=10.0):
        print("no /gazebo/set_entity_state -- is isaac_sim.py running?")
        return 1

    # Move the whole tier-1 pick geometry to the low board, then make the
    # physical boxes match (phantoms follow via shelf_box_center at call time).
    G.SHELF_TIER_TOPS[1] = LOW_BOARD_TOP
    if not ic.reset_boxes(node):
        print("box reset failed")
        return 1
    if not spp.bringup(node):
        return 1

    results = {}
    placed = 0
    for idx in TEST_IDXS:
        box_model = spp.shelf_box_model(spp.TIER, idx)
        pocket = ic.N_POCKETS - 1 - placed
        if not ic.move_agv(node, station_anchor_x(idx), AGV_Y_LOW):
            results[box_model] = "AGV park failed"
            continue
        print(f"[test] {box_model} at z={G.shelf_box_center(spp.TIER, idx)[2]:.3f}"
              f" -> pocket {pocket}")
        ok = ic.pick_place(node, rec, idx, pocket,
                           os.path.join(OUT, f"low_{box_model}"))
        if ok and ic.box_in_pocket(node, box_model, spp.PLACE_ORDER_Y[pocket]):
            results[box_model] = "OK (picked from 0.72 board, seated in pocket)"
            placed += 1
        else:
            results[box_model] = "FAILED"
            if not ic.recover(node):
                break

    # Restore the normal layout (boxes back to the 1.22 board, phantoms follow).
    G.SHELF_TIER_TOPS[1] = 1.22
    ic.reset_boxes(node)
    node.update_shelf_collision()

    print("=" * 50)
    for m, r in results.items():
        print(f"RESULT {m}: {r}")
    node.destroy_node()
    rclpy.shutdown()
    return 0 if all(r.startswith("OK") for r in results.values()) else 2


if __name__ == "__main__":
    sys.exit(main())
