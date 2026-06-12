#!/usr/bin/env python3
"""IK-based reachability map for the CR7 arm (Link6 / TCP).

What it does
------------
This is the *inverse-kinematics* counterpart of the old FK Monte-Carlo map.
Instead of sampling joints and recording where the tool lands, it discretizes
the workspace into a voxel grid and asks, for every voxel, "from how many tool
orientations can the gripper actually reach this point?".

Method C - Voxel Map
  * The workspace is split into a uniform voxel grid (default 50 mm).
  * A voxel is "reachable" if at least one tested orientation has an IK
    solution that is also self-collision free.

Method D - 6D Reachability Map (orientation-aware)
  * For every voxel center we test a set of tool orientations (approach
    directions on a Fibonacci sphere x roll angles, with one direction forced
    to straight-down so the canonical grasp pose is always evaluated).
  * Reachability Index:  RI(voxel) = (#solved orientations) / (#tested).
      RI == 1.0  -> reachable from any tested pose  (Dexterous Workspace)
      0 < RI < 1 -> only some poses work
      RI == 0    -> not reachable at all
  * We also flag whether a near-vertical "down" approach is solvable (grasp).

IK engine
---------
pinocchio damped-least-squares (CLIK) toward an SE3 target, with several random
seed restarts per orientation so a single unlucky initial guess does not make a
truly-reachable pose look unreachable.  Each converged configuration is checked
against the same self-collision model used by the FK map, so only joint-limit-
respecting AND collision-free solutions count as reachable.

Workspace region
----------------
* --bounds xmin,xmax,ymin,ymax,zmin,zmax (meters, base_link frame) scans only
  that box -- e.g. the +x/-y quadrant at base height: 0,0.8,-0.8,0,-0.05,0.05.
* Without --bounds the box is taken from a fast FK envelope pass and only the
  FK-occupied voxels are tested (everything else is RI = 0 by definition).

Outputs (timestamped, under --out-dir)
  * CSV   x,y,z,ri,n_ok,n_total,down (+ a per-height farthest-reach summary)
  * PCD   _all (RI>0), _down (down-reachable), _dex (RI==1)
  * JSON  metadata (limits, voxel, orientation counts, IK tolerances, RI stats)

Visualization
-------------
Publishes the voxels two ways so they can be compared in RViz:
  * Marker (CUBE_LIST) on /reach_all and /reach_down -- each cube carries its
    own ColorRGBA, so RViz colors them directly with no Color Transformer.
  * PointCloud2 on /reach_all_cloud and /reach_down_cloud -- carries intensity
    (=RI) and a baked rgb field (Color Transformer = Intensity or RGB8).
/reach_all* is colored by RI (blue->green->red); /reach_down* is solid green.
IMPORTANT: set the RViz Fixed Frame to the data frame (base_link), not a stale
frame like dummy_link, or every display silently shows nothing while Status=OK.
Pure compute/save needs no simulator; RViz (with the robot TF) views the clouds.
"""

import os
import json
import math
import time
import tempfile
import argparse
import datetime
import numpy as np
import pinocchio as pin


# Default model / config -------------------------------------------------------

DEFAULT_XACRO = os.path.expanduser(
    '~/dobot_ws/install/cra_description/share/cra_description/urdf/cr7_robot.xacro')
DEFAULT_SRDF = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                            'cr7_moveit', 'config', 'cr7_robot.srdf')

ARM_JOINTS = ('joint1', 'joint2', 'joint3', 'joint4', 'joint5', 'joint6')
GRIPPER_JOINTS = ('gripper_gripper_joint', 'gripper_right_finger_joint')
EE_FRAME = 'Link6'

# Default joint limits (radians) = the limits used by test_w_gripper.py.
# (lower, upper) per joint J1..J6. These are the parameters to vary.
DEFAULT_LIMITS_DEG = [
    (-101.0, 90.0),    # J1
    (-70.0, 60.0),     # J2
    (-180.0, 180.0),   # J3
    (0.0, 120.0),      # J4
    (-120.0, 120.0),   # J5
    (-180.0, 180.0),   # J6 (free during IK; needed to realize orientations)
]

DOWN_WORLD = np.array([0.0, 0.0, -1.0])  # world -Z; gripper "down" approach axis

# Offset from Link6 origin to TCP along the tool z-axis (gripper body length).
# OnRobot 2FG7: gripper_2fg7_attach_joint has xyz="0 0 0", finger attachment
# joints are at z=0.12005 from gripper_base_link -> TCP ~= 0.12 m from Link6.
# Change this value when swapping grippers.
TCP_OFFSET_M = 0.12005


# Model + IK + self-collision --------------------------------------------------

