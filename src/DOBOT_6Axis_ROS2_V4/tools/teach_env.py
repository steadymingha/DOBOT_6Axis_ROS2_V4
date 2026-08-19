#!/usr/bin/env python3
"""TEACH the workspace surfaces (table, walls, trolley) into cr7_pnp/env/<DOBOT_ENV>_surfaces.json.

Setup-time, human-run, ONCE per workspace. The APPLY side (main.py registers the
file at start-up) is cr7_pnp/collision_env.py; this file imports from it, never
the reverse. Moved verbatim from test/cbirrt_p1p2_test.py (2026-08-17, plan 4.4a).

    # inside the ros2_dobot container, workspace sourced, .venv python:
    DOBOT_ENV=real python3 tools/teach_env.py --teach-surface z-            # tool ON the table
    DOBOT_ENV=real python3 tools/teach_env.py --teach-surface z- --name low_shelf --bound y -inf -0.34
    DOBOT_ENV=real python3 tools/teach_env.py --set-surface y+ --at 0.55 --name wall_y   # a number, no robot
    DOBOT_ENV=real python3 tools/teach_env.py --show
    DOBOT_ENV=real python3 tools/teach_env.py --forget-surface wall_y
    DOBOT_ENV=real python3 tools/teach_env.py --teach-shelf         # 2 points on the pick tier's front edge + tape numbers

--teach-surface MOVES NOTHING: you jog the tool against the surface first, it
only reads joint states and bisects the collision model. --set-surface / --show /
--forget-surface need no robot at all.
"""
import argparse
import json
import math
import os
import sys
import threading
import time

import numpy as np
import rclpy
from rclpy.executors import MultiThreadedExecutor
from rclpy.signals import SignalHandlerOptions

# tools/ lives one level below the package root; add the root so cr7_pnp imports.
_PKG_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, _PKG_ROOT)
os.environ.setdefault('CR7_REAL_ROBOT', '1')      # real controller: J1/J5/J6 sign flip
os.environ.setdefault('DOBOT_ENV', 'real')        # -> cr7_pnp/env/real_surfaces.json

from cr7_pnp.node import HubPickPlace                                   # noqa: E402
from cr7_pnp.robot_feed import RobotFeed, default_robot_ip             # noqa: E402
from cr7_pnp.collision_env import (                                    # noqa: E402
    SURFACES_FILE, AXES, parse_dir, dir_name, add_surface, move_surface,
    load_surfaces, describe_surface, collision_model_xacro)


def own_like_dir(path):
    """Give a just-written file the owner of its directory.

    This script runs inside the container as root, but the workspace is the
    HOST user's checkout: anything written lands root-owned, and the operator
    then needs sudo to edit or delete their own taught points and measurement
    logs. Matching the directory keeps the repo the user's. No-op when the
    owner already matches or when we lack the privilege.
    """
    try:
        want = os.stat(os.path.dirname(os.path.abspath(path)))
        have = os.stat(path)
        if (have.st_uid, have.st_gid) != (want.st_uid, want.st_gid):
            os.chown(path, want.st_uid, want.st_gid)
        os.chmod(path, 0o644)
    except OSError as e:
        print(f"  (note: could not match ownership on {path}: {e})")


def measure_surface(node, axis, sign, bounds=None, tol=2e-4):
    """Measure where the arm's collision geometry reaches furthest toward `sign`.

    Put the tool against the surface, then run this: it bisects the half-space
    inward from far away, and the deepest position that still leaves the current
    config collision-free has its face exactly on the arm's extreme point --
    which is where you just put it, on the table or against the wall.

    Measuring with the very model that will later enforce the plane is what
    makes this right, and it is why no tool dimension is needed. The modelled
    gripper is not the dummy gripper actually fitted, but both are rigid on
    Link6, so the offset between them cancels -- PROVIDED the wrist attitude
    when measuring matches the attitude when working. Reading the TCP instead
    would be wrong by a whole tool length.

    With `bounds` set, only the part of the arm inside them counts -- which is
    how the lower shelf gets measured without the arm over the base support
    interfering, and vice versa.

    Returns (at, touching_geometry_names) or (None, reason).
    """
    q = node.current_joints.tolist()
    # u grows as the solid region grows toward the arm, whichever way it faces,
    # so one bisection covers all six directions.
    def at_of(u):
        return -sign * u

    idx = add_surface(node, '_probe', axis, sign, at_of(-10.0), bounds)
    if not node.is_state_valid(q):
        return None, "the arm is already in self-collision at this config"
    lo, hi = -3.0, 3.0              # lo: clear of the arm. hi: engulfs the arm.
    move_surface(node, idx, axis, sign, at_of(hi), bounds)
    if node.is_state_valid(q):
        return None, ("bracket failed: the slab never reaches the arm -- with "
                      "bounds, that means no part of the arm is inside them")
    while hi - lo > tol:
        mid = 0.5 * (lo + hi)
        move_surface(node, idx, axis, sign, at_of(mid), bounds)
        if node.is_state_valid(q):
            lo = mid
        else:
            hi = mid
    move_surface(node, idx, axis, sign, at_of(hi), bounds)   # just into contact
    touching = [b if a.startswith('surface_') else a
                for a, b in node.collision.colliding_pairs(q)]
    move_surface(node, idx, axis, sign, at_of(lo), bounds)
    return at_of(lo), touching


