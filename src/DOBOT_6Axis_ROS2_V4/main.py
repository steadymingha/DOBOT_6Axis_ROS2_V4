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
    shelf      -> shelf.bringup (hub + pocket clear) + shelf.locate_shelf
                  (LOCATE, tier ArUco) + shelf.pick_place_one_box
                  (walks box_idx over the four tier-1 boxes)

One trigger == one transfer; re-trigger for the next unit (shelf walks its box index).

Run (sim up, AGV parked roughly facing the device):
    source /opt/ros/humble/setup.bash
    source ~/dobot_ws/install/setup.bash
    cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
    python3 vision/tag_vision_node.py               # terminal A (system cv2)
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

from cr7_pnp import HubPickPlace

# The two flows live in sequences/, the MCS protocol in comms/; add both to the
# path so they import by name.
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), 'sequences'))
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), 'comms'))
import wirebonder_pick_place as wb  # noqa: E402
import shelf_pick_place as shelf  # noqa: E402
import mcs_protocol as proto  # noqa: E402
from vision import pocket_vision as pockets  # noqa: E402

# --- location registry --------------------------------------------------------
# id -> (type, *params). One concrete job per id (re-trigger for the next unit).
#   wirebonder: ('wirebonder', device_name, (src, dst) Locations)
#   shelf:      ('shelf',)  -- walks node.box_idx over the four tier-1 boxes
#               (0-3); LOCATE reads the tier ArUco (shelf.locate_shelf)
# Add a line per real location; the AMR/MCS bridge will key into this by id.
REGISTRY = {
    'wb1':   ('wirebonder', 'wb1', wb.SEQUENCES['1']),   # base   -> slot A
    'wb2':   ('wirebonder', 'wb1', wb.SEQUENCES['2']),   # slot B -> slot C
    'wb3':   ('wirebonder', 'wb1', wb.SEQUENCES['3']),   # slot D -> base pocket
    'shelf': ('shelf',),
}


def refresh_collision_world(node):
    """Re-anchor EVERY world-fixed phantom to the live TF, before any motion of
    a mission. The phantoms are STATIC base-frame geometry: once the AGV drives
    (MCS sent it to another location), they all sit at stale coords and can
    float into the workspace (measured: shelf_stock_t12 blocked the wb slot-A
    place). One seam for ALL of them, run at every mission start -- a new
    station's phantom added later gets re-anchored by adding one line HERE.
    Coarse poses are fine (anchor / last capture): the LOCATE step re-places
    its own target at the fresh vision-read pose right after."""
    node.update_shelf_collision(getattr(node, 'shelf_pose', None))
    for dev_pose in wb.DEVICES.values():
        node.update_wirebonder_collision(dev_pose)


def locate(node, kind, *params):
    """LOCATE: secure the target frame. True on success (abort the mission on False)."""
    if kind == 'wirebonder':
        # AprilTag look-before-you-transfer: refresh every device pose from
        # /vision/device_pose at the capture viewpoint, then return to the hub.
        return wb.capture_device(node)
    if kind == 'shelf':
        # Tier ArUco look-before-you-pick: read the tag at the capture viewpoint
        # into node.shelf_pose and re-place the shelf collision at the live pose
        # (shelf analog of capture_device). Fails -> mission aborts rather than
        # placing boxes at a possibly-wrong SHELF_WORLD_POSE default.
        return shelf.locate_shelf(node)
    return False


def pick_place(node, kind, *params):
    """PICK/PLACE: run the matching transfer. True/False."""
    if kind == 'wirebonder':
        device, (src, dst) = params
        return wb.transfer(node, src, dst)
    if kind == 'shelf':
        idx = node.box_idx
        n = len(shelf.PLACE_ORDER_Y)
        if idx >= n:
            print(f"[shelf] all {n} boxes placed; nothing left")
            return False
        # Tier-1, fixed park (AGV spawned in position): no in-sequence driving.
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
    # The AGV may have driven since the last mission: swap the whole collision
    # world in BEFORE the first arm motion (LOCATE's capture move included).
    refresh_collision_world(node)
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

    # Vision layer: both poses arrive from tag_vision_node.py (odom). Cache the
    # latest of each; wb.refresh_device_pose() / shelf.refresh_shelf_pose() read
    # them during their LOCATE captures.
    node._vision_pose = None
    node.create_subscription(PoseStamped, '/vision/device_pose',
                             lambda m: setattr(node, '_vision_pose', m), 10)
    node._shelf_vision_pose = None
    node.shelf_pose = None
    node.create_subscription(PoseStamped, '/vision/shelf_pose',
                             lambda m: setattr(node, '_shelf_vision_pose', m), 10)
    # Drives the two-view capture: data=0 resets, data=1 grabs a view.
    node._cap_pub = node.create_publisher(Int32, '/vision/capture', 10)
    # Base-pocket occupancy + Gazebo model-name caches: transfers/places look at
    # the base once per command and pick the next usable pocket; grasps resolve
    # the box model name at runtime instead of hardcoding it.
    pockets.subscribe(node)
    pockets.subscribe_models(node)

    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2)  # wait for joint states

    # Shared hub: both flows use the same HUB_TCP (0.33, 0, 0.32) and azimuth
    # (PLACE_YAW == PICK_YAW == pi), so shelf.bringup serves both. It places the
    # shelf boards + resting-stock phantoms at the anchor pose and moves to the hub.
    if not shelf.bringup(node):
        node.get_logger().error("Hub bring-up failed; adjust HUB_TCP and retry")
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
