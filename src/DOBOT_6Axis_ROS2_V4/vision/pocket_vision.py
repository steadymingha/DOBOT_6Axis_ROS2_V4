"""Base-pocket state from the wrist camera: which of the four pockets hold a box.

Replaces the hardcoded pocket index / box model names in the sequences: at
command time the arm looks down at the base ONCE from the hub, reads per-pocket
occupancy, and the sequence takes the next usable pocket in POCKET_ORDER_Y
order -- place into the next FREE pocket, pick from the next FILLED one.

Occupancy is DEPTH-based, not marker-based: the pockets are rigid to base_link
at known coordinates, so the depth along the ray through each pocket centre is
bimodal -- the pocket surface when empty, the box top (BOX_H closer) when
occupied. Anything else (gripper occluding, holed depth) reads UNKNOWN. No tags
to print or texture into the sim models, and the same check runs unchanged on
the real robot. (If pocket ArUcos are ever wanted anyway, only
pocket_occupancy() changes; the topic and the sequence seam stay.)

Split like tag_vision: this module is numpy-only (no cv2, no pinocchio; ROS
imports are lazy) so it imports in BOTH python envs --
    tag_vision_node.py (system cv2 env): pocket_occupancy() every tick ->
        /vision/pocket_state (Int32MultiArray, 4 x {-1 unknown, 0 empty, 1 box})
    sequences (.venv / system): subscribe(node) once at startup, then
        check_pockets(node) at command time (hub -> J5 wrist bend toward the
        base -> majority vote -> bend back) and next_free()/next_filled(). model_at()
        resolves the Gazebo model name of the box nearest a world point (via the
        gazebo_ros_state plugin's /gazebo/model_states), so picks no longer
        hardcode box names either.

Self-check (either env):  python3 vision/pocket_vision.py
"""

import math
import time

import numpy as np

# Pocket geometry in base_link, mirrored from cr7_pnp/geometry.py (this module
# must import in the system cv2 env, and importing cr7_pnp pulls pinocchio).
# POCKET_ORDER_Y is the PLACE order (== shelf_pick_place.PLACE_ORDER_Y).
POCKET_X = 0.3705
POCKET_ORDER_Y = (-0.177, -0.059, 0.059, 0.177)
POCKET_SURFACE_Z = -0.05
BOX_H = 0.14                    # box height (cr7_pnp.gripper_params.BOX_SIZE[2])

UNKNOWN, EMPTY, OCCUPIED = -1, 0, 1

# Classification knobs. Z_TOL splits the 140 mm empty/occupied depth gap; a
# reading near NEITHER level (gripper in the way, clutter) is UNKNOWN.
ROI_R = 6                       # half-size (px) of the depth patch per pocket
ROI_MIN_FRAC = 0.3              # need this fraction of finite pixels, else UNKNOWN
Z_TOL = 0.035                   # |measured - expected| tolerance (m)

# Look viewpoint: a pure WRIST BEND at the hub -- hub joints with only J5
# rotated until the camera looks at the pocket-row centre (the D405 faces
# forward off the flange, so bending the wrist pitches it down at the base).
# The delta is DERIVED per run from the camera extrinsic (FK + TF), scanned
# over J5 -- no IK, no RRT, no branch choice: the arm never leaves the hub's
# joint family, and the return is the same validity-checked line backwards.
# (The earlier auto-aim TCP pose put the tool ~horizontal -- 90 deg out of the
# hub family -- which contorted the IK branch and stalled the free RRT.)
LOOK_MAX_OFF_DEG = 20.0         # reject a J5 scan that can't aim within this
# Jogged 6-joint override (tools/jog_tcp.py) if the J5 scan can't frame the
# pockets in your cell -- same fallback style as CAPTURE_A_JOINTS.
# Current value (2026-07-15): the user-jogged fixed-jaw-mount viewpoint LIFTED
# +180 mm at the same TCP x/y and orientation (IK, same branch family): the
# jogged height put the END pockets' BOX-TOP points -2.7 deg OUTSIDE the FOV,
# so pockets 0/3 flickered to -1 depending on box seating. Verified offline
# (combined collision model): collision-free, hub->look straight sweep clean,
# all 4 pocket box-top points in FOV with >= +9.0 deg margin. The J5-only
# auto scan cannot aim on this arm, so this pin is required.
LOOK_JOINTS = (-0.32500, 0.19904, -1.00038, -0.05589, -2.95171, -0.50930)
LOOK_SETTLE_S = 1.5             # dwell before reading (arm drifts after "finished")
_N_FRAMES, _TIMEOUT_S = 9, 6.0


