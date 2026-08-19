"""Bounding SURFACES (table, walls, trolley) in the planning collision model.

APPLY side only: load the taught surfaces file and register each as a slab.
The TEACH side (drive the tool against a surface, bisect for its position, save)
lives in tools/teach_env.py and imports from here -- never the other way round.
Moved verbatim from test/cbirrt_p1p2_test.py (2026-08-17); one change, the frame
(see add_surface).

A surface is a HALF-SPACE, named by the direction from the robot base toward it:
'-z' is the table under the arm, '-y' the wall on the -y side, and so on. `at` is
the coordinate of its face along that axis, in base_link; everything beyond it
is solid. All are measured and enforced the same way, so the table is simply
the '-z' surface.

Where the file lives: cr7_pnp/env/<DOBOT_ENV>_surfaces.json, next to the other
measured constants -- surfaces are real-workspace data, so the sim normally has
none (nothing registered, one log line says so).
"""
import json
import os

import numpy as np
import pinocchio as pin

from .gripper_params import _ENV_NAME as ENV_NAME
from .geometry import XACRO_PATH, COMBINED_XACRO

SURFACES_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'env',
                             ENV_NAME + '_surfaces.json')

# Each surface is registered as a slab this THICK reaching AWAY from the robot,
# so the planner treats the whole far side as solid instead of as a sheet it
# could route around or under. Both numbers just have to outrun the arm's reach.
PLANE_THICK = 4.0
PLANE_EXTENT = 4.0

AXES = {'x': 0, 'y': 1, 'z': 2}


def parse_dir(spec):
    """'z-' -> ('z', -1). Either order works.

    Write the axis first ('z-', 'y-') on the command line: argparse reads a
    bare '-z' as a flag rather than as this option's value. The sign-first
    spelling still parses, for '--teach-surface=-z' and for stored files.
    """
    s = spec.strip().lower()
    if len(s) == 2:
        if s[0] in AXES and s[1] in '+-':
            return s[0], (1 if s[1] == '+' else -1)
        if s[0] in '+-' and s[1] in AXES:
            return s[1], (1 if s[0] == '+' else -1)
    raise ValueError(f"direction must be one of x- x+ y- y+ z- z+, got {spec!r}")


def dir_name(axis, sign):
    return f"{'+' if sign > 0 else '-'}{axis}"


def span_of(bounds, axis):
    """(lo, hi) of a surface along `axis`, unbounded sides filled with the slab.

    A bound is what makes a stepped workspace expressible: the base support and
    the lower shelf are both '-z' surfaces at different heights, told apart only
    by where each one starts and stops in y.
    """
    lo, hi = (bounds or {}).get(axis, (None, None))
    return (-PLANE_EXTENT / 2 if lo is None else float(lo),
            PLANE_EXTENT / 2 if hi is None else float(hi))


def box_of(axis, sign, at, bounds):
    """(size, centre) of the slab: PLANE_THICK along `axis` reaching to `sign`,
    and the bounded span along the other two."""
    size, centre = np.zeros(3), np.zeros(3)
    for ax, k in AXES.items():
        if ax == axis:
            size[k] = PLANE_THICK
            centre[k] = at + sign * PLANE_THICK / 2.0
        else:
            lo, hi = span_of(bounds, ax)
            size[k] = max(hi - lo, 1e-3)
            centre[k] = 0.5 * (lo + hi)
    return size, centre


def T_root_base(node):
    """base_link placement in the collision model's ROOT frame (constant: the
    AGV/cube joints are locked). Identity for the arm-only model, whose root IS
    base_link; the base_link -> mpo_base_link offset for the combined one.

    Surfaces are taught in base_link (the arm's own frame, what the robot's
    GetPose reports in). The old test code assumed root == base_link, which was
    only true when it had fallen back to the arm-only model; in main.py's
    combined model that put a taught table one base offset off. Compose here.
    """
    m = node.collision
    fid = m.model.getFrameId('base_link')
    if fid >= m.model.nframes:
        return pin.SE3.Identity()
    pin.forwardKinematics(m.model, m.data, pin.neutral(m.model))
    pin.updateFramePlacements(m.model, m.data)
    return pin.SE3(m.data.oMf[fid])


def _placement(node, axis, sign, at, bounds):
    _, centre = box_of(axis, sign, at, bounds)
    return T_root_base(node) * pin.SE3(np.eye(3), centre)


