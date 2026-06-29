#!/usr/bin/env python3
"""Deployment-orientation optimizer for the shelf->pocket pick&place round trip.

Why this exists
---------------
The carry from the shelf grasp to a magazine pocket kept stalling because the two
configurations sit ~5 rad apart in joint space: the arm is mounted yaw=pi on the
AMR, so the pockets sit at the arm FRONT (base +x) while the shelf is reached to
the SIDE, forcing a big J1 swing (and an elbow/wrist branch flip) that CBiRRT
cannot bridge under the orientation constraint. The voxel reachability map proved
that exact "straight-down" grasps are rare (~3.7% of the reachable workspace), so
forcing a down grasp everywhere is part of the problem.

This tool does NOT plan a path. It answers the deployment question the reach map
cannot: *how should the robot be oriented relative to the shelf, and with which
pick approach, so the round trip is a SHORT joint-space move instead of a 5 rad
reconfiguration?* It sweeps the genuinely-free install variables and, for each,
solves real IK at every waypoint (shelf grasp + the 4 pockets), measures the
joint-space gap of the round trip, and gates on collision + reachability.

Free variables swept
  * psi  -- the shelf azimuth in base_link (= how the AMR is parked relative to
            the shelf). Pockets are bolted to the AMR so they stay fixed in
            base_link; only the shelf azimuth moves. This is the lever that sets
            the relative pick<->place azimuth gap (the J1 swing).
  * pick approach -- DOWN (tool -Z, grasp the box from above) vs SIDE (tool
            horizontal, reach into the shelf). Place is always DOWN: a magazine
            pocket is a top-down insert.
  * (optional) global yaw -- rotates shelf AND pockets together, sliding the pair
            within the J1 limit window (the arm-mount-yaw lever). Off by default.

Objective (lower is better)
  round_cost(setup) = max over pockets of || q_pocket - q_shelf ||  (rad),
  where q_shelf is the collision-free shelf-grasp IK nearest the standby pose and
  q_pocket is the collision-free pocket IK nearest q_shelf (the branch the carry
  would actually connect to). A setup is INFEASIBLE if any waypoint has no
  collision-free IK. We also report the J1-only swing, the worst single gap, and
  whether a likely branch flip (any joint gap > ~pi) is present.

Run (uses the workspace venv that has pinocchio):
  /home/user/dobot_ws/.venv/bin/python deploy_optimizer.py
"""

import os
import csv
import math
import argparse
import datetime
import numpy as np

from reachability_map import ReachabilityModel, _rotation_from_axis, TCP_OFFSET_M


# Task geometry (base_link metres) -- matches cbirrt_pick_place.py ---------------
POCKET_X = 0.3705
POCKET_Y = [0.177, 0.059, -0.059, -0.177]
POCKET_Z = -0.05                       # pocket grasp/place TCP height

# Shelf grasp point in POLAR base_link form: the radius/height are physical (the
# shelf stands ~0.55 m out and the box centre is ~0.28 m above the arm base:
# box world z=0.97 - base world z=0.69). Only the azimuth (psi) is the free
# parking variable. Radius/height are sweepable to probe parking distance.
SHELF_R = 0.55
SHELF_Z = 0.28

# Standby pose (deg) the shelf grasp is measured from -- the arm's rest config.
STANDBY_DEG = [-8, -39, -105, 0, 0, 0]

# Loosened limits (deg). Policy: loosen until the sequence is feasible as long as
# collision is clear; J1 is opened widest since the round trip is a J1 swing.
DEFAULT_LIMITS_DEG = [
    (-180.0, 180.0),   # J1 (widest: the carry is a base swing)
    (-70.0, 60.0),     # J2
    (-180.0, 180.0),   # J3
    (0.0, 120.0),      # J4
    (-120.0, 120.0),   # J5
    (-180.0, 180.0),   # J6
]


