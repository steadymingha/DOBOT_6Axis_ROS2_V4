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

Profiles (docs/real_robot_pipeline_plan.md 4.1c). Default is ALWAYS sim; the real
robot must be asked for explicitly, twice: DOBOT_ENV=real (collision constants,
read at import) AND --profile real (everything else) -- they must agree or main.py
refuses to start.
    DOBOT_ENV=real python3 main.py --profile real              # no gripper, no attacher
    DOBOT_ENV=real python3 main.py --profile real --gripper    # real gripper server up
    DOBOT_ENV=real python3 main.py --profile real --preflight  # plan+validate, arm never moves
"""

import argparse
import os
import sys
import time
import json
import queue
import threading

import rclpy
from rclpy.executors import MultiThreadedExecutor
from geometry_msgs.msg import PoseStamped
from std_msgs.msg import Int32, String, UInt8MultiArray

from cr7_pnp import HubPickPlace, ENV_NAME
from cr7_pnp.gripper_params import load_env
from cr7_pnp.robot_feed import RobotFeed, default_robot_ip
from cr7_pnp.collision_env import register_surfaces, collision_model_xacro

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
    ok = node.update_shelf_collision(getattr(node, 'shelf_pose', None))
    for dev_pose in wb.DEVICES.values():
        ok = node.update_wirebonder_collision(dev_pose) and ok
    return ok


def locate_box(node, n=15, timeout=8.0):
    """Real-robot shelf LOCATE (the --vision --run chain, main.py side): hub -> taught
    'obs' viewpoint (test/points.json, --teach obs) -> /vision/capture -> median-n of
    the detector's box centre (/vision/device_pose, from vision_hover_node) ->
    node.vision_box -> retrace to the hub. Nothing new: the sanity box and heights
    are vision_target's. Returns True on a gated read; False aborts the mission
    (no silent fallback to the layout -- jog and edit env/real.json for that)."""
    sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), 'test'))
    import vision_target as vt                                # noqa: E402  (mocks cv2 if broken)
    import numpy as np
    try:
        with open(OBS_POINTS) as f:
            obs_q = json.load(f)['obs']['joints']
    except (OSError, KeyError) as e:
        return wb.fail(node, proto.ErrorCode.INIT_FAILED,
                       f"[locate_box] no 'obs' in {OBS_POINTS} ({e}); teach it: test/run.sh --teach obs")
    node.vision_box = None
    if not node.go_to_hub():
        return False

    def forward():
        if not node.go_to_config(obs_q):
            return False
        time.sleep(2.0)                    # settle: the snapshot needs a STILL arm
        return True
    ok_fwd, fwd = node.capture(forward)
    if not ok_fwd:
        node.go_to_hub()
        return wb.fail(node, proto.ErrorCode.PLAN_FAILED,
                       "[locate_box] obs viewpoint unreachable/blocked (RRT from the hub failed)")

    node._vision_pose = None
    node._cap_pub.publish(Int32(data=0))
    node._cap_pub.publish(Int32(data=1))
    samples, seen, t0 = [], set(), time.time()
    while len(samples) < n and time.time() - t0 < vt.SNAPSHOT_TIMEOUT_S + timeout:
        ps = node._vision_pose
        if ps is not None:
            key = (ps.header.stamp.sec, ps.header.stamp.nanosec)
            if key not in seen:
                seen.add(key)
                samples.append((ps.pose.position.x, ps.pose.position.y, ps.pose.position.z))
        time.sleep(0.02)
    node._cap_pub.publish(Int32(data=0))
    node.replay_reverse(fwd)               # obs -> hub, the recorded way (box-safe)
    node.go_to_hub()
    if not samples:
        return wb.fail(node, proto.ErrorCode.TAG_NOT_DETECTED,
                       "[locate_box] no /vision/device_pose -- vision_hover_node up, "
                       "vision_runner up (host), magazine in view?")
    arr = np.array(samples)
    box = np.median(arr, axis=0)
    spread = arr.std(axis=0) * 1000.0
    if spread.max() > CAPTURE_SPREAD_MAX_MM:
        return wb.fail(node, proto.ErrorCode.CAPTURE_SPREAD_HIGH,
                       f"[locate_box] REJECTED: spread {spread.round(1)} mm > {CAPTURE_SPREAD_MAX_MM} "
                       f"-- two publishers on /vision/device_pose or an unstable read")
    hover = box + [0.0, 0.0, vt.MAGAZINE_HEIGHT_M / 2.0 + vt.HOVER_CLEARANCE_M]
    for k, ax in (('x', 0), ('y', 1), ('z', 2)):
        lo, hi = vt.VISION_WORK_BOX[k]
        if not (lo <= hover[ax] <= hi):
            return wb.fail(node, proto.ErrorCode.CAPTURE_IMPLAUSIBLE,
                           f"[locate_box] REJECTED: hover {k}={hover[ax]:+.4f} m outside "
                           f"VISION_WORK_BOX {k} [{lo:+.3f}, {hi:+.3f}]")
    node.vision_box = [float(v) for v in box]
    node.last_error = proto.ErrorCode.OK
    node.get_logger().info(f"[locate_box] box centre {np.round(box, 4).tolist()} m (base_link) "
                           f"from {len(samples)} frames, spread {spread.round(1)} mm")
    return True


def locate(node, kind, *params):
    """LOCATE: secure the target frame. True on success (abort the mission on False)."""
    if kind == 'wirebonder':
        # AprilTag look-before-you-transfer: refresh every device pose from
        # /vision/device_pose at the capture viewpoint, then return to the hub.
        return wb.capture_device(node)
    if kind == 'shelf':
        if node.use_vision_box:            # real: AI magazine detector, not the tier tag
            return locate_box(node)
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


# location id -> TargetID, for the feedback frame (reverse of TARGET_LOCATION)
LOCATION_TARGET = {loc: tid for tid, loc in proto.TARGET_LOCATION.items()}


def report(node, loc_id, result):
    """The [REPORT] seam, now real: latch the mission outcome into the fields the
    /mcs/feedback frame carries (missionSeq bump = 'new result' marker for MCS)."""
    node.last_target = int(LOCATION_TARGET.get(loc_id, 255))
    node.last_result = int(result)
    node.mission_seq += 1


def run_mission(node, loc_id):
    """Dict-router FSM: IDLE -> LOCATE -> PICK/PLACE -> REPORT -> IDLE."""
    entry = REGISTRY.get(loc_id)
    if entry is None:
        print(f"[IDLE] unknown location '{loc_id}' (known: {', '.join(REGISTRY)})")
        return
    kind, *params = entry
    node.last_error = proto.ErrorCode.OK       # fresh mission: DONE reports code 0
    node.last_error_detail = ""
    # ponytail: coarse cooperative abort -- checked before LOCATE and before
    # PICK/PLACE, not mid-motion. True mid-swing STOP needs hooks inside
    # wb.transfer / shelf.pick_place_one_box; add there if the arm must halt instantly.
    if getattr(node, 'abort', False):
        print(f"[ABORT] {loc_id} cancelled before start")
        report(node, loc_id, proto.Result.ABORTED)
        return
    # The AGV may have driven since the last mission: swap the whole collision
    # world in BEFORE the first arm motion (LOCATE's capture move included).
    if not refresh_collision_world(node):
        # fail-closed: never move with a phantom parked far (node refuses anyway)
        node.last_error = proto.ErrorCode.TF_UNAVAILABLE
        node.last_error_detail = "collision world not placed (TF)"
        print(f"[REPORT] {loc_id} collision world not placed (TF) -> abort")
        report(node, loc_id, proto.Result.FAILED)
        return
    node.mcs_state = proto.RobotState.LOCATE
    print(f"[LOCATE] {loc_id} ({kind})")
    if not locate(node, kind, *params):
        if not node.preflight:
            print(f"[REPORT] {loc_id} LOCATE failed -> abort "
                  f"({proto.ErrorCode(node.last_error).name}: {node.last_error_detail})")
            report(node, loc_id, proto.Result.FAILED)
            return
        # Nothing moves in preflight, so validating the geometry at the anchor
        # pose is still worth it (that is what preflight is for) -- but say so.
        print(f"[preflight] {loc_id} LOCATE failed (vision not up?) -> continuing "
              f"on the ANCHOR pose for validation only")
    if getattr(node, 'abort', False):
        print(f"[ABORT] {loc_id} cancelled after LOCATE")
        report(node, loc_id, proto.Result.ABORTED)
        return
    node.mcs_state = proto.RobotState.PICKPLACE
    print(f"[PICK/PLACE] {loc_id}")
    ok = pick_place(node, kind, *params)
    node.vision_box = None                 # one capture == one pick; never reuse
    print(f"[REPORT] {loc_id} {'DONE' if ok else 'FAILED'} (arm at hub)"
          + ("" if ok else f" ({proto.ErrorCode(node.last_error).name}: {node.last_error_detail})"))
    report(node, loc_id, proto.Result.DONE if ok else
           (proto.Result.ABORTED if getattr(node, 'abort', False) else proto.Result.FAILED))


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


# One dict per profile; individual flags overlay it. Booleans, one implementation
# each -- no interface layer. Boundaries: collision constants (DOBOT_ENV, not
# here), vision (launch arg picks the node), gripper, attach, [descend: stage 4].
# vision_box: shelf target from the AI magazine detector (vision_hover_node) via
# locate_box(); sim keeps ArUco locate_shelf + layout. pockets: /vision/pocket_state
# occupancy look (tag_vision_node depth) -- no real publisher until stage 3c, so real
# uses the static default pocket instead of bending to look and aborting on silence.
# descend: contact-stop descend (cr7_pnp/contact.py, the --vision --run verified
# chain) at the two shelf descend legs; sim keeps the phantom-sensor analog.
PROFILES = {
    'sim':  dict(gripper=True,  attach=True,  vision_box=False, pockets=True,  descend=False),
    'real': dict(gripper=False, attach=False, vision_box=True,  pockets=False, descend=True),
}
OBS_POINTS = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'test', 'points.json')
CAPTURE_SPREAD_MAX_MM = 5.0     # republished ONE value -> spread ~0; more = 2 publishers/unstable


def parse_args(argv):
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('--profile', choices=sorted(PROFILES), default='sim')
    ap.add_argument('--gripper', action='store_true',
                    help='real gripper action server is up (overrides the profile)')
    ap.add_argument('--preflight', action='store_true',
                    help='validate the whole sequence with IK/collision, send NO motion')
    ap.add_argument('--selftest', action='store_true', help='registry sanity, no ROS')
    return ap.parse_args(argv)


def check_profile(a):
    """Boot-time cross-checks (plan 6.1). Any mismatch -> exit before ROS init.
    The most dangerous state this pipeline can be in is 'real robot, sim
    collision constants'; this is the 5-line insurance against it."""
    if a.profile != ENV_NAME:
        sys.exit(f"[profile] --profile {a.profile} but DOBOT_ENV={ENV_NAME}: the "
                 f"collision constants come from cr7_pnp/env/{ENV_NAME}.json. "
                 f"Set DOBOT_ENV={a.profile} (or change --profile). Refusing to start.")
    if a.profile == 'real':
        if not load_env().get('measured', False):
            msg = ("[profile] cr7_pnp/env/real.json is not marked \"measured\": true "
                   "-- its shelf/pocket/box values are still the sim copy.")
            if not a.preflight:
                sys.exit(msg + " Measure them (docs/collision_model_guide.md) first. "
                         "Refusing to start.")
            print(msg + " Allowed for --preflight only (nothing moves).")
        ip = default_robot_ip()
        feed = RobotFeed(ip)
        feed.start()
        if not feed.wait_ready(5.0):
            sys.exit(f"[profile] real: no 30004 feed from {ip} ({feed.error or 'timeout'}). "
                     "Is the controller on / reachable? Refusing to start.")
        feed.stop()
        # The URDF and the controller disagree on J1/J5/J6 sign; node.py flips
        # only when this is set. Forgetting it mirrors the arm -- so set it here.
        os.environ['CR7_REAL_ROBOT'] = '1'
        print(f"[profile] real: 30004 feed ok from {ip}, CR7_REAL_ROBOT=1")
    elif os.getenv('CR7_REAL_ROBOT') == '1':
        sys.exit("[profile] sim but CR7_REAL_ROBOT=1 (joint signs would be flipped "
                 "for Gazebo). Unset it. Refusing to start.")