class ReachabilityModel:
    """Reduced 6-DOF CR7 model (gripper locked) with FK, IK and self-collision."""

    def __init__(self, xacro_path=DEFAULT_XACRO, srdf_path=DEFAULT_SRDF,
                 ee_frame=EE_FRAME, lock_non_arm=False, arm_pairs_only=False,
                 xacro_mappings=None):
        # lock_non_arm: lock EVERY movable joint except the 6 arm joints (instead
        # of only the named gripper joints). Use this with the combined
        # cr7_on_mpo700 URDF so the cube platform and the MPO-700 AGV body become
        # fixed attached geometry that still participates in arm self-collision.
        # arm_pairs_only: after pruning, drop pairs whose BOTH geometries are on
        # fixed (non-arm) links -- their collision state never changes with the
        # arm config, so checking them every IK call only wastes time.
        urdf_path = self._xacro_to_urdf(xacro_path, xacro_mappings)

        # Full model + collision geometry, then lock the non-arm joints so the
        # planning model is the 6-DOF arm; the gripper/cube/AGV geometry stays
        # attached (frozen at neutral) and still participates in self-collision.
        full = pin.buildModelFromUrdf(urdf_path)
        full_geom = pin.buildGeomFromUrdf(full, urdf_path, pin.GeometryType.COLLISION)
        if lock_non_arm:
            locked_ids = [j for j in range(1, full.njoints)
                          if full.names[j] not in ARM_JOINTS]
        else:
            locked_ids = [full.getJointId(n) for n in GRIPPER_JOINTS
                          if full.existJointName(n)]
        self.model, geoms = pin.buildReducedModel(
            full, [full_geom], locked_ids, pin.neutral(full))
        self.geom = geoms[0]
        self.data = self.model.createData()

        # Collision pairs: all pairs, minus SRDF-disabled (adjacent/never) pairs,
        # minus any pair already colliding at the neutral config (mounting
        # overlaps such as gripper<->Link6) so they don't cause false positives.
        self.geom.addAllCollisionPairs()
        n_all = len(self.geom.collisionPairs)
        if srdf_path and os.path.exists(srdf_path):
            pin.removeCollisionPairs(self.model, self.geom, srdf_path)
        n_srdf = len(self.geom.collisionPairs)
        self.geom_data = self.geom.createData()
        n_neutral = self._disable_pairs_colliding_at(pin.neutral(self.model))
        if arm_pairs_only:
            self._keep_only_movable_pairs()
        self.pair_stats = (n_all, n_srdf, len(self.geom.collisionPairs), n_neutral)

        self.frame_id = self.model.getFrameId(ee_frame)
        # arm-joint order -> reduced-model configuration indices
        self.q_index = [self.model.idx_qs[self.model.getJointId(n)]
                        for n in ARM_JOINTS]
        # Joint limits used for IK seeds/clamping; set via set_joint_limits().
        self.lo_cfg = self.model.lowerPositionLimit.copy()
        self.hi_cfg = self.model.upperPositionLimit.copy()

    @staticmethod
    def _xacro_to_urdf(xacro_path, mappings=None):
        import xacro
        doc = xacro.process_file(xacro_path, mappings=mappings or {})
        # Unique per process: parallel workers each build their own model, so a
        # shared filename would race (one truncates while another reads -> empty
        # XML / "Unable to parse URDF").
        path = os.path.join(tempfile.gettempdir(),
                            f'cr7_reach_model_{os.getpid()}.urdf')
        with open(path, 'w') as f:
            f.write(doc.toxml())
        return path

    def _disable_pairs_colliding_at(self, q):
        """Remove collision pairs that are already in collision at config q."""
        pin.computeCollisions(self.model, self.data, self.geom, self.geom_data,
                              q, False)
        bad = [(cp.first, cp.second)
               for i, cp in enumerate(self.geom.collisionPairs)
               if self.geom_data.collisionResults[i].isCollision()]
        for a, b in bad:
            self.geom.removeCollisionPair(pin.CollisionPair(a, b))
        if bad:
            self.geom_data = self.geom.createData()
        return len(bad)

    def _keep_only_movable_pairs(self):
        """Drop collision pairs whose BOTH geometries sit on fixed (non-arm) links.

        A geometry attached to the universe joint (parentJoint == 0) never moves
        with the arm configuration, so a pair of two such geometries has a
        constant collision result and is pointless to recheck every IK call. We
        keep every pair that involves at least one movable (arm) link, i.e.
        arm<->arm self-collisions and arm<->(cube/AGV/base) collisions.
        """
        objs = self.geom.geometryObjects
        drop = [(cp.first, cp.second) for cp in self.geom.collisionPairs
                if objs[cp.first].parentJoint == 0
                and objs[cp.second].parentJoint == 0]
        for a, b in drop:
            self.geom.removeCollisionPair(pin.CollisionPair(a, b))
        if drop:
            self.geom_data = self.geom.createData()
        return len(drop)

    def is_collision_free(self, j1_to_j6):
        """True if the arm config (J1..J6) is free of any active collision.

        Maps the arm-joint vector to a full reduced-model config and runs the
        same self-collision model used by the reachability map. With a model
        built lock_non_arm=True from the combined URDF, this also catches the arm
        hitting the cube platform or the MPO-700 AGV body, not just itself.
        """
        return not self.self_collides(self.pin_q(j1_to_j6))

    def set_joint_limits(self, limits_rad):
        """Set the per-joint (lower, upper) limits used for IK seeds/clamping."""
        self.lo_cfg = pin.neutral(self.model).copy()
        self.hi_cfg = pin.neutral(self.model).copy()
        for k, idx in enumerate(self.q_index):
            self.lo_cfg[idx] = limits_rad[k][0]
            self.hi_cfg[idx] = limits_rad[k][1]

    def random_config(self, rng):
        """Random joint configuration uniformly inside the configured limits."""
        q = pin.neutral(self.model).copy()
        for idx in self.q_index:
            q[idx] = rng.uniform(self.lo_cfg[idx], self.hi_cfg[idx])
        return q

    def pin_q(self, j1_to_j6):
        """Map an arm-joint vector (J1..J6) to a full reduced-model config."""
        qp = pin.neutral(self.model)
        for k, idx in enumerate(self.q_index):
            qp[idx] = j1_to_j6[k]
        return qp

    def fk_tcp(self, qp):
        """TCP placement for a reduced-model config -> (position, rotation).

        Returns the TCP position (Link6 origin + TCP_OFFSET_M along tool z-axis)
        and the Link6 rotation, matching the FK-map convention.
        """
        pin.forwardKinematics(self.model, self.data, qp)
        pin.updateFramePlacement(self.model, self.data, self.frame_id)
        oMf = self.data.oMf[self.frame_id]
        tool_z = oMf.rotation[:, 2]
        tcp_pos = oMf.translation + TCP_OFFSET_M * tool_z
        return tcp_pos.copy(), oMf.rotation.copy()

    def link6_target_from_tcp(self, tcp_pos, R):
        """Link6 origin target for a desired TCP position and tool rotation R.

        IK drives the Link6 frame, but targets are expressed at the TCP, so we
        undo the fixed tool-z offset (inverse of fk_tcp's shift).
        """
        return np.asarray(tcp_pos) - TCP_OFFSET_M * R[:, 2]

    def inverse_kinematics(self, tcp_pos, R, seeds,
                           pos_tol=0.005, rot_tol=math.radians(5.0),
                           max_iter=100, damping=1e-6, dt=1.0):
        """Damped-least-squares IK toward an SE3 target at the TCP.

        Tries each seed config in turn; returns the first converged AND
        self-collision-free joint configuration, or None if all seeds fail.
        """
        target = pin.SE3(np.asarray(R), self.link6_target_from_tcp(tcp_pos, R))
        for q0 in seeds:
            q = np.array(q0, dtype=float)
            converged = False
            for _ in range(max_iter):
                pin.forwardKinematics(self.model, self.data, q)
                pin.updateFramePlacement(self.model, self.data, self.frame_id)
                iMd = self.data.oMf[self.frame_id].actInv(target)
                err = pin.log(iMd).vector              # 6D twist (lin, ang)
                if (np.linalg.norm(err[:3]) < pos_tol and
                        np.linalg.norm(err[3:]) < rot_tol):
                    converged = True
                    break
                J = pin.computeFrameJacobian(self.model, self.data, q,
                                             self.frame_id)
                J = -pin.Jlog6(iMd.inverse()).dot(J)
                v = -J.T.dot(np.linalg.solve(
                    J.dot(J.T) + damping * np.eye(6), err))
                q = pin.integrate(self.model, q, v * dt)
                np.clip(q, self.lo_cfg, self.hi_cfg, out=q)
            if converged and not self.self_collides(q):
                return q
        return None

    def max_reach(self):
        """Strict upper bound on TCP distance from the base origin (meters).

        Sum of the fixed link offsets along the chain plus the TCP offset. By the
        triangle inequality no configuration can place the TCP farther than this,
        so pruning voxels beyond it cannot drop any truly-reachable voxel.
        """
        total = 0.0
        for jp in self.model.jointPlacements:
            total += float(np.linalg.norm(jp.translation))
        total += float(np.linalg.norm(
            self.model.frames[self.frame_id].placement.translation))
        return total + TCP_OFFSET_M

    def self_collides(self, qp):
        """True if the config is in self-collision (stops at first contact)."""
        return pin.computeCollisions(self.model, self.data, self.geom,
                                     self.geom_data, qp, True)