# --- small SE3 helpers (local: tag_vision has these too, but it imports cv2) ---
def make_T(R, t):
    T = np.eye(4)
    T[:3, :3] = R
    T[:3, 3] = np.asarray(t, dtype=float)
    return T


def inv_T(T):
    R, t = T[:3, :3], T[:3, 3]
    Ti = np.eye(4)
    Ti[:3, :3] = R.T
    Ti[:3, 3] = -R.T @ t
    return Ti


def quat_to_R(x, y, z, w):
    n = math.sqrt(x * x + y * y + z * z + w * w) or 1.0
    x, y, z, w = x / n, y / n, z / n, w / n
    return np.array([
        [1 - 2 * (y * y + z * z), 2 * (x * y - z * w), 2 * (x * z + y * w)],
        [2 * (x * y + z * w), 1 - 2 * (x * x + z * z), 2 * (y * z - x * w)],
        [2 * (x * z - y * w), 2 * (y * z + x * w), 1 - 2 * (x * x + y * y)],
    ])


def R_to_quat(m):
    tr = m[0, 0] + m[1, 1] + m[2, 2]
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        w, x = 0.25 * s, (m[2, 1] - m[1, 2]) / s
        y, z = (m[0, 2] - m[2, 0]) / s, (m[1, 0] - m[0, 1]) / s
    elif m[0, 0] > m[1, 1] and m[0, 0] > m[2, 2]:
        s = math.sqrt(1.0 + m[0, 0] - m[1, 1] - m[2, 2]) * 2
        w, x = (m[2, 1] - m[1, 2]) / s, 0.25 * s
        y, z = (m[0, 1] + m[1, 0]) / s, (m[0, 2] + m[2, 0]) / s
    elif m[1, 1] > m[2, 2]:
        s = math.sqrt(1.0 + m[1, 1] - m[0, 0] - m[2, 2]) * 2
        w, x = (m[0, 2] - m[2, 0]) / s, (m[0, 1] + m[1, 0]) / s
        y, z = 0.25 * s, (m[1, 2] + m[2, 1]) / s
    else:
        s = math.sqrt(1.0 + m[2, 2] - m[0, 0] - m[1, 1]) * 2
        w, x = (m[1, 0] - m[0, 1]) / s, (m[0, 2] + m[2, 0]) / s
        y, z = (m[1, 2] + m[2, 1]) / s, 0.25 * s
    return (x, y, z, w)


# --- detection (vision-node side): depth frame -> per-pocket state --------------
def pocket_occupancy(depth, K, T_base_optical):
    """Per-pocket state list ({-1, 0, 1}, POCKET_ORDER_Y order) from one aligned
    depth frame and the live base_link <- optical transform.

    Each pocket's BOX-TOP CENTRE (pocket centre lifted by BOX_H) is projected
    into the image and the ROI median depth there is compared to that point's
    distance: at it -> a box top -> OCCUPIED; beyond it -> the ray passed
    through empty box space -> EMPTY; closer -> something else in the way
    (gripper/arm) -> UNKNOWN. Sampling at box-top height (not the surface) is
    what makes an OBLIQUE view safe: until the target the ray stays ABOVE the
    box-top plane, so it can never graze a NEIGHBOUR pocket's box -- the
    surface-point scheme did exactly that and read a false OCCUPIED on the far
    pocket (measured: empty pocket 0 read [1] once 3 boxes sat in 1..3).
    Out-of-frame / holed pixels are UNKNOWN, so the topic is meaningful ONLY
    while the camera actually frames the pockets -- consumers vote over a
    burst taken at the look pose (check_pockets)."""
    T_bo = np.asarray(T_base_optical, dtype=float)
    cam_z = T_bo[2, 3]
    if cam_z < POCKET_SURFACE_Z + BOX_H + 0.1:      # not a look-down viewpoint
        return [UNKNOWN] * len(POCKET_ORDER_Y)
    T_ob = inv_T(T_bo)
    K = np.asarray(K, dtype=float)
    fx, fy, cx, cy = K[0, 0], K[1, 1], K[0, 2], K[1, 2]
    H, W = depth.shape
    out = []
    for y in POCKET_ORDER_Y:
        b = T_ob @ np.array([POCKET_X, y, POCKET_SURFACE_Z + BOX_H, 1.0])
        if b[2] < 0.07:                             # behind / inside the camera
            out.append(UNKNOWN)
            continue
        u = int(round(fx * b[0] / b[2] + cx))
        v = int(round(fy * b[1] / b[2] + cy))
        if not (ROI_R <= u < W - ROI_R and ROI_R <= v < H - ROI_R):
            out.append(UNKNOWN)
            continue
        roi = depth[v - ROI_R:v + ROI_R + 1, u - ROI_R:u + ROI_R + 1]
        finite = roi[np.isfinite(roi)]
        if finite.size < ROI_MIN_FRAC * roi.size:
            out.append(UNKNOWN)
            continue
        d = float(np.median(finite))
        if abs(d - b[2]) < Z_TOL:
            out.append(OCCUPIED)                    # box top right where it belongs
        elif d > b[2] + Z_TOL:
            out.append(EMPTY)                       # ray sailed through empty box space
        else:
            out.append(UNKNOWN)                     # something closer: gripper/arm
    return out


