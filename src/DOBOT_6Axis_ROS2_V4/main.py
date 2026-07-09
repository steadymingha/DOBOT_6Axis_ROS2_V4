"""Minimal mission DISPATCHER: route an AMR/MCS "stop + location(id)" trigger by
location TYPE to a vision LOCATE step plus the matching pick-place flow.

This is the in-process, blocking FSM the architecture doc settled on (dict-router,
NOT orchestrator.py's async SingleThreadedExecutor model -- that one collides with
the blocking pinocchio planner and only fits once the arm/vision live behind
action/service boundaries). One HubPickPlace node, one MultiThreadedExecutor + spin
thread, one shared hub (both flows use the same HUB_TCP and a pocket-family seed).

The two flows are imported AS LIBRARIES and their sequence functions reused -- no
logic is duplicated here:
    wirebonder -> wb.capture_device (LOCATE, AprilTag) + wb.transfer
    shelf      -> shelf.pick_place_one_box        (LOCATE is a STUB: the shelf frame
                  is hardcoded; AI magazine detection is deferred)

One trigger == one transfer; re-trigger for the next unit (shelf walks its box index).

Run (sim up, AGV parked roughly facing the device):
    source /opt/ros/humble/setup.bash
    source ~/dobot_ws/install/setup.bash
    cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
    python3 vision/wirebonder_vision_node.py               # terminal A (system cv2)
    ~/dobot_ws/.venv/bin/python3 main.py                   # terminal B (.venv)

    ~/dobot_ws/.venv/bin/python3 main.py --selftest        # registry sanity, no ROS run
"""

import os
import sys
import time
import json
import queue
import threading

import rclpy
from rclpy.executors import MultiThreadedExecutor
from geometry_msgs.msg import PoseStamped
from std_msgs.msg import Int32, String

from cr7_pnp import (
    HubPickPlace, pose_at, quat_mul, quat_about_z, DOWN,
    GRASP_TCP_ABOVE, GRASP_LATERAL_M,
)

# The two flows live in sequences/, the MCS protocol in comms/; add both to the
# path so they import by name.
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), 'sequences'))
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), 'comms'))
import wirebonder_pick_place as wb  # noqa: E402
import shelf_pick_place as shelf  # noqa: E402
import mcs_protocol as proto  # noqa: E402

# --- location registry --------------------------------------------------------
# id -> (type, *params). One concrete job per id (re-trigger for the next unit).
#   wirebonder: ('wirebonder', device_name, (src, dst) Locations)
#   shelf:      ('shelf',)  -- walks node.box_idx across SHELF_BOXES
# Add a line per real location; the AMR/MCS bridge will key into this by id.
REGISTRY = {
    'wb1':   ('wirebonder', 'wb1', wb.SEQUENCES['1']),   # base   -> slot A
    'wb2':   ('wirebonder', 'wb1', wb.SEQUENCES['2']),   # slot B -> slot C
    'wb3':   ('wirebonder', 'wb1', wb.SEQUENCES['3']),   # slot D -> base pocket
    'shelf': ('shelf',),
}


def locate(node, kind, *params):
    """LOCATE: secure the target frame. True on success (abort the mission on False)."""
    if kind == 'wirebonder':
        # AprilTag look-before-you-transfer: refresh every device pose from
        # /vision/device_pose at the capture viewpoint, then return to the hub.
        return wb.capture_device(node)
    if kind == 'shelf':
        # STUB: the shelf frame is hardcoded (shelf.SHELF_BOXES). AI magazine
        # detection plugs in here later.
        return True
    return False


def pick_place(node, kind, *params):
    """PICK/PLACE: run the matching transfer. True/False."""
    if kind == 'wirebonder':
        device, (src, dst) = params
        return wb.transfer(node, src, dst)
    if kind == 'shelf':
        idx = node.box_idx
        if idx >= len(shelf.SHELF_BOXES):
            print(f"[shelf] all {len(shelf.SHELF_BOXES)} boxes placed; nothing left")
            return False
        if shelf.pick_place_one_box(node, idx):
            node.box_idx += 1
            return True
        return False
    return False


def run_mission(node, loc_id):
    """Dict-router FSM: IDLE -> LOCATE -> PICK/PLACE -> REPORT -> IDLE."""
    entry = REGISTRY.get(loc_id)
    if entry is None:
        print(f"[IDLE] unknown location '{loc_id}' (known: {', '.join(REGISTRY)})")
        return
    kind, *params = entry
    # ponytail: coarse cooperative abort -- checked before LOCATE and before
    # PICK/PLACE, not mid-motion. True mid-swing STOP needs hooks inside
    # wb.transfer / shelf.pick_place_one_box; add there if the arm must halt instantly.
    if getattr(node, 'abort', False):
        print(f"[ABORT] {loc_id} cancelled before start")
        return
    print(f"[LOCATE] {loc_id} ({kind})")
    if not locate(node, kind, *params):
        print(f"[REPORT] {loc_id} LOCATE failed -> abort")   # seam: MCS bridge reports
        return
    if getattr(node, 'abort', False):
        print(f"[ABORT] {loc_id} cancelled after LOCATE")
        return
    print(f"[PICK/PLACE] {loc_id}")
    ok = pick_place(node, kind, *params)
    print(f"[REPORT] {loc_id} {'DONE' if ok else 'FAILED'} (arm at hub)")  # seam: MCS bridge


