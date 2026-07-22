#!/usr/bin/env python3
"""Assemble the Gazebo `cr.world` scene (CR7 on MPO-700 AGV + shelf, wirebonder,
post_wb, boxes) into a single MuJoCo scene and open the viewer.

VISUALIZATION ONLY -- no actuators, no controllers. This just reproduces what
`run_mpo700_cr7.sh` shows in Gazebo so you can see the robot + AGV + objects in
MuJoCo. See README.md for dependencies and the known first-run fixups.

Pipeline:
  1. xacro-expand the robot, rewrite every mesh path to an absolute file that
     MuJoCo can read (STL kept as-is, DAE converted to OBJ -- MuJoCo cannot
     load COLLADA).
  2. Import the robot URDF with MuJoCo, save it back out as MJCF.
  3. Parse cr.world, convert each object's DAE visual mesh to OBJ, and add a
     static body at its world pose.
  4. Compile and launch the passive viewer (or --no-view to just write scene.xml).

Run through run.sh so ROS is sourced (needed for `xacro` and `ros2 pkg prefix`).
"""
import math
import re
import subprocess
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

import numpy as np

WS = Path("/home/user/dobot_ws")
HERE = Path(__file__).resolve().parent
CACHE = HERE / "meshes_obj"          # DAE->OBJ conversion cache
BUILD = HERE / "_build"              # intermediate URDF/MJCF
BLENDER = WS / "src" / "blender"

ROBOT_XACRO = WS / "src/DOBOT_6Axis_ROS2_V4/cra_description/urdf/cr7_on_mpo700.urdf.xacro"
WORLD = WS / "install/dobot_gazebo/share/dobot_gazebo/worlds/cr.world"

# AGV spawn pose from gazebo_mpo700_cr7.launch.py (the "sweet spot" in front of
# the shelf). yaw ~ 0.
ROBOT_XYZ = (0.683, 0.008, 0.0)
ROBOT_YAW = 0.0

_pkg_share_cache = {}


def pkg_share(pkg):
    """Resolve a ROS package's share dir (for $(find PKG) mesh tokens)."""
    if pkg not in _pkg_share_cache:
        prefix = subprocess.check_output(["ros2", "pkg", "prefix", pkg], text=True).strip()
        _pkg_share_cache[pkg] = Path(prefix) / "share" / pkg
    return _pkg_share_cache[pkg]


def resolve_path(raw):
    """Turn a URDF mesh filename into an absolute path: strip file://, expand
    $(find PKG) and package://PKG."""
    p = raw.replace("file://", "")
    m = re.match(r"package://([A-Za-z0-9_]+)/(.*)", p)
    if m:
        return pkg_share(m.group(1)) / m.group(2)
    m = re.search(r"\$\(find ([A-Za-z0-9_]+)\)", p)
    if m:
        p = p.replace(m.group(0), str(pkg_share(m.group(1))))
    return Path(p)


def to_mujoco_mesh(raw):
    """Return an absolute path to a mesh MuJoCo can load. STL/OBJ/MSH pass
    through; DAE is converted to OBJ once and cached."""
    src = resolve_path(str(raw))
    if src.suffix.lower() in (".stl", ".obj", ".msh"):
        return src
    if src.suffix.lower() != ".dae":
        raise ValueError(f"unsupported mesh format: {src}")
    CACHE.mkdir(parents=True, exist_ok=True)
    out = CACHE / (src.stem + ".obj")
    if not out.exists():
        import trimesh  # needs trimesh + pycollada
        mesh = trimesh.load(str(src), force="mesh")
        mesh.export(str(out))
    return out


def rpy_to_quat(r, p, y):
    """SDF/URDF extrinsic RPY (Rz*Ry*Rx) -> MuJoCo quaternion [w, x, y, z]."""
    cr, sr = math.cos(r / 2), math.sin(r / 2)
    cp, sp = math.cos(p / 2), math.sin(p / 2)
    cy, sy = math.cos(y / 2), math.sin(y / 2)
    return [
        cr * cp * cy + sr * sp * sy,
        sr * cp * cy - cr * sp * sy,
        cr * sp * cy + sr * cp * sy,
        cr * cp * sy - sr * sp * cy,
    ]


def parse_xml(path):
    """Parse an SDF/world file, stripping comments first -- some models (shelf)
    have '--' inside XML comments, which Gazebo tolerates but strict parsers
    reject. Comments carry no geometry, so dropping them is safe."""
    text = re.sub(r"<!--.*?-->", "", Path(path).read_text(), flags=re.DOTALL)
    return ET.fromstring(text)


def parse_pose(text):
    """SDF pose text 'x y z r p y' -> list of 6 floats (missing -> zeros)."""
    vals = (text or "").split()
    return [float(vals[i]) if i < len(vals) else 0.0 for i in range(6)]