# Orientation sampling ---------------------------------------------------------

def fibonacci_sphere(n):
    """`n` approximately uniform unit vectors on the sphere."""
    if n <= 1:
        return np.array([[0.0, 0.0, -1.0]])
    ga = math.pi * (3.0 - math.sqrt(5.0))   # golden angle
    out = np.empty((n, 3))
    for i in range(n):
        y = 1.0 - 2.0 * i / (n - 1)
        r = math.sqrt(max(0.0, 1.0 - y * y))
        theta = ga * i
        out[i] = (math.cos(theta) * r, y, math.sin(theta) * r)
    return out


def _rotation_from_axis(z_axis, roll):
    """Rotation matrix whose 3rd column is z_axis, with `roll` about that axis."""
    z = np.asarray(z_axis, dtype=float)
    z /= np.linalg.norm(z)
    ref = np.array([1.0, 0.0, 0.0]) if abs(z[0]) < 0.9 else np.array([0.0, 1.0, 0.0])
    x = np.cross(ref, z)
    x /= np.linalg.norm(x)
    y = np.cross(z, x)
    R0 = np.column_stack((x, y, z))
    c, s = math.cos(roll), math.sin(roll)
    Rz = np.array([[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]])
    return R0.dot(Rz)


def build_orientations(n_dir, n_roll, down_tol_deg):
    """Tool orientations to test per voxel.

    Returns (rotations list of 3x3, down_flags bool array). The approach axis
    (tool z) directions come from a Fibonacci sphere; the direction closest to
    straight-down is snapped to exact -Z so the canonical grasp pose is always
    tested. Each direction is spun by `n_roll` roll angles.
    """
    dirs = fibonacci_sphere(n_dir)
    j = int(np.argmax(dirs.dot(DOWN_WORLD)))   # snap nearest to exact down
    dirs[j] = DOWN_WORLD
    cos_tol = math.cos(math.radians(down_tol_deg))
    rolls = [2.0 * math.pi * k / n_roll for k in range(max(1, n_roll))]
    rots, down = [], []
    for d in dirs:
        is_down = float(d.dot(DOWN_WORLD)) >= cos_tol
        for roll in rolls:
            rots.append(_rotation_from_axis(d, roll))
            down.append(is_down)
    return rots, np.array(down, dtype=bool)


# Workspace voxels -------------------------------------------------------------

def voxel_keys(points, voxel):
    """Set of integer voxel indices occupied by the points."""
    if len(points) == 0:
        return set()
    idx = np.floor(np.asarray(points) / voxel).astype(np.int64)
    return set(map(tuple, idx))


def keys_to_centers(keys, voxel):
    """Voxel-index tuples -> voxel-center coordinates."""
    if not keys:
        return np.zeros((0, 3))
    return (np.asarray(list(keys), dtype=np.float64) + 0.5) * voxel


def bounds_to_centers(bounds, voxel):
    """All voxel centers whose index falls inside an axis-aligned box."""
    xmn, xmx, ymn, ymx, zmn, zmx = bounds

    def irange(lo, hi):
        return range(int(math.floor(lo / voxel)), int(math.floor(hi / voxel)) + 1)

    centers = [((ix + 0.5) * voxel, (iy + 0.5) * voxel, (iz + 0.5) * voxel)
               for ix in irange(xmn, xmx)
               for iy in irange(ymn, ymx)
               for iz in irange(zmn, zmx)]
    return np.asarray(centers) if centers else np.zeros((0, 3))


