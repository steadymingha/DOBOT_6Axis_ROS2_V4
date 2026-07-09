"""Wirebonder AprilTag vision: detect tag -> device/slot poses in base_link.

Pure detection + SE3 geometry (no rclpy here, so it stays offline-testable). The
ROS plumbing (image/camera_info subscriptions, TF lookup) lives in the diagnostic
node (__main__ of wirebonder_vision_node.py, Task 2) and later in the pick/place
script (Task 3); both feed this module the captured frame, the camera intrinsics
and the base_link <- d405_optical_frame transform.

Markers are AprilTag 36h11 (textures april_36h11-*.png), 30 mm, IDs 0/1. We use
ID 0 only -- it sits on the device's left column above slots A/B and one tag gives
a full 6-DoF pose. Detected with cv2.aruco (OpenCV 4.5.4 old API).

Frames:
    optical : d405_optical_frame (ROS image convention, z forward)
    tag_cv  : OpenCV marker frame from estimatePoseSingleMarkers
    model   : wirebonder model frame (SLOT_OFFSET / SDF poses live here)
    base    : base_link (the robot arm root)

Pipeline: detect -> T_base_tagcv -> (R_CV_TO_SDF) -> T_base_tagsdf ->
@ inv(T_MODEL_TAG) -> T_base_model, then slots_in_base().

NOTE: R_CV_TO_SDF (OpenCV marker frame vs. the SDF tag-plate frame) is the one
convention that cannot be derived purely -- it was pinned empirically in Task 2 by
the side-by-side print against the DEVICES ground truth (it is Rz(-90)).
"""

import math

import numpy as np
import cv2

# --- device model geometry (model frame; authoritative source) ----------------
# Slot magazine centres in the wirebonder model frame. Mirrors
# wirebonder_pick_place.SLOT_OFFSET; Task 3 makes that script import these from
# here so there is a single source. Slot naming: A=H_L, B=G_L, C=G_R, D=H_R.
SLOT_OFFSET = {
    'A': (-0.348, -0.059, 0.896),   # H_L  (left, lower)
    'B': (-0.348, -0.059, 1.281),   # G_L  (left, upper)
    'C': (+0.348, -0.059, 1.281),   # G_R  (right, upper)
    'D': (+0.348, -0.059, 0.896),   # H_R  (right, lower)
}

# AprilTag ID 0 pose in the device model frame, from wirebonder/model.sdf
# (visual 'aruco_G_L'): centred under the left column, mid-height, plate turned to
# face the robot (-Y) by roll = pi/2.
TAG_ID = 0
TAG_SIZE_M = 0.03
TAG0_XYZ = (-0.348, -0.1205, 1.2)
TAG0_RPY = (math.pi / 2.0, 0.0, 0.0)

# OpenCV marker frame vs the SDF tag-plate frame: a fixed rotation about the tag
# normal. Pinned in Task 2 -- with this, vision matched the DEVICES ground truth
# (device-in-base ~ Rz(180)) to mm. It is Rz(-90).
R_CV_TO_SDF = np.array([[0., 1., 0.],
                        [-1., 0., 0.],
                        [0., 0., 1.]])

ARUCO_DICT_ID = cv2.aruco.DICT_APRILTAG_36h11


# --- small SE3 / rotation helpers (kept local so this module needs only numpy+cv2)
def rpy_to_R(roll, pitch, yaw):
    """URDF/SDF rpy (fixed-axis x,y,z) -> rotation matrix R = Rz @ Ry @ Rx."""
    cr, sr = math.cos(roll), math.sin(roll)
    cp, sp = math.cos(pitch), math.sin(pitch)
    cy, sy = math.cos(yaw), math.sin(yaw)
    Rx = np.array([[1, 0, 0], [0, cr, -sr], [0, sr, cr]])
    Ry = np.array([[cp, 0, sp], [0, 1, 0], [-sp, 0, cp]])
    Rz = np.array([[cy, -sy, 0], [sy, cy, 0], [0, 0, 1]])
    return Rz @ Ry @ Rx