def main(args=None):
    a = parse_args(sys.argv[1:])
    if a.selftest:
        selftest()
        return
    check_profile(a)
    prof = dict(PROFILES[a.profile])
    if a.gripper:
        prof['gripper'] = True

    rclpy.init(args=args)
    node = HubPickPlace()
    node.use_gripper, node.use_attach = prof['gripper'], prof['attach']
    node.use_vision_box = prof['vision_box']
    node.preflight = a.preflight
    if a.profile == 'sim' and (node.attach_client is None or
                               not node.attach_client.wait_for_service(timeout_sec=5.0)):
        # sim without the link attacher == wrong launch (or wrong profile)
        node.get_logger().fatal("[profile] sim but /ATTACHLINK is absent -- is Gazebo "
                                "up with gazebo_ros_link_attacher? Refusing to start.")
        node.destroy_node(); rclpy.shutdown(); return
    node.get_logger().info(f"[profile] {a.profile} env={ENV_NAME} gripper={prof['gripper']} "
                           f"attach={prof['attach']} vision_box={prof['vision_box']} "
                           f"pockets={prof['pockets']} preflight={a.preflight}")
    # Combined (arm+cube+AGV) collision model where it builds (sim); the real
    # Jetson has no neo_simulation2, so it falls back to arm-only + taught surfaces.
    node.setup_planner(combined_xacro=collision_model_xacro())
    # Taught table/walls (cr7_pnp/env/<DOBOT_ENV>_surfaces.json; none in sim).
    register_surfaces(node)

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
    if prof['pockets']:
        pockets.subscribe(node)            # real: not until stage 3c -> static default pocket
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
    node.last_error_detail = ""            # both land in the /mcs/feedback frame
    cmd_q = queue.Queue()

    # ---- MCS feedback (Arm -> bridge -> MCS) --------------------------------
    # ONE topic, ONE fixed 240-byte frame (ros2api_v2.xlsx). The bridge forwards
    # the bytes verbatim. Robot half comes from the 30004 feed (real profile);
    # sim has no controller, so those fields ride as zeros -- the frame is still
    # published so the bridge/MCS side can be tested against the sim.
    node.mcs_state = proto.RobotState.IDLE
    node.mission_seq = 0
    node.last_target = 255
    node.last_result = int(proto.Result.NONE)
    feed = None
    if a.profile == 'real':
        feed = RobotFeed(default_robot_ip())
        feed.start()                       # check_profile already proved 30004 is up
        if prof['descend'] and not a.preflight:
            # Contact-stop descend kit: dashboard proxies + torque detector on the
            # ONE feed. arm_touchoff = SetCollisionLevel(3) + post-collision PAUSE +
            # BackDistance 0 -- the exact configuration the --vision --run cycles
            # were verified with. If it cannot be configured, do not run missions:
            # the descend would have no firmware backstop.
            from cr7_pnp.contact import Dashboard, ContactDetector
            if not feed.wait_ready(5.0):
                node.get_logger().fatal("[descend] 30004 feed lost after startup; refusing")
                node.destroy_node(); rclpy.shutdown(); return
            dash = Dashboard(node)
            if not dash.arm_touchoff(3):
                node.get_logger().fatal("[descend] controller touch-off config failed "
                                        "(SetCollisionLevel/PostCollisionMode); refusing to start")
                node.destroy_node(); rclpy.shutdown(); return
            node.contact_kit = (dash, ContactDetector(feed), feed)
            node.use_real_descend = True
            node.get_logger().info("[descend] contact-stop active: 5 mm/s, torque trip "
                                   "(soft) + controller collision level 3 (hard)")
    fb_pub = node.create_publisher(UInt8MultiArray, '/mcs/feedback', 10)
    fb_tick = [0]

    def publish_feedback():
        fb_tick[0] += 1
        state = (proto.RobotState.ERROR_HOLD if getattr(node, '_unenforced', None)
                 else node.mcs_state)
        if state == proto.RobotState.IDLE and fb_tick[0] % 10:
            return                          # IDLE 1 Hz, missions 10 Hz
        st = feed.state()[0] if feed is not None else None
        z6 = (0.0,) * 6
        frame = proto.pack_feedback(
            robot_ts=st['timestamp'] if st else 0,
            run_time=st['run_time'] if st else 0,
            jetson_ts=int(time.time() * 1000),
            mission_seq=node.mission_seq, last_target=node.last_target,
            result=node.last_result, error_code=int(node.last_error),
            robot_state=int(state),
            robot_mode=st['robot_mode'] if st else 0,
            enable=st['enable'] if st else 0, error=st['error'] if st else 0,
            collision=st['collision'] if st else 0, safety=st['safety'] if st else 0,
            q_actual=st['q_actual'] if st else z6,
            qd_actual=st['qd_actual'] if st else z6,
            tool_vector=st['tool_vector'] if st else z6,
            m_actual=st['m_actual'] if st else z6)
        fb_pub.publish(UInt8MultiArray(data=frame))
    node.create_timer(0.1, publish_feedback)

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
            node.mcs_state = proto.RobotState.IDLE
    finally:
        if feed is not None:
            feed.stop()
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
