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


def device_pose_in_base(T_base_optical, solutions, depth=None, K=None, corners=None):
    """Compose the detection(s) with the base<-optical TF and the known tag-in-model
    pose to get T_base_model (the device model frame expressed in base_link).

    `solutions` is the (rvec, tvec) list from detect_tag. With full depth
    (`depth`+`K`+`corners` given and trustworthy) the pose is built from depth ALONE
    (upright construction, see below) and PnP contributes nothing but the detection.
    With partial/holed depth each depth measurement that IS available still corrects
    the PnP solution per-axis:
      - orientation: fit the tag plane normal from depth (unambiguous 3D), use it to
        pick the right IPPE solution AND snap that solution's normal to it -- so the
        out-of-plane orientation comes from depth (stable), the in-plane from PnP.
      - range: replace tvec with the depth-deprojected tag centre.
    Depth that is holed / out of range is ignored per-axis and PnP stands (the
    fallback), so bad depth never makes it worse. Without depth: the device is known
    vertical, so keep the IPPE solution whose model z-axis is most aligned with base z
    (T[2,2] closest to +1). ponytail: assumes base_link z ~ world up (flat floor)."""
    if not isinstance(solutions, list):        # tolerate a single (rvec, tvec)
        solutions = [solutions]

    have_depth = depth is not None and K is not None and corners is not None
    n_depth = _tag_plane_normal(depth, corners, K) if have_depth else None
    tvec_d = _depth_corrected_tvec(depth, corners, K) if have_depth else None

    # FULL-DEPTH UPRIGHT pose: with both the tag centre and the plane normal
    # measured from depth, and the device known upright (base z ~ world up --
    # the same assumption the no-depth fallback already makes), the pose has NO
    # PnP degree of freedom left: yaw = the normal's azimuth, position = the
    # centre. This kills the dominant error of the PnP hybrid: PnP's IN-PLANE
    # rotation (about the tag normal) is viewpoint-dependent by ~0.1-0.4 deg,
    # and the ~1.25 m tag->device-origin lever arm turns that into a +1..+7 mm
    # device-x swing across capture configs (measured; see
    # tools/diag_camera_geometry.py). Guard: the measured normal must be
    # near-horizontal in base -- if not (tilted device? bad plane fit), fall
    # through to the PnP hybrid below.
    if n_depth is not None and tvec_d is not None:
        T_bo = np.asarray(T_base_optical, dtype=float)
        n_base = T_bo[:3, :3] @ n_depth
        if abs(n_base[2]) < 0.2:
            c_base = (T_bo @ np.append(tvec_d, 1.0))[:3]
            n_model = T_MODEL_TAG[:3, 2]          # tag normal in the model frame
            yaw = (math.atan2(n_base[1], n_base[0])
                   - math.atan2(n_model[1], n_model[0]))
            c, s = math.cos(yaw), math.sin(yaw)
            R = np.array([[c, -s, 0.], [s, c, 0.], [0., 0., 1.]])
            return make_T(R, c_base - R @ T_MODEL_TAG[:3, 3])

    # Disambiguate the (up to 2) IPPE solutions. With a depth normal: pick the one whose
    # tag normal matches it (unambiguous). Else: the base z-up heuristic.
    if n_depth is not None:
        rvec, tvec = max(solutions,
                         key=lambda rt: float(T_optical_tagcv(rt[0], rt[1])[:3, 2] @ n_depth))
    else:
        rvec, tvec = max(solutions,
                         key=lambda rt: _T_base_model_from(T_base_optical, rt[0], rt[1])[2, 2])

    R = T_optical_tagcv(rvec, tvec)[:3, :3]
    if n_depth is not None:                     # snap out-of-plane orientation to depth
        rvec = cv2.Rodrigues(_align_normal(R, n_depth))[0].ravel()
    if tvec_d is not None:                       # replace the weak range axis with depth
        tvec = tvec_d
    return _T_base_model_from(T_base_optical, rvec, tvec)


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


# --- depth (RGBD range) --------------------------------------------------------
def read_depth(data, height, width, encoding):
    """Depth image bytes -> HxW float32 in METERS, invalid pixels as NaN.

    The two encodings this project sees: sim Gazebo publishes 32FC1 (already
    metres); a real D405 via realsense2_camera publishes 16UC1 in millimetres.
    This one function absorbs that difference -- it is the whole depth half of the
    sim<->real seam (the other half is the topic remap). 0 / non-finite pixels
    (no stereo return) become NaN so the ROI median and _depth_valid skip them."""
    if encoding == '32FC1':
        d = np.frombuffer(data, np.float32).reshape(height, width).astype(np.float32)
    elif encoding == '16UC1':
        d = np.frombuffer(data, np.uint16).reshape(height, width).astype(np.float32) / 1000.0
    else:
        raise ValueError(f"unsupported depth encoding: {encoding!r}")
    d[~np.isfinite(d)] = np.nan
    d[d <= 0.0] = np.nan
    return d


