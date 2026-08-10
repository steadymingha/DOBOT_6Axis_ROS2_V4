"""verify_samples.json 분해 — 산포가 랜덤인지 자세에 따른 계통오차인지."""
import json
import sys

import numpy as np

D = json.load(open(sys.argv[1]))
S = D["samples"]
n = len(S)
P = np.array([s["P_base"] for s in S])
F = np.array([s["T_base_flange"] for s in S])
X = np.array(json.load(open(sys.argv[2]))["T_flange_cam"])

mean = P.mean(axis=0)
res = (P - mean) * 1000                      # mm
dist = np.linalg.norm(res, axis=1)

camz = np.array([s["detection"]["xyz_cam"][2] for s in S])
bbox = np.array([s["detection"]["bbox"] for s in S])
u = (bbox[:, 0] + bbox[:, 2]) / 2
v = (bbox[:, 1] + bbox[:, 3]) / 2
w = bbox[:, 2] - bbox[:, 0]
h = bbox[:, 3] - bbox[:, 1]
valid = np.array([s["detection"]["valid_pct"] for s in S])
sd_cm = np.array([s["detection"]["sd_cm"] for s in S])

# 카메라 광축(+Z)의 base 방향, 그리고 카메라 위치
axis = np.array([(F[i][:3, :3] @ X[:3, :3])[:, 2] for i in range(n)])
campos = np.array([(F[i] @ X)[:3, 3] for i in range(n)])

print("표본 %d개" % n)
print("P_base 평균 [%+.4f %+.4f %+.4f] m" % tuple(mean))
print("per-axis std  X %.2f  Y %.2f  Z %.2f mm" % tuple(P.std(axis=0) * 1000))
print("잔차 |d|  중앙값 %.2f  p95 %.2f  최대 %.2f mm"
      % (np.median(dist), np.percentile(dist, 95), dist.max()))
print("cam z  %.3f ~ %.3f m   (폭 %.0f mm)" % (camz.min(), camz.max(),
                                               (camz.max() - camz.min()) * 1000))
print("bbox 중심 u %.0f~%.0f  v %.0f~%.0f   크기 %.0fx%.0f ~ %.0fx%.0f"
      % (u.min(), u.max(), v.min(), v.max(), w.min(), h.min(), w.max(), h.max()))
print("valid %.0f~%.0f%%   sd %.2f~%.2f cm" % (valid.min(), valid.max(),
                                               sd_cm.min(), sd_cm.max()))

print("\n── 잔차가 무엇을 따라가는가 (Pearson r, |r|>0.5 면 계통) ──")


def corr(name, x):
    out = []
    for i, ax in enumerate("XYZ"):
        r = np.corrcoef(x, res[:, i])[0, 1]
        out.append("%s %+.2f%s" % (ax, r, "*" if abs(r) > 0.5 else " "))
    print("  %-22s  %s" % (name, "   ".join(out)))


corr("cam z (거리)", camz)
corr("bbox u (좌우)", u)
corr("bbox v (상하)", v)
corr("bbox w (크기)", w)
corr("광축 base X", axis[:, 0])
corr("광축 base Y", axis[:, 1])
corr("광축 base Z", axis[:, 2])
corr("카메라 base X", campos[:, 0])
corr("카메라 base Y", campos[:, 1])
corr("카메라 base Z", campos[:, 2])

print("\n── 거리 구간별 평균 P_base (거리 의존 = depth 스케일/단차 의심) ──")
order = np.argsort(camz)
for lo, hi in ((0, n // 3), (n // 3, 2 * n // 3), (2 * n // 3, n)):
    idx = order[lo:hi]
    m = P[idx].mean(axis=0) * 1000
    print("  cam z %.3f~%.3f m (%2d개)  P_base [%+.1f %+.1f %+.1f] mm  "
          "평균에서 %+.1f mm"
          % (camz[idx].min(), camz[idx].max(), len(idx), *m,
             np.linalg.norm(m - mean * 1000)))

print("\n── 가장 벗어난 표본 5개 ──")
for i in np.argsort(-dist)[:5]:
    print("  #%2d |d|=%5.2fmm  res[%+6.2f %+6.2f %+6.2f]  camz=%.3f "
          "u=%4.0f v=%4.0f %3.0fx%3.0f valid=%2.0f%% sd=%.2f"
          % (i, dist[i], *res[i], camz[i], u[i], v[i], w[i], h[i],
             valid[i], sd_cm[i]))

print("\n── 그 표본들을 빼면 ──")
keep = np.argsort(dist)[:-3]
d2 = np.linalg.norm((P[keep] - P[keep].mean(axis=0)) * 1000, axis=1)
print("  상위 3개 제외 시 p95 %.2f mm (전체 %.2f mm)"
      % (np.percentile(d2, 95), np.percentile(dist, 95)))
