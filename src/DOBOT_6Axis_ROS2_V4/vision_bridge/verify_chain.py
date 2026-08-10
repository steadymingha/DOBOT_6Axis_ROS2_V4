#!/usr/bin/env python3
"""변환 사슬 검증 — 매거진을 고정해두고 팔을 손으로 조깅하며 P_base 가 불변인지 본다.

hand-eye verify 와 같은 원리를 러너 파이프라인 전체에 적용한 것:
    YOLOX bbox → ring depth → 역투영(xyz_cam) → T_base_flange @ T_flange_cam
어느 자세에서 봐도 같은 물체는 base 좌표에서 같은 점이어야 한다.

★ 로봇에는 아무것도 보내지 않는다. 30004 실시간 피드백만 읽는다 (read-only).
   콘솔 전용, ROS2 노드 아님 — bringup 없이 단독 실행된다.

실행:
    python3 verify_chain.py
    (호스트에서 vision_runner 가 이미 떠 있어야 한다)

조작: 팔을 손으로 여기저기 옮기고 잠깐씩 멈춘다. 멈출 때마다 표본이 하나 쌓인다.
      끝내려면 q + Enter, 또는 Ctrl+C.
"""
import argparse
import json
import os
import select
import sys
import time

import numpy as np

import config
from gate import Gate
from transform import Transform
from vision_client import VisionClient, detections

sys.path.insert(0, config.HANDEYE_DIR)
from handeye_calib import RobotFeed, STILL_DPS, rot_angle   # noqa: E402


def pose_is_new(T, taken):
    """handeye_calib._pose_is_new 와 같은 식. 임계만 포락선에 맞춰 줄였다
    (원본 3cm/8° 는 ±2cm 포락선보다 커서 두 번째 표본이 영영 안 잡힌다)."""
    for U in taken:
        if (np.linalg.norm(T[:3, 3] - U[:3, 3]) < config.NOVELTY_TRANS_M
                and rot_angle(U[:3, :3].T @ T[:3, :3]) < config.NOVELTY_ROT_DEG):
            return False
    return True


def coverage(flanges):
    """표본 flange 자세들의 최대 상호 이동/회전 (판정에 쓰는 것과 같은 계산)."""
    tr = ro = 0.0
    for i in range(len(flanges)):
        for j in range(i + 1, len(flanges)):
            tr = max(tr, float(np.linalg.norm(
                flanges[i][:3, 3] - flanges[j][:3, 3]) * 1000))
            ro = max(ro, rot_angle(flanges[i][:3, :3].T @ flanges[j][:3, :3]))
    return tr, ro


def progress_line(samples, flanges):
    """표본이 다 찼는지 한 줄로. 조깅하며 언제 멈춰도 되는지 알 수 있게."""
    tr, ro = coverage(flanges)
    n = len(samples)
    ok_tr = tr >= config.VERIFY_MIN_TRAVEL_MM
    ok_ro = ro >= config.VERIFY_MIN_ROT_DEG
    ok_n = n >= config.VERIFY_WANT_SAMPLES
    mark = lambda ok: "OK" if ok else "..."           # noqa: E731
    need = []
    if not ok_tr:
        need.append("이동 %.0fmm 더" % (config.VERIFY_MIN_TRAVEL_MM - tr))
    if not ok_ro:
        need.append("회전 %.1f° 더" % (config.VERIFY_MIN_ROT_DEG - ro))
    if not ok_n:
        need.append("표본 %d개 더" % (config.VERIFY_WANT_SAMPLES - n))
    tail = ("→ 필요: " + ", ".join(need)) if need else "→ 조건 충족. q + Enter 로 종료 가능"
    return ("       진행: travel %5.1f/%.0fmm %s   회전 %4.1f/%.1f° %s   "
            "표본 %2d/%d %s   %s"
            % (tr, config.VERIFY_MIN_TRAVEL_MM, mark(ok_tr),
               ro, config.VERIFY_MIN_ROT_DEG, mark(ok_ro),
               n, config.VERIFY_WANT_SAMPLES, mark(ok_n), tail))