# Depth-validity tuning knobs (REAL-ROBOT knobs; sim depth is hole-free so these
# never reject in sim -- see docs/real_robot_transition.md). ROI shrink dodges the
# flickery tag border; min_frac guards against a holed tag; the z window is the
# D405 usable range for this task.
DEPTH_ROI_SHRINK = 0.5        # scale corners toward the centre before sampling
DEPTH_MIN_FRAC = 0.5          # need this fraction of finite pixels in the ROI
DEPTH_MIN_Z, DEPTH_MAX_Z = 0.07, 1.0   # metres


def _tag_center_px(corners):
    """Mean of the 4 detected corners = tag-centre pixel (= where PnP's tvec, the
    tag origin, projects, since the marker object points are centred)."""
    return np.asarray(corners, dtype=float).reshape(4, 2).mean(0)


def _tag_roi(depth, corners, shrink=DEPTH_ROI_SHRINK):
    """(sub-array, x0, y0): depth over the tag, eroded toward the centre by `shrink`
    so border/occlusion holes don't pollute it; x0,y0 map the sub-array back to full
    image pixels (the plane fit needs that). Bbox of the shrunk corners (a rectangle
    inside the tag), not the exact quad -- close enough."""
    c = np.asarray(corners, dtype=float).reshape(4, 2)
    inner = c.mean(0) + (c - c.mean(0)) * shrink
    H, W = depth.shape
    x0, x1 = int(max(0, np.floor(inner[:, 0].min()))), int(min(W, np.ceil(inner[:, 0].max())))
    y0, y1 = int(max(0, np.floor(inner[:, 1].min()))), int(min(H, np.ceil(inner[:, 1].max())))
    return depth[y0:y1, x0:x1], x0, y0


def _deproject(px, z, K):
    """Pixel (u,v) + depth Z (metres, the optical-axis distance a depth image
    stores) -> 3D point in the optical frame via the pinhole model."""
    u, v = px
    K = np.asarray(K, dtype=float)
    return np.array([(u - K[0, 2]) / K[0, 0] * z, (v - K[1, 2]) / K[1, 1] * z, z])


def _depth_corrected_tvec(depth, corners, K):
    """Return the tag-centre position in the optical frame from depth, or None if
    the depth over the tag is not trustworthy (too many holes / out of range).
    Correcting along the trusted PnP ray fixes the whole translation, not just Z."""
    roi, _, _ = _tag_roi(depth, corners)
    finite = roi[np.isfinite(roi)]
    if roi.size == 0 or finite.size < DEPTH_MIN_FRAC * roi.size:
        return None
    z = float(np.median(finite))
    if not (DEPTH_MIN_Z <= z <= DEPTH_MAX_Z):
        return None
    return _deproject(_tag_center_px(corners), z, K)


def _tag_plane_normal(depth, corners, K):
    """Fit a plane to the tag's depth points; return its unit normal in the OPTICAL
    frame, oriented toward the camera (optical -z side), or None if too few finite
    points. This is the tag's facing direction measured DIRECTLY in 3D -- monocular
    PnP has a two-fold planar flip that swings the device yaw ~20 deg between nearly
    equal viewpoints; the depth normal has no such ambiguity, so it pins that axis."""
    roi, x0, y0 = _tag_roi(depth, corners)
    ys, xs = np.where(np.isfinite(roi))
    if xs.size < 8:                              # too few points for a stable plane
        return None
    z = roi[ys, xs].astype(float)
    K = np.asarray(K, dtype=float)
    X = (xs + x0 - K[0, 2]) / K[0, 0] * z
    Y = (ys + y0 - K[1, 2]) / K[1, 1] * z
    P = np.stack([X, Y, z], axis=1)              # tag-surface points in optical frame
    _, _, Vt = np.linalg.svd(P - P.mean(0), full_matrices=False)
    n = Vt[2]                                    # least-variance direction = normal
    if n[2] > 0:                                 # orient toward the camera (optical -z)
        n = -n
    return n / np.linalg.norm(n)