def parse_bounds(bound_args):
    """[['y','-0.34','inf'], ...] -> {'y': (-0.34, None)}. inf/none = open."""
    def edge(v):
        s = str(v).strip().lower()
        return None if s in ('inf', '+inf', '-inf', 'none', '*', '') else float(s)
    out = {}
    for ax, lo, hi in bound_args or []:
        ax = ax.strip().lower()
        if ax not in AXES:
            raise ValueError(f"--bound axis must be x, y or z, got {ax!r}")
        out[ax] = (edge(lo), edge(hi))
    return out


def save_surface(name, entry):
    surfaces = load_surfaces()
    surfaces[name] = entry
    with open(SURFACES_FILE, 'w') as f:
        json.dump(surfaces, f, indent=2)
    own_like_dir(SURFACES_FILE)
    print(f"\nsaved '{name}' -> {SURFACES_FILE}")
    print("  Every plan from now on refuses to put any arm link past it.")
    print("  Re-teach p1/p2 if they were taught beyond it.")


def teach_surface(node, a, spec):
    """--teach-surface DIR: record the surface the tool is currently touching."""
    axis, sign = parse_dir(spec)
    name = a.name or dir_name(axis, sign)
    bounds = parse_bounds(a.bound)
    print(f"\nmeasuring '{name}': the surface facing {dir_name(axis, sign)}"
          + (f", bounded to {bounds}" if bounds else "") + " ...")
    at, info = measure_surface(node, axis, sign, bounds)
    if at is None:
        print(f"  measurement failed: {info}")
        return False
    print(f"  the arm reaches {axis} = {at:+.4f} m (base_link) at this pose")
    print(f"  the part that touches first: {', '.join(sorted(set(info)))}")
    if not any('Link6' in n or 'gripper' in n.lower() for n in info):
        # If the tool were really against the surface while some other link
        # measured further, that link would be INSIDE it -- impossible. So this
        # is not a reading of the surface, and saving it would register a plane
        # in the wrong place for every later plan.
        print("  !! that is NOT the flange/tool: at this pose something else on")
        print("     the arm sticks out further, so this is not the surface.")
        print("     Re-pose until the tool is the part touching it, then")
        print("     measure again. NOT saved.")
        print("     (--yes overrides, if you really mean this position.)")
        if not a.yes:
            return False
    at += sign * a.surface_offset       # positive offset = away from the robot
    if a.surface_offset:
        way = "outward" if a.surface_offset > 0 else "toward the robot"
        print(f"  applying --surface-offset {a.surface_offset:+.4f} m "
              f"({way}) -> {at:+.4f} m")
    save_surface(name, dict(
        axis=axis, sign=sign, at=float(at), bounds=bounds,
        measured_at_joints_deg=[round(math.degrees(v), 3)
                                for v in node.current_joints],
        touched=sorted(set(info)),
        taught_at=time.strftime('%Y-%m-%d %H:%M:%S')))
    return True


def set_surface(a, spec):
    """--set-surface DIR --at V: define a surface from a NUMBER, not the arm.

    For anything the tool cannot or should not be driven against: a wall that
    is not built yet, a distance taken off a drawing, or the vertical step
    between two levels. No robot needed.
    """
    axis, sign = parse_dir(spec)
    name = a.name or dir_name(axis, sign)
    bounds = parse_bounds(a.bound)
    save_surface(name, dict(
        axis=axis, sign=sign, at=float(a.at), bounds=bounds,
        touched=['(given, not measured)'],
        taught_at=time.strftime('%Y-%m-%d %H:%M:%S')))
    return True


def _ask(prompt, default=None):
    """float from the terminal; Enter keeps `default` (None = required)."""
    while True:
        raw = input(prompt + (f" [{default}]" if default is not None else "") + ": ").strip()
        if not raw and default is not None:
            return float(default)
        try:
            return float(raw)
        except ValueError:
            print("  a number, please")


