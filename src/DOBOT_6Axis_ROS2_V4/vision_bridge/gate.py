"""채택 게이트 — 스냅샷 하나를 만들거나 None.

**스냅샷 정의**: 프레임별 AND 조건을 전부 통과한 같은 track_id 가 연속
GATE_SNAPSHOT_N(=10) 장 모이면, 그 10장의 **평균**이 스냅샷이다.
검증(verify_chain)과 운용(FSM)이 같은 정의를 쓴다 — 다르면 검증이 운용을 대변하지 못한다.

★ 평균은 카메라 광학 프레임에서 이뤄진다. 그 10장 동안 카메라(=팔)가 움직이면
  서로 다른 좌표계의 점을 평균 내는 셈이 된다. 호출부가 정지 판정을 하고,
  움직이는 동안에는 reset() 을 불러 누적을 버려야 한다.

탈락 사유를 함께 돌려준다 (상태줄에 그대로 쓰기 위함).
"""
import numpy as np

import config


def _edge_margin(bbox, w, h):
    """bbox 네 변이 프레임 경계에서 떨어진 최소 거리 (px)."""
    return min(bbox[0], bbox[1], w - bbox[2], h - bbox[3])


def check(det, frame_w=None, frame_h=None):
    """단일 검출 판정. (통과여부, 사유). 통과면 사유는 ''."""
    frame_w = frame_w or config.FRAME_W
    frame_h = frame_h or config.FRAME_H

    if det.get("state") != "TRACKING":
        return False, "state=%s" % det.get("state")
    if det.get("stale_frames", 0) != 0:
        return False, "stale=%d" % det.get("stale_frames", 0)

    xyz = det.get("xyz_cam")
    if xyz is None:
        return False, "xyz_cam=None"

    vp = det.get("valid_pct")
    if vp is None or vp < config.GATE_VALID_PCT_MIN:
        return False, "valid=%s<%.0f" % (vp, config.GATE_VALID_PCT_MIN)

    sd = det.get("sd_cm")
    if sd is None or sd > config.GATE_SD_CM_MAX:
        return False, "sd=%s>%.2f" % (sd, config.GATE_SD_CM_MAX)

    bbox = det.get("bbox")
    if not bbox:
        return False, "bbox=None"
    m = _edge_margin(bbox, frame_w, frame_h)
    if m < config.GATE_EDGE_MARGIN_PX:
        # 경계에 걸리면 ring ROI 가 비대칭이 되어 avg_dist 의 기준점이 bbox 중심에서
        # 밀린다 → 역투영 대표픽셀 가정이 깨진다
        return False, "edge=%.0fpx<%d" % (m, config.GATE_EDGE_MARGIN_PX)

    z = xyz[2]
    if not (config.GATE_Z_MIN <= z <= config.GATE_Z_MAX):
        return False, "z=%.3f 범위밖(%.2f~%.2f)" % (z, config.GATE_Z_MIN,
                                                    config.GATE_Z_MAX)

    if config.GATE_SIZE_CHECK:
        ok, why = _size_ok(det, bbox, z)
        if not ok:
            return False, why

    return True, ""


def _size_ok(det, bbox, z):
    """크기 교차검증: 핀홀상 K = px * z 이므로 K/px 가 곧 거리 추정치다."""
    grp = config.CLASS_GROUP.get(det.get("cls"))
    k = config.SIZE_K.get(grp)
    if k is None:
        return True, ""                     # 미등록 클래스는 통과
    w_px, h_px = bbox[2] - bbox[0], bbox[3] - bbox[1]
    for name, kk, px in (("w", k["w"], w_px), ("h", k["h"], h_px)):
        if px <= 0:
            return False, "size_%s px<=0" % name
        rel = abs(kk / px - z) / z
        if rel > config.GATE_SIZE_TOL:
            return False, "size_%s %.0f%%>%.0f%%" % (name, rel * 100,
                                                     config.GATE_SIZE_TOL * 100)
    return True, ""


