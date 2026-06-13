# TODO

## Goal
Refactor the 738-line `cbirrt_pick_place.py` into a `pick_place/` subpackage: split
geometry/config/planner/diagnostics/cycle into separate modules, and move the
diagnostic-only `log_gripper_box_clearance` into its own module. Behaviour stays
identical; `cbirrt_pick_place.py` becomes a thin entrypoint.
(2026-06-13 update: `move_constrained` is NO LONGER dead -- the carry (step 7) now
uses it to keep the gripper pointing down while loaded. Keep it in planner.py.)

## Target layout
```
src/DOBOT_6Axis_ROS2_V4/
  cbirrt_pick_place.py        # thin entrypoint: docstring + `from pick_place.cycle import main`
  pick_place/
    __init__.py
    geometry.py               # quat_to_R, quat_about_z, quat_mul, pose_at
    config.py                 # all xacro paths + shelf + workspace/pose constants
    diagnostics.py            # log_gripper_box_clearance (now a function taking node)
    planner.py                # CBiRRTPickPlace class (incl. move_constrained -- used by the carry)
    cycle.py                  # wait_for_spacebar, shelf_to_base_cycle, main
```
Flat sibling scripts (`test_w_gripper.py`, `constrained_cbirrt.py`, `reachability_map.py`)
are NOT modified; they stay importable because the entrypoint dir is on sys.path.

## Tasks

### 1. Create the package skeleton
- [ ] Create `pick_place/__init__.py` (empty package marker).

### 2. geometry.py
- [ ] Move `quat_to_R`, `quat_about_z`, `quat_mul`, `pose_at` verbatim into `pick_place/geometry.py`.

### 3. config.py
- [ ] Move all module-level constants into `pick_place/config.py`:
      `XACRO_PATH`, `COMBINED_XACRO`, `SHELF_WORLD_XY`, `SHELF_BOARD_TOPS`,
      `SHELF_FOOTPRINT`, `SHELF_BOARD_THICK`, `GRIPPER_OPEN`, `GRIPPER_CLOSE`,
      `SHELF_BOX_WORLD/MODEL/LINK`, `POCKET_X/Y`, `POCKET_SURFACE_Z`, `DOWN`,
      `GRASP_YAW_OFFSET`, `PLACE_YAW`, `GRIPPER_YAW_TWIST`, `GRASP_TCP_ABOVE`,
      `INSERT_LIFT`, `PREGRASP_BACK`, `POCKET_HOVER`, `PLACE_TCP_ABOVE`,
      `STANDBY_POSE_DEG` (keep their explanatory comments).

### 4. diagnostics.py
- [ ] Convert `log_gripper_box_clearance` from a method into a module function
      `log_gripper_box_clearance(node, box, row_dir, insert_dir, label="clearance")`
      using `node.tf_buffer` / `node.get_logger()`. Keep the logic identical.

### 5. planner.py
- [ ] Move the `CBiRRTPickPlace` class into `pick_place/planner.py` with imports from
      `.config`, `.geometry`, and the flat sibling scripts.
- [ ] KEEP `move_constrained` (NOT dead anymore: step 7 carry calls it since
      2026-06-13 to hold the gripper down while loaded).
- [ ] REMOVE `log_gripper_box_clearance` from the class (now in diagnostics.py).
- [ ] Keep `self.cbirrt = ConstrainedPlanner(...)` — still used by `linear_servo`.

### 6. cycle.py
- [ ] Move `wait_for_spacebar`, `shelf_to_base_cycle`, and `main` into `pick_place/cycle.py`.
- [ ] Update the one diagnostic call to `log_gripper_box_clearance(node, box, row_dir, insert_dir, label="pre-descend")`.

### 7. Thin entrypoint
- [ ] Rewrite `cbirrt_pick_place.py` to keep only the module docstring (run instructions)
      plus `from pick_place.cycle import main` and the `if __name__ == '__main__'` guard.

### 8. Verify import integrity
- [ ] Run `.venv/bin/python3 -c "import pick_place.cycle"` (and a byte-compile of the
      entrypoint) from the package dir to confirm all imports resolve. No ROS/sim run
      needed — just that modules load without ImportError.

## Follow-up (separate from the code-move refactor above)

See `docs/PICK_PLACE_TROUBLESHOOTING.md` for the full 06-12~13 debugging record
(phantom collision, jaw-align, attach/lumping, contact tuning). Open item:
grasp jitter at close. (The carry-orientation item is resolved: step 7 now uses
`move_constrained` / tilt-locked CBiRRT.)

### Unify the triplicated gripper URDF definition
The Blender fixed-jaw gripper (links, finger joint, collision boxes) is copy-pasted
in THREE places that have already drifted once (old solid-AABB collision / flipped
joint axis / mount 0.1443 vs 0.1401 caused the box-eject and IK mismatches):
- `cra_description/urdf/cr7_on_mpo700.urdf.xacro`  (Gazebo spawn + planning collision; CURRENT reference)
- `cra_description/urdf/cr7_robot.xacro`            (IK/servo model; still has the OLD gripper: axis +X, mount 0.1443, phantom jaws)
- `cr7_moveit/config/cr7_robot.urdf.xacro`          (move_group model; copied from cr7_on_mpo700 on 2026-06-13)

- [ ] Extract the gripper into one `xacro:macro` file (e.g.
      `cra_description/urdf/blender_gripper.xacro`, params: `parent`, gazebo on/off)
      using the cr7_on_mpo700 version (axis -X, mount 0.1401, split C-jaw collision
      boxes: beam + column+pad) as the single source of truth.
- [ ] Replace the three inline copies with `<xacro:include>` + macro call.
      NOTE: cr7_robot.xacro gets a behaviour change on purpose (its old phantom-jaw
      collision and +X axis are bugs, not features) — re-verify IK/reachability after.
- [ ] Keep the hardcoded jaw constants in sync in ONE place in Python too
      (JAW_FIXED_PAD_X etc. in cbirrt_pick_place.py -> pick_place/config.py, and the
      diagnostic prims in log_gripper_box_clearance).

## 참고사항
- Decision: subpackage folder layout (not flat modules); separate the diagnostic
  function. (User-confirmed.) `move_constrained` was originally slated for deletion
  as dead code, but as of 2026-06-13 the carry (step 7) uses it -- keep it.
- Relative imports (`from .config import ...`) inside the package; absolute imports
  (`from test_w_gripper import ...`) for the flat siblings on sys.path.
- All comments/outputs stay in English (per CLAUDE.md).
- Behaviour must be byte-for-byte equivalent; this is a pure code-move refactor.
