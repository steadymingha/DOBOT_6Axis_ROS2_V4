"""Find how the controller's joint angles map onto the URDF's, from one measured
pose. Throwaway diagnostic: reads nothing from the robot, moves nothing.

urdf_i = s_i * ctrl_i + o_i, with s_i in {+1,-1} and o_i a multiple of 90 deg.

J1 only spins the whole arm about base z, so the flange's RADIUS and Z do not
depend on it -- search J2..J6 against those two numbers (10^5 combos), then read
J1 straight off the azimuth. That is 10x cheaper than a blind 10^6 search and it
separates the one joint that is degenerate with the others.
"""
import itertools, math, sys
import numpy as np
import pinocchio as pin

sys.path.insert(0, '/root/dobot_ws/src/DOBOT_6Axis_ROS2_V4')
from cr7_pnp.model import ReachabilityModel
from cr7_pnp.geometry import XACRO_PATH

# Measured on the real robot: GetAngle() and GetPose(user=0,tool=0), tool 0 so
# the pose is the FLANGE, not any gripper TCP.
CTRL = [274.9270, -68.5230, -88.6950, 67.1170, 89.0550, 16.9960]
FLANGE = np.array([-90.6716, -594.9800, -103.8902]) / 1000.0
TARGET_R = float(np.hypot(FLANGE[0], FLANGE[1]))
TARGET_Z = float(FLANGE[2])
TARGET_AZ = math.degrees(math.atan2(FLANGE[1], FLANGE[0]))

m = ReachabilityModel(xacro_path=XACRO_PATH)
model, data, fid = m.model, m.data, m.frame_id


def flange(q_deg):
    qp = m.pin_q([math.radians(v) for v in q_deg])
    pin.forwardKinematics(model, data, qp)
    pin.updateFramePlacement(model, data, fid)
    return data.oMf[fid].translation.copy(), data.oMf[fid].rotation.copy()


print(f"target: r={TARGET_R*1000:.1f} mm  z={TARGET_Z*1000:.1f} mm  az={TARGET_AZ:.1f} deg")
base, _ = flange(CTRL)
print(f"as-is : r={np.hypot(base[0], base[1])*1000:.1f} mm  z={base[2]*1000:.1f} mm  "
      f"az={math.degrees(math.atan2(base[1], base[0])):.1f} deg\n")

SIGNS = (1, -1)
OFFSETS = (-180, -90, 0, 90, 180)
TOL_M = 0.005          # 5 mm on both radius and height

hits = []
combos = itertools.product(*[list(itertools.product(SIGNS, OFFSETS))] * 5)
for n, (c2, c3, c4, c5, c6) in enumerate(combos):
    q = [0.0]
    for (s, o), v in zip((c2, c3, c4, c5, c6), CTRL[1:]):
        q.append(s * v + o)
    p, _ = flange(q)
    if abs(np.hypot(p[0], p[1]) - TARGET_R) < TOL_M and abs(p[2] - TARGET_Z) < TOL_M:
        az0 = math.degrees(math.atan2(p[1], p[0]))
        need = (TARGET_AZ - az0 + 180) % 360 - 180        # J1 rotation required
        for s1 in SIGNS:
            o1 = (need - s1 * CTRL[0] + 180) % 360 - 180
            if min(abs(o1 - k) for k in (-180, -90, 0, 90, 180)) < 1.0:
                hits.append(((s1, round(o1)), c2, c3, c4, c5, c6))
print(f"searched {n+1} combos for J2..J6\n")

if not hits:
    print("NO clean sign/offset combo reproduces the measured flange.")
    print("The mismatch is not a per-joint sign/90-deg-offset relabelling.")
else:
    print(f"{len(hits)} candidate mapping(s):")
    for h in hits[:20]:
        q = [h[0][0] * CTRL[0] + h[0][1]]
        for (s, o), v in zip(h[1:], CTRL[1:]):
            q.append(s * v + o)
        p, R = flange(q)
        err = np.linalg.norm(p - FLANGE) * 1000
        print("  " + "  ".join(f"J{i+1}:{'+' if s > 0 else '-'}{o:+4d}"
                               for i, (s, o) in enumerate(h)) +
              f"   pos err {err:5.1f} mm   tool axis ({R[0,2]:+.2f},{R[1,2]:+.2f},{R[2,2]:+.2f})")