def _snapshot(buf):
    """통과 프레임 buf(10장)의 평균 = 스냅샷. 흔들림 진단용 표준편차도 함께 낸다."""
    xyz = np.array([d["xyz_cam"] for d in buf])
    bbox = np.array([d["bbox"] for d in buf])
    first = buf[0]
    return {
        "track_id": first["track_id"],
        "cls": first["cls"],
        "state": first["state"],
        "n_frames": len(buf),
        "xyz_cam": xyz.mean(axis=0).tolist(),
        # 이 스냅샷 안에서의 흔들림. 자세 간 산포와 구분해서 보려고 싣는다
        "xyz_std_mm": (xyz.std(axis=0) * 1000).tolist(),
        "bbox": bbox.mean(axis=0).tolist(),
        "bbox_center_std_px": [
            float(((bbox[:, 0] + bbox[:, 2]) / 2).std()),
            float(((bbox[:, 1] + bbox[:, 3]) / 2).std()),
        ],
        "score": float(np.mean([d["score"] for d in buf])),
        "avg_dist_cm": float(np.mean([d["avg_dist_cm"] for d in buf])),
        "valid_pct": float(np.mean([d["valid_pct"] for d in buf])),
        "sd_cm": float(np.mean([d["sd_cm"] for d in buf])),
    }


class Gate:
    """상태 있는 게이트. 메시지마다 select() 를 한 번씩 호출한다.

    통과 프레임을 모으다가 GATE_SNAPSHOT_N 장이 차면 평균 스냅샷을 내고 버퍼를 비운다.
    """

    def __init__(self, frame_w=None, frame_h=None, snapshot_n=None):
        self.frame_w = frame_w or config.FRAME_W
        self.frame_h = frame_h or config.FRAME_H
        self.snapshot_n = snapshot_n or config.GATE_SNAPSHOT_N
        self._id = None
        self._buf = []
        self._last_seq = None

    def reset(self):
        """누적 폐기. 팔이 움직이기 시작하면 호출부가 반드시 불러야 한다."""
        self._id, self._buf = None, []

    @property
    def n_buffered(self):
        return len(self._buf)

    def select(self, payload):
        """(스냅샷 dict 또는 None, 사유 문자열)."""
        if not payload:
            return None, "no payload"
        if payload.get("seq") == self._last_seq:
            return None, "same seq"         # 같은 메시지를 두 번 세지 않는다
        self._last_seq = payload.get("seq")

        if payload.get("mode") != "MAGAZINE":
            self.reset()
            return None, "mode=%s" % payload.get("mode")
        if payload.get("status") != "ok":
            self.reset()
            return None, "status=%s" % payload.get("status")

        passed, reasons = [], []
        for d in payload.get("detections") or []:
            ok, why = check(d, self.frame_w, self.frame_h)
            (passed if ok else reasons).append(d if ok else
                                               "ID%s %s" % (d.get("track_id"), why))
        if not passed:
            self.reset()
            return None, ("게이트 탈락: " + "; ".join(reasons)) if reasons \
                else "검출 없음"

        # 여럿이면 score 가 가장 높은 것. 매거진이 한 대씩 오는 전제라 단순 선택으로 족하다
        best = max(passed, key=lambda d: d.get("score", 0.0))
        tid = best.get("track_id")
        if tid == self._id:
            self._buf.append(best)
        else:
            self._id, self._buf = tid, [best]

        if len(self._buf) < self.snapshot_n:
            return None, "누적 %d/%d (ID%s)" % (len(self._buf), self.snapshot_n, tid)

        snap = _snapshot(self._buf)
        self._buf = []          # 스냅샷 하나당 프레임 10장을 새로 모은다
        return snap, ""