# --- pocket selection -----------------------------------------------------------
def _scan(occ, want, start, step):
    for k in range(len(POCKET_ORDER_Y)):
        i = (start + step * k) % len(POCKET_ORDER_Y)
        if occ[i] == want:
            return i
    return None


def next_free(occ, start=0, step=1):
    """First EMPTY pocket scanning from `start` in `step` direction (wrapping),
    or None. step=-1 scans high index -> low (fill from the other end). UNKNOWN
    pockets are never used -- conservative on both sides."""
    return _scan(occ, EMPTY, start, step)


def next_filled(occ, start=0, step=1):
    """First OCCUPIED pocket scanning from `start` in `step` direction
    (wrapping), or None."""
    return _scan(occ, OCCUPIED, start, step)


# --- sequence-side plumbing -------------------------------------------------------
def subscribe(node):
    """Cache the latest pocket state on the node. Call once at startup (vision
    mode only); check_pockets() reads the cache and returns None (-> static
    default pocket) if this was never called (--no-vision)."""
    from std_msgs.msg import Int32MultiArray
    node._pocket_state = None
    node.create_subscription(Int32MultiArray, '/vision/pocket_state',
                             lambda m: setattr(node, '_pocket_state', m), 10)


def subscribe_models(node):
    """Cache the latest Gazebo model states for model_at(). Call once at
    startup, vision or not -- grasps always resolve the box name this way in
    sim. No-op on the real robot (no gazebo_msgs / nothing publishes)."""
    node._model_states = None
    try:
        from gazebo_msgs.msg import ModelStates
        node.create_subscription(ModelStates, '/gazebo/model_states',
                                 lambda m: setattr(node, '_model_states', m), 10)
    except ImportError:
        pass


def model_at(node, world_xyz, tol=0.06):
    """Gazebo model whose origin is nearest `world_xyz` (within `tol` m), or
    None. Resolves WHICH box sits at a pocket/slot so picks stop hardcoding
    names. Needs the gazebo_ros_state plugin in the world (publishes
    /gazebo/model_states); odom == the Gazebo world frame in this project.
    tol < the 0.118 m pocket pitch, so a neighbour box can never match."""
    ms = getattr(node, '_model_states', None)
    if ms is None or world_xyz is None:
        return None
    p = np.asarray(world_xyz, dtype=float)
    best, best_d = None, tol
    for name, pose in zip(ms.name, ms.pose):
        d = float(np.linalg.norm(
            p - [pose.position.x, pose.position.y, pose.position.z]))
        if d < best_d:
            best, best_d = name, d
    return best


