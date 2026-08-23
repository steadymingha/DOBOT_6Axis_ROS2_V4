#!/usr/bin/env python3
"""Standalone real-robot connectivity check -- no ROS2, no colcon build needed.

Opens the dashboard (29999) and real-time feedback (30004) TCP sockets directly
and prints robot state. Only *query* commands go to 29999 (RobotMode/GetErrorID/
GetAngle/GetPose); nothing that moves or changes the robot, except
--request-control which sends RequestControl() (see below).

Run before ros2 launch, to sanity check "is the robot there, is param.json right":
    python3 check_real_robot.py                # ip from param.json
    python3 check_real_robot.py 192.168.5.1     # explicit ip

If every query comes back "Control Mode Is Not Tcp": the pendant isn't in
TCP/IP secondary-development mode, or another client (pendant/another PC) is
holding control. Try:
    python3 check_real_robot.py --request-control
RequestControl() is the only command here that changes robot state, and only
takes effect while the robot is disabled (rejected while running/dragging/
paused -- it can't hijack a running program, it just fails).
"""
import json
import os
import socket
import struct
import sys

DASH_PORT = 29999
RT_PORT = 30004
RT_LEN = 1440
TIMEOUT_S = 3.0

# offsets into the 1440-byte real-time struct we actually care about
# (full layout: dobot_bringup_v4/include/dobot_bringup/command.h RealTimeData_t)
OFF_LEN = 0
OFF_ROBOT_MODE = 24
OFF_Q_ACTUAL = 432       # double[6], degrees
OFF_TOOL_VECTOR = 624    # double[6], x,y,z,rx,ry,rz
OFF_ENABLE_STATUS = 1026
OFF_RUNNING_STATUS = 1028
OFF_ERROR_STATUS = 1029


def default_ip():
    cfg = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                        'dobot_bringup_v4', 'config', 'param.json')
    with open(cfg) as f:
        data = json.load(f)
    node = data['node_info'][data['current_robot'] - 1]
    return node['ip_address']


def dash_query(sock, cmd):
    sock.sendall(cmd.encode())
    buf = b''
    while not buf.endswith(b';'):
        chunk = sock.recv(1024)
        if not chunk:
            break
        buf += chunk
    text = buf.decode(errors='replace')
    err_id = None
    payload = None
    if ',' in text:
        head, rest = text.split(',', 1)
        try:
            err_id = int(head)
        except ValueError:
            pass
        if '{' in rest and '}' in rest:
            payload = rest[rest.index('{') + 1:rest.index('}')]
    return err_id, payload, text.strip()


def read_realtime(ip):
    with socket.create_connection((ip, RT_PORT), timeout=TIMEOUT_S) as s:
        s.settimeout(TIMEOUT_S)
        buf = b''
        while len(buf) < RT_LEN:
            chunk = s.recv(RT_LEN - len(buf))
            if not chunk:
                raise ConnectionError('real-time feed closed before 1440 bytes')
            buf += chunk
    (length,) = struct.unpack_from('<H', buf, OFF_LEN)
    if length != RT_LEN:
        print(f'[warn] real-time packet len={length}, expected {RT_LEN} (out of sync?)')
    robot_mode, = struct.unpack_from('<Q', buf, OFF_ROBOT_MODE)
    q_actual = struct.unpack_from('<6d', buf, OFF_Q_ACTUAL)
    tool_vector = struct.unpack_from('<6d', buf, OFF_TOOL_VECTOR)
    enable, = struct.unpack_from('<b', buf, OFF_ENABLE_STATUS)
    running, = struct.unpack_from('<b', buf, OFF_RUNNING_STATUS)
    error, = struct.unpack_from('<b', buf, OFF_ERROR_STATUS)
    return dict(robot_mode=robot_mode, q_actual=q_actual, tool_vector=tool_vector,
                enable=enable, running=running, error=error)


def main():
    args = sys.argv[1:]
    request_control = '--request-control' in args
    args = [a for a in args if a != '--request-control']
    ip = args[0] if args else default_ip()

    print(f'--- {ip} ---')
    try:
        with socket.create_connection((ip, DASH_PORT), timeout=TIMEOUT_S) as dash:
            dash.settimeout(TIMEOUT_S)
            if request_control:
                _, _, raw = dash_query(dash, 'RequestControl()')
                print(f'RequestControl() -> {raw}')
                return
            for cmd in ('RobotMode()', 'GetErrorID()', 'GetAngle()', 'GetPose(user=0,tool=0)'):
                err_id, payload, raw = dash_query(dash, cmd)
                print(f'{cmd:24s} -> {raw}')
    except OSError as e:
        print(f'[dashboard 29999] connect/recv failed: {e}')
        return

    try:
        rt = read_realtime(ip)
    except OSError as e:
        print(f'[real-time 30004] connect/recv failed: {e}')
        return

    print(f"robot_mode={rt['robot_mode']}  enable={rt['enable']}  "
          f"running={rt['running']}  error={rt['error']}")
    print('q_actual (deg): ' + ', '.join(f'{v:8.3f}' for v in rt['q_actual']))
    print('tcp pose:       ' + ', '.join(f'{v:8.3f}' for v in rt['tool_vector']))


if __name__ == '__main__':
    main()