def offset_hint(T, T_ref):
    """기준 자세에서 얼마나 벗어나 있는지 — 조깅하며 포락선을 가늠하라고 붙인다."""
    if T_ref is None:
        return " [기준자세]"
    dp = np.linalg.norm(T[:3, 3] - T_ref[:3, 3]) * 1000
    dr = rot_angle(T_ref[:3, :3].T @ T[:3, :3])
    return " [기준 대비 %4.1fmm / %3.1f°, 한계 %.0f/%.1f]" % (
        dp, dr, config.ENVELOPE_POS_MM, config.ENVELOPE_ROT_DEG)


def in_envelope(T, T_ref):
    """기준 자세 대비 AMR 정차 오차 범위 안인지. (통과, 사유)."""
    if T_ref is None:
        return True, ""
    dp = np.linalg.norm(T[:3, 3] - T_ref[:3, 3]) * 1000
    dr = rot_angle(T_ref[:3, :3].T @ T[:3, :3])
    if dp > config.ENVELOPE_POS_MM:
        return False, "포락선 밖: 기준자세에서 %.0fmm (한계 %.0f)" % (
            dp, config.ENVELOPE_POS_MM)
    if dr > config.ENVELOPE_ROT_DEG:
        return False, "포락선 밖: 기준자세에서 %.1f° (한계 %.1f)" % (
            dr, config.ENVELOPE_ROT_DEG)
    return True, ""


def quit_requested():
    """터미널에 q 가 들어왔는지 (논블로킹). tty 가 아니면 항상 False."""
    if not sys.stdin.isatty():
        return False
    if not select.select([sys.stdin], [], [], 0)[0]:
        return False
    return sys.stdin.readline().strip().lower().startswith("q")


