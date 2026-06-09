# Fixed-Jaw Parallel Gripper

Gazebo model of a fixed-jaw parallel gripper for robot-arm pick-and-place of the
`box` model (0.236 × 0.081 × 0.14 m). One fixed wide L-jaw + one driven pusher
that clamps the box against it. Includes an eye-in-hand camera shelf (mount your
own RealSense `<sensor>` on top of the L-plate; the shelf protrudes +25 mm past
the body front).

## Files (this is the gripper-control file set)
| File | Purpose |
|------|---------|
| `model.sdf` | The model: 2 links + 1 prismatic joint + collisions + inertials |
| `model.config` | Gazebo model metadata (so it shows up as `model://gripper`) |
| `meshes/base.dae` | Visual mesh of the fixed part (flange + body + L-jaw + pad) |
| `meshes/finger.dae` | Visual mesh of the moving pusher + pad |
| `README.md` | This file |

Editable source: `../gripper_source.blend` (Blender). Re-export `base.dae` /
`finger.dae` from there if you change the shape.

## Kinematics
- **Links:** `base_link` (fixed, bolts to the wrist) and `finger_link` (the pusher).
- **Actuated joint:** `finger_joint` — **prismatic, axis +X**.
  - `+X` = closing (pusher → L-jaw), `−X` = opening.
  - Position `0` = clamped on the 8.1 cm box.
  - Limits: lower `−0.05` (open ~5 cm), upper `+0.07` (fully closed, empty).
  - effort 60 N, velocity 0.1 m/s.
- **Tool frame:** grip is centred on the Z axis (x = y = 0) so a held box hangs
  directly under the flange (minimal wrist moment). Flange **mount face** is the
  top of the flange at **z = +0.1401 m** in the model frame.
- **Dimensions:** flange Ø82 mm; integral L-jaw (1 cm plate + 10 cm box-contact
  lip + camera shelf protruding +25 mm past the body front); body 121×91×12 mm.

## Mounting to the arm
Attach with a **fixed joint** between the arm's wrist/tool link and this model's
`base_link`, offset so the wrist mates with the flange face (z = +0.1443).
Either: (a) `<include>` `model://gripper` in your world and add a fixed joint, or
(b) fold these two links + the prismatic joint into the arm's URDF/SDF.

## Driving the finger (control)
The model already exposes `finger_joint`; pick whichever fits your stack:
- **ros2_control (Gazebo Classic):** add a `gazebo_ros2_control` plugin + a
  `joint_trajectory_controller`/`position_controllers/GripperActionController`
  bound to `finger_joint` (usual choice when the gripper is part of the arm URDF).
- **MoveIt:** expose `finger_joint` as the gripper group / GripperCommand action.
- **Quick test (no controllers):** in Gazebo Classic apply a force/position to
  `finger_joint` via the `gazebo_ros` API or the Gazebo GUI joint panel.

## Companion models (already in this repo)
- `model://shelf` — 4-tier shelf, `model://box` — the dynamic box to grasp,
  `model://aruco_box` — ArUco cube. See `../shelf_world.sdf` for a layout.

> Set `GAZEBO_MODEL_PATH` (Classic) to include `design_gz/` so `model://gripper`
> and the companion `model://...` URIs resolve.
