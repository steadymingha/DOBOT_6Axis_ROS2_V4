"""Constrained Bi-directional RRT (CBiRRT) for the CR7 arm, pure Python.

Copied from constrained_cbirrt.py (the dead lift_path wrapper is dropped).

This is Berenson's CBiRRT algorithm implemented directly (no OMPL planner
classes): the pip-installed OMPL python wheel cannot set/read states of a
ProjectedStateSpace from Python, so OMPL's constrained framework is not usable
here. The algorithm is identical regardless of who runs it:

  - sample a random config, PROJECT it onto the constraint manifold,
  - grow two trees (from start and goal) with projected, collision-checked
    steps, and connect them.

By default the constraint keeps the end-effector TILT fixed at the grasp pose
(rotation about the tool/approach axis is left free), so a grasped object stays
level while being carried. Pass lock_tilt_only=False to lock the full 3-DOF
orientation instead.

Forward kinematics and the constraint Jacobian come from pinocchio. Collision
checking is delegated to an external callback (the motion node passes its
pinocchio whole-robot is_state_valid).
"""

import os
import math
import tempfile
import numpy as np
import pinocchio as pin


class ConstrainedPlanner:
    def __init__(self, xacro_path,
                 arm_joints=('joint1', 'joint2', 'joint3', 'joint4', 'joint5', 'joint6'),
                 locked_joints=('gripper_finger_joint',),
                 ee_frame='Link6', lock_tilt_only=True):
        # lock_tilt_only=True locks the EE tilt (rotation about R0's x/y axes, i.e.
        # the approach/tool axis stays fixed) while leaving yaw about the tool axis
        # free -> a grasped box stays level. This is a 2-DOF constraint, which gives
        # CBiRRT a wide 4-DOF manifold and makes planning fast and reliable.
        # lock_tilt_only=False locks the full 3-DOF orientation (exact grasp pose).
        self.n = 2 if lock_tilt_only else 3
        urdf_path = self._xacro_to_urdf(xacro_path)
        full = pin.buildModelFromUrdf(urdf_path)
        # Lock the gripper joints so the planning model is the 6-DOF arm only.
        locked_ids = [full.getJointId(n) for n in locked_joints if full.existJointName(n)]
        self.model = pin.buildReducedModel(full, locked_ids, pin.neutral(full))
        self.data = self.model.createData()
        self.frame_id = self.model.getFrameId(ee_frame)
        self.arm_joints = list(arm_joints)
        # Map arm-joint order -> reduced-model configuration / velocity indices.
        self.q_index = [self.model.idx_qs[self.model.getJointId(n)] for n in self.arm_joints]
        self.v_index = [self.model.idx_vs[self.model.getJointId(n)] for n in self.arm_joints]
        self.R0 = np.eye(3)  # reference (grasp) orientation; set via set_reference()

    def set_reference(self, quat_xyzw):
        """Set the orientation to hold (the grasp pose orientation), xyzw order."""
        x, y, z, w = quat_xyzw
        self.R0 = pin.Quaternion(w, x, y, z).normalized().matrix()

    @staticmethod
    def _xacro_to_urdf(xacro_path):
        import xacro
        doc = xacro.process_file(xacro_path)
        path = os.path.join(tempfile.gettempdir(), 'cr7_cbirrt_model.urdf')
        with open(path, 'w') as f:
            f.write(doc.toxml())
        return path

    # --- kinematics / constraint (pinocchio) ---

    def _pin_q(self, q):
        qp = np.zeros(self.model.nq)
        for k, idx in enumerate(self.q_index):
            qp[idx] = q[k]
        return qp

    def _err_and_jac(self, q):
        """Orientation error e = log3(R0^T R(q)) and its 3x6 Jacobian d e / dq.

        Using the local angular Jacobian Jw_local (Rdot = R [w_local]x) gives
        e_dot = Jlog3(R0^T R) @ Jw_local, which converges robustly even when the
        sampled configuration is far from the manifold.
        """
        qp = self._pin_q(q)
        pin.forwardKinematics(self.model, self.data, qp)
        pin.updateFramePlacement(self.model, self.data, self.frame_id)
        R = self.data.oMf[self.frame_id].rotation
        R_err = self.R0.T @ R
        e = pin.log3(R_err)
        J = pin.computeFrameJacobian(self.model, self.data, qp, self.frame_id, pin.LOCAL)
        Jw_local = J[3:6, :][:, self.v_index]
        Jc = pin.Jlog3(R_err) @ Jw_local
        return e, Jc

    def linear_path(self, start_q, delta, is_valid_fn, bounds,
                    step=0.003, margin=math.radians(2.0), max_substeps=120,
                    damp=1e-3):
        """Cartesian straight-line move: translate the EE by the 3-vector `delta`
        (metres, in the model/base_link frame) while holding its CURRENT full
        orientation, via a pinocchio-Jacobian servo seeded from start_q (no IK
        branch jump). Advances in small steps along the delta direction and STOPS
        safely as soon as the next step would hit a joint limit (within margin),
        be in collision, or fail to converge. Returns (path, reached_metres,
        reason) where reached_metres is the straight-line distance actually
        travelled and reason is one of "done" (full distance reached),
        "singular" (IK failed to converge -> singular/unreachable), "limit"
        (would cross a joint limit) or "collision"."""
        lo = np.array([b[0] for b in bounds], dtype=float) + margin
        hi = np.array([b[1] for b in bounds], dtype=float) - margin
        delta = np.asarray(delta, dtype=float)
        total = float(np.linalg.norm(delta))
        q = np.array(start_q, dtype=float)
        qp = self._pin_q(q)
        pin.forwardKinematics(self.model, self.data, qp)
        pin.updateFramePlacement(self.model, self.data, self.frame_id)
        p0 = self.data.oMf[self.frame_id].translation.copy()
        R0 = self.data.oMf[self.frame_id].rotation.copy()

        path = [list(q)]
        reached = 0.0
        reason = "done"
        self.last_invalid_q = None   # colliding config, for the caller's diagnostics
        if total < 1e-9:
            return path, reached, reason
        unit = delta / total
        # ceil so the final step lands exactly on `total` (int() would undershoot
        # by up to one `step`, making a full move look short to the caller).
        n = max(1, int(math.ceil(total / step)))
        for k in range(1, n + 1):
            dist = min(step * k, total)
            oMdes = pin.SE3(R0, p0 + unit * dist)
            q_try = q.copy()
            for _ in range(max_substeps):
                qp = self._pin_q(q_try)
                pin.forwardKinematics(self.model, self.data, qp)
                pin.updateFramePlacement(self.model, self.data, self.frame_id)
                iMd = self.data.oMf[self.frame_id].actInv(oMdes)
                err = pin.log6(iMd).vector
                if np.linalg.norm(err) < 1e-5:
                    break
                J = pin.computeFrameJacobian(self.model, self.data, qp, self.frame_id, pin.LOCAL)
                J = -np.dot(pin.Jlog6(iMd.inverse()), J)[:, self.v_index]
                # Damped least squares: damp^2 regularises J near singularities so
                # the update stays bounded (1e-8 was effectively undamped -> blew up
                # at sigma_min ~ 0.017). damp is a position-vs-stability tradeoff.
                v = -J.T @ np.linalg.solve(J @ J.T + damp * np.eye(6), err)
                q_try = q_try + 0.5 * v
            if np.linalg.norm(err) > 1e-3:
                # DLS loop did not converge for this 3 mm step. Report the smallest
                # singular value of the GEOMETRIC frame Jacobian at the last GOOD
                # config `q` (not the diverged q_try) so it reflects the actual
                # manipulability at the wall: sigma_min ~ 0 => true singularity /
                # reach boundary; sigma_min >> 0 => numerical (raise damp).
                qp_g = self._pin_q(q)
                pin.forwardKinematics(self.model, self.data, qp_g)
                Jg = pin.computeFrameJacobian(
                    self.model, self.data, qp_g, self.frame_id, pin.LOCAL)[:, self.v_index]
                sigma_min = float(np.linalg.svd(Jg, compute_uv=False)[-1])
                reason = f"singular(sigma_min={sigma_min:.4f})"; break
            if np.any(q_try < lo) or np.any(q_try > hi):
                reason = "limit"; break    # would hit a joint limit -> stop here
            if not is_valid_fn(list(q_try)):
                self.last_invalid_q = list(q_try)
                reason = "collision"; break  # collision -> stop here
            q = q_try
            path.append(list(q))
            reached = dist
        return path, reached, reason

    def _project(self, q, tol=1e-3, iters=150):
        """Newton projection of q onto the orientation manifold (or None)."""
        q = np.array(q, dtype=float)
        for _ in range(iters):
            e, J = self._err_and_jac(q)
            e, J = e[:self.n], J[:self.n]
            if np.linalg.norm(e) < tol:
                return q
            dq = np.linalg.lstsq(J, e, rcond=None)[0]
            q = q - dq
        return q if np.linalg.norm(self._err_and_jac(q)[0][:self.n]) < tol else None

    # --- bidirectional, projected RRT (CBiRRT) ---

    def plan(self, start_q, goal_q, is_valid_fn, bounds,
             step=0.3, max_iter=5000, tol=1e-3, edge_res=0.1, connect_bias=0.3,
             time_limit=30.0):
        import time as _time
        t_start = _time.time()
        lo = np.array([b[0] for b in bounds], dtype=float)
        hi = np.array([b[1] for b in bounds], dtype=float)

        start = self._project(start_q, tol=tol)
        goal = self._project(goal_q, tol=tol)
        if start is None or goal is None:
            return None
        if not is_valid_fn(list(start)) or not is_valid_fn(list(goal)):
            return None

        def nearest(tree, q):
            return int(np.argmin([np.linalg.norm(n[0] - q) for n in tree]))

        def edge_valid(q1, q2):
            n = max(1, int(np.linalg.norm(q2 - q1) / edge_res))
            for k in range(1, n + 1):
                if not is_valid_fn(list(q1 + (q2 - q1) * (k / n))):
                    return False
            return True

        def extend(tree, q_target):
            i = nearest(tree, q_target)
            q_near = tree[i][0]
            d = q_target - q_near
            dist = np.linalg.norm(d)
            q_step = q_target if dist < step else q_near + d / dist * step
            q_new = self._project(q_step, tol=tol)
            if q_new is None or np.any(q_new < lo) or np.any(q_new > hi):
                return None, None
            if not edge_valid(q_near, q_new):
                return None, None
            tree.append((q_new, i))
            return len(tree) - 1, q_new

        def connect(tree, q_target):
            # Greedily extend toward q_target. Bail out if (a) the time budget is
            # spent or (b) a step stops making progress -- on the constraint
            # manifold the projection can snap each step back without getting
            # closer, which would otherwise spin here forever and never let the
            # outer loop re-check the time limit.
            idx = None
            prev_dist = np.inf
            while True:
                if _time.time() - t_start > time_limit:
                    return None
                new_idx, q_new = extend(tree, q_target)
                if new_idx is None:
                    return None
                idx = new_idx
                dist = np.linalg.norm(q_new - q_target)
                if dist < step:
                    return idx
                if dist > prev_dist - 1e-3:   # no meaningful progress -> give up
                    return None
                prev_dist = dist

        def trace(tree, idx):
            path = []
            while idx != -1:
                path.append(tree[idx][0])
                idx = tree[idx][1]
            return path[::-1]

        Ta, Tb = [(start, -1)], [(goal, -1)]
        a_is_start = True
        for _ in range(max_iter):
            if _time.time() - t_start > time_limit:
                return None
            if np.random.rand() < connect_bias:
                q_rand = Tb[0][0]  # bias toward the other tree's root
            else:
                q_rand = self._project(lo + np.random.rand(len(lo)) * (hi - lo), tol=tol)
                if q_rand is None:
                    continue
            idx_a, q_new = extend(Ta, q_rand)
            if idx_a is not None:
                idx_b = connect(Tb, q_new)
                if idx_b is not None:
                    path_a = trace(Ta, idx_a)
                    path_b = trace(Tb, idx_b)
                    if a_is_start:
                        full = path_a + path_b[::-1][1:]
                    else:
                        full = path_b + path_a[::-1][1:]
                    return [list(map(float, q)) for q in full]
            Ta, Tb = Tb, Ta
            a_is_start = not a_is_start
        return None
