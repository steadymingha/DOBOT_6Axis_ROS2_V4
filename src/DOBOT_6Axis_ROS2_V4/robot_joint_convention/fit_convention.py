"""Fit urdf_q = sign * ctrl_q + offset to one measured (joints, flange pose) pair.

The discrete search over 90-degree offsets found nothing, so let the offsets be
CONTINUOUS. For a fixed sign pattern this is just IK: FK(s*ctrl + o) must equal
the measured flange placement, and d/do == dq, so the frame Jacobian is the
Jacobian of the fit. 64 sign patterns, one damped-Newton solve each.

Six constraints, six unknowns -- exactly determined, so most sign patterns will
"fit" something. What matters is which solution lands on offsets that look like
a real convention (multiples of 90 deg, or zero), rather than arbitrary numbers.
Throwaway diagnostic: touches no robot.
"""
import itertools, math, sys
import numpy as np
import pinocchio as pin

sys.path.insert(0, '/root/dobot_ws/src/DOBOT_6Axis_ROS2_V4')
from cr7_pnp.model import ReachabilityModel
from cr7_pnp.geometry import XACRO_PATH

CTRL = np.array([274.9270, -68.5230, -88.6950, 67.1170, 89.0550, 16.9960])
POS = np.array([-90.6716, -594.9800, -103.8902]) / 1000.0     # flange, base frame
RPY = np.radians([-179.9217, -0.6372, 167.6904])              # rx, ry, rz

m = ReachabilityModel(xacro_path=XACRO_PATH)
model, data, fid = m.model, m.data, m.frame_id
vidx = [model.idx_vs[model.getJointId(f'joint{i}')] for i in range(1, 7)]

# base_link is 30 mm above the model root and axis-aligned with it, so the
# measured pose (which is in the robot's BASE frame) has to be lifted by that
# much before it can be compared with frames the model reports in its root.
BASE_Z = m.data.oMf[model.getFrameId('base_link')].translation[2] if True else 0.0
pin.forwardKinematics(model, data, m.pin_q([0] * 6))
pin.updateFramePlacements(model, data)
BASE = data.oMf[model.getFrameId('base_link')].copy()

R_meas = (pin.utils.rotate('z', RPY[2]) @ pin.utils.rotate('y', RPY[1])
          @ pin.utils.rotate('x', RPY[0]))          # ZYX / rpy convention
TARGET = BASE * pin.SE3(R_meas, POS)                # measured pose, in model root


def fk(q_rad):
    qp = m.pin_q(q_rad)
    pin.forwardKinematics(model, data, qp)
    pin.updateFramePlacement(model, data, fid)
    return data.oMf[fid].copy(), qp


def solve(seed, iters=200, damp=1e-6):
    q = np.array(seed, dtype=float)
    for _ in range(iters):
        oMf, qp = fk(q)
        err = pin.log6(oMf.actInv(TARGET)).vector
        if np.linalg.norm(err) < 1e-8:
            return q, float(np.linalg.norm(err))
        J = pin.computeFrameJacobian(model, data, qp, fid, pin.LOCAL)[:, vidx]
        q = q + J.T @ np.linalg.solve(J @ J.T + damp * np.eye(6), err)
    oMf, _ = fk(q)
    return q, float(np.linalg.norm(pin.log6(oMf.actInv(TARGET)).vector))


def niceness(off_deg):
    """How far the offsets sit from the nearest multiple of 90 deg, worst joint."""
    return max(min(abs(((o + 45) % 90) - 45) for _ in [0]) for o in off_deg)


results = []
for signs in itertools.product((1, -1), repeat=6):
    seed = np.radians(np.array(signs) * CTRL)
    q, res = solve(seed)
    if res > 1e-6:
        continue
    off = np.degrees(q) - np.array(signs) * CTRL
    off = (off + 180) % 360 - 180
    worst = max(abs(((o + 45) % 90) - 45) for o in off)
    results.append((worst, signs, off, res))

results.sort(key=lambda r: r[0])
print(f"{len(results)} sign patterns converged. Best fits, by how close the "
      f"offsets are to multiples of 90 deg:\n")
for worst, signs, off, res in results[:8]:
    print("  signs " + " ".join(f"{s:+d}" for s in signs) +
          "   offsets(deg) " + " ".join(f"{o:+8.2f}" for o in off) +
          f"   worst-from-90x {worst:6.2f}")
if results and results[0][0] > 5.0:
    print("\nNo sign pattern lands on a clean convention: the offsets it needs "
          "are arbitrary angles.\nWith only ONE measured pose this fit is "
          "exactly determined, so it can absorb anything --\nit cannot tell a "
          "real convention from a coincidence. More poses are required.")