def _align_normal(R, n_target):
    """Rotate R (left-multiply) so its tag normal R[:,2] points along n_target, keeping
    the in-plane axes -- transfers depth's stable normal into the PnP rotation. Minimal
    (Rodrigues) rotation between the two normals; identity when already aligned."""
    a = R[:, 2] / np.linalg.norm(R[:, 2])
    b = np.asarray(n_target, dtype=float) / np.linalg.norm(n_target)
    v = np.cross(a, b)
    s = np.linalg.norm(v)
    c = float(a @ b)
    if s < 1e-9:                                 # already aligned (disambiguation ensures c>0)
        return R
    vx = np.array([[0, -v[2], v[1]], [v[2], 0, -v[0]], [-v[1], v[0], 0]])
    R_align = np.eye(3) + vx + vx @ vx * ((1.0 - c) / (s * s))
    return R_align @ R


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
    dx, dy, dz, dyaw = (2.35, 0.5, 0.0, 0.0)     # 
    
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

    # 6) Depth decode: 32FC1 (sim, metres) and 16UC1 (real D405, mm) must both
    #    yield the same metres, and 0 / non-finite must become NaN.
    m = np.array([[0.30, 0.0], [np.inf, 0.45]], np.float32)     # 0 and inf = no return
    d32 = read_depth(m.tobytes(), 2, 2, '32FC1')
    mm = np.array([[300, 0], [65535, 450]], np.uint16)          # mm; 0 = no return
    d16 = read_depth(mm.tobytes(), 2, 2, '16UC1')
    assert abs(d32[0, 0] - 0.30) < 1e-6 and abs(d16[0, 0] - 0.30) < 1e-6, "depth metres"
    assert np.isnan(d32[0, 1]) and np.isnan(d16[0, 1]), "zero -> NaN"
    assert np.isnan(d32[1, 0]), "inf -> NaN"
    assert abs(d16[1, 1] - 0.45) < 1e-6, "16UC1 mm->m"

    # 7) Depth range correction: a PnP solution with the WRONG range (same viewing
    #    ray, z too far) + a depth map that reads the true range must recover the
    #    true-range pose; an all-NaN (holed) depth must fall back to PnP unchanged.
    K7 = np.array([[600., 0., 424.], [0., 600., 240.], [0., 0., 1.]])
    p_true = np.array([0.05, -0.02, 0.35])                    # tag centre in optical
    u0 = 600 * p_true[0] / p_true[2] + 424
    v0 = 600 * p_true[1] / p_true[2] + 240
    corners7 = np.array([[u0 - 20, v0 - 20], [u0 + 20, v0 - 20],
                         [u0 + 20, v0 + 20], [u0 - 20, v0 + 20]])
    depth7 = np.full((480, 848), np.nan, np.float32)
    depth7[int(v0) - 15:int(v0) + 15, int(u0) - 15:int(u0) + 15] = p_true[2]
    rvec7, tvec_wrong = np.zeros(3), p_true * (0.50 / 0.35)   # same ray, wrong range
    T_pnp = device_pose_in_base(np.eye(4), [(rvec7, tvec_wrong)])
    T_truth = device_pose_in_base(np.eye(4), [(rvec7, p_true)])
    T_dep = device_pose_in_base(np.eye(4), [(rvec7, tvec_wrong)],
                                depth=depth7, K=K7, corners=corners7)
    assert np.allclose(T_dep, T_truth, atol=1e-4), "depth did not fix the range"
    assert not np.allclose(T_pnp, T_truth, atol=1e-2), "no-depth should keep the wrong range"
    T_holed = device_pose_in_base(np.eye(4), [(rvec7, tvec_wrong)],
                                  depth=np.full((480, 848), np.nan, np.float32),
                                  K=K7, corners=corners7)
    assert np.allclose(T_holed, T_pnp), "holed depth must fall back to PnP"

    # 8) Depth plane-normal orientation: fit the tag plane from a synthetic depth image
    #    and confirm (a) the recovered normal matches the true tag normal, (b) _align_normal
    #    snaps to it, and (c) it pulls a PnP solution with a PERTURBED (tilted) normal back
    #    toward truth -- the flip/yaw fix depth gives that a single view PnP cannot.
    K8 = np.array([[600., 0., 424.], [0., 600., 240.], [0., 0., 1.]])
    # Tag built directly in the optical frame: centred 0.4 m ahead, facing the camera
    # (R_fronto), tilted 0.3 rad so its normal is non-trivial. Device pose derived back.
    R_fronto = np.array([[1., 0., 0.], [0., -1., 0.], [0., 0., -1.]])
    R8 = rpy_to_R(0.0, 0.3, 0.0) @ R_fronto
    t8 = np.array([0.0, 0.0, 0.4])
    T_bm8 = make_T(R8, t8) @ make_T(R_CV_TO_SDF, np.zeros(3)) @ inv_T(T_MODEL_TAG)
    P8 = np.array([R8 @ p + t8 for p in _marker_object_points(TAG_SIZE_M)])
    corners8 = np.array([(K8 @ (P / P[2]))[:2] for P in P8])          # tag corners -> pixels
    n_true = R8[:, 2] if R8[2, 2] < 0 else -R8[:, 2]                  # toward the camera
    depth8 = np.full((480, 848), np.nan, np.float32)                 # synth tag-plane depth
    umin, umax = int(np.floor(corners8[:, 0].min())), int(np.ceil(corners8[:, 0].max()))
    vmin, vmax = int(np.floor(corners8[:, 1].min())), int(np.ceil(corners8[:, 1].max()))
    for v in range(max(0, vmin), min(480, vmax + 1)):
        for u in range(max(0, umin), min(848, umax + 1)):
            ray = np.array([(u - 424) / 600., (v - 240) / 600., 1.0])
            denom = n_true @ ray
            if abs(denom) > 1e-9:
                Z = (n_true @ t8) / denom
                if Z > 0:
                    depth8[v, u] = Z
    n_fit = _tag_plane_normal(depth8, corners8, K8)
    assert n_fit is not None and n_fit @ n_true > 0.999, "plane normal off"
    assert _align_normal(rpy_to_R(0.0, 0.21, 0.0) @ R8, n_true)[:, 2] @ n_true > 0.9999, "snap"

    def _rot_err(A, B):
        return math.acos(max(-1.0, min(1.0, (np.trace(A.T @ B) - 1) / 2)))
    rvec_p = cv2.Rodrigues(rpy_to_R(0.0, 0.21, 0.0) @ R8)[0].ravel()  # ~12 deg normal tilt
    T_no = device_pose_in_base(np.eye(4), [(rvec_p, t8)])
    T_dp = device_pose_in_base(np.eye(4), [(rvec_p, t8)], depth=depth8, K=K8, corners=corners8)
    assert _rot_err(T_dp[:3, :3], T_bm8[:3, :3]) < _rot_err(T_no[:3, :3], T_bm8[:3, :3]), \
        "depth normal did not improve orientation"
    assert _rot_err(T_dp[:3, :3], T_bm8[:3, :3]) < math.radians(3), "depth orientation not tight"

    # 9) Full-depth UPRIGHT construction: an upright device (horizontal tag normal
    #    in base), synthetic plane depth + corners, and a GARBAGE PnP solution ->
    #    the recovered pose must match truth anyway (position from the depth centre,
    #    yaw from the depth normal; no PnP DOF). This is the primary sim/real path.
    K9 = np.array([[600., 0., 424.], [0., 600., 240.], [0., 0., 1.]])
    T_truth9 = make_T(rpy_to_R(0.0, 0.0, 0.25), [1.8, 0.3, 0.0])
    c9 = (T_truth9 @ np.append(TAG0_XYZ, 1.0))[:3]              # tag centre, base
    n9 = T_truth9[:3, :3] @ T_MODEL_TAG[:3, 2]                  # tag normal, base
    cam9 = _look_at_optical(c9 + 0.35 * n9, c9)                 # straight-on view
    T_tagcv9 = T_truth9 @ T_MODEL_TAG @ inv_T(make_T(R_CV_TO_SDF, np.zeros(3)))
    P9 = np.array([(T_tagcv9 @ np.append(p, 1.0))[:3] for p in
                   _marker_object_points(TAG_SIZE_M)])
    corners9 = np.array([_project(cam9, X, K9) for X in P9])
    Ti9 = inv_T(cam9)
    p0_opt = Ti9[:3, :3] @ c9 + Ti9[:3, 3]                      # plane point, optical
    n_opt9 = Ti9[:3, :3] @ n9
    depth9 = np.full((480, 848), np.nan, np.float32)
    umin, umax = int(corners9[:, 0].min()) - 2, int(corners9[:, 0].max()) + 2
    vmin, vmax = int(corners9[:, 1].min()) - 2, int(corners9[:, 1].max()) + 2
    for v in range(max(0, vmin), min(480, vmax + 1)):
        for u in range(max(0, umin), min(848, umax + 1)):
            ray = np.array([(u - 424) / 600., (v - 240) / 600., 1.0])
            denom = n_opt9 @ ray
            if abs(denom) > 1e-9:
                Z = (n_opt9 @ p0_opt) / denom
                if Z > 0:
                    depth9[v, u] = Z
    garbage = (np.array([0.5, -0.3, 1.1]), np.array([9.9, 9.9, 9.9]))
    T_up = device_pose_in_base(cam9, [garbage], depth=depth9, K=K9, corners=corners9)
    assert np.allclose(T_up, T_truth9, atol=2e-3), \
        f"upright construction off: {np.abs(T_up - T_truth9).max():.4f}"
    assert abs(T_up[2, 2] - 1.0) < 1e-9, "upright pose not exactly upright"

    print("wirebonder_vision self-check: OK")


if __name__ == '__main__':
    _demo()