def make_T(R, t):
    """4x4 homogeneous transform from a 3x3 rotation and a 3-vector."""
    T = np.eye(4)
    T[:3, :3] = R
    T[:3, 3] = np.asarray(t, dtype=float)
    return T


def inv_T(T):
    """Inverse of a homogeneous transform."""
    R = T[:3, :3]
    t = T[:3, 3]
    Ti = np.eye(4)
    Ti[:3, :3] = R.T
    Ti[:3, 3] = -R.T @ t
    return Ti


def R_to_quat(R):
    """Rotation matrix -> quaternion (x, y, z, w)."""
    m = R
    tr = m[0, 0] + m[1, 1] + m[2, 2]
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        w = 0.25 * s
        x = (m[2, 1] - m[1, 2]) / s
        y = (m[0, 2] - m[2, 0]) / s
        z = (m[1, 0] - m[0, 1]) / s
    elif m[0, 0] > m[1, 1] and m[0, 0] > m[2, 2]:
        s = math.sqrt(1.0 + m[0, 0] - m[1, 1] - m[2, 2]) * 2
        w = (m[2, 1] - m[1, 2]) / s
        x = 0.25 * s
        y = (m[0, 1] + m[1, 0]) / s
        z = (m[0, 2] + m[2, 0]) / s
    elif m[1, 1] > m[2, 2]:
        s = math.sqrt(1.0 + m[1, 1] - m[0, 0] - m[2, 2]) * 2
        w = (m[0, 2] - m[2, 0]) / s
        x = (m[0, 1] + m[1, 0]) / s
        y = 0.25 * s
        z = (m[1, 2] + m[2, 1]) / s
    else:
        s = math.sqrt(1.0 + m[2, 2] - m[0, 0] - m[1, 1]) * 2
        w = (m[1, 0] - m[0, 1]) / s
        x = (m[0, 2] + m[2, 0]) / s
        y = (m[1, 2] + m[2, 1]) / s
        z = 0.25 * s
    return (x, y, z, w)


# Constant: tag-0 pose in the model frame (from the SDF).
T_MODEL_TAG = make_T(rpy_to_R(*TAG0_RPY), TAG0_XYZ)


# --- detection -----------------------------------------------------------------
def _aruco_dict():
    return cv2.aruco.getPredefinedDictionary(ARUCO_DICT_ID)


def _detector_params():
    # OpenCV 4.5.4 old API.
    return cv2.aruco.DetectorParameters_create()


def _marker_object_points(size):
    """4 marker corners in the OpenCV tag frame (x right, y up, z out), order
    matching cv2.aruco (TL, TR, BR, BL) -- so R_CV_TO_SDF stays valid."""
    h = size / 2.0
    return np.array([[-h, h, 0], [h, h, 0], [h, -h, 0], [-h, -h, 0]], dtype=float)


def detect_tag(bgr, K, dist=None, tag_id=TAG_ID, size=TAG_SIZE_M):
    """Detect AprilTag `tag_id` in a BGR image and estimate its pose.

    K: 3x3 camera intrinsics. dist: distortion coeffs (zeros for the sim pinhole).
    Returns a LIST of candidate (rvec, tvec) solutions in the OpenCV optical frame,
    or None if not found. A planar tag (IPPE_SQUARE) yields up to 2 solutions; near
    fronto-parallel they have near-equal reprojection error, so the caller must
    disambiguate (device_pose_in_base picks the upright one) -- taking just the
    first solution is the ambiguity flip that swings the device yaw ~25 deg.
    """
    if dist is None:
        dist = np.zeros(5)
    gray = cv2.cvtColor(bgr, cv2.COLOR_BGR2GRAY) if bgr.ndim == 3 else bgr
    corners, ids, _ = cv2.aruco.detectMarkers(
        gray, _aruco_dict(), parameters=_detector_params())
    if ids is None:
        return None
    ids = ids.flatten()
    match = np.where(ids == tag_id)[0]
    if match.size == 0:
        return None
    i = int(match[0])
    _, rvecs, tvecs, _ = cv2.solvePnPGeneric(
        _marker_object_points(size), corners[i].reshape(4, 2),
        np.asarray(K, dtype=float), np.asarray(dist, dtype=float),
        flags=cv2.SOLVEPNP_IPPE_SQUARE)
    return [(r.ravel(), t.ravel()) for r, t in zip(rvecs, tvecs)]


