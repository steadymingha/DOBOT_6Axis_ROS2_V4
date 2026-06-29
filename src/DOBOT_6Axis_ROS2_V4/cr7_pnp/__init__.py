"""cr7_pnp -- CR7 pick-and-place library.

Self-contained motion module for the Dobot CR7V on the MPO-700 AGV: a reusable
ROS2 node (IK, collision, CBiRRT/servo primitives, hub routing, carried-box
collision) plus the tuned geometry constants and pure helpers. Sequence scripts
(shelf, device A/B/C/D, ...) import from here and compose the primitives; the
trigger (SPACE now, AMR/MCS later) stays out of the node.
"""

from .node import HubPickPlace, CBiRRTPickPlace, CR7Node, RRTNode
from .geometry import (  # noqa: F401  (re-exported for sequence scripts)
    quat_to_R, quat_about_z, quat_mul, pose_at, wait_for_spacebar,
    XACRO_PATH, COMBINED_XACRO,
    SHELF_WORLD_XY, SHELF_BOARD_TOPS, SHELF_FOOTPRINT, SHELF_BOARD_THICK,
    JAW_FIXED_PAD_X, JAW_MOVING_PAD_X0, JAW_GAP_AT_ZERO, PAD_BOTTOM_BELOW_FLANGE,
    BOX_SHORT, FIXED_PAD_CLEARANCE, GRIPPER_OPEN, CLOSE_SQUEEZE, GRIPPER_CLOSE,
    SHELF_BOX_WORLD, SHELF_BOX_MODEL, SHELF_BOX_LINK,
    POCKET_X, POCKET_Y, POCKET_SURFACE_Z,
    DOWN, GRASP_YAW_OFFSET, PLACE_YAW, GRIPPER_YAW_TWIST,
    GRASP_TCP_ABOVE, INSERT_TCP_ABOVE, PREGRASP_BACK, POCKET_HOVER, PLACE_TCP_ABOVE,
    STANDBY_POSE_DEG, BOX_SIZE, BOX_IN_LINK6_XYZ, GRASP_LATERAL_M, MAGAZINE_LINK,
)

__all__ = ['HubPickPlace', 'CBiRRTPickPlace', 'CR7Node', 'RRTNode']