def selftest():
    """Registry sanity (no ROS): every entry routes to a known kind and a valid job."""
    for loc, (kind, *params) in REGISTRY.items():
        assert kind in ('wirebonder', 'shelf'), f"{loc}: bad kind {kind!r}"
        if kind == 'wirebonder':
            device, seq = params
            assert device in wb.DEVICES, f"{loc}: unknown device {device!r}"
            src, dst = seq
            assert hasattr(src, 'kind') and hasattr(dst, 'kind'), f"{loc}: bad sequence"
    print("selftest OK")


def main(args=None):
    if '--selftest' in sys.argv:
        selftest()
        return

    rclpy.init(args=args)
    node = HubPickPlace()
    node.setup_planner()

    # Register the wirebonder BODY so the free RRTs route around the device.
    WB_STL_DIR = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                              'blender', 'wirebonder', 'collision')
    node.add_wirebonder_meshes(WB_STL_DIR)

    # Vision layer: device pose arrives on /vision/device_pose (odom) from
    # wirebonder_vision_node.py. Cache the latest; wb.refresh_device_pose() reads it.
    node._vision_pose = None
    node.create_subscription(PoseStamped, '/vision/device_pose',
                             lambda m: setattr(node, '_vision_pose', m), 10)
    # Drives the two-view capture: data=0 resets, data=1 grabs a view.
    node._cap_pub = node.create_publisher(Int32, '/vision/capture', 10)

    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2)  # wait for joint states

    # Shared hub: both flows use the same HUB_TCP and a pocket-family seed, so one
    # init_hub serves both. Seed from the wirebonder base pocket (the first flow to
    # verify); the shelf flow reads node.hub_q the same way.
    base = wb.base_loc()
    ref = pose_at([base.ref[0], base.ref[1], base.ref[2] + GRASP_TCP_ABOVE],
                  quat_mul(quat_about_z(base.yaw), DOWN))
    if not node.init_hub(ref, wb.HUB_TCP, GRASP_LATERAL_M):
        node.get_logger().error("Hub bring-up failed; adjust HUB_TCP and retry")
        node.destroy_node(); rclpy.shutdown(); return
    if not node.go_to_hub():
        node.get_logger().error("Could not reach the hub from the spawn pose")
        node.destroy_node(); rclpy.shutdown(); return
    node.box_idx = 0  # shelf box counter (shelf.pick_place_one_box walks it)

    # ---- MCS intake ---------------------------------------------------------
    # Command channel: ONE atomic message (target + pos + gripper + START), so the
    # fields can't arrive half-applied. Placeholder type is std_msgs/String with a
    # JSON body -- swap for the comms team's custom msg when ready (only this type and
    # the field reads below change; the queue hand-off stays). The callback runs in
    # the executor thread and only ENQUEUES; the main loop runs the blocking planner.
    node.abort = False
    node.last_error = proto.ErrorCode.OK   # code (category) + detail (exact message);
    node.last_error_detail = ""            # both set by the flows on failure, report TBD
    cmd_q = queue.Queue()

    def on_command(data):
        try:
            d = json.loads(data)                 # spec field names: TargetID, Command, ...
        except json.JSONDecodeError:
            node.get_logger().warn(f"[mcs] bad command JSON: {data!r}"); return
        if d.get('Command') != proto.Command.START:
            node.get_logger().warn(f"[mcs] /mcs/command ignores non-START: {d.get('Command')}")
            return
        loc = proto.TARGET_LOCATION.get(d.get('TargetID'))
        if loc is None:
            node.get_logger().warn(f"[mcs] no sequence for TargetID {d.get('TargetID')!r}"); return
        node.abort = False                       # a fresh goal clears a prior STOP
        cmd_q.put(loc)                            # RelPos/Gripper not consumed yet
    node.create_subscription(String, '/mcs/command', lambda m: on_command(m.data), 10)

    # STOP on a SEPARATE, param-free channel so it acts immediately and never waits
    # on command data. Sets a flag; run_mission honors it at its checkpoints.
    def on_stop(label):
        node.abort = True
        node.get_logger().warn(f"[mcs] STOP ({label}) -- aborting at next checkpoint")
    node.create_subscription(String, '/mcs/stop', lambda m: on_stop(m.data), 10)

    # Keyboard seam -- DEBUG ONLY (standalone runs). Bridge-driven deployment has no
    # TTY, and input() there would EOF immediately and quit main.py; so only feed the
    # queue from stdin when attached to a terminal. Normal mode runs purely on topics.
    def _stdin_feed():
        try:
            while True:
                cmd_q.put(input("location id> ").strip())
        except (EOFError, KeyboardInterrupt):
            cmd_q.put('q')
    if sys.stdin.isatty():
        threading.Thread(target=_stdin_feed, daemon=True).start()

    print("\n" + "=" * 60)
    print(" Mission dispatcher ready (park the AGV facing the location):")
    for loc, (kind, *_) in REGISTRY.items():
        print(f"   {loc:8s} -> {kind}")
    print(" Type a location id or publish on /mcs/command (STOP on /mcs/stop). (q/Enter quits)")
    print("=" * 60)

    try:
        while rclpy.ok():
            try:
                loc = cmd_q.get(timeout=0.5)  # timeout so rclpy.ok() stays checked
            except queue.Empty:
                continue
            if loc in ('q', 'quit', ''):
                break
            run_mission(node, loc)
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