def teach_shelf(mon, node, env_file):
    """--teach-shelf: the whole shelf block of env/<DOBOT_ENV>.json in one go.

    1. jog the tool tip onto the PICK tier's board top at the front edge, LEFT end,
       Enter; then RIGHT end, Enter.  -> yaw (row direction), origin, and the
       board-top z measured under the tool at each point (model bisection like
       --teach-surface, no tool length; the higher of the two is kept).
    2. answer a few tape measurements (Enter keeps the value already in the file).
    Writes shelf.pose_in_base / footprint / board_thick / tier_tops / board_tops.
    Nothing moves; the arm is jogged by you."""
    with open(env_file) as fh:
        env = json.load(fh)
    sh = env['shelf']
    pts, tops = [], []
    for label in ("LEFT", "RIGHT"):
        input(f"  jog the tool tip onto the pick tier's board top at the front edge, "
              f"{label} end (tool vertical, arm still), then Enter ... ")
        f = mon.latest(0.5)
        if f is None:
            print(f"  no fresh feed ({mon.error or 'stale'}); aborting"); return False
        if float(np.max(np.abs(f['qd']))) > 0.5:
            print("  arm is still moving; hold it still and retry"); return False
        p = f['tool'][:3] / 1000.0
        time.sleep(0.3)                           # joint_states catch up with the feed
        z_top, info = measure_surface(node, 'z', -1)
        if z_top is None:
            print(f"  board top under the tool: measurement failed ({info})"); return False
        pts.append(p); tops.append(z_top)
        print(f"  {label}: flange x={p[0]:+.4f} y={p[1]:+.4f}  board top z={z_top:+.4f} m "
              f"(touching {', '.join(sorted(set(info)))}; tool length {p[2] - z_top:.4f} m)")
    d = pts[1] - pts[0]
    if np.hypot(d[0], d[1]) < 0.10:
        print("  the two points are < 10 cm apart -- too short for a reliable yaw"); return False
    yaw = math.atan2(d[1], d[0])
    x_axis = np.array([math.cos(yaw), math.sin(yaw)])
    y_axis = np.array([-math.sin(yaw), math.cos(yaw)])       # shelf +y = into the shelf
    z_top = round(float(max(tops)), 4)                        # higher = conservative

    print("\n  tape measurements (metres). Enter keeps the current value.")
    ends = input("  were the two points the board's two ENDS? [Y/n]: ").strip().lower() != 'n'
    if ends:
        length = float(np.hypot(d[0], d[1]))
        left_to_centre = length / 2.0
    else:
        length = _ask("  board length along the row", sh['footprint'][0])
        left_to_centre = _ask("  distance LEFT point -> board centre along the row")
    depth = _ask("  board depth (front edge -> back)", sh['footprint'][1])
    thick = _ask("  board thickness", sh['board_thick'])
    tier = int(_ask("  pick tier number", 1))
    gap_up = _ask("  gap from this board top to the NEXT board top (0 = no ceiling)", 0)
    gap_dn = _ask("  gap from this board top down to the board BELOW (0 = none)", 0)

    centre = pts[0][:2] + left_to_centre * x_axis + 0.5 * depth * y_axis
    boards = [z_top]
    if gap_dn > 0:
        boards.insert(0, round(z_top - gap_dn, 4))
    if gap_up > 0:
        boards.append(round(z_top + gap_up, 4))
    sh['pose_in_base'] = [round(float(centre[0]), 4), round(float(centre[1]), 4), round(float(yaw), 5)]
    sh['footprint'] = [round(length, 3), round(depth, 3)]
    sh['board_thick'] = thick
    sh['tier_tops'] = {str(tier): z_top}
    sh['board_tops'] = boards
    sh['_taught'] = dict(left=[round(float(v), 4) for v in pts[0]], right=[round(float(v), 4) for v in pts[1]],
                         board_top_measured=[round(float(v), 4) for v in tops],
                         board_ends=ends, taught_at=time.strftime('%Y-%m-%d %H:%M:%S'))
    for k in ('_yaw_taught', '_origin_note', '_tier_taught'):
        sh.pop(k, None)
    with open(env_file, 'w') as fh:
        json.dump(env, fh, indent=2, ensure_ascii=False); fh.write('\n')
    own_like_dir(env_file)
    print(f"\n  -> {env_file}")
    for k in ('pose_in_base', 'footprint', 'board_thick', 'tier_tops', 'board_tops'):
        print(f"     shelf.{k} = {sh[k]}")
    print(f"  yaw {math.degrees(yaw):+.2f} deg. Still by hand: box_xs / box_y (tape), pocket.*, "
          f"then \"measured\": true")
    return True


