"""Single 30004 real-time feed parser for the whole repo. READ-ONLY.

The controller's real-time port serves any number of read-only clients, so this
runs alongside bringup (which holds its own connection) or with no bringup at all.
Nothing is ever sent on this socket -- commands go through ServoJ / the
dashboard services, feedback comes from here, and the two never mix.

Importable without ROS: stdlib + numpy only. `cr7_pnp/__init__` pulls in rclpy,
so ROS-free tools (handeye_calib, check_real_robot) import this file by path
(sys.path.insert(cr7_pnp dir); `from robot_feed import RobotFeed`) instead of
through the package.

Consumers and what they read:
    test/cbirrt_p1p2_test.py   state() / torque_window() / wait_ready() / history
    handeye_calib, vision_bridge/verify_chain, test/vision_target   latest()

Open ONE instance per process. Snapshot averaging and contact detection judge
"still" on the same frames only if they look at the same feed.
"""
import socket
import struct
import threading
import time
from collections import deque

import numpy as np

RT_PORT = 30004
RT_FRAME = 1440
RT_LEN = RT_FRAME                       # old handeye name

# Offsets into the 1440-byte RealTimeData_t
# (dobot_bringup_v4/include/dobot_bringup/command.h). Edit here and nowhere else.
OFF_LEN, OFF_ROBOT_MODE = 0, 24
OFF_TIMESTAMP, OFF_RUNTIME = 32, 40   # uint64 ms (PDF p125: controller Unix ms / uptime)
OFF_Q_ACTUAL = 432          # double[6], degrees, CONTROLLER sign convention
OFF_QD_ACTUAL = 480         # double[6], deg/s
OFF_TOOL_VECTOR = 624       # double[6], x/y/z mm + rx/ry/rz deg == GetPose(user=0,tool=0)
OFF_TCP_FORCE = 720
OFF_ENABLE, OFF_DRAG, OFF_RUNNING, OFF_ERROR = 1026, 1027, 1028, 1029
OFF_COLLISION = 1038
OFF_M_ACTUAL = 1120         # double[6], joint torque N*m
OFF_SAFETY = 1420

MODE_ENABLE = 5             # ROBOT_MODE_ENABLE (idle)
MODE_PAUSE = 10             # ROBOT_MODE_PAUSE
MODE_COLLISION = 11         # ROBOT_MODE_COLLISION
MODE_NAMES = {1: "INIT", 2: "BRAKE_OPEN", 4: "DISABLED", 5: "ENABLE",
              6: "BACKDRIVE(drag)", 7: "RUNNING", 8: "RECORDING",
              9: "ERROR", 10: "PAUSE", 11: "COLLISION"}


def parse_frame(frame):
    """One 1440-byte frame -> dict, or None if the length field says out-of-sync."""
    (length,) = struct.unpack_from('<H', frame, OFF_LEN)
    if length != RT_FRAME:
        return None
    return dict(
        robot_mode=struct.unpack_from('<Q', frame, OFF_ROBOT_MODE)[0],
        timestamp=struct.unpack_from('<Q', frame, OFF_TIMESTAMP)[0],
        run_time=struct.unpack_from('<Q', frame, OFF_RUNTIME)[0],
        q_actual=struct.unpack_from('<6d', frame, OFF_Q_ACTUAL),
        qd_actual=struct.unpack_from('<6d', frame, OFF_QD_ACTUAL),
        tool_vector=struct.unpack_from('<6d', frame, OFF_TOOL_VECTOR),
        tcp_force=struct.unpack_from('<6d', frame, OFF_TCP_FORCE),
        m_actual=struct.unpack_from('<6d', frame, OFF_M_ACTUAL),
        enable=struct.unpack_from('<b', frame, OFF_ENABLE)[0],
        drag=struct.unpack_from('<b', frame, OFF_DRAG)[0],
        running=struct.unpack_from('<b', frame, OFF_RUNNING)[0],
        error=struct.unpack_from('<b', frame, OFF_ERROR)[0],
        collision=struct.unpack_from('<b', frame, OFF_COLLISION)[0],
        safety=frame[OFF_SAFETY],
    )


def default_robot_ip():
    """Controller IP from dobot_bringup_v4/config/param.json (what bringup uses)."""
    import json, os
    cfg = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                       'dobot_bringup_v4', 'config', 'param.json')
    with open(cfg) as f:
        data = json.load(f)
    return data['node_info'][data['current_robot'] - 1]['ip_address']


