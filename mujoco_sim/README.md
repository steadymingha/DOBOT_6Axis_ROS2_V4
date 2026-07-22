# mujoco_sim

View the `run_mpo700_cr7.sh` scene — CR7 on the MPO-700 AGV plus the shelf,
wirebonder, post_wb and boxes — in MuJoCo. **Visualization only:** no
actuators, no controllers. It reproduces the layout so you can see the robot,
AGV and objects; driving/planning is a later step.

## Install (once)

System python has no pip; use `uv` into a local venv (no sudo, no clash with
ROS python):

```bash
cd ~/dobot_ws/mujoco_sim
uv venv .venv
uv pip install --python .venv/bin/python mujoco trimesh pycollada
```

`mujoco` is the sim/viewer, `trimesh`+`pycollada` convert the objects' COLLADA
(`.dae`) meshes to OBJ (MuJoCo cannot load DAE). `run.sh` uses `.venv`.

## Run

```bash
./run.sh              # build + open the viewer
./run.sh --no-view    # just write scene.xml
```

`run.sh` sources ROS (needed for `xacro` and `ros2 pkg prefix`), then runs
`build_scene.py`.

## What it does

1. `xacro`-expands `cr7_on_mpo700.urdf.xacro` and rewrites every mesh path to an
   absolute file (STL kept, DAE converted to OBJ into `meshes_obj/`).
2. Imports the robot URDF into MuJoCo, saves it as MJCF (`_build/robot.xml`).
3. Parses `cr.world`, places each object at its world pose.
4. Writes `scene.xml` and opens the viewer.

Generated (git-ignore these): `scene.xml`, `_build/`, `meshes_obj/`.

## Known first-run fixups

Nothing here has been run yet — expect to iterate on:

- **Zero-inertia links** — the AGV/sensor links may still trip MuJoCo's compile;
  the `boundmass`/`boundinertia` hints in `build_scene.py` usually cover it, bump
  them if it complains.
- **MjSpec API drift** — `add_mesh`/`add_body`/`add_geom` signatures assume
  mujoco ≥ 3.2. If they differ, that's the line to adjust.
- **Object sub-poses** — each object is placed at its `<include>` world pose; any
  internal `<visual><pose>` offset inside the model's SDF is ignored (fine for a
  layout view, tweak per-object if something sits off).
- **DAE conversion** — needs `pycollada`; if a mesh fails, convert it once by
  hand and drop the `.obj` into `meshes_obj/`.