def parse_args(argv=None):
    p = argparse.ArgumentParser(description=__doc__.split('\n')[0],
                                formatter_class=argparse.RawDescriptionHelpFormatter)
    mode = p.add_mutually_exclusive_group(required=True)
    mode.add_argument('--teach-surface', metavar='DIR',
                      help="measure the table or wall the TOOL is touching and "
                           "register it as a collision plane from then on. DIR "
                           "is the direction from the robot base toward it, "
                           "AXIS FIRST: z- the table underneath, y- the wall on "
                           "the -y side, x+ the wall on the +x side, ...")
    mode.add_argument('--set-surface', metavar='DIR',
                      help="define a surface from --at instead of measuring it: "
                           "for a wall that is not built yet, or a distance off "
                           "a drawing. Needs no robot contact.")
    mode.add_argument('--forget-surface', metavar='NAME',
                      help="delete one taught surface by NAME, or 'all'.")
    mode.add_argument('--show', action='store_true', help="print the taught surfaces")
    mode.add_argument('--teach-shelf', action='store_true',
                      help="the whole shelf block of cr7_pnp/env/<DOBOT_ENV>.json: two jogged "
                           "points on the pick tier's front edge (yaw, origin, board z) + a few "
                           "tape numbers asked interactively. Nothing moves.")
    p.add_argument('--name', default=None,
                   help="name for the surface being taught/set (default: its "
                        "direction). Two surfaces facing the same way -- a base "
                        "support and a lower shelf, both 'z-' -- need distinct names.")
    p.add_argument('--at', type=float, default=None,
                   help="--set-surface only: the coordinate of the surface, m")
    p.add_argument('--bound', action='append', nargs=3, metavar=('AXIS', 'LO', 'HI'),
                   help="limit the surface along AXIS to [LO, HI] metres; 'inf' for "
                        "an open side. Repeatable. e.g. --bound y -0.34 inf")
    p.add_argument('--surface-offset', type=float, default=0.0,
                   help="push the measured surface OUTWARD by this many metres "
                        "(use if the tool was pressed into it); negative pulls "
                        "it toward the robot, which is the conservative way")
    p.add_argument('--ip', default=None, help="robot IP (default: from param.json)")
    p.add_argument('--yes', action='store_true', help="save even if the touching part is not the tool")
    a = p.parse_args(argv)
    for spec in (a.teach_surface, a.set_surface):
        if spec:
            try:
                parse_dir(spec)
            except ValueError as e:
                p.error(str(e))
    try:
        parse_bounds(a.bound)
    except ValueError as e:
        p.error(str(e))
    if a.set_surface and a.at is None:
        p.error("--set-surface needs --at (the surface coordinate, metres)")
    return a


def main(argv=None):
    a = parse_args(argv)
    print(f"surfaces file: {SURFACES_FILE}")

    if a.set_surface:                        # numbers only: no robot, no planner
        return 0 if set_surface(a, a.set_surface) else 1

    if a.forget_surface:                     # a file edit: likewise nothing to connect to
        surfaces = load_surfaces()
        if a.forget_surface.strip().lower() == 'all':
            removed, surfaces = sorted(surfaces), {}
        else:
            removed = [a.forget_surface] if a.forget_surface in surfaces else []
            surfaces.pop(a.forget_surface, None)
        if not removed:
            print(f"nothing to forget: '{a.forget_surface}' is not taught "
                  f"(have: {', '.join(sorted(surfaces)) or 'none'})")
            return 1
        with open(SURFACES_FILE, 'w') as f:
            json.dump(surfaces, f, indent=2)
        own_like_dir(SURFACES_FILE)
        print(f"forgot {', '.join(removed)}; {len(surfaces)} surface(s) still taught")
        return 0

    if a.show:
        surfaces = load_surfaces()
        if surfaces:
            for name, s in sorted(surfaces.items()):
                describe_surface(name, s)
        else:
            print("  no surfaces taught (run --teach-surface z- for the table)")
        return 0

    # --teach-surface / --teach-shelf: need the live robot + joint states. Nothing moves.
    ip = a.ip or default_robot_ip()
    mon = RobotFeed(ip)
    mon.start()
    if not mon.wait_ready():
        print(f"cannot read the robot's real-time feed: {mon.error or 'timed out'} (ping {ip})")
        return 1
    env_file = os.path.join(_PKG_ROOT, 'cr7_pnp', 'env', os.environ['DOBOT_ENV'] + '.json')
    rclpy.init(signal_handler_options=SignalHandlerOptions.NO)
    node = HubPickPlace()
    node.setup_planner(combined_xacro=collision_model_xacro())
    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    try:
        t0 = time.time()
        while node.current_joints is None:
            if time.time() - t0 > 15.0:
                print("no /joint_states in 15 s -- is dobot_joint.launch.py up?")
                return 1
            time.sleep(0.1)
        print("joint states live (URDF convention): " +
              " ".join(f"{math.degrees(v):+.1f}" for v in node.current_joints))
        if a.teach_shelf:
            return 0 if teach_shelf(mon, node, env_file) else 1
        return 0 if teach_surface(node, a, a.teach_surface) else 1
    finally:
        mon.stop()
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    sys.exit(main())
