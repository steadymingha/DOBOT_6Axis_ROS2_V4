"""게이트를 조였다면 p95 가 어떻게 됐을지 — 이미 모은 표본으로 사후 계산."""
import json
import sys

import numpy as np

S = json.load(open(sys.argv[1]))["samples"]
P = np.array([s["P_base"] for s in S])
camz = np.array([s["detection"]["xyz_cam"][2] for s in S])
valid = np.array([s["detection"]["valid_pct"] for s in S])
sd = np.array([s["detection"]["sd_cm"] for s in S])
score = np.array([s["detection"]["score"] for s in S])
F = np.array([s["T_base_flange"] for s in S])


def rot_angle(R):
    t = (np.trace(R) - 1.0) / 2.0
    return np.degrees(np.arccos(max(-1.0, min(1.0, t))))


def stats(mask, label):
    if mask.sum() < 3:
        print("  %-34s 표본 %d개 — 부족" % (label, mask.sum()))
        return
    p = P[mask]
    d = np.linalg.norm((p - p.mean(axis=0)) * 1000, axis=1)
    f = F[mask]
    tr = ro = 0.0
    for i in range(len(f)):
        for j in range(i + 1, len(f)):
            tr = max(tr, np.linalg.norm(f[i][:3, 3] - f[j][:3, 3]) * 1000)
            ro = max(ro, rot_angle(f[i][:3, :3].T @ f[j][:3, :3]))
    verdict = ("INCONCLUSIVE" if tr < 100 or ro < 30
               else "PASS" if np.percentile(d, 95) < 10 else "FAIL")
    print("  %-34s n=%2d  p95 %5.2f  중앙값 %4.2f  travel %5.1fmm/%4.1f°  %s"
          % (label, mask.sum(), np.percentile(d, 95), np.median(d), tr, ro, verdict))


print("현재 게이트 (valid>=70, sd<=0.5, z 0.10~0.45):")
stats(np.ones(len(S), bool), "그대로")

print("\nvalid_pct 를 올렸다면:")
for t in (80, 85, 88, 90):
    stats(valid >= t, "valid >= %d%%" % t)

print("\nsd_cm 을 조였다면:")
for t in (0.35, 0.30, 0.25):
    stats(sd <= t, "sd <= %.2fcm" % t)

print("\n거리 상한을 낮췄다면:")
for t in (0.30, 0.28, 0.25, 0.22):
    stats(camz <= t, "cam z <= %.2fm" % t)

print("\nscore 를 게이트에 넣었다면 (현재 미검사):")
for t in (0.85, 0.90, 0.93):
    stats(score >= t, "score >= %.2f" % t)

print("\n조합:")
stats((valid >= 85) & (camz <= 0.30), "valid>=85 & z<=0.30")
stats((valid >= 85) & (sd <= 0.30), "valid>=85 & sd<=0.30")
stats((valid >= 85) & (camz <= 0.30) & (sd <= 0.30), "valid>=85 & z<=0.30 & sd<=0.30")

print("\n참고: score 와 valid 분포")
print("  score %.2f~%.2f (중앙 %.2f)" % (score.min(), score.max(), np.median(score)))
print("  valid %.0f~%.0f%% (중앙 %.0f%%)" % (valid.min(), valid.max(), np.median(valid)))