def base_to_world(node, xyz):
    """base_link point -> world (odom) xyz via the live TF tree, or None."""
    import rclpy
    from rclpy.duration import Duration
    try:
        tf = node.tf_buffer.lookup_transform(
            'odom', 'base_link', rclpy.time.Time(), timeout=Duration(seconds=3.0))
    except Exception as e:
        node.get_logger().error(f"[pockets] TF odom<-base_link failed: {e}")
        return None
    t, q = tf.transform.translation, tf.transform.rotation
    return quat_to_R(q.x, q.y, q.z, q.w) @ np.asarray(xyz, dtype=float) \
        + np.array([t.x, t.y, t.z])


def _extrinsic(node):
    """Constant TCP<-optical (4x4) measured at the CURRENT config from FK + TF
    (both are rigid to the flange), or None if the TF is unavailable."""
    import rclpy
    from rclpy.duration import Duration
    pos, R = node.ik_model.fk_tcp(node.ik_model.pin_q(node.current_joints.tolist()))
    try:
        tf = node.tf_buffer.lookup_transform(
            'base_link', 'd405_optical_frame', rclpy.time.Time(),
            timeout=Duration(seconds=3.0))
    except Exception as e:
        node.get_logger().error(f"[pockets] TF base<-optical failed: {e}")
        return None
    t, q = tf.transform.translation, tf.transform.rotation
    return inv_T(make_T(R, pos)) @ make_T(
        quat_to_R(q.x, q.y, q.z, q.w), [t.x, t.y, t.z])


def _look_joints(node):
    """Hub joints with ONLY J5 bent so the camera aims at the pocket-row centre.
    Scans the J5 delta with the constant extrinsic (optical pose for any config
    = FK(q) @ T_tcp_opt) and returns the collision-free config with the smallest
    aim error, or None if no bend aims within LOOK_MAX_OFF_DEG (then jog a full
    config and set LOOK_JOINTS). Pure math -- the arm does not move here."""
    T_tcp_opt = _extrinsic(node)
    if T_tcp_opt is None:
        return None
    target = np.array([POCKET_X, 0.0, POCKET_SURFACE_Z])
    hub = np.array(node.hub_q, dtype=float)
    lo, hi = node.joint_limits[4]
    cands = []
    for d in np.arange(-2.4, 2.4001, 0.05):
        q = hub.copy()
        q[4] += d
        if not (lo <= q[4] <= hi):
            continue
        pos, R = node.ik_model.fk_tcp(node.ik_model.pin_q(q.tolist()))
        T_bo = make_T(R, pos) @ T_tcp_opt
        v = target - T_bo[:3, 3]
        n = np.linalg.norm(v)
        if n < 0.15:                    # camera on top of the pockets: useless
            continue
        ang = math.degrees(math.acos(max(-1.0, min(1.0, float(T_bo[:3, 2] @ v) / n))))
        if ang <= LOOK_MAX_OFF_DEG:
            cands.append((ang, q))
    for ang, q in sorted(cands, key=lambda c: c[0]):
        if node.is_state_valid(q.tolist()):
            node.get_logger().info(
                f"[pockets] look = hub + J5 {q[4] - hub[4]:+.2f} rad "
                f"(aim {ang:.0f} deg off row centre)")
            return q.tolist()
    node.get_logger().error(
        "[pockets] no J5 bend aims the camera at the pockets "
        f"(<{LOOK_MAX_OFF_DEG:.0f} deg, collision-free); jog a viewpoint "
        "(tools/jog_tcp.py) and set pocket_vision.LOOK_JOINTS")
    return None


