"""축별 분해 — 어느 축이 p95 를 만들었나."""
import json
import sys

import numpy as np


def report(name, path):
    D = json.load(open(path))
    S = D["samples"]
    P = np.array([s["P_base"] for s in S])
    mean = P.mean(axis=0)
    dev = np.abs(P - mean) * 1000                 # 축별 |편차| mm
    d3 = np.linalg.norm(P - mean, axis=1) * 1000  # 3D 거리 mm

    print("\n%s  (n=%d, travel %.0fmm / %.0f°)"
          % (name, D["n"], D["travel_mm"], D["rot_deg"]))
    print("  축   std     p95    최대   |  3D 거리에 대한 기여")
    tot = 0.0
    for i, ax in enumerate("XYZ"):
        p95 = np.percentile(dev[:, i], 95)
        tot += p95 ** 2
        print("  %s  %5.2f  %6.2f  %6.2f mm" % (ax, dev[:, i].std(), p95, dev[:, i].max()))
    print("  3D  %5s  %6.2f  %6.2f mm   (축별 p95 를 제곱합하면 %.2f)"
          % ("-", np.percentile(d3, 95), d3.max(), np.sqrt(tot)))

    # 상위 표본에서 어느 축이 지배적인가
    worst = np.argsort(-d3)[:5]
    share = dev[worst] ** 2 / (d3[worst] ** 2)[:, None] * 100
    print("  상위 5표본의 축별 기여율(%):")
    for k, i in enumerate(worst):
        print("    |d|=%5.2fmm  X %3.0f%%  Y %3.0f%%  Z %3.0f%%"
              % (d3[i], *share[k]))
    return P, mean


runs = [("run1 (넓은 자세)", sys.argv[1]), ("run2 (넓은 자세)", sys.argv[2])]
mats = []
for name, path in runs:
    mats.append(report(name, path))

print("\n합친 46표본")
P = np.vstack([m[0] for m in mats])
mean = P.mean(axis=0)
dev = np.abs(P - mean) * 1000
d3 = np.linalg.norm(P - mean, axis=1) * 1000
for i, ax in enumerate("XYZ"):
    print("  %s  std %5.2f  p95 %6.2f  최대 %6.2f mm"
          % (ax, dev[:, i].std(), np.percentile(dev[:, i], 95), dev[:, i].max()))
print("  3D          p95 %6.2f  최대 %6.2f mm"
      % (np.percentile(d3, 95), d3.max()))
