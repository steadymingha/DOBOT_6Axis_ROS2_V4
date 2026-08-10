"""화면 중심에서 벗어난 정도(off-axis)가 잔차를 설명하는지."""
import json
import sys

import numpy as np

D = json.load(open(sys.argv[1]))
S = D["samples"]
P = np.array([s["P_base"] for s in S])
F = np.array([s["T_base_flange"] for s in S])
bbox = np.array([s["detection"]["bbox"] for s in S])
camz = np.array([s["detection"]["xyz_cam"][2] for s in S])
CX, CY, FX, FY = 633.64, 363.55, 650.63, 648.75      # 러너 실측 K (1280x720)
u = (bbox[:, 0] + bbox[:, 2]) / 2
v = (bbox[:, 1] + bbox[:, 3]) / 2
off_px = np.hypot(u - CX, v - CY)
off_deg = np.degrees(np.arctan2(off_px, (FX + FY) / 2))

print("summary: n=%d p95=%.2fmm travel=%.1fmm rot=%.1f°"
      % (D["n"], D["p95_mm"], D["travel_mm"], D["rot_deg"]))
print("off-axis: %.0f~%.0f px  (%.1f~%.1f°)"
      % (off_px.min(), off_px.max(), off_deg.min(), off_deg.max()))

res = (P - P.mean(axis=0)) * 1000
d = np.linalg.norm(res, axis=1)
r = np.corrcoef(off_px, d)[0, 1]
print("\n|잔차| vs off-axis 거리 : r = %+.2f %s" % (r, "★ 계통" if abs(r) > 0.5 else ""))
print("  선형 기울기 %.3f mm/px  (= %.1f mm 당 100px)"
      % (np.polyfit(off_px, d, 1)[0], np.polyfit(off_px, d, 1)[0] * 100))


def rot_angle(R):
    t = (np.trace(R) - 1.0) / 2.0
    return np.degrees(np.arccos(max(-1.0, min(1.0, t))))


def stats(mask, label):
    if mask.sum() < 3:
        print("  %-28s 표본 %d개 — 부족" % (label, mask.sum()))
        return
    p = P[mask]
    dd = np.linalg.norm((p - p.mean(axis=0)) * 1000, axis=1)
    f = F[mask]
    tr = ro = 0.0
    for i in range(len(f)):
        for j in range(i + 1, len(f)):
            tr = max(tr, np.linalg.norm(f[i][:3, 3] - f[j][:3, 3]) * 1000)
            ro = max(ro, rot_angle(f[i][:3, :3].T @ f[j][:3, :3]))
    verdict = ("INCONCLUSIVE" if tr < 100 or ro < 30
               else "PASS" if np.percentile(dd, 95) < 10 else "FAIL")
    print("  %-28s n=%2d  p95 %5.2f  중앙값 %4.2f  travel %5.1fmm/%4.1f°  %s"
          % (label, mask.sum(), np.percentile(dd, 95), np.median(dd), tr, ro, verdict))


print("\n화면 중심 근처만 채택했다면:")
for t in (350, 300, 250, 200, 150):
    stats(off_px <= t, "off-axis <= %dpx" % t)

print("\n(대조) valid_pct 를 올렸다면:")
valid = np.array([s["detection"]["valid_pct"] for s in S])
for t in (80, 85, 90):
    stats(valid >= t, "valid >= %d%%" % t)

print("\n표본별 off-axis 대 잔차:")
for i in np.argsort(-off_px):
    print("  #%2d off %4.0fpx (%4.1f°)  |d|=%5.2fmm  res[%+6.2f %+6.2f %+6.2f]  z=%.3f"
          % (i, off_px[i], off_deg[i], d[i], *res[i], camz[i]))