def add_surface(node, name, axis, sign, at, bounds=None):
    """Register a surface in the planning collision model.

    Paired against every movable arm link, the way node.py registers the shelf
    boards. `at` is in base_link; the placement is composed into the model root
    (T_root_base). Returns the geometry index, for move_surface().
    """
    import coal
    geom = node.collision.geom
    objs = geom.geometryObjects
    # The node is HubPickPlace, whose setup_planner parks a 'carried_box'
    # phantom on Link6 (its collision pairs stay OFF unless attach_box_collision
    # is called). It is not fitted on the real robot today, so it must not be
    # fenced by the taught table/walls either -- pairing it here would reject
    # plans for a box that is not there.
    arm_links = [i for i in range(len(objs))
                 if objs[i].parentJoint != 0 and objs[i].name != 'carried_box']
    size, _ = box_of(axis, sign, at, bounds)
    go = pin.GeometryObject(f"surface_{name}", 0, _placement(node, axis, sign, at, bounds),
                            coal.Box(*size))
    idx = geom.addGeometryObject(go)
    for i in arm_links:
        geom.addCollisionPair(pin.CollisionPair(i, idx))
    node.collision.geom_data = geom.createData()
    return idx


def move_surface(node, idx, axis, sign, at, bounds=None):
    """Slide a registered surface to a new `at`. Its size does not depend on
    `at`, so only the placement moves -- which is what the bisection needs."""
    node.collision.geom.geometryObjects[idx].placement = _placement(node, axis, sign, at, bounds)
    node.collision.geom_data = node.collision.geom.createData()


def load_surfaces(path=SURFACES_FILE):
    if not os.path.exists(path):
        return {}
    with open(path) as f:
        return json.load(f)


def describe_surface(name, s):
    face = f"{s['axis']} = {s['at']:+.4f} m, solid on the " \
           f"{'+' if s['sign'] > 0 else '-'} side"
    limits = ", ".join(
        f"{ax} in [{'-inf' if lo is None else f'{lo:+.3f}'}, "
        f"{'+inf' if hi is None else f'{hi:+.3f}'}]"
        for ax, (lo, hi) in sorted((s.get('bounds') or {}).items()))
    print(f"  '{name}': {face}" + (f"   bounded {limits}" if limits else ""))
    print(f"        from {', '.join(s['touched'])} at {s['taught_at']}")


def register_surfaces(node, path=SURFACES_FILE):
    """Put the taught surfaces into the model before anything is planned.
    Logs the count in the same '[collision] added N ...' form as the shelf --
    the manual's check (collision_model_guide.md 4.5) reads this line. Returns
    the number registered."""
    surfaces = load_surfaces(path)
    if not surfaces:
        node.get_logger().warn(
            f"[collision] added 0 surfaces: none taught in {path} (tools/teach_env.py "
            f"--teach-surface z- for the table). Only what the URDF/phantoms model "
            f"is enforced.")
        return 0
    for name, s in sorted(surfaces.items()):
        add_surface(node, name, s['axis'], s['sign'], s['at'], s.get('bounds'))
        node.get_logger().info(
            f"[collision] surface '{name}' registered: {s['axis']} = "
            f"{s['at']:+.4f} m (from {s['taught_at']})")
    node.get_logger().info(f"[collision] added {len(surfaces)} surfaces "
                           f"(table/walls, base_link -> model root composed)")
    return len(surfaces)


def collision_model_xacro():
    """Which xacro to collision-check against: combined if it builds, else arm-only.

    The combined model is the sim rig -- arm on a cube platform on an MPO-700
    AGV -- and it reaches into neo_simulation2 for the AGV body, a simulation
    package a real-robot workspace need not carry. On a pedestal-mounted arm
    that AGV is not there to hit, so the arm-only model is the CORRECT one, not
    a degraded one; it still checks the arm against itself and its own gripper.

    What no model here covers either way is the WORKSPACE: the table, the
    fixture and the object are not represented anywhere. Vertical approach is
    covered by the contact detection; a free RRT leg (approach(), when the tool
    has to be reoriented) is not, which is why it warns before it runs.

    Returns None to mean "use node.py's default (combined)".
    """
    import xacro
    try:
        xacro.process_file(COMBINED_XACRO, mappings={'use_gazebo': 'false'})
        return None
    except Exception as e:
        reason = str(e).split('"')[1] if '"' in str(e) else str(e)
        print("\n[collision] the combined (arm + cube + AGV) model is unavailable:")
        print(f"     {reason[:160]}")
        print("   Using the ARM-ONLY model: self- and gripper-collision checked.")
        print("   Correct for a pedestal-mounted arm. Note that the table, the")
        print("   fixture and the object are not modelled in EITHER case -- the")
        print("   descend relies on contact detection, so watch the first cycle.\n")
        return XACRO_PATH
