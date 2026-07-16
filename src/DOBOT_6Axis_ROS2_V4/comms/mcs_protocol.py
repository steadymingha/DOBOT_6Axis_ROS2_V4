"""MCS <-> arm wire protocol: the single source of truth for the command frame
layout, field names, and enum values shared by the comms bridge, the server stub,
and main.py. Import this instead of re-declaring the enums anywhere.

Field names / types are the 관제 spec (15-byte packed frame, no padding):
    TargetID       uint8    IN/OUT (shelf) or slot A/B/C (wirebonder)
    TargetRelPosX  float    ArUco-relative X
    TargetRelPosY  float    ArUco-relative Y
    TargetRelPosZ  float    ArUco-relative Z
    Command        uint8    START/STOP/PAUSE/RESUME
    Gripper        uint8    OPEN/CLOSE

Enum values are 0,1,... in the order the spec lists them.
"""
import struct
from enum import IntEnum

FRAME = '<BfffBB'                         # CONFIRM endianness with 관제 ('<' = little)
FRAME_SIZE = struct.calcsize(FRAME)       # 15
FIELDS = ('TargetID', 'TargetRelPosX', 'TargetRelPosY', 'TargetRelPosZ',
          'Command', 'Gripper')


class TargetID(IntEnum):
    IN = 0
    OUT = 1
    A = 2
    B = 3
    C = 4


class Command(IntEnum):
    START = 0
    STOP = 1
    PAUSE = 2
    RESUME = 3


class Gripper(IntEnum):
    OPEN = 0
    CLOSE = 1


# TargetID -> main.py REGISTRY id. A/B/C are the wirebonder sequences; IN is the
# shelf. OUT has no sequence yet.
TARGET_LOCATION = {TargetID.A: 'wb1', TargetID.B: 'wb2', TargetID.C: 'wb3',
                   TargetID.IN: 'shelf'}
CONTROL = {Command.STOP, Command.PAUSE, Command.RESUME}   # -> /mcs/stop (immediate)

# Arm -> MCS status/error reporting. Set node.last_error to one of these; the report
# channel back to MCS is wired later (TODO). Split finer as new failures surface.
class ErrorCode(IntEnum):
    OK = 0
    TAG_NOT_DETECTED = 1       # no /vision/device_pose at all (tag out of FOV / node down)
    CAPTURE_SPREAD_HIGH = 2    # pose jitter over threshold: stale 2nd vision node / unstable solve
    CAPTURE_IMPLAUSIBLE = 3    # captured pose too far from the anchor: bad triangulation
    UNREACHABLE = 4            # target not reachable: preflight leg OR IK has no solution
    TF_UNAVAILABLE = 5         # world/AGV TF missing -- "park the AGV" / "reposition"
    PLAN_FAILED = 6            # RRT/CBiRRT found no collision-free path to a reachable goal
    COLLISION = 7             # start/goal state in collision (already blocked)
    EXEC_FAILED = 8            # trajectory goal rejected or a motion leg failed mid-execution
    ATTACH_FAILED = 9          # ATTACHLINK/DETACHLINK (grasp/release) failed
    GRIPPER_FAULT = 10         # gripper action server unavailable or goal rejected (hardware)
    INIT_FAILED = 11           # hub bring-up failed / could not reach the hub at startup
    NO_POCKET = 12             # pocket occupancy scan: no usable base pocket (pick:
                               # none holds a box; place: none free)
    # split finer only if MCS needs to act differently on a sub-case


def pack(target, x, y, z, command, gripper) -> bytes:
    return struct.pack(FRAME, target, x, y, z, command, gripper)


def unpack(frame: bytes) -> dict:
    """15-byte frame -> dict keyed by the spec field names (plain int/float values)."""
    return dict(zip(FIELDS, struct.unpack(FRAME, frame)))


def take_frames(buf: bytes):
    """Slice complete FRAME_SIZE-byte frames off the buffer (TCP has no message
    boundaries: a frame can straddle recv()s). Returns (list[bytes], rest)."""
    frames = []
    while len(buf) >= FRAME_SIZE:
        frames.append(buf[:FRAME_SIZE])
        buf = buf[FRAME_SIZE:]
    return frames, buf


if __name__ == '__main__':
    assert FRAME_SIZE == 15, FRAME_SIZE
    assert [TargetID.IN, TargetID.OUT, TargetID.A, TargetID.B, TargetID.C] == [0, 1, 2, 3, 4]
    assert [Command.START, Command.STOP, Command.PAUSE, Command.RESUME] == [0, 1, 2, 3]
    assert [Gripper.OPEN, Gripper.CLOSE] == [0, 1]
    f = pack(TargetID.A, 0.1, 0.2, 0.3, Command.START, Gripper.CLOSE)
    d = unpack(f)
    assert d['TargetID'] == TargetID.A and d['Command'] == Command.START
    assert abs(d['TargetRelPosX'] - 0.1) < 1e-6
    assert TARGET_LOCATION.get(d['TargetID']) == 'wb1'          # int key matches IntEnum
    frames, rest = take_frames(f + f[:5])
    assert len(frames) == 1 and len(rest) == 5, (len(frames), len(rest))
    assert Command['START'] == 0 and TargetID['A'].name == 'A'  # name<->value both ways
    print("mcs_protocol selftest OK")
