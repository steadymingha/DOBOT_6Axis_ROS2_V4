"""Reduced 6-DOF CR7 model with FK, IK and self-collision (pinocchio).

Runtime IK + collision engine, copied from reachability_map.py. Only the
ReachabilityModel class is kept here -- the offline reachability-map builder
(voxel sweep, PCD/RViz output, CLI main) lives on in reachability_map.py and is
not needed by the pick-and-place runtime.

Two instances are used by the pick-and-place node:
  * whole-robot collision model (arm+cube+AGV+gripper): is_collision_free()
  * arm-only IK model: inverse_kinematics() (used instead of MoveIt's KDL IK,
    so "reachable on the map" == "IK solvable here").
"""

import os
import math
import tempfile

import numpy as np
import pinocchio as pin


# Default model / config -------------------------------------------------------

DEFAULT_XACRO = os.path.expanduser(
    '~/dobot_ws/install/cra_description/share/cra_description/urdf/cr7_robot.xacro')
# SRDF sits next to the original scripts (one level above this package).
DEFAULT_SRDF = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    'cr7_moveit', 'config', 'cr7_robot.srdf')

# Gripper / tool geometry: single source of truth in gripper_params.py (pure,
# ROS-free) so the planner and the standalone reachability_map.py share it.
from .gripper_params import (  # noqa: F401  (re-exported via cr7_pnp internals)
    ARM_JOINTS, GRIPPER_JOINTS, EE_FRAME, TCP_OFFSET_M, FINGER_OPEN_M)


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
        # Freeze the locked joints at neutral EXCEPT the gripper finger, frozen
        # at its widest commanded opening (FINGER_OPEN_M): the arm travels with
        # the gripper OPEN, and a q=0 freeze leaves the real moving jaw 30 mm
        # outside the collision model (it brushed shelf boxes the planner had
        # cleared, measured).
        q_freeze = pin.neutral(full)
        for jname in GRIPPER_JOINTS:
            if full.existJointName(jname):
                q_freeze[full.joints[full.getJointId(jname)].idx_q] = FINGER_OPEN_M
        self.model, geoms = pin.buildReducedModel(
            full, [full_geom], locked_ids, q_freeze)
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

    def colliding_pairs(self, j1_to_j6):
        """Geometry-pair names in collision at the config (diagnostic only)."""
        pin.computeCollisions(self.model, self.data, self.geom, self.geom_data,
                              self.pin_q(j1_to_j6), False)
        objs = self.geom.geometryObjects
        return [(objs[cp.first].name, objs[cp.second].name)
                for cp, res in zip(self.geom.collisionPairs,
                                   self.geom_data.collisionResults)
                if res.isCollision()]

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