def detect_tag_corners(bgr, tag_id=TAG_ID):
    """Detect tag_id and return its 4x2 image corners (order TL, TR, BR, BL, as
    cv2.aruco gives them), or None. Used by the two-view triangulation, which needs
    the raw pixels (not a per-view pose) so it can triangulate corners across the
    two camera positions -- that is what recovers depth a single view cannot."""
    gray = cv2.cvtColor(bgr, cv2.COLOR_BGR2GRAY) if bgr.ndim == 3 else bgr
    corners, ids, _ = cv2.aruco.detectMarkers(
        gray, _aruco_dict(), parameters=_detector_params())
    if ids is None:
        return None
    ids = ids.flatten()
    match = np.where(ids == tag_id)[0]
    if match.size == 0:
        return None
    return corners[int(match[0])].reshape(4, 2)


def T_optical_tagcv(rvec, tvec):
    """(rvec, tvec) from detect_tag -> 4x4 tag pose in the optical frame."""
    R, _ = cv2.Rodrigues(np.asarray(rvec, dtype=float))
    return make_T(R, np.asarray(tvec, dtype=float))


def _T_base_model_from(T_base_optical, rvec, tvec):
    """One detection solution -> T_base_model. tagcv (OpenCV marker) -> tagsdf (SDF
    plate) is the fixed R_CV_TO_SDF rotation; the tag's pose in the model is
    T_MODEL_TAG (includes TAG0_XYZ), so the model in base is
    T_base_tagsdf @ inv(T_MODEL_TAG)."""
    T_base_tagcv = np.asarray(T_base_optical, dtype=float) @ T_optical_tagcv(rvec, tvec)
    T_base_tagsdf = T_base_tagcv @ make_T(R_CV_TO_SDF, np.zeros(3))
    return T_base_tagsdf @ inv_T(T_MODEL_TAG)


def device_pose_in_base(T_base_optical, solutions):
    """Compose the detection(s) with the base<-optical TF and the known tag-in-model
    pose to get T_base_model (the device model frame expressed in base_link).

    `solutions` is the (rvec, tvec) list from detect_tag. The device is known
    vertical, so of the (up to 2) ambiguous IPPE solutions we keep the one whose
    model z-axis is most aligned with base z (T[2,2] closest to +1) -- this rejects
    the near-fronto-parallel flip that a single solution would silently pick.
    ponytail: assumes base_link z ~ world up (AGV on a flat floor)."""
    if not isinstance(solutions, list):        # tolerate a single (rvec, tvec)
        solutions = [solutions]
    cands = [_T_base_model_from(T_base_optical, r, t) for r, t in solutions]
    return max(cands, key=lambda T: T[2, 2])


# --- slot resolution -----------------------------------------------------------
def slots_in_base(T_base_model, letters=('A', 'B', 'C', 'D')):
    """Given the device model frame in base_link, return each slot's magazine-centre
    pose: {letter: (xyz_np, quat_xyzw)}. Slots share the model orientation."""
    R = T_base_model[:3, :3]
    quat = R_to_quat(R)
    out = {}
    for L in letters:
        p_model = np.array(SLOT_OFFSET[L], dtype=float)
        p_base = (T_base_model @ np.append(p_model, 1.0))[:3]
        out[L] = (p_base, quat)
    return out


# --- two-view triangulation (motion stereo) -----------------------------------
# A single small tag can't fix range (monocular scale is the weak axis). Two views
# from DIFFERENT camera POSITIONS (not just a rotation) triangulate the tag corners
# with a known baseline (the arm's motion, from FK/TF), which recovers depth. A
# view = (T_odom_optical 4x4, corners 4x2 px, K 3x3).
def _corner_rays_odom(view):
    """View -> (camera centre in odom, 4x3 unit ray dirs in odom) for the corners."""
    T, corners, K = view
    Kinv = np.linalg.inv(np.asarray(K, dtype=float))
    R, C = np.asarray(T)[:3, :3], np.asarray(T)[:3, 3]
    dirs = []
    for u, v in np.asarray(corners, dtype=float).reshape(4, 2):
        d = R @ (Kinv @ np.array([u, v, 1.0]))
        dirs.append(d / np.linalg.norm(d))
    return C, np.array(dirs)