def check_pockets(node):
    """From the hub: bend the wrist (J5 only) to look at the base, majority-vote
    a burst of /vision/pocket_state, and bend back. Returns the 4-entry
    occupancy list ({-1, 0, 1}, POCKET_ORDER_Y order); all-UNKNOWN when the look
    or the read FAILED (callers then abort -- next_free/next_filled find
    nothing usable); None only when vision is not configured at all
    (subscribe() never called, --no-vision) -- only THAT falls back to the
    caller's static default pocket."""
    if getattr(node, '_pocket_state', '_unsub') == '_unsub':
        return None                                 # --no-vision
    failed = [UNKNOWN] * len(POCKET_ORDER_Y)
    q_look = list(LOOK_JOINTS) if LOOK_JOINTS is not None else _look_joints(node)
    if q_look is None:
        return failed
    if not node.joint_move(q_look):
        node.get_logger().error("[pockets] look bend blocked; back to hub")
        node.joint_move(list(node.hub_q))
        return failed
    time.sleep(LOOK_SETTLE_S)
    node._pocket_state = None
    votes, t0 = [], time.time()
    while len(votes) < _N_FRAMES and time.time() - t0 < _TIMEOUT_S:
        m = node._pocket_state
        if m is not None:
            node._pocket_state = None
            votes.append(list(m.data))
        time.sleep(0.05)
    # Exact hub return: the same joint line backwards (just swept, so valid).
    if not node.joint_move(list(node.hub_q)):
        node.get_logger().error("[pockets] hub return blocked")
        return failed
    if not votes:
        node.get_logger().error(
            "[pockets] no /vision/pocket_state -- is tag_vision_node.py running?")
        return failed
    arr = np.array(votes, dtype=int)
    occ = [int(np.bincount(arr[:, i] + 1, minlength=3).argmax()) - 1
           for i in range(arr.shape[1])]
    node.get_logger().info(
        f"[pockets] occupancy {occ} (1=box, 0=empty, -1=unknown; "
        f"y order {POCKET_ORDER_Y}) from {len(votes)} frames")
    return occ


# --- self-check ----------------------------------------------------------------
def _demo():
    # 1) selection: wrap, skip UNKNOWN, None when exhausted.
    occ = [OCCUPIED, EMPTY, UNKNOWN, OCCUPIED]
    assert next_free(occ, 0) == 1 and next_free(occ, 2) == 1, "free wrap"
    assert next_filled(occ, 1) == 3 and next_filled(occ, 3) == 3, "filled scan"
    assert next_free([OCCUPIED] * 4) is None, "no free -> None"
    assert next_filled([EMPTY, UNKNOWN, EMPTY, UNKNOWN]) is None, "no box -> None"
    # reverse fill (place from the high-index end): 3 -> 2 -> 1 -> 0
    assert next_free([EMPTY] * 4, 3, step=-1) == 3, "reverse start"
    assert next_free(occ, 3, step=-1) == 1, "reverse skips occupied/unknown"

    # 2) SE3 round-trips.
    R = quat_to_R(*R_to_quat(np.array([[0., -1., 0.], [-1., 0., 0.], [0., 0., -1.]])))
    assert np.allclose(R, [[0, -1, 0], [-1, 0, 0], [0, 0, -1]], atol=1e-9), "quat rt"
    T = make_T(R, [0.1, -0.2, 0.3])
    assert np.allclose(inv_T(T) @ T, np.eye(4), atol=1e-12), "inv_T"

    # 3) occupancy classification on a synthetic look-down frame, sampling at
    #    the BOX-TOP projections: pocket 0 empty (ray reads DEEPER than the
    #    box-top point), 1 occupied (reads AT it), 2 holed (NaN), 3 occluded
    #    (reads CLOSER, e.g. the gripper) -> [0, 1, -1, -1].
    K = np.array([[500., 0., 320.], [0., 500., 240.], [0., 0., 1.]])
    cam = (POCKET_X, 0.0, POCKET_SURFACE_Z + 0.5)
    T_bo = make_T(np.array([[0., -1., 0.], [-1., 0., 0.], [0., 0., -1.]]), cam)
    depth = np.full((480, 640), np.nan, np.float32)
    T_ob = inv_T(T_bo)
    offsets = [+0.15, 0.0, np.nan, -0.15]          # vs the box-top point distance
    for y, dv in zip(POCKET_ORDER_Y, offsets):
        b = T_ob @ np.array([POCKET_X, y, POCKET_SURFACE_Z + BOX_H, 1.0])
        u, v = int(500 * b[0] / b[2] + 320), int(500 * b[1] / b[2] + 240)
        depth[v - 10:v + 11, u - 10:u + 11] = b[2] + dv
    assert pocket_occupancy(depth, K, T_bo) == [EMPTY, OCCUPIED, UNKNOWN, UNKNOWN], \
        "classification"

    # 4) a non-look-down viewpoint (camera at pocket height) must read UNKNOWN.
    T_side = make_T(np.eye(3), (POCKET_X, 0.0, POCKET_SURFACE_Z))
    assert pocket_occupancy(depth, K, T_side) == [UNKNOWN] * 4, "side view guard"

    print("pocket_vision self-check: OK")


if __name__ == '__main__':
    _demo()
