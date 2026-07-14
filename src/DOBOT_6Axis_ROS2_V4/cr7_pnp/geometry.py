"""Pure helpers and workspace/gripper constants shared by every sequence.

No ROS node, no planner state -- just math helpers, the trigger primitive, and
the tuned geometry constants. Copied out of cbirrt_pick_place.py so sequences
(shelf, device A/B/C/D, ...) import the same numbers from one place.
"""

import os
import math

import numpy as np
from geometry_msgs.msg import PoseStamped


# --- model paths --------------------------------------------------------------

# Arm selected by ARM_TYPE (default cr7 -> original CR7 paths, 100% unchanged).
# Set ARM_TYPE=cr10 (etc.) to jog a swapped arm; see run_mpo700_cr10.sh.
_ARM = os.getenv('ARM_TYPE', 'cr7')

if _ARM == 'cr7':
    # Arm-only model for the orientation constraint / Cartesian servo (FK + Jacobian).
    XACRO_PATH = os.path.expanduser(
        '~/dobot_ws/install/cra_description/share/cra_description/urdf/cr7_robot.xacro')
    # Combined robot (arm + cube platform + MPO-700 AGV + gripper) for collision
    # checking: catches the arm hitting the cube/AGV, not just itself. Source the ROS
    # workspace before running so xacro can resolve the package includes.
    COMBINED_XACRO = os.path.expanduser(
        '~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/cra_description/urdf/cr7_on_mpo700.urdf.xacro')
else:
    # Swapped arm: <arm>_arm.xacro is the stripped arm-only chain (base_link+Link1..6).
    XACRO_PATH = os.path.expanduser(
        f'~/dobot_ws/install/cra_description/share/cra_description/urdf/{_ARM}_arm.xacro')
    # ponytail: the combined collision model uses arm_on_mpo700's default arm_type
    # (cr10). IK/FK above is always arm-correct via <arm>_arm.xacro; for cr12/16/20
    # the collision-combined stays cr10 unless arm_on_mpo700's default is updated.
    COMBINED_XACRO = os.path.expanduser(
        '~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/cra_description/urdf/arm_on_mpo700.urdf.xacro')


# --- shelf collision model ----------------------------------------------------

# The Gazebo 'shelf' world model is not part of the robot URDF, so its horizontal
# boards are added as thin boxes to the planning collision model and placed at
# the AGV's current pose via TF each cycle. Boards (not a solid block) so the GAPS
# between tiers stay open for the straight insert servo to reach the box.
#
# SHELF_WORLD_POSE is the cr.world spawn pose -- the --no-vision fallback and the
# plausibility anchor for the ArUco read. At runtime the LIVE shelf pose comes
# from /vision/shelf_pose (per-tier AprilTags baked into shelf/model.sdf) and
# everything below composes with it, so "shelf moved" is no longer a code edit.
SHELF_WORLD_POSE = (0.8, 0.5, 0.0)      # shelf model origin in world (x, y, yaw)
# Board tops (world z). Shelf raised +0.32 in cr.world (pose z=0.32) so tier-1
# top is 0.90->1.22 (= robot base 1.0 m + 0.22, the June pick-proven relative
# height -- +0.28 left box d marginal: all-branch singular ~20 mm short once
# the base ratcheted a few mm) and tier-2 1.40->1.72; the mesh
# still bakes 0.40/0.90/1.40/1.95, so the pose offset is what lifts it. Keep
# these in sync with the cr.world shelf pose. (June note: base heights 0.90/1.40
# were pick-proven; 0.685 jammed the upper arm on the next board, measured.)
SHELF_BOARD_TOPS = (0.72, 1.22, 1.72, 2.27)   # board top heights (world z), 4-tier
SHELF_FOOTPRINT = (2.0, 0.30)           # board size (x, y) in metres
SHELF_BOARD_THICK = 0.018               # board thickness (z), real shelf board

# Pickable tiers: tier number -> board top height (subset of SHELF_BOARD_TOPS).
SHELF_TIER_TOPS = {1: 1.22, 2: 1.72}

# Box centres in the SHELF MODEL frame: x offsets in PICK ORDER (a,b = inner
# pair, c,d = outer flank, one box-pitch 0.181 out), y centred on the board.
# Mirrors the box_t<tier><a-d> spawns in cr.world -- keep both in sync.
SHELF_BOX_XS = (-0.0905, +0.0905, -0.2715, +0.2715)
SHELF_BOX_Y = 0.0


