#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Read the Dobot V4 real-time feedback (port 30004) directly, no ROS.

Each packet is a fixed 1440-byte little-endian struct. We only unpack the few
fields that matter for "is the robot moving / why not": mode, joint angles,
enable/running/error/drag flags, and the safety (E-stop) byte.
Offsets from the TCP/IP Remote Control Interface Guide V4.6.0, section 3.
"""

import socket
import struct

ROBOT_IP = "47.84.110.95"
ROBOT_PORT = 30004
PACKET = 1440
N_READS = 5   # how many packets to print (8ms apart)

MODE = {1: "INIT", 2: "BRAKE_OPEN", 4: "DISABLED", 5: "ENABLE",
        6: "BACKDRIVE(drag)", 7: "RUNNING", 8: "RECORDING",
        9: "ERROR", 10: "PAUSE", 11: "JOG"}


def recv_exact(sock, n):
    buf = b""
    while len(buf) < n:
        chunk = sock.recv(n - len(buf))
        if not chunk:
            raise ConnectionError("feedback socket closed")
        buf += chunk
    return buf


def parse(pkt):
    size = struct.unpack_from("<H", pkt, 0)[0]
    mode = struct.unpack_from("<Q", pkt, 24)[0]
    q = struct.unpack_from("<6d", pkt, 432)           # actual joint angles (deg)
    enable, drag, running, error = (pkt[1026], pkt[1027], pkt[1028], pkt[1029])
    safety = pkt[1420]
    return size, mode, q, enable, drag, running, error, safety


def main():
    with socket.create_connection((ROBOT_IP, ROBOT_PORT), timeout=5) as sock:
        sock.settimeout(3)
        for _ in range(N_READS):
            size, mode, q, enable, drag, running, error, safety = parse(recv_exact(sock, PACKET))
            joints = ", ".join(f"{a:7.2f}" for a in q)
            print(f"size={size} mode={mode}({MODE.get(mode, '?')}) "
                  f"enable={enable} running={running} error={error} drag={drag} "
                  f"safety=0x{safety:02x}")
            print(f"  QActual(deg): [{joints}]")


if __name__ == "__main__":
    main()
