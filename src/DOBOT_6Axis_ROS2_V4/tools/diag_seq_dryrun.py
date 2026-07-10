#!/usr/bin/env python3
"""Offline dry-run of the three wirebonder transfers -- NO arm motion.

Reproduces every Cartesian/joint leg of sequences 1/2/3 with the same solvers
the live run uses (cbirrt.linear_path / is_state_valid), chained from the hub
config, and reports per-leg feasibility + colliding pairs. Purpose:

  a. reproduce the seq-1 "place wb1:A insert stopped at 48 mm" collision
     offline and see WHERE it diverges from preflight_place's dry pass;
  b. answer whether a straight JOINT interpolation back to the hub after the
     seq-2 top place is collision-free (candidate replacement for the free
     go_to_hub RRT that swings into the shelf);
  c. prototype the full-transfer preflight (every leg checked before moving).

Run with the sim up (uses live TF for odom->base_link; does not command the arm):
    /usr/bin/python3 tools/diag_seq_dryrun.py
"""

import math
import os
import sys
import threading
import time

import numpy as np
import rclpy
from rclpy.executors import MultiThreadedExecutor

_PKG = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, _PKG)
sys.path.insert(0, os.path.join(_PKG, 'sequences'))
sys.path.insert(0, os.path.join(_PKG, 'comms'))
from cr7_pnp import (  # noqa: E402
    HubPickPlace, pose_at, quat_mul, quat_about_z, DOWN,
    GRASP_TCP_ABOVE, GRASP_LATERAL_M,
)
import wirebonder_pick_place as wb  # noqa: E402

HOVER = wb.HOVER_ABOVE


def fk_pos(node, q):
    return node.ik_model.fk_tcp(node.ik_model.pin_q(list(q)))[0]


def leg_delta(node, q, delta, label):
    """Dry Cartesian servo by a base-frame delta from config q. Returns end
    config or None; prints reach + colliding pairs on failure."""
    path, reached, reason = node.cbirrt.linear_path(
        list(q), list(delta), node.is_state_valid, node.joint_limits)
    want = float(np.linalg.norm(delta))
    if reached < want - 1e-3:
        bad = getattr(node.cbirrt, 'last_invalid_q', None)
        pairs = node.collision.colliding_pairs(bad) if (
            reason == 'collision' and bad is not None) else '-'
        print(f"    FAIL {label}: {reached*1000:.0f}/{want*1000:.0f} mm "
              f"({reason})  pairs={pairs}")
        return None
    print(f"    ok   {label}: {reached*1000:.0f} mm")
    return path[-1]


def leg_to_pose(node, q, pose, label):
    """Dry version of servo_to: straight servo from q's TCP to pose's position."""
    p = pose.pose.position
    return leg_delta(node, q, np.array([p.x, p.y, p.z]) - fk_pos(node, q), label)


def joint_interp_valid(node, q0, q1, n=48, label="joint"):
    """Validity-sweep a straight joint interpolation q0->q1 (what a checked
    joint_move would execute). Prints the first colliding config if any."""
    q0, q1 = np.array(q0, float), np.array(q1, float)
    for t in np.linspace(0.0, 1.0, n):
        q = list(q0 + (q1 - q0) * t)
        if not node.is_state_valid(q):
            print(f"    FAIL {label}: collides at t={t:.2f}  "
                  f"pairs={node.collision.colliding_pairs(q)}")
            return False
    print(f"    ok   {label}: straight joint path collision-free ({n} checks)")
    return True


def dq_str(qa, qb):
    d = np.array(qb, float) - np.array(qa, float)
    return '[' + ', '.join(f'{v:+.3f}' for v in d) + ']'


def seq1(node):
    print("\n=== seq 1: base -> slot A (front place) ===")
    src, dst = wb.SEQUENCES['1']
    node.update_wirebonder_collision(wb.DEVICES[dst.ref[0]])
    approach = wb.slot_target(node, dst, 'approach')
    seat = wb.slot_target(node, dst, 'seat')
    if approach is None or seat is None:
        return

    print("  [A] preflight_place replica (from hub, as today):")
    node.attach_box_collision()
    qa1 = leg_to_pose(node, node.hub_q, approach, "hub->approach (box)")
    node.detach_box_collision()
    qa2 = leg_to_pose(node, qa1, seat, "approach->seat") if qa1 is not None else None

    print("  [B] full live chain (pick first, then place):")
    d_hov = wb.base_hover_delta(src)
    q = leg_delta(node, node.hub_q, d_hov, "pick approach")
    if q is not None:
        q = leg_delta(node, q, [0, 0, -HOVER + 0.01], "pick descend")
    if q is not None:
        q = leg_delta(node, q, [0, 0, HOVER], "pick ascend")
    if q is not None:
        q = leg_delta(node, q, -d_hov, "pick retract")
    if q is not None:
        print(f"    post-pick q vs hub_q: dq={dq_str(node.hub_q, q)}")
        node.attach_box_collision()
        qb1 = leg_to_pose(node, q, approach, "place approach (box)")
        node.detach_box_collision()
        if qb1 is not None:
            if qa1 is not None:
                print(f"    live approach-end vs dry approach-end: dq={dq_str(qa1, qb1)}")
            qb2 = leg_to_pose(node, qb1, seat, "place insert")
            if qb2 is not None:
                leg_delta(node, qb2, [0, 0, -wb.SLOT_PLACE_DROP], "place descend")


