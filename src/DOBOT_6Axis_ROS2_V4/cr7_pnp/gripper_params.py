"""Single source of truth for the gripper + tool geometry.

Shared by the motion package (cr7_pnp.geometry / cr7_pnp.model) AND the
standalone reachability_map.py. PURE module -- only the stdlib `math`, NO ROS and
NO numpy -- so reachability_map.py can load it under the bare venv (without the
ROS workspace sourced) while cr7_pnp imports it normally.

If the gripper, the magazine box, or the Link6 tool offset changes, edit HERE
once: both the planner and the reachability map follow.

NOTE on the robot base: the arm-mount / cube / AGV offsets live in the URDF
(cr7_on_mpo700.urdf.xacro), so "robot base moved" is a URDF edit, not a constant
here. The reachability map already reads that base offset from the same xacro
(--combined), so it follows automatically.
"""

import json as _json
import os as _os

_ENV_NAME = _os.getenv('DOBOT_ENV', 'sim')


def load_env(name=None):
    """cr7_pnp/env/<name>.json (default DOBOT_ENV, default 'sim') as a dict."""
    with open(_os.path.join(_os.path.dirname(_os.path.abspath(__file__)), 'env',
                            (name or _ENV_NAME) + '.json')) as f:
        return _json.load(f)


# --- arm / gripper joints and the end-effector (flange) frame -----------------
ARM_JOINTS = ('joint1', 'joint2', 'joint3', 'joint4', 'joint5', 'joint6')
# Movable joints of the Blender fixed-jaw gripper on Link6. gripper_attach_joint
# is FIXED (merged into Link6), so only the prismatic finger joint is movable.
GRIPPER_JOINTS = ('gripper_finger_joint',)
EE_FRAME = 'Link6'

# --- gripper jaw geometry (measured from the gripper_long pad_*.stl) -----------
# gripper_base_link frame: flange face at z=0.1401, mounted flipped on Link6,
# fixed jaw on +X. finger joint axis -X: q > 0 opens; pad gap = JAW_GAP_AT_ZERO+q.
JAW_FIXED_PAD_X = 0.1832     # fixed pad inner face (gripper x), pad_fixed.stl Xmin
JAW_MOVING_PAD_X0 = 0.1022   # moving pad inner face at q=0, pad_moving.stl Xmax
# Widest finger opening any sequence commands (GRIPPER_OPEN). The planning
# collision model freezes the finger HERE: frozen at neutral (q=0) the real
# OPEN moving jaw travels 30 mm outside the model and brushes shelf boxes
# the planner thinks it cleared (measured).
FINGER_OPEN_M = 0.015
JAW_GAP_AT_ZERO = JAW_FIXED_PAD_X - JAW_MOVING_PAD_X0   # 81.0 mm (== BOX_SHORT)
PAD_BOTTOM_BELOW_FLANGE = 0.0821   # pad lower edge, metres below the flange face
BOX_SHORT = 0.081            # box graspable width (short side)
FIXED_PAD_CLEARANCE = 0.003  # fixed pad <-> box gap on descend; TUNE IN SIM

# --- magazine box -------------------------------------------------------------
# (short, long, height) metres -- from cr7_pnp/env/<DOBOT_ENV>.json (sim default),
# same file geometry.py reads for the shelf/pocket constants.
BOX_SIZE = tuple(load_env()['box_size'])

# --- TCP / grasp offsets along the tool z-axis (from the Link6 flange) ---------
# IK-target TCP: abstract point (legacy OnRobot 2FG7) the planner's IK drives. It
# is NOT a physical part -- it sits ~38 mm BELOW the jaw tips (which bottom out at
# PAD_BOTTOM_BELOW_FLANGE). Keep for the motion node's fk_tcp / grasp convention.
TCP_OFFSET_M = 0.12005
GRASP_TCP_ABOVE = 0.015    # TCP above box centre at grasp -> pads wrap ~17 mm
INSERT_TCP_ABOVE = 0.105   # TCP above box centre while travelling inside the gap

# Grasp centre (= box centre when grasped) along the tool axis. This is where the
# magazine physically sits, so it is the right TCP for a grasp-reachability map:
# its reported z = graspable magazine-centre height. reachability_map.py uses it.
GRASP_CENTER_OFFSET_M = TCP_OFFSET_M + GRASP_TCP_ABOVE   # 0.135
BOX_IN_LINK6_XYZ = (0.0, 0.0, GRASP_CENTER_OFFSET_M)     # box centre in Link6 frame

# Lateral hang of the grasp centre off the tool axis (toward the fixed jaw): the
# long fixed jaw holds the box ~140 mm to the side, so the flange is NOT centred
# over the box. Pointing straight down this is purely horizontal (shifts the
# grasp x,y, not its height); the motion node applies it via GRASP_LATERAL_M.
GRASP_LATERAL_M = JAW_FIXED_PAD_X - FIXED_PAD_CLEARANCE - BOX_SHORT / 2.0