# Per-tier ArUco placard placement in the SHELF MODEL frame (x, y at the board
# front edge left of the box row; centre rises half a plate above the board top).
# MIRRORS shelf/model.sdf (aruco_tier1/2) and vision/shelf_vision.py -- keep the
# three in sync. The sequence uses this only to AIM the capture pose; the pose
# solve itself lives in the vision node.
SHELF_TAG_XY = (-0.45, -0.1505)
SHELF_TAG_RISE = 0.0375 / 2.0


def shelf_tag_world(tier, shelf_pose=SHELF_WORLD_POSE):
    """Expected world (odom) centre of the `tier` tag placard for a shelf at
    shelf_pose (x, y, yaw). Used to aim the camera before the capture read."""
    x, y, yaw = shelf_pose
    tx, ty = SHELF_TAG_XY
    c, s = math.cos(yaw), math.sin(yaw)
    return (x + c * tx - s * ty, y + s * tx + c * ty,
            SHELF_TIER_TOPS[tier] + SHELF_TAG_RISE)


def shelf_box_center(tier, i, shelf_pose=SHELF_WORLD_POSE):
    """World (odom) centre of shelf box `i` (pick order) on `tier`, composing the
    model-frame layout with a live shelf pose (x, y, yaw). THIS is the seam the
    AI magazine detector replaces later: today the centre is layout-derived from
    the ArUco shelf pose; the detector will return a measured centre instead."""
    x, y, yaw = shelf_pose
    bx, by = SHELF_BOX_XS[i], SHELF_BOX_Y
    c, s = math.cos(yaw), math.sin(yaw)
    from .gripper_params import BOX_SIZE as _BS
    return (x + c * bx - s * by, y + s * bx + c * by,
            SHELF_TIER_TOPS[tier] + _BS[2] / 2.0)


# --- pure helpers -------------------------------------------------------------

def quat_to_R(x, y, z, w):
    """Unit quaternion (xyzw) -> 3x3 rotation matrix."""
    n = math.sqrt(x * x + y * y + z * z + w * w) or 1.0
    x, y, z, w = x / n, y / n, z / n, w / n
    return np.array([
        [1 - 2 * (y * y + z * z), 2 * (x * y - z * w), 2 * (x * z + y * w)],
        [2 * (x * y + z * w), 1 - 2 * (x * x + z * z), 2 * (y * z - x * w)],
        [2 * (x * z - y * w), 2 * (y * z + x * w), 1 - 2 * (x * x + y * y)],
    ])


def quat_about_z(angle):
    """Quaternion (xyzw) for a rotation `angle` (rad) about the z-axis."""
    return (0.0, 0.0, math.sin(angle / 2.0), math.cos(angle / 2.0))


def quat_mul(q1, q2):
    """Hamilton product q1 (x) q2, both (xyzw); applies q1 after q2."""
    x1, y1, z1, w1 = q1
    x2, y2, z2, w2 = q2
    return (
        w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2,
        w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2,
        w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2,
        w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2,
    )


def pose_at(xyz, quat):
    """PoseStamped in base_link at position xyz with orientation quat (xyzw)."""
    p = PoseStamped()
    p.header.frame_id = "base_link"
    p.pose.position.x, p.pose.position.y, p.pose.position.z = (
        float(xyz[0]), float(xyz[1]), float(xyz[2]))
    (p.pose.orientation.x, p.pose.orientation.y,
     p.pose.orientation.z, p.pose.orientation.w) = (float(v) for v in quat)
    return p


def wait_for_spacebar():
    """Block until the user presses SPACE (-> 'go') or q/Esc/Ctrl-C (-> 'quit').

    Reads one keypress in raw terminal mode. Isolated here so it can later be
    swapped for a ROS topic/service trigger (the AMR/MCS will signal over
    TCP/IP/MQTT) without touching the pick-and-place logic."""
    import sys
    import termios
    import tty
    fd = sys.stdin.fileno()
    old = termios.tcgetattr(fd)
    try:
        tty.setraw(fd)
        while True:
            ch = sys.stdin.read(1)
            if ch == ' ':
                return 'go'
            if ch in ('q', '\x1b', '\x03'):   # q, Esc, Ctrl-C
                return 'quit'
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, old)


# ----------------------------------------------------------------------------
# Workspace constants for the shelf-to-base sequence. base_link metres unless
# noted. Many heights/clearances are first guesses to be tuned in the simulator.
# ----------------------------------------------------------------------------