def read_one(ip, port=RT_PORT, timeout=3.0):
    """Blocking one-shot read (diagnostics). Raises OSError on connect/recv failure."""
    with socket.create_connection((ip, port), timeout=timeout) as s:
        s.settimeout(timeout)
        buf = b''
        while len(buf) < RT_FRAME:
            chunk = s.recv(RT_FRAME - len(buf))
            if not chunk:
                raise ConnectionError('real-time feed closed before 1440 bytes')
            buf += chunk
    return parse_frame(buf)


class RobotFeed(threading.Thread):
    """Passive tap on the 30004 feed, ~125 Hz. Keeps the latest frame + a ~3 s
    torque history. Was RealtimeMonitor (test) and RobotFeed (handeye)."""

    def __init__(self, ip, port=RT_PORT):
        super().__init__(daemon=True)
        self.ip, self.port = ip, port
        self._lock = threading.Lock()
        self._state = None
        self._stamp = 0.0
        self._running = True
        self.error = None
        self.history = deque(maxlen=400)      # (t, m_actual) ~3 s at 125 Hz

    def run(self):
        try:
            sock = socket.create_connection((self.ip, self.port), timeout=5.0)
        except OSError as e:
            self.error = f"real-time feed {self.ip}:{self.port} unreachable: {e}"
            return
        buf = b''
        with sock:
            sock.settimeout(5.0)
            while self._running:
                try:
                    chunk = sock.recv(65536)
                except OSError as e:
                    self.error = f"real-time feed read failed: {e}"
                    return
                if not chunk:
                    self.error = "real-time feed closed by the controller"
                    return
                buf += chunk
                while len(buf) >= RT_FRAME:
                    frame, buf = buf[:RT_FRAME], buf[RT_FRAME:]
                    st = parse_frame(frame)
                    if st is None:
                        continue                # out of sync; skip this frame
                    now = time.time()
                    with self._lock:
                        self._state, self._stamp = st, now
                        self.history.append((now, st['m_actual']))

    def stop(self):
        self._running = False

    # -- test/cbirrt_p1p2_test.py interface (RealtimeMonitor) --------------------
    def state(self):
        """(dict, wall_stamp) of the latest frame; (None, 0.0) before the first."""
        with self._lock:
            return self._state, self._stamp

    def wait_ready(self, timeout=5.0):
        t0 = time.time()
        while time.time() - t0 < timeout:
            if self.error:
                return False
            if self.state()[0] is not None:
                return True
            time.sleep(0.02)
        return False

    def torque_window(self, t_from, t_to):
        """m_actual samples stamped within [t_from, t_to] -> list of 6-tuples."""
        with self._lock:
            return [m for t, m in self.history if t_from <= t <= t_to]

    # -- handeye_calib / vision_bridge interface (old RobotFeed) -----------------
    def latest(self, max_age_s=0.5):
        """Freshest frame as {mode, q, qd, tool, wall}, or None if stale/absent.
        Stale is treated as absent: a frozen feed must not pass for a still arm."""
        st, stamp = self.state()
        if st is None or time.time() - stamp > max_age_s:
            return None
        return dict(mode=st['robot_mode'], q=np.array(st['q_actual']),
                    qd=np.array(st['qd_actual']), tool=np.array(st['tool_vector']),
                    wall=stamp)


RealtimeMonitor = RobotFeed             # old test name


if __name__ == '__main__':
    # self-check: parse_frame round-trips a synthetic frame; latest() mirrors state()
    f = bytearray(RT_FRAME)
    struct.pack_into('<H', f, OFF_LEN, RT_FRAME)
    struct.pack_into('<Q', f, OFF_ROBOT_MODE, MODE_ENABLE)
    struct.pack_into('<6d', f, OFF_Q_ACTUAL, *range(6))
    struct.pack_into('<6d', f, OFF_TOOL_VECTOR, *range(10, 16))
    st = parse_frame(bytes(f))
    assert st['robot_mode'] == MODE_ENABLE and st['q_actual'] == tuple(float(i) for i in range(6))
    assert parse_frame(bytes(RT_FRAME)) is None
    fd = RobotFeed('0.0.0.0')
    fd._state, fd._stamp = st, time.time()
    lt = fd.latest()
    assert lt['mode'] == MODE_ENABLE and lt['tool'].tolist() == [10., 11., 12., 13., 14., 15.]
    assert fd.latest(max_age_s=-1) is None
    print('robot_feed self-check ok')