if __name__ == "__main__":
    # 자가시험: 각 조건이 실제로 막는지. 하드웨어 불필요.
    base = {"track_id": 3, "cls": "front_grill1", "state": "TRACKING",
            "stale_frames": 0, "score": 0.9, "bbox": [552, 134, 839, 647],
            "avg_dist_cm": 17.7, "xyz_cam": [0.017, 0.007, 0.1773],
            "valid_pct": 88.0, "sd_cm": 0.21}
    assert check(base)[0], check(base)
    for field, bad in (("state", "LOST"), ("stale_frames", 2), ("valid_pct", 50.0),
                       ("sd_cm", 1.2), ("xyz_cam", None)):
        d = dict(base, **{field: bad})
        ok, why = check(d)
        assert not ok, "%s=%r 를 통과시켰다" % (field, bad)
        print("  막힘 %-14s -> %s" % (field, why))
    d = dict(base, bbox=[552, 134, 839, 715])            # 아래 5px 만 남음
    assert not check(d)[0] and "edge" in check(d)[1]
    print("  막힘 %-14s -> %s" % ("bbox edge", check(d)[1]))
    d = dict(base, xyz_cam=[0.0, 0.0, 0.9])
    assert not check(d)[0] and "범위밖" in check(d)[1]
    print("  막힘 %-14s -> %s" % ("z range", check(d)[1]))

    # 스냅샷: 10장이 차야 나오고, 값은 그 평균이어야 한다
    g = Gate()
    got = None
    for i in range(1, config.GATE_SNAPSHOT_N + 1):
        d = dict(base, xyz_cam=[0.017, 0.007, 0.170 + 0.001 * i])   # z 를 1mm 씩 흔든다
        got, why = g.select({"mode": "MAGAZINE", "status": "ok", "seq": i,
                             "detections": [d]})
        if got is None:
            print("  누적 %2d -> %s" % (i, why))
    assert got is not None, "%d장인데 스냅샷이 안 나왔다" % config.GATE_SNAPSHOT_N
    want_z = sum(0.170 + 0.001 * i for i in range(1, config.GATE_SNAPSHOT_N + 1)) \
        / config.GATE_SNAPSHOT_N
    assert abs(got["xyz_cam"][2] - want_z) < 1e-12, \
        "평균이 아니다: %.6f vs %.6f" % (got["xyz_cam"][2], want_z)
    assert got["n_frames"] == config.GATE_SNAPSHOT_N
    print("  스냅샷 -> n=%d  z=%.4fm (평균 확인)  z흔들림 %.2fmm"
          % (got["n_frames"], got["xyz_cam"][2], got["xyz_std_mm"][2]))

    # 스냅샷을 낸 뒤에는 버퍼가 비어 다시 10장을 모은다
    got2, why = g.select({"mode": "MAGAZINE", "status": "ok", "seq": 500,
                          "detections": [base]})
    assert got2 is None and "누적 1/" in why, why
    print("  스냅샷 후 버퍼 비움 -> %s" % why)

    # 끊기면 다시 처음부터
    for i in range(600, 604):
        g.select({"mode": "MAGAZINE", "status": "ok", "seq": i,
                  "detections": [base]})
    g.select({"mode": "MAGAZINE", "status": "no_detection", "seq": 700,
              "detections": []})
    got, why = g.select({"mode": "MAGAZINE", "status": "ok", "seq": 701,
                         "detections": [base]})
    assert got is None and "누적 1/" in why, why
    print("  끊김 후 리셋 -> %s" % why)

    # 팔이 움직이면 호출부가 reset() — 그 뒤에도 처음부터
    for i in range(800, 805):
        g.select({"mode": "MAGAZINE", "status": "ok", "seq": i,
                  "detections": [base]})
    g.reset()
    got, why = g.select({"mode": "MAGAZINE", "status": "ok", "seq": 900,
                         "detections": [base]})
    assert got is None and "누적 1/" in why, why
    print("  reset() 후 -> %s" % why)
    print("\nPASS")