def pose_to_mat(pose):
    """6-tuple (x,y,z,r,p,y) -> 4x4 transform (rotation Rz*Ry*Rx)."""
    x, y, z, r, p, yw = pose
    cr, sr = math.cos(r), math.sin(r)
    cp, sp = math.cos(p), math.sin(p)
    cy, sy = math.cos(yw), math.sin(yw)
    Rz = np.array([[cy, -sy, 0], [sy, cy, 0], [0, 0, 1]])
    Ry = np.array([[cp, 0, sp], [0, 1, 0], [-sp, 0, cp]])
    Rx = np.array([[1, 0, 0], [0, cr, -sr], [0, sr, cr]])
    T = np.eye(4)
    T[:3, :3] = Rz @ Ry @ Rx
    T[:3, 3] = [x, y, z]
    return T


def mat_to_pos_quat(T):
    """4x4 transform -> ([x,y,z], [w,x,y,z])."""
    R = T[:3, :3]
    tr = np.trace(R)
    if tr > 0:
        s = math.sqrt(tr + 1.0) * 2
        w = 0.25 * s
        x = (R[2, 1] - R[1, 2]) / s
        y = (R[0, 2] - R[2, 0]) / s
        z = (R[1, 0] - R[0, 1]) / s
    else:
        i = int(np.argmax([R[0, 0], R[1, 1], R[2, 2]]))
        j, k = (i + 1) % 3, (i + 2) % 3
        s = math.sqrt(R[i, i] - R[j, j] - R[k, k] + 1.0) * 2
        q = [0, 0, 0]
        q[i] = 0.25 * s
        q[j] = (R[j, i] + R[i, j]) / s
        q[k] = (R[k, i] + R[i, k]) / s
        w = (R[k, j] - R[j, k]) / s
        x, y, z = q
    return list(T[:3, 3]), [w, x, y, z]


def place_model(spec, mujoco, model_name, world_pose, idx):
    """Add every <visual> (mesh or box primitive) of a Gazebo model's SDF at its
    composed world pose. This is what grounds multi-part models (e.g. the shelf's
    box legs) that a single main mesh would leave floating."""
    root = parse_xml(BLENDER / model_name / "model.sdf")
    model = root.find("model")
    T_world = pose_to_mat(world_pose) @ pose_to_mat(parse_pose(model.findtext("pose")))
    n = 0
    for link in model.iter("link"):
        T_link = T_world @ pose_to_mat(parse_pose(link.findtext("pose")))
        for vis in link.findall("visual"):
            geom = vis.find("geometry")
            if geom is None:
                continue
            mesh_uri = geom.findtext("mesh/uri")
            box_size = geom.findtext("box/size")
            if mesh_uri is None and box_size is None:
                continue  # sphere/cylinder/etc. not present in these models
            pos, quat = mat_to_pos_quat(T_link @ pose_to_mat(parse_pose(vis.findtext("pose"))))
            name = f"{model_name}_{idx}_{n}"
            n += 1
            body = spec.worldbody.add_body(name=name, pos=pos, quat=quat)
            if mesh_uri is not None:
                mf = to_mujoco_mesh(BLENDER / mesh_uri.replace("model://", ""))
                spec.add_mesh(name=name, file=str(mf))
                body.add_geom(type=mujoco.mjtGeom.mjGEOM_MESH, meshname=name)
            else:
                half = [float(v) / 2 for v in box_size.split()]
                body.add_geom(type=mujoco.mjtGeom.mjGEOM_BOX, size=half,
                              rgba=[0.7, 0.7, 0.7, 1])


def build_robot_mjcf():
    """xacro -> URDF (mesh paths rewritten) -> MuJoCo import -> MJCF file."""
    import mujoco

    BUILD.mkdir(parents=True, exist_ok=True)
    print(f"[robot] xacro-expanding {ROBOT_XACRO.name} ...")
    urdf = subprocess.check_output(["xacro", str(ROBOT_XACRO)], text=True)

    # Rewrite every <mesh filename="..."> to an absolute MuJoCo-loadable path.
    def repl(m):
        return 'filename="%s"' % to_mujoco_mesh(m.group(1))

    urdf = re.sub(r'filename="([^"]+\.(?:stl|STL|dae|DAE|obj|OBJ))"', repl, urdf)

    # MuJoCo URDF import hints: keep visual meshes, and bound mass/inertia so
    # zero-inertia AGV/sensor links don't abort the compile (viz only).
    hint = ('<mujoco><compiler discardvisual="false" balanceinertia="true" '
            'boundmass="0.01" boundinertia="0.01" fusestatic="false"/></mujoco>')
    urdf = re.sub(r"(<robot\b[^>]*>)", r"\1\n  " + hint, urdf, count=1)

    robot_urdf = BUILD / "robot.urdf"
    robot_urdf.write_text(urdf)
    print("[robot] importing into MuJoCo ...")
    model = mujoco.MjModel.from_xml_path(str(robot_urdf))
    robot_mjcf = BUILD / "robot.xml"
    mujoco.mj_saveLastXML(str(robot_mjcf), model)
    print(f"[robot] MJCF written: {robot_mjcf}")
    return robot_mjcf


def parse_world():
    """Return [(model_name, (x,y,z,r,p,yaw)), ...] for includes whose model lives
    in src/blender (skips ground_plane/sun)."""
    root = parse_xml(WORLD)
    objs = []
    for inc in root.iter("include"):
        name = inc.findtext("uri", "").replace("model://", "").strip()
        if not (BLENDER / name / "model.sdf").exists():
            continue
        objs.append((name, parse_pose(inc.findtext("pose"))))
    return objs


