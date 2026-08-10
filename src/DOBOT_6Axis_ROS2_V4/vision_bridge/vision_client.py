"""호스트 vision_runner 의 ZMQ 클라이언트 (SUB 결과 + REQ 커맨드).

페이로드 스키마 (러너 ipc.py 가 고정):
    공통 : req_id, mode, seq, mono_stamp, depth_scale, status, git_vision, git_runner
    status: ok | no_detection | not_ready | error | degraded_to_idle
    MAGAZINE: detections[] = {track_id, cls, state, stale_frames, lost_count,
                              score, bbox, avg_dist_cm, xyz_cam, valid_pct, sd_cm}
    ARUCO   : marker_id, T_cam_marker, plane_ok, n_samples, P_cam_target

SUB 은 CONFLATE 로 열어 항상 최신 한 장만 남긴다 — 검증은 지금 팔이 어디 있는지와
짝이 맞아야 하므로, 밀린 큐를 따라가면 과거 프레임을 현재 자세와 곱하게 된다.
"""
import time

import msgpack
import zmq

import config


def detections(payload):
    """MAGAZINE 검출 리스트. 페이로드가 없거나 다른 모드면 빈 리스트."""
    if not payload:
        return []
    return payload.get("detections") or []


class VisionClient:
    def __init__(self, host=None, pub_port=None, rep_port=None):
        host = host or config.RUNNER_HOST
        pub_port = pub_port or config.PUB_PORT
        rep_port = rep_port or config.REP_PORT

        ctx = zmq.Context.instance()
        self.sub = ctx.socket(zmq.SUB)
        self.sub.setsockopt(zmq.SUBSCRIBE, b"")
        self.sub.setsockopt(zmq.RCVHWM, 2)
        self.sub.setsockopt(zmq.CONFLATE, 1)   # connect 前에 걸어야 먹는다
        self.sub.connect("tcp://%s:%d" % (host, pub_port))

        self.req = ctx.socket(zmq.REQ)
        self.req.setsockopt(zmq.RCVTIMEO, config.REQ_TIMEOUT_MS)
        self.req.setsockopt(zmq.SNDTIMEO, config.REQ_TIMEOUT_MS)
        self.req.setsockopt(zmq.LINGER, 0)
        self.req.connect("tcp://%s:%d" % (host, rep_port))

        self._latest = None
        self._last_rx = 0.0
        self._req_id = 0

    # ── SUB ───────────────────────────────────────────────────────
    def poll(self, timeout_ms=0):
        """새 메시지가 있으면 최신 것을 반환, 없으면 None.

        직전 페이로드를 대신 돌려주지 않는다 — 검증 루프가 매번 무언가를 받으면
        러너가 멈춘 사이에도 과거 프레임이 현재 팔 자세와 짝지어져 표본이 된다.
        마지막으로 받은 것이 필요하면 .latest 를 쓸 것.
        """
        got = None
        while self.sub.poll(timeout_ms):
            got = msgpack.unpackb(self.sub.recv(), raw=False)
            timeout_ms = 0
        if got is not None:
            self._latest = got
            self._last_rx = time.time()
        return got

    @property
    def latest(self):
        return self._latest

    @property
    def silent(self):
        """마지막 수신 후 SILENT_AFTER_S 경과. 러너가 죽었거나 네트워크가 끊긴 상태."""
        return time.time() - self._last_rx > config.SILENT_AFTER_S

    @property
    def age_s(self):
        return time.time() - self._last_rx if self._last_rx else float("inf")

    # ── REQ ───────────────────────────────────────────────────────
    def _rpc(self, obj):
        """실패하면 REQ 소켓이 잠기므로(엄격한 req/rep 교대) 예외를 그대로 올린다."""
        self.req.send(msgpack.packb(obj, use_bin_type=True))
        return msgpack.unpackb(self.req.recv(), raw=False)

    def set_mode(self, mode):
        """req_id 는 클라이언트가 증가시켜 관리한다. (accepted, 응답dict)"""
        self._req_id += 1
        r = self._rpc({"cmd": "set_mode", "req_id": self._req_id, "mode": mode})
        return bool(r.get("accepted")), r

    def ping(self):
        return self._rpc({"cmd": "ping"})

    @property
    def req_id(self):
        return self._req_id

    def close(self):
        self.sub.close(0)
        self.req.close(0)


if __name__ == "__main__":
    # 자가시험: 러너가 떠 있어야 한다. set_mode → 5초 수신 → IDLE 복귀
    c = VisionClient()
    print("set_mode MAGAZINE ->", c.set_mode("MAGAZINE"))
    t0 = time.time()
    n = 0
    while time.time() - t0 < 5.0:
        p = c.poll(200)
        if p is None:
            continue
        n += 1
        if n % 20 == 1:
            print("seq=%s status=%s dets=%d silent=%s"
                  % (p["seq"], p["status"], len(detections(p)), c.silent))
    print("받은 메시지 %d개, silent=%s" % (n, c.silent))
    print("set_mode IDLE ->", c.set_mode("IDLE"))
    c.close()