def triangulate_rays(C1, d1, C2, d2):
    """Least-squares closest point of two rays (origin Ci, unit dir di)."""
    b = C2 - C1
    d = d1 @ d2
    denom = 1.0 - d * d
    if abs(denom) < 1e-9:                       # near-parallel: no parallax
        return 0.5 * (C1 + C2)
    t1 = (b @ d1 - (b @ d2) * d) / denom
    t2 = ((b @ d1) * d - b @ d2) / denom
    return 0.5 * ((C1 + t1 * d1) + (C2 + t2 * d2))


def _kabsch(src, dst):
    """Rigid transform (R, t) with dst ~= R @ src + t (no scaling)."""
    cs, cd = src.mean(0), dst.mean(0)
    H = (src - cs).T @ (dst - cd)
    U, _, Vt = np.linalg.svd(H)
    D = np.diag([1.0, 1.0, np.sign(np.linalg.det(Vt.T @ U.T))])
    R = Vt.T @ D @ U.T
    return R, cd - R @ cs


def device_pose_from_two_views(view_a, view_b, size=TAG_SIZE_M):
    """Triangulate the 4 tag corners across the two views, rigid-fit the tag frame,
    and compose to T_odom_model (device model frame in odom). Both views must be in
    the SAME (odom) frame and taken from different camera POSITIONS."""
    Ca, da = _corner_rays_odom(view_a)
    Cb, db = _corner_rays_odom(view_b)
    P = np.array([triangulate_rays(Ca, da[k], Cb, db[k]) for k in range(4)])
    R, t = _kabsch(_marker_object_points(size), P)   # odom <- tag_cv
    T_odom_tagcv = make_T(R, t)
    T_odom_tagsdf = T_odom_tagcv @ make_T(R_CV_TO_SDF, np.zeros(3))
    return T_odom_tagsdf @ inv_T(T_MODEL_TAG)


# --- self-check ----------------------------------------------------------------
def _look_at_optical(cam_pos, target):
    """T_odom_optical for a pinhole at cam_pos looking at target (z forward)."""
    z = target - cam_pos
    z = z / np.linalg.norm(z)
    up = np.array([0.0, 0.0, -1.0])
    x = np.cross(up, z)
    if np.linalg.norm(x) < 1e-6:
        x = np.cross(np.array([0.0, 1.0, 0.0]), z)
    x = x / np.linalg.norm(x)
    y = np.cross(z, x)
    return make_T(np.column_stack([x, y, z]), cam_pos)


def _project(T_odom_optical, X, K):
    """odom 3D point -> pixel in the given optical view."""
    Ti = inv_T(T_odom_optical)
    Xo = Ti[:3, :3] @ X + Ti[:3, 3]
    p = np.asarray(K) @ (Xo / Xo[2])
    return p[:2]


