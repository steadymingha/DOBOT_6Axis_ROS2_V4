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
import os
import socket
import sys

DASH_PORT = 29999
TIMEOUT_S = 3.0

# 30004 parsing lives in cr7_pnp/robot_feed.py; imported by path so this stays
# runnable without ROS (cr7_pnp/__init__ pulls in rclpy).
sys.path.append(os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'cr7_pnp'))
from robot_feed import read_one, default_robot_ip as default_ip  # noqa: E402


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
        rt = read_one(ip, timeout=TIMEOUT_S)
    except OSError as e:
        print(f'[real-time 30004] connect/recv failed: {e}')
        return
    if rt is None:
        print('[real-time 30004] packet length mismatch (out of sync?)')
        return

    print(f"robot_mode={rt['robot_mode']}  enable={rt['enable']}  "
          f"running={rt['running']}  error={rt['error']}")
    print('q_actual (deg): ' + ', '.join(f'{v:8.3f}' for v in rt['q_actual']))
    print('tcp pose:       ' + ', '.join(f'{v:8.3f}' for v in rt['tool_vector']))


if __name__ == '__main__':
    main()