def seq2(node):
    print("\n=== seq 2: slot B -> slot C (top->top, direct) ===")
    src, dst = wb.SEQUENCES['2']
    node.update_wirebonder_collision(wb.DEVICES[dst.ref[0]])
    g_src = wb.top_grasp_pose(node, src)
    g_dst = wb.top_grasp_pose(node, dst)
    if g_src is None or g_dst is None:
        return

    def hover(ps):
        p = ps.pose.position
        return pose_at([p.x, p.y, p.z + HOVER],
                       (ps.pose.orientation.x, ps.pose.orientation.y,
                        ps.pose.orientation.z, ps.pose.orientation.w))

    q = leg_to_pose(node, node.hub_q, hover(g_src), "pick hover")
    if q is not None:
        q = leg_delta(node, q, [0, 0, -HOVER], "pick descend")
    if q is not None:
        q = leg_delta(node, q, [0, 0, HOVER], "pick ascend")
    if q is not None:
        node.attach_box_collision()
        q = leg_to_pose(node, q, hover(g_dst), "place approach (box)")
        node.detach_box_collision()
    if q is not None:
        q = leg_delta(node, q, [0, 0, -HOVER], "place descend")
    if q is not None:
        q = leg_delta(node, q, [0, 0, HOVER], "place ascend")
    if q is not None:
        print(f"    post-place q vs hub_q: dq={dq_str(node.hub_q, q)}")
        print("  [return candidates]")
        joint_interp_valid(node, q, node.hub_q, label="straight joint_move to hub")


def seq3(node):
    print("\n=== seq 3: slot D (staged front pick) -> base pocket ===")
    src, dst = wb.SEQUENCES['3']
    node.update_wirebonder_collision(wb.DEVICES[src.ref[0]])
    approach = wb.slot_target(node, src, 'approach')
    seat = wb.slot_target(node, src, 'seat')
    if approach is None or seat is None:
        return
    stage = wb.STAGE_JOINTS[src.ref[1]]

    q_sw = list(np.array(node.hub_q, float))
    q_sw[0] = stage[0]
    if not joint_interp_valid(node, node.hub_q, q_sw, label="J1 swing"):
        return
    # jaw squaring is a small J6 twist; skip the exact err (TF-dependent) and
    # check the servo legs from the swung config directly.
    q = leg_to_pose(node, q_sw, approach, "pick approach")
    if q is not None:
        q = leg_to_pose(node, q, seat, "pick seat")
    if q is not None:
        q = leg_delta(node, q, [0, 0, -wb.SLOT_PICK_DROP], "pick descend")
    # reverse replay retraces the outbound path -> ends near the hub; then base place:
    d_hov = wb.base_hover_delta(dst)
    q2 = leg_delta(node, node.hub_q, d_hov, "place approach (from hub)")
    if q2 is not None:
        leg_delta(node, q2, -d_hov, "place retract")


def main():
    rclpy.init()
    node = HubPickPlace()
    node.setup_planner()
    node.add_wirebonder_meshes(os.path.join(os.path.dirname(_PKG),
                                            'blender', 'wirebonder', 'collision'))
    ex = MultiThreadedExecutor()
    ex.add_node(node)
    threading.Thread(target=ex.spin, daemon=True).start()
    time.sleep(2)

    base = wb.base_loc()
    ref = pose_at([base.ref[0], base.ref[1], base.ref[2] + GRASP_TCP_ABOVE],
                  quat_mul(quat_about_z(base.yaw), DOWN))
    if not node.init_hub(ref, wb.HUB_TCP, GRASP_LATERAL_M):
        sys.exit("hub init failed")
    print(f"hub_q: {[f'{v:+.3f}' for v in node.hub_q]}")
    print(f"DEVICES['wb1'] = {wb.DEVICES['wb1']}  (placeholder anchor; live vision"
          f" read differs by <2 mm)")

    node.detach_box_collision()
    try:
        seq1(node)
        seq2(node)
        seq3(node)
    finally:
        node.detach_box_collision()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