def _demo():
    # 1) SE3 algebra round-trip: pick a known device pose, derive the tag pose it
    #    implies, recover the device pose, and check slots match the direct compose.
    R_truth = rpy_to_R(0.0, 0.0, 0.7)            # device yawed 0.7 rad in base
    T_base_model_truth = make_T(R_truth, [1.5, 0.3, 0.1])
    T_base_tag = T_base_model_truth @ T_MODEL_TAG
    T_base_model_rec = T_base_tag @ inv_T(T_MODEL_TAG)
    assert np.allclose(T_base_model_rec, T_base_model_truth, atol=1e-9), "SE3 round-trip"

    got = slots_in_base(T_base_model_rec)
    for L in 'ABCD':
        want = (T_base_model_truth @ np.append(SLOT_OFFSET[L], 1.0))[:3]
        assert np.allclose(got[L][0], want, atol=1e-9), f"slot {L} compose"

    # 2) Ground-truth match vs the existing wirebonder_pick_place slot_world():
    #    with T_base_model = the device WORLD pose, slot centres must equal the
    #    DEVICES x SLOT_OFFSET world coordinates the pick/place script computes.
    dx, dy, dz, dyaw = (2.35, 0.5, 0.0, 0.0)     # DEVICES['wb1']
    T_world_model = make_T(rpy_to_R(0, 0, dyaw), [dx, dy, dz])
    got_w = slots_in_base(T_world_model)
    for L in 'ABCD':
        ox, oy, oz = SLOT_OFFSET[L]
        c, s = math.cos(dyaw), math.sin(dyaw)
        want_w = np.array([dx + c * ox - s * oy, dy + s * ox + c * oy, dz + oz])
        assert np.allclose(got_w[L][0], want_w, atol=1e-9), f"slot {L} world gt"

    # 3) Detection smoke: render tag 0, detect it, confirm id + a plausible range.
    side = 400
    img = np.full((side, side), 255, np.uint8)
    marker = cv2.aruco.drawMarker(_aruco_dict(), TAG_ID, 300)
    off = (side - 300) // 2
    img[off:off + 300, off:off + 300] = marker
    img = cv2.cvtColor(img, cv2.COLOR_GRAY2BGR)
    K = np.array([[500., 0., side / 2], [0., 500., side / 2], [0., 0., 1.]])
    res = detect_tag(img, K)
    assert res, "tag 0 not detected"
    _, tvec = res[0]
    assert tvec[2] > 0, f"tag should be in front of the camera, got z={tvec[2]:.3f}"

    # 4) Disambiguation: given an UPRIGHT device pose and its flipped twin (rolled),
    #    device_pose_in_base must return the upright one. Synthesize both detections
    #    by inverting the pipeline with T_base_optical = I.
    inv_cv = inv_T(make_T(R_CV_TO_SDF, np.zeros(3)))
    sols = []
    for roll in (0.0, 0.6):                       # upright, then a 0.6 rad flip
        T_model = make_T(rpy_to_R(roll, 0.0, 0.2), [1.4, 0.2, 0.0])
        T_tagcv = T_model @ T_MODEL_TAG @ inv_cv
        rvec, _ = cv2.Rodrigues(T_tagcv[:3, :3])
        sols.append((rvec.ravel(), T_tagcv[:3, 3]))
    picked = device_pose_in_base(np.eye(4), sols)
    assert picked[2, 2] > 0.99, f"disambiguation kept the flip (z-axis {picked[2,2]:.3f})"

    # 5) Two-view triangulation round-trip: a known device pose, project its tag
    #    corners into two cameras offset in POSITION (baseline in x), recover the
    #    device pose from the pixels alone. This is the range-fixing path.
    T_model_truth = make_T(rpy_to_R(0.0, 0.0, 0.3), [2.0, 0.4, 0.05])
    T_tagcv = T_model_truth @ T_MODEL_TAG @ inv_T(make_T(R_CV_TO_SDF, np.zeros(3)))
    objp = _marker_object_points(TAG_SIZE_M)
    corners_odom = np.array([(T_tagcv @ np.append(p, 1.0))[:3] for p in objp])
    center = corners_odom.mean(0)
    K2 = np.array([[600., 0., 320.], [0., 600., 240.], [0., 0., 1.]])
    base = center + np.array([0.0, -0.4, 0.0])          # ~0.4 m in front of the tag
    camA = _look_at_optical(base + np.array([-0.06, 0, 0]), center)
    camB = _look_at_optical(base + np.array([0.10, 0, 0]), center)   # baseline in x
    cA = np.array([_project(camA, X, K2) for X in corners_odom])
    cB = np.array([_project(camB, X, K2) for X in corners_odom])
    rec = device_pose_from_two_views((camA, cA, K2), (camB, cB, K2))
    assert np.allclose(rec, T_model_truth, atol=1e-6), "two-view pose round-trip"

    print("wirebonder_vision self-check: OK")


if __name__ == '__main__':
    _demo()