def build_scene(view=True, gravity=False):
    import mujoco

    robot_mjcf = build_robot_mjcf()
    spec = mujoco.MjSpec.from_file(str(robot_mjcf))

    # Place the AGV base at its Gazebo spawn pose (the whole robot rides on the
    # first worldbody child).
    if spec.worldbody.bodies:
        base = spec.worldbody.bodies[0]
        base.pos = list(ROBOT_XYZ)
        base.quat = rpy_to_quat(0, 0, ROBOT_YAW)

    print(f"[world] parsing {WORLD.name} ...")
    for i, (name, pose) in enumerate(parse_world()):
        place_model(spec, mujoco, name, pose, i)
        print(f"[world]  + {name}_{i} at ({pose[0]:.3f}, {pose[1]:.3f}, {pose[2]:.3f})")

    # Position servos on the arm joints -- the MuJoCo equivalent of Gazebo's
    # cr7_group_controller. They actively hold each joint (default target 0), so
    # the robot stands under gravity instead of collapsing. Set data.ctrl[i] to
    # command a joint later.
    # Implicit integrator stays stable with stiff position servos (an explicit
    # Euler step oscillates/blows up at these gains).
    spec.option.integrator = mujoco.mjtIntegrator.mjINT_IMPLICITFAST

    ARM_JOINTS = ["joint1", "joint2", "joint3", "joint4", "joint5", "joint6"]
    for jname in ARM_JOINTS:
        act = spec.add_actuator()
        act.name = jname
        act.target = jname
        act.trntype = mujoco.mjtTrn.mjTRN_JOINT
        act.set_to_position(kp=1000, dampratio=1.0, inheritrange=True)
    grip = spec.add_actuator()
    grip.name = "gripper_finger_joint"
    grip.target = "gripper_finger_joint"
    grip.trntype = mujoco.mjtTrn.mjTRN_JOINT
    grip.set_to_position(kp=50, dampratio=1.0, inheritrange=True)

    # Default: gravity off -> the servos hold the home pose exactly and the robot
    # stands with no visible droop (clean layout view). --gravity turns physics
    # on so you see the servos holding under load like Gazebo's controller (a
    # plain P servo droops a little without gravity compensation).
    if not gravity:
        spec.option.gravity = [0, 0, 0]

    # Brighten: strong headlight (follows the camera) + a fill light so nothing
    # reads as dark/dull.
    spec.visual.headlight.ambient = [0.5, 0.5, 0.5]
    spec.visual.headlight.diffuse = [0.7, 0.7, 0.7]
    spec.visual.headlight.specular = [0.2, 0.2, 0.2]

    # Gazebo's ground_plane + sun were skipped above; add a sky, floor and light.
    sky = spec.add_texture()
    sky.name = "skybox"
    sky.type = mujoco.mjtTexture.mjTEXTURE_SKYBOX
    sky.builtin = mujoco.mjtBuiltin.mjBUILTIN_GRADIENT
    sky.rgb1 = [0.6, 0.75, 0.95]
    sky.rgb2 = [0.35, 0.45, 0.6]
    sky.width = 512
    sky.height = 512

    # Checker floor, like MuJoCo's default ground.
    grid_tex = spec.add_texture()
    grid_tex.name = "grid"
    grid_tex.type = mujoco.mjtTexture.mjTEXTURE_2D
    grid_tex.builtin = mujoco.mjtBuiltin.mjBUILTIN_CHECKER
    grid_tex.rgb1 = [0.3, 0.32, 0.35]
    grid_tex.rgb2 = [0.5, 0.52, 0.55]
    grid_tex.width = 300
    grid_tex.height = 300
    grid_mat = spec.add_material()
    grid_mat.name = "grid"
    grid_mat.textures[mujoco.mjtTextureRole.mjTEXROLE_RGB] = "grid"
    grid_mat.texrepeat = [4, 4]
    grid_mat.texuniform = True
    grid_mat.reflectance = 0.2

    floor = spec.worldbody.add_geom()
    floor.type = mujoco.mjtGeom.mjGEOM_PLANE
    floor.size = [20, 20, 0.1]
    floor.material = "grid"

    light = spec.worldbody.add_light()
    light.pos = [1.5, 0.5, 4.0]
    light.dir = [0, 0, -1]
    light.type = mujoco.mjtLightType.mjLIGHT_DIRECTIONAL
    light.diffuse = [1.0, 1.0, 1.0]
    light.ambient = [0.4, 0.4, 0.4]

    model = spec.compile()
    scene_xml = HERE / "scene.xml"
    scene_xml.write_text(spec.to_xml())
    print(f"[scene] written: {scene_xml}")

    if view:
        import mujoco.viewer
        print("[scene] launching viewer (Ctrl+C or close window to exit) ...")
        mujoco.viewer.launch(model)


if __name__ == "__main__":
    build_scene(view="--no-view" not in sys.argv, gravity="--gravity" in sys.argv)