# Gripper jaw geometry + box + TCP/grasp offsets now live in gripper_params.py
# (single source of truth, ROS-free, shared with reachability_map.py). Imported
# here so the existing sequence-script imports (via cr7_pnp/__init__) keep working.
from .gripper_params import (  # noqa: F401
    JAW_FIXED_PAD_X, JAW_MOVING_PAD_X0, JAW_GAP_AT_ZERO, PAD_BOTTOM_BELOW_FLANGE,
    BOX_SHORT, FIXED_PAD_CLEARANCE, BOX_SIZE, BOX_IN_LINK6_XYZ,
    TCP_OFFSET_M, GRASP_TCP_ABOVE, INSERT_TCP_ABOVE, GRASP_CENTER_OFFSET_M,
    GRASP_LATERAL_M, FINGER_OPEN_M)

# == FINGER_OPEN_M: the collision model freezes the finger at this opening, so
# commanding wider than this would put the real jaw outside the planned model.
GRIPPER_OPEN = [FINGER_OPEN_M]  # gap 111 mm; after jaw-align the moving pad still
                             # clears the box face by ~19 mm on the descend, and
                             # the shorter close sweep hits the box at ~12 mm/s
                             # instead of ~32 mm/s (0.07), which the contact
                             # solver tolerates much better
# NEGATIVE = leave CLEARANCE, do not touch the box. The box is held by the
# ATTACHLINK fixed joint, NOT by pad pressure, so the pads need not contact it.
# At gap == box width the position-controlled finger keeps driving into the
# (shelf-supported / heavy) box -> the reaction feeds back through the arm and it
# THRASHES ("요동"). A ~2 mm gap (1 mm/side) removes the pad-box contact fight.
# Set POSITIVE only if you must fall back on friction when the attacher misses
# (then expect the contact jitter back). TUNE IN SIM.
CLOSE_SQUEEZE = -0.002
GRIPPER_CLOSE = [BOX_SHORT - JAW_GAP_AT_ZERO - CLOSE_SQUEEZE]   # gap = box + 2 mm (clearance)

# Shelf box Gazebo naming: model 'box_t<tier><a-d>' (a-d in PICK ORDER, matching
# SHELF_BOX_XS), link 'box_link'. Used by the link attacher.
SHELF_BOX_LINK = 'box_link'


def shelf_box_model(tier, i):
    """Gazebo model name of shelf box `i` (pick order) on `tier`."""
    return f"box_t{tier}{'abcd'[i]}"

# Base magazine pockets: CONSTANT in base_link (rigid to the arm base), so no TF
# needed. 0.236 m along base_link x, 0.081 m along y, 11.8 cm y-pitch.
POCKET_X = 0.3705
POCKET_Y = [0.177, 0.059, -0.059, -0.177]
POCKET_SURFACE_Z = -0.05                # rear-half top surface height in base_link

# Orientation. DOWN = gripper straight down (known-good). The grasp yaw is built
# at runtime from the shelf row direction (world x) so the jaw aligns with the
# inter-magazine gap; GRASP_YAW_OFFSET absorbs the fixed Link6->jaw azimuth and
# is the main thing to tune in sim. PLACE_YAW orients the box in the pocket.
DOWN = (0.707, 0.707, 0.0, 0.0)
GRASP_YAW_OFFSET = 0.0   # rad, TUNE IN SIM
PLACE_YAW = 0.0          # rad about base z for the place orientation, TUNE IN SIM
# In-gap wrist twist: pure J6 roll, no IK. +90 deg by default; flip to -90
# (negate) if the jaw twists the wrong way in sim.
GRIPPER_YAW_TWIST = math.radians(90)   # rad, J6 in-place rotation; sign TUNE IN SIM

# Heights / clearances. GRASP_TCP_ABOVE / INSERT_TCP_ABOVE come from
# gripper_params (TCP = Link6 + 0.12005 along tool z, IK-target convention).
PREGRASP_BACK = 0.25       # start this far in front of the shelf (along -insertion)
POCKET_HOVER = 0.18        # tip height above the pocket surface to hover before placing
PLACE_TCP_ABOVE = 0.08     # TCP above the pocket surface at release
# Folded standby pose: arm stays tucked low until a trigger moves it out.
STANDBY_POSE_DEG = [-8, -39, -105, 0, 0, 0]


# --- carried-object collision phantom + magazine ------------------------------

# Carried-box collision phantom, rigidly parented to the gripper (Link6) so it
# tracks the wrist during planning. BOX_SIZE / BOX_IN_LINK6_XYZ / GRASP_LATERAL_M
# come from gripper_params (the box centre sits BOX_IN_LINK6_XYZ below the flange
# along the down tool axis, and hangs GRASP_LATERAL_M off the tool axis).

# Gazebo SDF link a placed box is link-attached to, so it rides rigidly with the
# AGV when it drives (planar_move moves the whole model; a loose box would slide
# in the pocket). Same lumping reason gripper_base_link is attached as 'Link6'.
MAGAZINE_LINK = 'mpo_base_link'
