"""Rough MCS (control) server stub: pack operator-typed commands into the binary
frame (see mcs_protocol) and broadcast to any connected bridge over TCP. Stands in
for the real control system.

    python3 comms/mcs_server.py --port 9100
    python3 comms/mcs_server.py --selftest
    # prompt: <target> <command> [gripper] [x y z]
    #   e.g.  A START            (slot A, START, gripper CLOSE, pos 0/0/0)
    #         A START OPEN 0.1 0.2 0.3
    #         IN START           (shelf)
"""
import argparse
import socket
import sys
import threading

import mcs_protocol as proto

clients = []
lock = threading.Lock()


def pack_line(line: str) -> bytes:
    """<target> <command> [gripper] [x y z] -> binary frame. Raises KeyError/ValueError
    on a bad token so the operator sees it instead of sending garbage."""
    t = line.split()
    if len(t) < 2:
        raise ValueError("need at least: <target> <command>")
    target = proto.TargetID[t[0].upper()]
    command = proto.Command[t[1].upper()]
    grippers = proto.Gripper.__members__
    grip = grippers[t[2].upper()] if len(t) > 2 and t[2].upper() in grippers else proto.Gripper.CLOSE
    nums = [tok for tok in t[2:] if tok.upper() not in grippers]
    x, y, z = (list(map(float, nums)) + [0.0, 0.0, 0.0])[:3]
    return proto.pack(target, x, y, z, command, grip)


def accept_loop(srv):
    while True:
        conn, addr = srv.accept()
        with lock:
            clients.append(conn)
        print(f"[mcs] client connected: {addr}")


def _send(conn, data):
    try:
        conn.sendall(data)
        return True
    except OSError:
        return False


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--port', type=int, default=9100)
    ap.add_argument('--selftest', action='store_true')
    args = ap.parse_args()

    if args.selftest:
        d = proto.unpack(pack_line('A START OPEN 0.1 0.2 0.3'))
        assert d['TargetID'] == proto.TargetID.A and d['Command'] == proto.Command.START
        assert d['Gripper'] == proto.Gripper.OPEN and abs(d['TargetRelPosZ'] - 0.3) < 1e-6
        assert proto.unpack(pack_line('IN START'))['TargetID'] == proto.TargetID.IN
        assert proto.unpack(pack_line('A STOP'))['Command'] == proto.Command.STOP
        print("selftest OK")
        return

    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind(('localhost', args.port))
    srv.listen()
    print(f"[mcs] listening on :{args.port} -- '<target> <command> [gripper] [x y z]'")
    threading.Thread(target=accept_loop, args=(srv,), daemon=True).start()

    try:
        while True:
            line = input("mcs send> ").strip()
            if not line:
                continue
            try:
                data = pack_line(line)
            except (ValueError, KeyError) as e:
                print(f"[mcs] bad command ({e}); try 'A START' or 'IN START OPEN 0.1 0.2 0.3'")
                continue
            with lock:
                dead = [c for c in clients if not _send(c, data)]
                for c in dead:
                    clients.remove(c)
                n = len(clients)
            print(f"[mcs] sent {len(data)}B to {n} client(s)")
    except (EOFError, KeyboardInterrupt):
        pass


if __name__ == '__main__':
    sys.exit(main())