def build_model(limits_deg):
    rm = ReachabilityModel()
    rm.set_joint_limits([(math.radians(a), math.radians(b)) for a, b in limits_deg])
    return rm


def arm_of(rm, qfull):
    return np.array([float(qfull[i]) for i in rm.q_index])


def wrap_R(heading):
    """Link6 rotation for the 90deg-REMOUNTED gripper doing a top-down wrap:
    the finger axis (descent) points world -Z, the flange normal (Link6 z) is
    HORIZONTAL with azimuth `heading`, and the pad-close axis is horizontal too.
    Columns = (pad-close, finger=-Z, flange-normal)."""
    c, s = math.cos(heading), math.sin(heading)
    R = np.column_stack(([c, s, 0.0],          # pad-close axis (horizontal)
                         [0.0, 0.0, -1.0],      # finger/descent axis (down)
                         [-s, c, 0.0]))         # flange normal (horizontal)
    return R


# 'pusher' mount bracket: the arm flange bolts BEHIND the moving jaw (pusher),
# so Link6 is offset horizontally back along the pad-close axis (not above), at
# roughly the gripper-body height. Rough bracket numbers, sweepable.
PUSHER_BACK = 0.12     # Link6 behind the grasp along -pad-close (m)
PUSHER_UP = 0.04       # Link6 above the grasp centre (m)


def pusher_R(heading):
    """Link6 rotation for the BEHIND-THE-PUSHER mount: gripper unchanged (finger
    axis = descent = -Z), but the flange bolts behind the moving jaw so Link6 z
    (flange normal) points along the pad-close axis (horizontal, azimuth
    `heading`). Columns = (Link6 x, finger=-Z, flange-normal=pad-close)."""
    c, s = math.cos(heading), math.sin(heading)
    return np.column_stack(([s, -c, 0.0],          # Link6 x (horizontal)
                            [0.0, 0.0, -1.0],       # finger/descent (down)
                            [c, s, 0.0]))           # flange normal = pad-close


def grasp_candidates(approach, grasp_pos, psi, n=12):
    """Yield (link6_pos, R) candidates over this grasp's free wrist DOF.

    DOWN/SIDE keep the unmodified gripper (TCP along Link6 z), sweeping wrist
    roll. WRAP is the 90deg-remounted gripper with Link6 straight ABOVE the
    grasp. PUSHER is the unmodified gripper with the flange bracketed BEHIND the
    pusher, so Link6 is offset horizontally back along the pad-close axis. Both
    WRAP and PUSHER descend (finger = -Z) and sweep the flange heading.
    """
    g = np.asarray(grasp_pos, float)
    if approach == 'wrap':
        for h in np.linspace(0, 2 * math.pi, n, endpoint=False):
            R = wrap_R(h)
            l6 = g - TCP_OFFSET_M * np.array([0.0, 0.0, -1.0])
            yield l6, R
    elif approach == 'pusher':
        for h in np.linspace(0, 2 * math.pi, n, endpoint=False):
            R = pusher_R(h)
            # Link6 sits behind the grasp along -flange-normal (= -pad-close),
            # raised by PUSHER_UP.
            l6 = g - PUSHER_BACK * R[:, 2] + PUSHER_UP * np.array([0.0, 0.0, 1.0])
            yield l6, R
    else:
        zaxis = (np.array([0.0, 0.0, -1.0]) if approach == 'down'
                 else np.array([math.cos(psi), math.sin(psi), 0.0]))
        for roll in np.linspace(0, 2 * math.pi, n, endpoint=False):
            R = _rotation_from_axis(zaxis, roll)
            l6 = g - TCP_OFFSET_M * R[:, 2]
            yield l6, R