def fk_envelope_centers(rm, args, limits_rad):
    """Fast FK Monte-Carlo pass -> voxel centers reachable in some orientation.

    Used to pick candidate voxels when --bounds is not given: only voxels the
    arm can land in (orientation-agnostic) are worth IK-testing; the rest are
    RI = 0 by definition. Returns (centers Nx3, bounding-box tuple).
    """
    rng = np.random.default_rng(args.seed)
    lo = np.array([limits_rad[i][0] for i in range(6)])
    hi = np.array([limits_rad[i][1] for i in range(6)])
    pts = np.empty((args.envelope_samples, 3))
    for i in range(args.envelope_samples):
        p, _ = rm.fk_tcp(rm.pin_q(rng.uniform(lo, hi)))
        pts[i] = p
    keys = voxel_keys(pts, args.voxel)
    centers = keys_to_centers(keys, args.voxel)
    bbox = (float(pts[:, 0].min()), float(pts[:, 0].max()),
            float(pts[:, 1].min()), float(pts[:, 1].max()),
            float(pts[:, 2].min()), float(pts[:, 2].max()))
    return centers, bbox


# Reachability Index computation (Method C + D) --------------------------------

def _fmt_hms(seconds):
    """Format a duration in seconds as H:MM:SS."""
    s = int(round(seconds))
    return f'{s // 3600}:{(s % 3600) // 60:02d}:{s % 60:02d}'


def solve_voxel(rm, c, orientations, down_flags, params, rng):
    """Reachability of a single voxel center over all orientations.

    Returns (n_ok, down). For each orientation the IK is seeded with the
    previous orientation's solution FIRST (warm-start) and then the full set of
    random restarts, so warm-starting can only add successes, never remove them
    (recall >= the random-only baseline). The computation depends ONLY on this
    voxel's seed and orientation set -- not on neighbors or process layout -- so
    serial and parallel runs give identical results.
    """
    rot_tol = params['rot_tol']
    n_restart = params['seed_restarts']
    ok = 0
    down = False
    warm_seed = None
    for oi, R in enumerate(orientations):
        seeds = []
        if warm_seed is not None:
            seeds.append(warm_seed)
        seeds.extend(rm.random_config(rng) for _ in range(n_restart))
        q = rm.inverse_kinematics(
            c, R, seeds, pos_tol=params['pos_tol'], rot_tol=rot_tol,
            max_iter=params['max_iter'])
        if q is not None:
            ok += 1
            warm_seed = q                       # seed next orientation
            if down_flags[oi]:
                down = True
    return ok, down


def _solve_chunk(rm, gidx, centers, orientations, down_flags, params,
                 base_seed):
    """Solve a block of voxels given their global indices (for seeding)."""
    n = len(centers)
    n_ok = np.zeros(n, dtype=np.int32)
    down = np.zeros(n, dtype=bool)
    for i, c in enumerate(centers):
        # Deterministic per-voxel RNG keyed by the TRUE global index, so the
        # random restarts are identical no matter how voxels are split.
        rng = np.random.default_rng([base_seed, int(gidx[i])])
        ok, dn = solve_voxel(rm, c, orientations, down_flags, params, rng)
        n_ok[i] = ok
        down[i] = dn
    return n_ok, down


# Worker globals (one model per process, built once in the initializer).
_W_RM = None
_W_ORI = None
_W_DOWN = None
_W_PARAMS = None
_W_SEED = None


def _worker_init(xacro, srdf, limits_rad, orientations, down_flags, params,
                 base_seed):
    global _W_RM, _W_ORI, _W_DOWN, _W_PARAMS, _W_SEED
    _W_RM = ReachabilityModel(xacro, srdf)
    _W_RM.set_joint_limits(limits_rad)
    _W_ORI = orientations
    _W_DOWN = down_flags
    _W_PARAMS = params
    _W_SEED = base_seed


def _worker_task(task):
    gidx, centers = task
    n_ok, down = _solve_chunk(_W_RM, gidx, centers, _W_ORI, _W_DOWN, _W_PARAMS,
                              _W_SEED)
    return gidx, n_ok, down


