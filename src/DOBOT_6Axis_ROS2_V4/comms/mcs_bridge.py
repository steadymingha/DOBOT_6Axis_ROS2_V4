"""Rough MCS comms bridge: TCP client -> ROS2 topics.

Stands in for the comms teammate's ROS2 client + MessageDispatcher until theirs is
up. Connects to the control (MCS) server, reads fixed-size binary command frames
(see mcs_protocol), and routes each to one of two channels main.py subscribes to:
  * START  -> /mcs/command  (one atomic JSON message: the spec fields verbatim)
  * STOP / PAUSE / RESUME -> /mcs/stop  (param-free, acts immediately)

    python3 comms/mcs_bridge.py --host 127.0.0.1 --port 9100
    python3 comms/mcs_bridge.py --selftest        # route check, no ROS/socket
"""
import argparse
import json
import socket
import sys
import time

import mcs_protocol as proto


def route(fields: dict):
    """Where a decoded frame goes:
       ('stop', label)      control command -> /mcs/stop, acts immediately
       ('command', fields)  START           -> /mcs/command, one atomic JSON
       (None, cmd)          nothing to send (unknown command)"""
    cmd = fields['Command']
    if cmd in proto.CONTROL:
        return 'stop', proto.Command(cmd).name
    if cmd == proto.Command.START:
        return 'command', fields
    return None, cmd


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--host', default='127.0.0.1')
    ap.add_argument('--port', type=int, default=9100)
    ap.add_argument('--selftest', action='store_true')
    args = ap.parse_args()

    if args.selftest:
        f = proto.pack(proto.TargetID.A, 0.1, 0.2, 0.3, proto.Command.START, proto.Gripper.CLOSE)
        kind, payload = route(proto.unpack(f))
        assert kind == 'command' and payload['TargetID'] == proto.TargetID.A, payload
        assert route(proto.unpack(proto.pack(proto.TargetID.A, 0, 0, 0,
                     proto.Command.STOP, 0))) == ('stop', 'STOP')
        assert route(proto.unpack(proto.pack(proto.TargetID.IN, 0, 0, 0,
                     proto.Command.START, 0)))[0] == 'command'
        print("selftest OK")
        return

    import rclpy
    from rclpy.node import Node
    from std_msgs.msg import String

    rclpy.init()
    node = Node('mcs_bridge')
    cmd_pub = node.create_publisher(String, '/mcs/command', 10)
    stop_pub = node.create_publisher(String, '/mcs/stop', 10)

    while rclpy.ok():
        try:
            with socket.create_connection((args.host, args.port), timeout=5) as sock:
                node.get_logger().info(f"connected to MCS {args.host}:{args.port}")
                sock.settimeout(1.0)
                buf = b''
                while rclpy.ok():
                    try:
                        chunk = sock.recv(1024)
                    except TimeoutError:
                        continue
                    if not chunk:
                        node.get_logger().warn("MCS closed the connection")
                        break
                    buf += chunk
                    frames, buf = proto.take_frames(buf)
                    for fr in frames:
                        fields = proto.unpack(fr)
                        node.get_logger().info(f"frame: {fields}")
                        kind, payload = route(fields)
                        if kind == 'stop':
                            stop_pub.publish(String(data=payload))
                            node.get_logger().info(f"-> /mcs/stop: {payload}")
                        elif kind == 'command':
                            cmd_pub.publish(String(data=json.dumps(payload)))
                            node.get_logger().info(f"-> /mcs/command: {payload}")
                        else:
                            node.get_logger().warn(f"unknown command, dropped: {payload}")
        except OSError as e:
            node.get_logger().warn(f"MCS connect failed ({e}); retry in 3s")
            time.sleep(3)

    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    sys.exit(main())