def summarize(samples, out_path, rejects=None):
    print("\n" + "=" * 72)
    n = len(samples)
    if rejects:
        print("거부 사유 (프레임 수):")
        for k, v in sorted(rejects.items(), key=lambda kv: -kv[1]):
            print("  %-14s %6d" % (k, v))
        print("-" * 72)
    if n == 0:
        print("표본 0개 — 판정 불가. 위 거부 사유를 볼 것.")
        print("=" * 72)
        return

    P = np.array([s["P_base"] for s in samples])
    F = np.array([s["T_base_flange"] for s in samples])
    mean = P.mean(axis=0)
    std_mm = P.std(axis=0) * 1000
    dist_mm = np.linalg.norm(P - mean, axis=1) * 1000
    p95_mm = float(np.percentile(dist_mm, 95)) if n > 1 else 0.0

    # 팔이 실제로 얼마나 돌아다녔나 — 표본 flange 자세들의 최대 상호 거리/각도
    travel_mm = rot_deg = 0.0
    for i in range(n):
        for j in range(i + 1, n):
            travel_mm = max(travel_mm,
                            float(np.linalg.norm(F[i][:3, 3] - F[j][:3, 3]) * 1000))
            rot_deg = max(rot_deg, rot_angle(F[i][:3, :3].T @ F[j][:3, :3]))

    print("표본        : %d개%s" % (n, "" if n >= config.VERIFY_WANT_SAMPLES
                                       else "  ← %d개 미만이라 p95 가 불안정하다"
                                            % config.VERIFY_WANT_SAMPLES))
    print("P_base 평균 : [%+.4f, %+.4f, %+.4f] m" % tuple(mean))
    print("per-axis std: X %.2f  Y %.2f  Z %.2f mm" % tuple(std_mm))
    print("산포 p95    : %.2f mm   (평균점까지 거리의 95퍼센타일)" % p95_mm)
    print("             최대 %.2f mm / 중앙값 %.2f mm"
          % (dist_mm.max(), float(np.median(dist_mm))))
    dev = np.abs(P - mean) * 1000
    print("축별 p95    : X %.2f  Y %.2f  Z %.2f mm   (평균 대비 |편차|)"
          % tuple(np.percentile(dev, 95, axis=0)))
    print("축별 최대   : X %.2f  Y %.2f  Z %.2f mm" % tuple(dev.max(axis=0)))
    print("팔 이동량   : travel %.1f mm   wrist %.1f deg  (표본 간 최대)"
          % (travel_mm, rot_deg))
    intra = np.array([s["detection"].get("xyz_std_mm", [0, 0, 0]) for s in samples])
    bstd = np.array([s["detection"].get("bbox_center_std_px", [0, 0]) for s in samples])
    print("프레임내    : xyz std 중앙값 %.2f/%.2f/%.2f mm, "
          "bbox 중심 %.1f/%.1f px  (평균으로 이미 줄인 몫)"
          % (*np.median(intra, axis=0), *np.median(bstd, axis=0)))
    print("-" * 72)

    if travel_mm < config.VERIFY_MIN_TRAVEL_MM or rot_deg < config.VERIFY_MIN_ROT_DEG:
        print("판정: INCONCLUSIVE")
        short = []
        if travel_mm < config.VERIFY_MIN_TRAVEL_MM:
            short.append("이동 %.1fmm 부족 (%.1f / 기준 %.1f)"
                         % (config.VERIFY_MIN_TRAVEL_MM - travel_mm,
                            travel_mm, config.VERIFY_MIN_TRAVEL_MM))
        if rot_deg < config.VERIFY_MIN_ROT_DEG:
            short.append("회전 %.1f° 부족 (%.1f / 기준 %.1f)"
                         % (config.VERIFY_MIN_ROT_DEG - rot_deg,
                            rot_deg, config.VERIFY_MIN_ROT_DEG))
        print("  팔을 더 조깅할 것 — " + ", ".join(short))
        print("  자세가 비슷하면 사슬이 틀려도 P_base 가 같이 나온다. 구분이 안 된다.")
    elif p95_mm < config.VERIFY_PASS_P95_MM:
        print("판정: PASS   (p95 %.2f mm < %.0f mm)"
              % (p95_mm, config.VERIFY_PASS_P95_MM))
        print("  정확도 예산에 p95 %.2f mm 로 기록할 것." % p95_mm)
    else:
        print("판정: FAIL   (p95 %.2f mm >= %.0f mm)"
              % (p95_mm, config.VERIFY_PASS_P95_MM))
        print("  의심 순서: hand-eye → 역투영 대표픽셀(bbox 중심) → ring-전면 단차")
        print("  표본 json 의 cam z 와 bbox 를 보면 갈라진다.")
        print("  자세에 따라 체계적으로(랜덤 아니라) 이동하면 hand-eye 또는 곱 순서.")

    # 덮어쓰기 전에 이전 세션을 옆으로 밀어둔다. 표본 json 은 나중에
    # 기준선·재분석·가설검정에 쓰이므로 조용히 날아가면 안 된다.
    if os.path.exists(out_path):
        keep = "%s_%s.json" % (out_path[:-5],
                               time.strftime("%m%d_%H%M",
                                             time.localtime(os.path.getmtime(out_path))))
        os.replace(out_path, keep)
        print("이전 세션 보관: %s" % keep)

    with open(out_path, "w") as fp:
        json.dump({
            "n": n,
            "mean_m": mean.tolist(),
            "std_mm": std_mm.tolist(),
            "p95_mm": p95_mm,
            "p95_axis_mm": np.percentile(np.abs(P - mean) * 1000, 95, axis=0).tolist(),
            "envelope": {"pos_mm": config.ENVELOPE_POS_MM,
                         "rot_deg": config.ENVELOPE_ROT_DEG,
                         "z_m": [config.ENVELOPE_Z_MIN, config.ENVELOPE_Z_MAX]},
            "travel_mm": travel_mm,
            "rot_deg": rot_deg,
            "samples": samples,
        }, fp, indent=2)
    print("-" * 72)
    print("원시 표본 저장: %s" % out_path)

    cls = samples[0]["detection"].get("cls", "?")
    ax = np.percentile(np.abs(P - mean) * 1000, 95, axis=0)
    print("-" * 72)
    print("[정확도 예산 기록용]")
    print("  vision→base 재현성 p95 %.2f mm (축별 X %.2f / Y %.2f / Z %.2f), "
          "표본 %d개" % (p95_mm, ax[0], ax[1], ax[2], n))
    print("  조건: 포락선 ±%.0fmm / ±%.1f°, 작업거리 %.2f~%.2f m, "
          "대상 %s, 표본=게이트통과 %d프레임 평균"
          % (config.ENVELOPE_POS_MM, config.ENVELOPE_ROT_DEG,
             config.ENVELOPE_Z_MIN, config.ENVELOPE_Z_MAX,
             cls, config.GATE_SNAPSHOT_N))
    print("  커버리지: travel %.1f mm / wrist %.1f deg" % (travel_mm, rot_deg))
    print("=" * 72)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--host", default=config.RUNNER_HOST)
    ap.add_argument("--robot-ip", default=config.ROBOT_IP)
    ap.add_argument("--out", default=config.SAMPLES_FILE)
    ap.add_argument("--append", action="store_true",
                    help="기존 --out 표본에 이어붙인다 (기준 자세도 그 세션 것을 그대로 쓴다)")
    args = ap.parse_args()

    tf = Transform()
    print("[bridge] X = T_flange_cam 로드 OK  t=%s m" % np.round(tf.X[:3, 3], 4))

    client = VisionClient(host=args.host)
    try:
        accepted, resp = client.set_mode("MAGAZINE")
    except Exception as e:
        print("[bridge] 러너 REP 응답 없음 (%s:%d): %s"
              % (args.host, config.REP_PORT, e))
        print("         호스트에서 vision_runner 를 먼저 띄울 것:")
        print("         source ~/venv_ammr/bin/activate && cd ~/robot_vision "
              "&& python runner.py")
        return 2
    if not accepted:
        print("[bridge] set_mode 거부됨: %s" % resp)
        return 2
    print("[bridge] set_mode MAGAZINE -> %s" % resp)

    feed = RobotFeed(args.robot_ip)
    feed.start()
    print("[bridge] 로봇 피드 %s:30004 (read-only)" % args.robot_ip)

    # ── 이어붙이기 ──────────────────────────────────────────────
    # 기준 자세는 반드시 원래 세션의 첫 표본에서 복원한다. 새로 잡으면 포락선이
    # 통째로 옮겨가 앞뒤 표본이 서로 다른 범위에서 모인 것이 된다.
    prev = []
    if args.append and os.path.exists(args.out):
        with open(args.out) as fp:
            prev = json.load(fp).get("samples", [])
        print("[bridge] 이어붙이기: %s 에서 표본 %d개 로드" % (args.out, len(prev)))

    gate = Gate()
    rejects = {}

    def reject(kind):
        rejects[kind] = rejects.get(kind, 0) + 1

    samples = list(prev)
    taken_flange = [np.array(s["T_base_flange"]) for s in prev]
    T_ref = taken_flange[0] if taken_flange else None   # 첫 표본 자세 = 포락선 기준
    if taken_flange:
        tr0, ro0 = coverage(taken_flange)
        print("[bridge] 기준 자세 = 기존 첫 표본. 현재 커버리지 "
              "travel %.1fmm / %.1f°" % (tr0, ro0))
    last_status = None
    last_ping = time.time()

    print("\n팔을 손으로 옮기고 잠깐씩 멈추세요. 표본 하나 = 정지 상태에서 "
          "게이트 통과 %d프레임 평균 (약 %.1f초)" % (config.GATE_SNAPSHOT_N,
                                                config.GATE_SNAPSHOT_N / 13.0))
    print("포락선: 기준 자세에서 ±%.0fmm / ±%.1f°, 작업거리 %.2f~%.2fm "
          "(AMR 정차 오차 수준)"
          % (config.ENVELOPE_POS_MM, config.ENVELOPE_ROT_DEG,
             config.ENVELOPE_Z_MIN, config.ENVELOPE_Z_MAX))
    print("(직전 표본에서 %.0fmm 또는 %.1f° 이상 움직여야 새 표본으로 셉니다)"
          % (config.NOVELTY_TRANS_M * 1000, config.NOVELTY_ROT_DEG))
    print("종료: q + Enter 또는 Ctrl+C\n")

    def status(msg):
        nonlocal last_status
        if msg != last_status:
            last_status = msg
            print("  ... %s" % msg, flush=True)

    try:
        while True:
            if quit_requested():
                break

            # 러너는 10초간 커맨드가 없으면 IDLE 로 강등된다 (5초 주기 ping 전제)
            if time.time() - last_ping >= 5.0:
                last_ping = time.time()
                try:
                    client.ping()
                except Exception as e:
                    status("러너 ping 실패: %s" % e)

            payload = client.poll(50)
            if payload is None:
                if client.silent:
                    status("runner silent (%s)"
                           % ("첫 메시지 아직 없음" if client.age_s == float("inf")
                              else "%.1fs 무소식" % client.age_s))
                continue

            frame = feed.latest(config.FEED_MAX_AGE_S)
            if frame is None:
                status("로봇 피드 없음/오래됨 (%s)" % (feed.error or "stale"))
                continue

            # ★ 스냅샷은 프레임 10장을 카메라 프레임에서 평균한다 → 그동안 팔이 정지해
            #   있어야 한다. 움직이면 누적을 버린다 (다른 좌표계의 점을 섞지 않도록).
            if float(np.max(np.abs(frame["qd"]))) >= STILL_DPS:
                gate.reset()
                reject("moving")
                status("움직이는 중 (max qd %.2f deg/s)"
                       % float(np.max(np.abs(frame["qd"]))))
                continue

            T_bf = tf.flange(frame["tool"])
            ok, why = in_envelope(T_bf, T_ref)
            if not ok:
                gate.reset()
                reject("envelope")
                status(why)
                continue
            if not pose_is_new(T_bf, taken_flange):
                gate.reset()
                reject("same_pose")
                status("같은 자세 — %.0fmm 또는 %.1f° 이상 옮긴 뒤 다시 멈추세요"
                       % (config.NOVELTY_TRANS_M * 1000, config.NOVELTY_ROT_DEG))
                continue

            det, why = gate.select(payload)
            if det is None:
                reject("gate" if "누적" not in why else "accumulating")
                status(why + offset_hint(T_bf, T_ref))
                continue

            z = det["xyz_cam"][2]
            if not (config.ENVELOPE_Z_MIN <= z <= config.ENVELOPE_Z_MAX):
                gate.reset()
                reject("work_z")
                status("작업거리 밖: cam z=%.3fm (포락선 %.2f~%.2f)"
                       % (z, config.ENVELOPE_Z_MIN, config.ENVELOPE_Z_MAX))
                continue

            P_base = tf.to_base(det["xyz_cam"], frame["tool"])
            if T_ref is None:
                T_ref = T_bf
                print("  기준 자세 고정 — 이후 ±%.0fmm / ±%.1f° 안에서만 표본을 받습니다"
                      % (config.ENVELOPE_POS_MM, config.ENVELOPE_ROT_DEG))
            taken_flange.append(T_bf)
            gate.reset()                 # 표본 하나당 새 10장을 모은다
            samples.append({
                "P_base": P_base.tolist(),
                "tool_vector": np.asarray(frame["tool"]).tolist(),
                "T_base_flange": T_bf.tolist(),
                "detection": det,
                "payload": {k: payload[k] for k in
                            ("req_id", "mode", "seq", "status", "depth_scale",
                             "git_vision", "git_runner") if k in payload},
                "wall": time.time(),
            })
            last_status = None
            print("[n=%2d]%s P_base = [%+.4f, %+.4f, %+.4f] m   "
                  "(%d프레임 평균, cam z=%.3fm, track=%s, valid=%.0f%%, sd=%.2fcm, "
                  "프레임내 흔들림 %.1f/%.1f/%.1fmm)"
                  % (len(samples), offset_hint(T_bf, T_ref),
                     P_base[0], P_base[1], P_base[2],
                     det["n_frames"], det["xyz_cam"][2], det["track_id"],
                     det["valid_pct"], det["sd_cm"], *det["xyz_std_mm"]),
                  flush=True)
            print(progress_line(samples, taken_flange), flush=True)
    except KeyboardInterrupt:
        print("\n-- 중단됨; 지금까지 모은 표본으로 요약합니다")
    finally:
        feed.stop()
        summarize(samples, args.out, rejects)
        try:
            print("[bridge] set_mode IDLE -> %s" % (client.set_mode("IDLE")[1],))
        except Exception as e:
            print("[bridge] IDLE 복귀 실패: %s" % e)
        client.close()
    return 0


if __name__ == "__main__":
    sys.exit(main())