def compute_reachability(rm, centers, orientations, down_flags, args,
                         limits_rad=None):
    """Per-voxel Reachability Index via IK over all orientations.

    Returns (ri N, n_ok N int, n_total int, down_reach N bool). Voxels beyond the
    arm's maximum reach are skipped (RI=0) for free. The rest are solved either
    serially (--jobs 1) or across `--jobs` worker processes; both share the same
    deterministic per-voxel seeding so the result is independent of job count.
    """
    rot_tol = math.radians(args.ik_rot_tol_deg)
    n_total = len(orientations)
    n = len(centers)
    ri = np.zeros(n)
    n_ok = np.zeros(n, dtype=np.int32)
    down_reach = np.zeros(n, dtype=bool)
    params = {'rot_tol': rot_tol, 'pos_tol': args.ik_pos_tol,
              'max_iter': args.ik_max_iter, 'seed_restarts': args.seed_restarts}
    base_seed = args.seed + 1

    # Reach-radius pruning: only voxels within the (upper-bound) reach are tested.
    r_max = args.max_reach if args.max_reach is not None else rm.max_reach()
    r_max += math.sqrt(3.0) * args.voxel        # +1 voxel diagonal safety margin
    dist = np.linalg.norm(centers, axis=1)      # base origin is (0,0,0) here
    active = np.nonzero(dist <= r_max)[0]
    print(f'[reach] reach prune: max reach <= {r_max:.3f} m -> '
          f'{len(active)}/{n} voxels need IK ({n - len(active)} skipped)',
          flush=True)
    if len(active) == 0:
        return ri, n_ok, n_total, down_reach
    act_centers = centers[active]

    jobs = max(1, args.jobs)
    t_start = time.time()

    if jobs == 1:
        # Serial path (debug / reproducibility reference).
        na = len(active)
        report_every = max(1, na // 20)
        for i, gi in enumerate(active):
            rng = np.random.default_rng([base_seed, int(gi)])
            ok, dn = solve_voxel(rm, act_centers[i], orientations,
                                 down_flags, params, rng)
            n_ok[gi] = ok
            down_reach[gi] = dn
            if (i + 1) % report_every == 0:
                el = time.time() - t_start
                eta = el / (i + 1) * (na - i - 1)
                print(f'[reach]   voxels {i + 1}/{na} '
                      f'(reachable {int((n_ok > 0).sum())}, '
                      f'dexterous {int((n_ok >= n_total).sum())}) '
                      f'elapsed {_fmt_hms(el)}, ETA {_fmt_hms(eta)}', flush=True)
    else:
        # Parallel path: contiguous chunks keep warm-start locality. Each task
        # carries its voxels' true global indices for reassembly and seeding.
        import multiprocessing as mp
        n_chunks = max(jobs * 4, 1)
        chunk = max(1, (len(active) + n_chunks - 1) // n_chunks)
        tasks = [(active[s:s + chunk], act_centers[s:s + chunk])
                 for s in range(0, len(active), chunk)]
        print(f'[reach] parallel: {jobs} workers, {len(tasks)} chunks '
              f'(~{chunk} voxels each)', flush=True)
        init_args = (args.xacro, args.srdf, limits_rad, orientations,
                     down_flags, params, base_seed)
        done = 0
        na = len(active)
        report_step = max(1, na // 20)          # ~5% milestones, not every chunk
        next_report = report_step
        with mp.Pool(jobs, initializer=_worker_init,
                     initargs=init_args) as pool:
            for gidx, ok_chunk, down_chunk in pool.imap_unordered(
                    _worker_task, tasks):
                n_ok[gidx] = ok_chunk
                down_reach[gidx] = down_chunk
                done += len(gidx)
                if done >= next_report or done == na:
                    next_report += report_step
                    el = time.time() - t_start
                    eta = el / done * (na - done) if done else 0.0
                    print(f'[reach]   voxels {done}/{na} '
                          f'(reachable {int((n_ok > 0).sum())}, '
                          f'dexterous {int((n_ok >= n_total).sum())}) '
                          f'elapsed {_fmt_hms(el)}, ETA {_fmt_hms(eta)}',
                          flush=True)

    ri = np.where(n_total > 0, n_ok / n_total, 0.0)
    return ri, n_ok, n_total, down_reach


# Saving -----------------------------------------------------------------------

def write_pcd(path, pts, intensity=None):
    """Write an ASCII PCD (xyz [+ intensity]) for pcl_viewer/CloudCompare/open3d."""
    pts = np.asarray(pts, dtype=np.float32)
    n = len(pts)
    if intensity is None:
        fields = "FIELDS x y z\nSIZE 4 4 4\nTYPE F F F\nCOUNT 1 1 1\n"
        rows = (f"{p[0]:.6f} {p[1]:.6f} {p[2]:.6f}" for p in pts)
    else:
        intensity = np.asarray(intensity, dtype=np.float32)
        fields = ("FIELDS x y z intensity\nSIZE 4 4 4 4\n"
                  "TYPE F F F F\nCOUNT 1 1 1 1\n")
        rows = (f"{p[0]:.6f} {p[1]:.6f} {p[2]:.6f} {v:.6f}"
                for p, v in zip(pts, intensity))
    header = ("# .PCD v0.7 - Point Cloud Data file format\nVERSION 0.7\n"
              + fields + f"WIDTH {n}\nHEIGHT 1\nVIEWPOINT 0 0 0 1 0 0 0\n"
              f"POINTS {n}\nDATA ascii\n")
    with open(path, 'w') as f:
        f.write(header)
        f.write('\n'.join(rows))
        if n:
            f.write('\n')


def save_clouds(centers, ri, n_ok, n_total, down_reach, args, limits_deg, stats):
    """Save CSV (x,y,z,ri,n_ok,n_total,down) + all/down/dex PCD + JSON meta."""
    os.makedirs(args.out_dir, exist_ok=True)
    ts = datetime.datetime.now().strftime('%Y%m%d_%H%M%S')
    base = os.path.join(args.out_dir, f'reach_{ts}')
    csv_path = base + '.csv'
    all_pcd, down_pcd, dex_pcd = base + '_all.pcd', base + '_down.pcd', base + '_dex.pcd'
    meta_path = base + '_meta.json'

    reach_mask = ri > 0.0
    dex_mask = ri >= 1.0
    all_centers = centers[reach_mask]
    all_ri = ri[reach_mask]
    down_centers = centers[down_reach]
    dex_centers = centers[dex_mask]

    with open(csv_path, 'w') as f:
        f.write('x,y,z,ri,n_ok,n_total,down\n')
        for p, r, k, d in zip(centers, ri, n_ok, down_reach):
            f.write(f'{p[0]:.6f},{p[1]:.6f},{p[2]:.6f},'
                    f'{r:.4f},{int(k)},{n_total},{int(d)}\n')

        # Per-height reach summary: for each z level, the farthest down-reachable
        # point and its horizontal distance from the base.
        if len(down_centers) > 0:
            dc = np.asarray(down_centers)
            dist_xy = np.sqrt(dc[:, 0]**2 + dc[:, 1]**2)
            z_bins = np.round(dc[:, 2] / args.voxel) * args.voxel
            f.write('\n# reach_down summary: farthest TCP per height\n')
            f.write('# z_m,max_dist_from_base_m,farthest_x,farthest_y\n')
            for z in sorted(np.unique(z_bins)):
                mask = z_bins == z
                idx_max = int(np.argmax(dist_xy[mask]))
                max_d = dist_xy[mask][idx_max]
                px, py = dc[mask][idx_max, 0], dc[mask][idx_max, 1]
                f.write(f'# {z:+.3f},{max_d:.3f},{px:.3f},{py:.3f}\n')

    write_pcd(all_pcd, all_centers, intensity=all_ri)
    write_pcd(down_pcd, down_centers)
    write_pcd(dex_pcd, dex_centers)

    meta = {
        'timestamp': ts,
        'method': 'IK voxel reachability (Method C + D)',
        'xacro': args.xacro,
        'srdf': args.srdf,
        'joint_limits_deg': {f'J{i+1}': list(limits_deg[i]) for i in range(6)},
        'voxel_m': args.voxel,
        'bounds': args.bounds,
        'orientations': {'n_dir': args.n_dir, 'n_roll': args.n_roll,
                         'n_total': int(n_total)},
        'ik': {'seed_restarts': args.seed_restarts, 'pos_tol_m': args.ik_pos_tol,
               'rot_tol_deg': args.ik_rot_tol_deg, 'max_iter': args.ik_max_iter},
        'down_tol_deg': args.down_tol_deg,
        'seed': args.seed,
        'frame_id': args.frame_id,
        'stats': stats,
        'files': {'csv': csv_path, 'all_pcd': all_pcd,
                  'down_pcd': down_pcd, 'dex_pcd': dex_pcd},
    }
    with open(meta_path, 'w') as f:
        json.dump(meta, f, indent=2)
    print(f'[reach] saved:\n  {csv_path}\n  {all_pcd}\n  {down_pcd}\n  {dex_pcd}\n'
          f'  {meta_path}', flush=True)
    return meta


# Visualization ----------------------------------------------------------------

def ri_color_rgb(v):
    """Map a single RI in [0,1] to an (r, g, b) tuple in [0,1] (blue->green->red)."""
    v = max(0.0, min(1.0, float(v)))
    return (v, 1.0 - abs(2.0 * v - 1.0), 1.0 - v)


def _make_marker(header, pts, rgb, voxel, ns, marker_id):
    """visualization_msgs/Marker (CUBE_LIST) carrying a per-point ColorRGBA.

    Each cube holds its own color, so RViz renders it directly without any
    Color Transformer selection (it is bypassed entirely). CUBE_LIST draws real
    3D cubes (one per voxel) which render reliably, unlike screen-space POINTS.
    `rgb` is a list of (r, g, b) tuples in [0, 1], one per point; `voxel` sets
    the cube edge length.
    """
    from visualization_msgs.msg import Marker
    from geometry_msgs.msg import Point
    from std_msgs.msg import ColorRGBA

    m = Marker()
    m.header = header
    m.ns = ns
    m.id = marker_id
    m.type = Marker.CUBE_LIST
    m.action = Marker.ADD
    m.scale.x = float(voxel)            # cube edge length (m)
    m.scale.y = float(voxel)
    m.scale.z = float(voxel)
    m.pose.orientation.w = 1.0
    # Top-level color: per-cube `colors` is what RViz draws, but some RViz
    # versions gate *_LIST visibility on color.a, so keep it opaque.
    m.color.r = m.color.g = m.color.b = 1.0
    m.color.a = 1.0
    m.points = [Point(x=float(p[0]), y=float(p[1]), z=float(p[2])) for p in pts]
    m.colors = [ColorRGBA(r=float(c[0]), g=float(c[1]), b=float(c[2]), a=1.0)
                for c in rgb]
    return m


def _pack_rgb(r, g, b):
    """Pack 0-255 RGB into a float (PCL 'rgb' field convention: 0x00RRGGBB)."""
    import struct
    u = (int(r) << 16) | (int(g) << 8) | int(b)
    return struct.unpack('f', struct.pack('I', u))[0]


def _make_cloud(header, pts, intensity=None, rgb=None):
    """sensor_msgs/PointCloud2 with XYZ (+ optional intensity and/or 'rgb').

    `intensity` (per-point float, e.g. RI) feeds RViz Color Transformer =
    Intensity; a baked `rgb` field (per-point packed float) lets RViz color the
    cloud with Color Transformer = RGB8 (or no transformer at all).
    """
    from sensor_msgs.msg import PointCloud2, PointField
    import struct
    fields = [
        PointField(name='x', offset=0, datatype=PointField.FLOAT32, count=1),
        PointField(name='y', offset=4, datatype=PointField.FLOAT32, count=1),
        PointField(name='z', offset=8, datatype=PointField.FLOAT32, count=1),
    ]
    offset, fmt = 12, 'fff'
    if intensity is not None:
        fields.append(PointField(name='intensity', offset=offset,
                                 datatype=PointField.FLOAT32, count=1))
        offset, fmt = offset + 4, fmt + 'f'
    if rgb is not None:
        fields.append(PointField(name='rgb', offset=offset,
                                 datatype=PointField.FLOAT32, count=1))
        offset, fmt = offset + 4, fmt + 'f'

    data = bytearray()
    for i, p in enumerate(pts):
        vals = [float(p[0]), float(p[1]), float(p[2])]
        if intensity is not None:
            vals.append(float(intensity[i]))
        if rgb is not None:
            vals.append(float(rgb[i]))
        data += struct.pack(fmt, *vals)

    msg = PointCloud2()
    msg.header = header
    msg.height = 1
    msg.width = len(pts)
    msg.fields = fields
    msg.is_bigendian = False
    msg.point_step = offset
    msg.row_step = offset * len(pts)
    msg.data = bytes(data)
    msg.is_dense = True
    return msg


def load_csv(path, out_dir):
    """Load a saved reach_*.csv -> (centers Nx3, ri N, down_reach N bool).

    Reads only the data rows (skips the header and the '#' summary block) so a
    finished run can be republished without recomputing anything.
    """
    import glob
    if path == 'latest':
        files = sorted(glob.glob(os.path.join(out_dir, 'reach_*.csv')),
                       key=os.path.getmtime)
        if not files:
            raise SystemExit(f'--load latest: no reach_*.csv in {out_dir}')
        path = files[-1]
    centers, ri, down = [], [], []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith('#') or line.startswith('x,'):
                continue
            v = line.split(',')           # x,y,z,ri,n_ok,n_total,down
            centers.append((float(v[0]), float(v[1]), float(v[2])))
            ri.append(float(v[3]))
            down.append(bool(int(v[6])))
    print(f'[reach] loaded {len(centers)} voxels from {path}', flush=True)
    return (np.asarray(centers), np.asarray(ri),
            np.asarray(down, dtype=bool))


def publish_clouds(centers, ri, down_reach, args):
    """Publish reachability voxels as BOTH Marker and PointCloud2 to compare.

    Marker (CUBE_LIST) on /reach_all, /reach_down: each cube carries its own
    ColorRGBA, so RViz renders colors directly with no Color Transformer.
    PointCloud2 on /reach_all_cloud, /reach_down_cloud: carries `intensity` (=RI)
    and a baked `rgb` field, so RViz can color by Intensity or RGB8.
    All are republished every 3 s so RViz catches a message regardless of order.
    Set RViz Fixed Frame to base_link (the data frame); not dummy_link.
    """
    import rclpy
    from rclpy.node import Node
    from std_msgs.msg import Header
    from visualization_msgs.msg import Marker  # noqa: F401
    from sensor_msgs.msg import PointCloud2  # noqa: F401

    reach_mask = ri > 0.0
    all_centers, all_ri = centers[reach_mask], ri[reach_mask]
    all_rgb = [ri_color_rgb(v) for v in all_ri]           # RI -> blue..red
    all_rgb_packed = np.array(                            # same colors for cloud
        [_pack_rgb(int(255 * c[0]), int(255 * c[1]), int(255 * c[2]))
         for c in all_rgb], dtype=np.float32)
    down_centers = centers[down_reach]
    down_rgb = [(0.0, 0.8, 0.0)] * len(down_centers)      # solid green
    down_rgb_packed = np.full(len(down_centers), _pack_rgb(0, 200, 0),
                              dtype=np.float32)

    from rclpy.parameter import Parameter
    rclpy.init()
    # Use sim time so stamps match RViz's clock when Gazebo is running. Otherwise
    # wall-clock stamps look "in the future" to a sim-time RViz and a TF lookup
    # through a sim-time dynamic chain fails, dropping the cloud/marker silently.
    node = Node('reachability_map', parameter_overrides=[
        Parameter('use_sim_time', Parameter.Type.BOOL, True)])
    pub_all = node.create_publisher(Marker, '/reach_all', 10)
    pub_down = node.create_publisher(Marker, '/reach_down', 10)
    pub_all_cloud = node.create_publisher(PointCloud2, '/reach_all_cloud', 10)
    pub_down_cloud = node.create_publisher(PointCloud2, '/reach_down_cloud', 10)

    def republish():
        h = Header()
        h.frame_id = args.frame_id
        h.stamp = node.get_clock().now().to_msg()
        pub_all.publish(_make_marker(h, all_centers, all_rgb, args.voxel,
                                     'reach_all', 0))
        pub_down.publish(_make_marker(h, down_centers, down_rgb, args.voxel,
                                      'reach_down', 1))
        pub_all_cloud.publish(_make_cloud(h, all_centers, intensity=all_ri,
                                          rgb=all_rgb_packed))
        pub_down_cloud.publish(_make_cloud(h, down_centers, rgb=down_rgb_packed))

    republish()
    node.create_timer(3.0, republish)
    node.get_logger().info(
        f'Publishing Marker (/reach_all, /reach_down) AND PointCloud2 '
        f'(/reach_all_cloud, /reach_down_cloud): all={len(all_centers)} pts, '
        f'down={len(down_centers)} pts, every 3s. RViz Fixed Frame='
        f'{args.frame_id} (NOT dummy_link). Add Marker + PointCloud2 displays '
        f'to compare. Ctrl+C to stop.')
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


# CLI / main -------------------------------------------------------------------

def parse_args(argv=None):
    p = argparse.ArgumentParser(description=__doc__,
                                formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument('--xacro', default=DEFAULT_XACRO, help='robot xacro path')
    p.add_argument('--srdf', default=DEFAULT_SRDF,
                   help='SRDF with disabled collision pairs')
    p.add_argument('--voxel', type=float, default=0.05,
                   help='voxel size in meters (default 0.05)')
    p.add_argument('--bounds', type=str, default=None,
                   help='scan box xmin,xmax,ymin,ymax,zmin,zmax in meters '
                        '(base_link frame). Omit to use the FK envelope.')
    p.add_argument('--n-dir', type=int, default=12,
                   help='approach-axis directions per voxel (default 12)')
    p.add_argument('--n-roll', type=int, default=2,
                   help='roll angles per direction (default 2)')
    p.add_argument('--seed-restarts', type=int, default=4,
                   help='random IK seeds per orientation (default 4)')
    p.add_argument('--ik-pos-tol', type=float, default=0.005,
                   help='IK position tolerance in meters (default 0.005)')
    p.add_argument('--ik-rot-tol-deg', type=float, default=5.0,
                   help='IK orientation tolerance in degrees (default 5)')
    p.add_argument('--ik-max-iter', type=int, default=100,
                   help='max IK iterations per seed (default 100)')
    p.add_argument('--envelope-samples', type=int, default=200000,
                   help='FK samples to pick candidate voxels when no --bounds '
                        '(default 200000)')
    p.add_argument('--down-tol-deg', type=float, default=15.0,
                   help='tilt tolerance (deg) for the "down" orientation set '
                        '(default 15)')
    p.add_argument('--seed', type=int, default=1, help='RNG seed')
    p.add_argument('--jobs', type=int, default=os.cpu_count() or 1,
                   help='worker processes for IK (default = CPU cores; '
                        '1 = serial/reproducible reference)')
    p.add_argument('--max-reach', type=float, default=None,
                   help='override the reach-prune radius in meters (default: '
                        'auto upper bound from the model link lengths)')
    p.add_argument('--limits-deg', type=str, default=None,
                   help='override joint limits, 12 comma values: '
                        'j1lo,j1hi,...,j6lo,j6hi (degrees)')
    p.add_argument('--out-dir', default=os.path.join(
        os.path.dirname(os.path.abspath(__file__)), 'reachability_out'),
        help='directory for CSV/PCD/meta output')
    p.add_argument('--load', type=str, default=None,
                   help='republish a previously saved CSV (no compute/model). '
                        'Pass the reach_*.csv path; "latest" picks the newest '
                        'CSV in --out-dir.')
    p.add_argument('--no-viz', action='store_true',
                   help='skip ROS publishing (compute + save only)')
    p.add_argument('--frame-id', default='base_link',
                   help='frame_id for the published clouds (default base_link)')
    return p.parse_args(argv)


def resolve_limits(args):
    if args.limits_deg:
        vals = [float(v) for v in args.limits_deg.split(',')]
        if len(vals) != 12:
            raise SystemExit('--limits-deg needs 12 comma-separated values')
        deg = [(vals[2 * i], vals[2 * i + 1]) for i in range(6)]
    else:
        deg = DEFAULT_LIMITS_DEG
    rad = [(math.radians(lo), math.radians(hi)) for lo, hi in deg]
    return deg, rad


def resolve_bounds(args):
    if not args.bounds:
        return None
    vals = [float(v) for v in args.bounds.split(',')]
    if len(vals) != 6:
        raise SystemExit('--bounds needs 6 comma-separated values: '
                         'xmin,xmax,ymin,ymax,zmin,zmax')
    return tuple(vals)


def main(argv=None):
    args = parse_args(argv)

    # Replay mode: load a saved CSV and just publish it (no model, no compute).
    if args.load:
        centers, ri, down_reach = load_csv(args.load, args.out_dir)
        publish_clouds(centers, ri, down_reach, args)
        return centers, ri, None, down_reach, None

    limits_deg, limits_rad = resolve_limits(args)
    bounds = resolve_bounds(args)

    print('[reach] loading model ...', flush=True)
    rm = ReachabilityModel(args.xacro, args.srdf)
    rm.set_joint_limits(limits_rad)
    n_all, n_srdf, n_active, n_neutral = rm.pair_stats
    print(f'[reach] collision pairs: {n_all} all -> {n_srdf} after SRDF '
          f'-> {n_active} active ({n_neutral} disabled as neutral-colliding)',
          flush=True)

    orientations, down_flags = build_orientations(
        args.n_dir, args.n_roll, args.down_tol_deg)
    print(f'[reach] orientations/voxel: {len(orientations)} '
          f'({args.n_dir} dir x {args.n_roll} roll, '
          f'{int(down_flags.sum())} flagged down)', flush=True)

    # Candidate voxels: explicit box, or FK-occupied voxels from an envelope pass.
    if bounds is not None:
        centers = bounds_to_centers(bounds, args.voxel)
        print(f'[reach] scanning --bounds box: {len(centers)} voxels '
              f'@ {args.voxel} m', flush=True)
    else:
        print(f'[reach] FK envelope pass ({args.envelope_samples} samples) '
              f'to pick candidate voxels ...', flush=True)
        centers, bbox = fk_envelope_centers(rm, args, limits_rad)
        print(f'[reach] envelope bbox (m): '
              f'x[{bbox[0]:.2f},{bbox[1]:.2f}] '
              f'y[{bbox[2]:.2f},{bbox[3]:.2f}] '
              f'z[{bbox[4]:.2f},{bbox[5]:.2f}] -> {len(centers)} candidate voxels',
              flush=True)

    print(f'[reach] solving IK for {len(centers)} voxels '
          f'x {len(orientations)} orientations '
          f'x {args.seed_restarts} seeds ...', flush=True)
    t0 = time.time()
    ri, n_ok, n_total, down_reach = compute_reachability(
        rm, centers, orientations, down_flags, args, limits_rad=limits_rad)
    elapsed = time.time() - t0
    rate = len(centers) / elapsed if elapsed > 0 else 0.0
    print(f'[reach] Elapsed Time: {_fmt_hms(elapsed)} ({elapsed:.1f}s) for '
          f'{len(centers)} voxels ({rate:.1f} voxel/s)', flush=True)

    stats = {
        'candidate_voxels': int(len(centers)),
        'reachable_voxels': int((ri > 0).sum()),
        'down_voxels': int(down_reach.sum()),
        'dexterous_voxels': int((ri >= 1.0).sum()),
        'ri_mean_over_reachable': float(ri[ri > 0].mean()) if (ri > 0).any() else 0.0,
        'ri_max': float(ri.max()) if len(ri) else 0.0,
    }
    print(f'[reach] voxels @ {args.voxel} m: candidate={stats["candidate_voxels"]}, '
          f'reachable={stats["reachable_voxels"]}, down={stats["down_voxels"]}, '
          f'dexterous={stats["dexterous_voxels"]}, '
          f'RI(mean over reachable)={stats["ri_mean_over_reachable"]:.3f}',
          flush=True)

    save_clouds(centers, ri, n_ok, n_total, down_reach, args, limits_deg, stats)

    if not args.no_viz:
        publish_clouds(centers, ri, down_reach, args)

    return centers, ri, n_ok, down_reach, stats


if __name__ == '__main__':
    main()
