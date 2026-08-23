#!/usr/bin/env python3
"""Low-board (0.72 m) AGV park calibration -- NO ARM MOTION.

The 2-tier collection parks every station at (cluster centre + AGV_X_OFFSET,
AGV_Y) -- one verified relative geometry reused everywhere. The low board needs
its own verified (x_offset, y): the first low-board run parked at y=-0.15 (only
spot-checked on 2 boxes) and 26/60 picks failed 'approach spoke infeasible'.

This finds the low-board anchor the same way the 2-tier one was found, but
automatically and with zero motion:

  Phase 1  sweep (x_offset, y) on the INNER station's boxes, pre-flight only
           (the exact gates pick_place uses: pocket-side spoke + shelf-side
           shelf_pick_to_hub(preflight_only=True)); score by boxes passing.
  Phase 2  take the best offset, apply it to ALL stations, report per-box
           pass/fail across the whole 10-box row -> does one offset generalise,
           or does the low board need per-station offsets?

Prints a grid; suggests the BOARD_AGV_Y / AGV_X_OFFSET values to bake in.
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
import isaac_collect as ic  # noqa: E402
spp = ic.spp
from cr7_pnp import geometry as G  # noqa: E402

LOW_BOARD_TOP = 0.72
# Candidate anchors to try. x_offset around the 2-tier -0.117; y across the band
# the earlier reach sweep found feasible for the centred pre-grasp (<= -0.05).
X_OFFSETS = [-0.20, -0.155, -0.117, -0.08]
Y_ANCHORS = [-0.08, -0.12, -0.16, -0.20, -0.24]
POCKET = 3            # representative pocket (shelf-side pre-flight is ~pocket-independent)
OUT = os.path.dirname(os.path.abspath(__file__))


def stations():
    by_x = sorted(range(spp.N_BOXES), key=lambda i: G.SHELF_BOX_XS[i])
    return [by_x[k:k + ic.N_POCKETS] for k in range(0, spp.N_BOXES, ic.N_POCKETS)]


def station_center_x(st):
    return float(np.mean([G.shelf_box_center(spp.TIER, i)[0] for i in st]))


def preflight_pick(node, idx, pocket=POCKET):
    """pick_place's feasibility gates with NO motion. True if this box is
    pickable-and-returnable from the current AGV park."""
    box_world, box_model = spp.shelf_box(node, idx)
    pocket_y = spp.PLACE_ORDER_Y[pocket]
    place_ref = spp.compute_place_ref(node, pocket_y)
    if place_ref is None:
        return False, "pocket seed IK"
    place_jaw_x = node.gripper_x_in_base_fk(place_ref)
    if place_jaw_x is None:
        return False, "pocket jaw FK"
    # pocket-side approach spoke (hub -> pocket hover), box phantom attached
    node.attach_box_collision()
    pocket_ok = node.plan_spoke(
        node.hub_q,
        spp.pose_at(spp.pocket_hover_xyz(pocket_y, place_jaw_x), spp.place_quat()),
        place_ref, label="sweep pocket") is not None
    node.detach_box_collision()
    if not pocket_ok:
        return False, "pocket spoke"
    # shelf-side full pre-flight (approach spoke + grasp branch + twisted return)
    ok = spp.shelf_pick_to_hub(node, box_world, box_model, (spp.TIER, idx),
                               place_ref, preflight_only=True)
    return (ok, "" if ok else "shelf preflight")


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
        print("no /gazebo/set_entity_state -- is isaac_sim.py running?")
        return 1

    G.SHELF_TIER_TOPS[1] = LOW_BOARD_TOP
    if not ic.reset_boxes(node):
        print("box reset failed")
        return 1
    if not spp.bringup(node):
        return 1

    sts = stations()
    inner = min(sts, key=lambda s: abs(station_center_x(s) - G.SHELF_WORLD_POSE[0]))
    print(f"stations (pick idx): {sts}")
    print(f"inner station: {inner} (center x={station_center_x(inner):.3f})\n")

    # ---- Phase 1: sweep offsets on the inner station ----
    print("=== Phase 1: (x_offset, y) sweep on inner station, boxes passing/4 ===")
    header = "x_off\\y  " + "".join(f"{y:>8.2f}" for y in Y_ANCHORS)
    print(header)
    scores = {}
    cx = station_center_x(inner)
    for xo in X_OFFSETS:
        row = f"{xo:>6.3f} "
        for y in Y_ANCHORS:
            if not ic.move_agv(node, cx + xo, y):
                row += f"{'park!':>8}"; continue
            npass = sum(preflight_pick(node, i)[0] for i in inner)
            scores[(xo, y)] = npass
            row += f"{npass:>8}"
        print(row)
    best = max(scores, key=lambda k: scores[k])
    print(f"\nbest anchor: x_offset={best[0]:+.3f}, y={best[1]:+.3f} "
          f"({scores[best]}/{len(inner)} inner boxes)\n")

    # ---- Phase 2: apply best offset to every station, whole-row report ----
    print("=== Phase 2: best offset across ALL stations (whole 10-box row) ===")
    xo, y = best
    total = 0
    for st in sts:
        cx = station_center_x(st)
        ic.move_agv(node, cx + xo, y)
        marks = []
        for i in st:
            ok, why = preflight_pick(node, i)
            total += ok
            marks.append(f"{spp.shelf_box_model(spp.TIER, i)}={'OK' if ok else 'X('+why+')'}")
        print(f"station@x{cx:+.2f} (park {cx+xo:+.3f},{y:+.3f}): " + "  ".join(marks))
    print(f"\nwhole row: {total}/{spp.N_BOXES} boxes pass pre-flight at "
          f"x_offset={xo:+.3f}, y={y:+.3f}")
    if total == spp.N_BOXES:
        print(f"-> bake in: BOARD_AGV_Y[0.72]={y:+.3f}"
              + (f" and a low-board AGV_X_OFFSET={xo:+.3f}" if abs(xo - ic.AGV_X_OFFSET) > 1e-3
                 else " (AGV_X_OFFSET unchanged)"))
    else:
        print("-> one offset does NOT cover the row; low board needs per-station "
              "offsets (rerun Phase 1 per station).")

    G.SHELF_TIER_TOPS[1] = 1.22
    ic.reset_boxes(node)
    node.update_shelf_collision()
    node.destroy_node()
    rclpy.shutdown()
    return 0


if __name__ == "__main__":
    sys.exit(main())
