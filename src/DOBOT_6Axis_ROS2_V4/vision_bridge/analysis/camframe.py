"""잔차를 카메라 광학 프레임으로 되돌려, 깊이(Z) 문제인지 측면(X/Y) 문제인지 가른다.

카메라 Z 가 크면  → ring depth / 전면 단차
카메라 X·Y 가 크면 → 역투영 대표픽셀(bbox 중심) 이나 hand-eye 회전
"""
import json
import sys

import numpy as np

X = np.array(json.load(open(sys.argv[-1]))["T_flange_cam"])


def report(name, path):
    D = json.load(open(path))
    S = D["samples"]
    P = np.array([s["P_base"] for s in S])
    F = np.array([s["T_base_flange"] for s in S])
    mean = P.mean(axis=0)
    res_b = (P - mean) * 1000

    # 각 표본의 카메라 회전 (base←cam), 잔차를 카메라 축으로 투영
    res_c = np.array([(F[i][:3, :3] @ X[:3, :3]).T @ res_b[i] for i in range(len(S))])

    axis_b = np.array([(F[i][:3, :3] @ X[:3, :3])[:, 2] for i in range(len(S))])

    print("\n%s (n=%d)" % (name, len(S)))
    print("  광축(cam +Z)의 base 성분 평균: [%+.2f %+.2f %+.2f]  "
          "→ 가장 가까운 base 축 %s"
          % (*axis_b.mean(axis=0),
             "XYZ"[int(np.argmax(np.abs(axis_b.mean(axis=0))))]))
    print("  잔차를 어느 프레임에서 보느냐:")
    for label, r in (("base  ", res_b), ("camera", res_c)):
        print("    %s  std [%5.2f %5.2f %5.2f]  p95 [%5.2f %5.2f %5.2f] mm"
              % (label, *r.std(axis=0),
                 *[np.percentile(np.abs(r[:, i]), 95) for i in range(3)]))
    # 카메라 프레임 기여율
    tot = (res_c ** 2).sum()
    share = (res_c ** 2).sum(axis=0) / tot * 100
    print("    카메라축 에너지 기여율:  X(가로) %.0f%%   Y(세로) %.0f%%   Z(깊이) %.0f%%"
          % tuple(share))

    # 깊이 잔차를 픽셀로 환산하면? (bbox 중심이 몇 px 움직인 셈인가)
    camz = np.array([s["detection"]["xyz_cam"][2] for s in S])
    fx = 650.63
    px_equiv = np.abs(res_c[:, 0]) / 1000.0 * fx / camz
    print("    카메라 X 잔차를 bbox 중심 이동으로 환산: 중앙값 %.1f px, p95 %.1f px"
          % (np.median(px_equiv), np.percentile(px_equiv, 95)))
    return res_c


rs = [report("run1", sys.argv[1]), report("run2", sys.argv[2])]
r = np.vstack(rs)
print("\n합친 %d표본 (카메라 프레임)" % len(r))
print("  std [%5.2f %5.2f %5.2f]  p95 [%5.2f %5.2f %5.2f] mm"
      % (*r.std(axis=0), *[np.percentile(np.abs(r[:, i]), 95) for i in range(3)]))
share = (r ** 2).sum(axis=0) / (r ** 2).sum() * 100
print("  기여율: X(가로) %.0f%%  Y(세로) %.0f%%  Z(깊이) %.0f%%" % tuple(share))