def ik_nearest(rm, approach, grasp_pos, psi, ref_arm, rng,
               n=12, n_restart=24):
    """Best collision-free IK for `approach` at `grasp_pos`, nearest to ref_arm.
    Sweeps the grasp's free wrist DOF and random restarts; seeds each with
    ref_arm so a connected (short-motion) branch is found when one exists."""
    best, best_d = None, None
    ref_full = rm.pin_q(ref_arm)
    for l6, R in grasp_candidates(approach, grasp_pos, psi, n=n):
        # Pre-add the offset so the model's link6_target_from_tcp() returns l6.
        tcp_arg = l6 + TCP_OFFSET_M * R[:, 2]
        seeds = [ref_full] + [rm.random_config(rng) for _ in range(n_restart)]
        q = rm.inverse_kinematics(tcp_arg, R, seeds)
        if q is None:
            continue
        arm = arm_of(rm, q)
        d = float(np.linalg.norm(arm - ref_arm))
        if best is None or d < best_d:
            best, best_d = arm, d
    return best, best_d


def shelf_pos(psi, r=SHELF_R, z=SHELF_Z):
    return np.array([r * math.cos(psi), r * math.sin(psi), z])


def evaluate(rm, psi, pick_approach, standby, rng, r=SHELF_R, z=SHELF_Z,
             place_approach='down'):
    """One setup -> dict of metrics, or None entries where IK fails.

    pick_approach is the shelf grasp (down/side/wrap); place_approach is the
    pocket insert. The gripper is one piece, so the 90deg-remount scenario uses
    wrap for BOTH (pass place_approach='wrap')."""
    p_shelf = shelf_pos(psi, r, z)
    q_shelf, _ = ik_nearest(rm, pick_approach, p_shelf, psi, standby, rng)
    res = {'psi_deg': math.degrees(psi), 'approach': pick_approach,
           'shelf_ok': q_shelf is not None}
    if q_shelf is None:
        res.update(round_cost=None, j1_swing=None, worst_gap=None,
                   flip=None, n_pockets_ok=0)
        return res

    gaps, j1s, worst = [], [], []
    n_ok = 0
    for py in POCKET_Y:
        ppos = np.array([POCKET_X, py, POCKET_Z])
        # psi for a wrap place uses the pocket azimuth (only matters for side).
        q_pk, _ = ik_nearest(rm, place_approach, ppos,
                             math.atan2(py, POCKET_X), q_shelf, rng)
        if q_pk is None:
            continue
        n_ok += 1
        d = float(np.linalg.norm(q_pk - q_shelf))
        gaps.append(d)
        j1s.append(abs(q_pk[0] - q_shelf[0]))
        worst.append(float(np.max(np.abs(q_pk - q_shelf))))
    if not gaps:
        res.update(round_cost=None, j1_swing=None, worst_gap=None,
                   flip=None, n_pockets_ok=0)
        return res
    res.update(round_cost=max(gaps),                 # objective
               j1_swing=max(j1s),
               worst_gap=max(worst),
               flip=bool(max(worst) > math.pi),       # >180deg on a joint -> flip
               n_pockets_ok=n_ok)
    return res


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--psi-step-deg', type=float, default=15.0)
    ap.add_argument('--psi-min-deg', type=float, default=-180.0)
    ap.add_argument('--psi-max-deg', type=float, default=180.0)
    ap.add_argument('--shelf-r', type=float, default=SHELF_R)
    ap.add_argument('--shelf-z', type=float, default=SHELF_Z)
    ap.add_argument('--approaches', default='down,side')
    ap.add_argument('--seed', type=int, default=1)
    ap.add_argument('--out-dir', default=os.path.join(
        os.path.dirname(os.path.abspath(__file__)), 'reachability_out'))
    args = ap.parse_args(argv)

    rm = build_model(DEFAULT_LIMITS_DEG)
    standby = np.array([math.radians(d) for d in STANDBY_DEG])
    rng = np.random.default_rng(args.seed)

    psis = np.arange(args.psi_min_deg, args.psi_max_deg + 1e-6, args.psi_step_deg)
    approaches = [a.strip() for a in args.approaches.split(',') if a.strip()]

    print(f"[deploy] limits(deg): " + ", ".join(
        f"J{i+1}[{a:.0f},{b:.0f}]" for i, (a, b) in enumerate(DEFAULT_LIMITS_DEG)))
    print(f"[deploy] shelf r={args.shelf_r:.2f} z={args.shelf_z:.2f}; "
          f"psi {args.psi_min_deg:.0f}..{args.psi_max_deg:.0f} step "
          f"{args.psi_step_deg:.0f} deg; approaches {approaches}")
    print(f"[deploy] pockets fixed at base_link az "
          f"{', '.join(f'{math.degrees(math.atan2(y, POCKET_X)):+.0f}' for y in POCKET_Y)} deg\n")

    rows = []
    for approach in approaches:
        # The gripper is one piece: a wrap/pusher PICK means the same PLACE (the
        # mount applies to both). down/side pick keep the top-down pocket place.
        place = approach if approach in ('wrap', 'pusher') else 'down'
        for psi in psis:
            r = evaluate(rm, math.radians(psi), approach, standby, rng,
                         r=args.shelf_r, z=args.shelf_z, place_approach=place)
            rows.append(r)

    # Ranked feasible setups by round_cost.
    feas = [r for r in rows if r['round_cost'] is not None
            and r['n_pockets_ok'] == len(POCKET_Y)]
    feas.sort(key=lambda r: r['round_cost'])

    print("=== feasible setups (all 4 pockets reachable+collision-free), "
          "best round trip first ===")
    print(f"{'approach':8} {'psi':>6} {'round_cost':>11} {'J1_swing':>9} "
          f"{'worst_gap':>10} {'flip':>5}")
    for r in feas[:12]:
        print(f"{r['approach']:8} {r['psi_deg']:+6.0f} "
              f"{math.degrees(r['round_cost']):8.0f}deg "
              f"{math.degrees(r['j1_swing']):6.0f}deg "
              f"{math.degrees(r['worst_gap']):7.0f}deg "
              f"{'YES' if r['flip'] else 'no':>5}")
    if not feas:
        print("  (none fully feasible -- loosen limits or change shelf r/z)")

    # Per-approach best, for the down-vs-side comparison (Q5).
    print("\n=== best per approach (Q5: down vs side) ===")
    for approach in approaches:
        fa = [r for r in feas if r['approach'] == approach]
        if fa:
            b = fa[0]
            print(f"  {approach:5}: best psi {b['psi_deg']:+.0f} deg -> "
                  f"round trip {math.degrees(b['round_cost']):.0f} deg "
                  f"(J1 {math.degrees(b['j1_swing']):.0f}, "
                  f"flip {'YES' if b['flip'] else 'no'})")
        else:
            print(f"  {approach:5}: no fully-feasible setup")

    os.makedirs(args.out_dir, exist_ok=True)
    ts = datetime.datetime.now().strftime('%Y%m%d_%H%M%S')
    path = os.path.join(args.out_dir, f'deploy_{ts}.csv')
    with open(path, 'w', newline='') as f:
        w = csv.writer(f)
        w.writerow(['approach', 'psi_deg', 'shelf_ok', 'n_pockets_ok',
                    'round_cost_deg', 'j1_swing_deg', 'worst_gap_deg', 'flip'])
        for r in rows:
            w.writerow([r['approach'], f"{r['psi_deg']:.1f}", int(r['shelf_ok']),
                        r['n_pockets_ok'],
                        '' if r['round_cost'] is None else f"{math.degrees(r['round_cost']):.1f}",
                        '' if r['j1_swing'] is None else f"{math.degrees(r['j1_swing']):.1f}",
                        '' if r['worst_gap'] is None else f"{math.degrees(r['worst_gap']):.1f}",
                        '' if r['flip'] is None else int(r['flip'])])
    print(f"\n[deploy] saved {path}")
    return rows


if __name__ == '__main__':
    main()
