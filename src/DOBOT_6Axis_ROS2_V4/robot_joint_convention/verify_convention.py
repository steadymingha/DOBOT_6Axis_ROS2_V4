"""Check the candidate convention against every measured pose.

Candidate (fitted on pose 1): urdf_q = [-J1, +J2, +J3, +J4, -J5, -J6], no offsets.
Pose 1 fitted it, so pose 1 proves nothing on its own. Pose 2 is the actual test:
it was never used to derive anything, so a match there is real evidence.

Also re-ranks all 64 sign patterns against BOTH poses at once -- if the candidate
is right it should be the only one that survives.
"""
import itertools, math, sys
import numpy as np
import pinocchio as pin

sys.path.insert(0, '/root/dobot_ws/src/DOBOT_6Axis_ROS2_V4')
from cr7_pnp.model import ReachabilityModel
from cr7_pnp.geometry import XACRO_PATH

# (controller joints deg, flange x/y/z mm + rx/ry/rz deg), GetAngle + GetPose(0,0)
SAMPLES = [
    ([274.9270, -68.5230, -88.6950, 67.1170, 89.0550, 16.9960],
     [-90.6716, -594.9800, -103.8902, -179.9217, -0.6372, 167.6904]),
    ([188.0490, -36.3090, -122.5100, 67.1180, 89.0560, 16.9970],
     [-462.0481, 77.4916, 55.7889, 178.5737, -1.1873, 80.9409]),
]
CANDIDATE = (-1, 1, 1, 1, -1, -1)

m = ReachabilityModel(xacro_path=XACRO_PATH)
model, data, fid = m.model, m.data, m.frame_id

pin.forwardKinematics(model, data, m.pin_q([0] * 6))
pin.updateFramePlacements(model, data)
BASE = data.oMf[model.getFrameId('base_link')].copy()   # model root -> base_link


def measured_se3(pose):
    p = np.array(pose[:3]) / 1000.0
    rx, ry, rz = np.radians(pose[3:])
    R = pin.utils.rotate('z', rz) @ pin.utils.rotate('y', ry) @ pin.utils.rotate('x', rx)
    return pin.SE3(R, p)


def flange_in_base(ctrl_deg, signs):
    q = [math.radians(s * v) for s, v in zip(signs, ctrl_deg)]
    qp = m.pin_q(q)
    pin.forwardKinematics(model, data, qp)
    pin.updateFramePlacement(model, data, fid)
    return BASE.actInv(data.oMf[fid])


def errors(signs):
    pos, rot = [], []
    for ctrl, pose in SAMPLES:
        got = flange_in_base(ctrl, signs)
        want = measured_se3(pose)
        pos.append(np.linalg.norm(got.translation - want.translation) * 1000)
        rot.append(math.degrees(np.linalg.norm(pin.log3(got.rotation.T @ want.rotation))))
    return pos, rot


print("candidate  urdf_q = [-J1, +J2, +J3, +J4, -J5, -J6]\n")
pos, rot = errors(CANDIDATE)
for i, (p, r) in enumerate(zip(pos, rot), 1):
    tag = "(fitted on this one)" if i == 1 else "(HELD OUT -- the real test)"
    print(f"  pose {i} {tag}")
    print(f"     position error {p:7.2f} mm     orientation error {r:6.2f} deg")

print("\nall 64 sign patterns ranked by worst position error over BOTH poses:\n")
ranked = []
for signs in itertools.product((1, -1), repeat=6):
    p, r = errors(signs)
    ranked.append((max(p), max(r), signs))
ranked.sort()
for worst_p, worst_r, signs in ranked[:5]:
    mark = "  <== candidate" if signs == CANDIDATE else ""
    print("  " + " ".join(f"{s:+d}" for s in signs) +
          f"   worst pos {worst_p:8.2f} mm   worst rot {worst_r:7.2f} deg{mark}")
