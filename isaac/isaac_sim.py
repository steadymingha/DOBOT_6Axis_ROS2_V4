#!/usr/bin/env python3
"""Isaac Sim replacement for the Gazebo sim (gazebo_mpo700_cr7.launch.py + cr.world).

Run with the Isaac Sim python:  ~/isaacsim/python.sh isaac_sim.py [--headless]
(source ROS 2 humble + the workspace first; run_mpo700_cr7_isaac.sh does this).

Provides the exact external interfaces the Gazebo sim provided, so
main.py / MoveIt / tag_vision_node.py run unchanged:
  /isaac_joint_states, /isaac_joint_commands  (topic_based_ros2_control)
  /clock
  /camera/d405/color/image_raw + camera_info, aligned_depth_to_color/image_raw
  /gazebo/model_states                        (pocket occupancy + model_at)
  /ATTACHLINK, /DETACHLINK                    (grasp fixed-joint attach)
Not provided: /gazebo/get|set_entity_state (node.level_base is a Gazebo-drift
repair; the base is fixed here so the failure class does not exist).
"""

import math
import os
import queue
import sys
import threading
import xml.etree.ElementTree as ET

from isaacsim import SimulationApp

HEADLESS = "--headless" in sys.argv
# Experiment: grip by friction + force-limited gripper instead of the Gazebo-style
# ATTACHLINK fixed joint. /ATTACHLINK then only verifies the box is at the jaws
# (sequences still get their success/failure signal); physics does the holding.
PHYS_GRASP = "--physical-grasp" in sys.argv
app = SimulationApp({"headless": HEADLESS, "width": 1600, "height": 900})

import numpy as np
import omni.kit.commands
import omni.usd
from isaacsim.core.api import World
from isaacsim.core.prims import SingleArticulation
from isaacsim.core.utils.extensions import enable_extension
from pxr import Gf, Sdf, Usd, UsdGeom, UsdLux, UsdPhysics, UsdShade

enable_extension("isaacsim.ros2.bridge")
enable_extension("isaacsim.asset.importer.urdf")
app.update()

import omni.graph.core as og
from omni.physx.scripts import utils as physx_utils

HOME = os.path.expanduser("~")
BLENDER = f"{HOME}/dobot_ws/src/blender"
URDF = f"{HOME}/dobot_ws/isaac/cr7_on_mpo700_isaac.urdf"
USD_CACHE = f"{HOME}/dobot_ws/isaac/usd_cache"
ROBOT_NAME = "cr7_on_mpo700"
# AGV park pose from gazebo_mpo700_cr7.launch.py spawn args (shelf sweet spot).
ROBOT_XY = (0.683, 0.008)

# World layout mirrors dobot_gazebo/worlds/cr.world (poses x y z r p y).
BOX_POSES = {
    "box_t1a": (0.7095, 0.5, 1.29, 0, 0, 1.5708),
    "box_t1b": (0.8905, 0.5, 1.29, 0, 0, 1.5708),
    "box_t1c": (0.5285, 0.5, 1.29, 0, 0, 1.5708),
    "box_t1d": (1.0715, 0.5, 1.29, 0, 0, 1.5708),
    "box_t2a": (0.7095, 0.5, 1.79, 0, 0, 1.5708),
    "box_t2b": (0.8905, 0.5, 1.79, 0, 0, 1.5708),
    "box_t2c": (0.5285, 0.5, 1.79, 0, 0, 1.5708),
    "box_t2d": (1.0715, 0.5, 1.79, 0, 0, 1.5708),
    "box_l2a": (2.002, 0.441, 1.355, 0, 0, 0),
    "box_l2b": (2.698, 0.441, 0.896, 0, 0, 0),
}
STATIC_MODELS = {
    # name: (pose, visual dae, [collision meshes])
    "shelf": ((0.8, 0.5, 0.32, 0, 0, 0), f"{BLENDER}/shelf/meshes/shelf.dae",
              [f"{BLENDER}/shelf/meshes/shelf_collision.dae"]),
    "wirebonder": ((2.35, 0.5, 0, 0, 0, 0), f"{BLENDER}/wirebonder/meshes/wirebonder.dae",
                   [f"{BLENDER}/wirebonder/collision/{n}" for n in sorted(
                       os.listdir(f"{BLENDER}/wirebonder/collision")) if n.endswith(".stl")]),
    "post_wb": ((4.11, 1.245, 0.635, 0, 0, -1.5708), f"{BLENDER}/post_wb/meshes/post_wb.dae",
                [f"{BLENDER}/post_wb/collision/{n}" for n in sorted(
                    os.listdir(f"{BLENDER}/post_wb/collision")) if n.endswith(".stl")]),
}
# AprilTag plates: (model, local pose, texture). 0.0375 m square quads, model.sdf poses.
TAGS = [
    ("shelf", (-0.45, -0.1505, 0.91875, 1.5708, 0, 0),
     f"{BLENDER}/shelf/materials/textures/april_36h11-2.png"),
    ("shelf", (-0.45, -0.1505, 1.41875, 1.5708, 0, 0),
     f"{BLENDER}/shelf/materials/textures/april_36h11-3.png"),
    ("wirebonder", (-0.348, -0.1205, 1.2, 1.5708, 0, 0),
     f"{BLENDER}/wirebonder/materials/textures/april_36h11-0.png"),
    ("wirebonder", (0.348, -0.1205, 1.2, 1.5708, 0, 0),
     f"{BLENDER}/wirebonder/materials/textures/april_36h11-1.png"),
]
TAG_SIZE = 0.0375


def axis_quat(x, y, z, deg):
    return Gf.Rotation(Gf.Vec3d(x, y, z), deg).GetQuat()


def rpy_to_quat(r, p, y):
    """URDF rpy -> Gf.Quatd. Column convention R = Rz(y)*Ry(p)*Rx(r); Gf.Quatd
    q1*q2 applies q2 first, so the product order below matches. (Gf.Rotation's
    operator* composes the other way round -- do NOT use it here.)"""
    return (axis_quat(0, 0, 1, math.degrees(y))
            * axis_quat(0, 1, 0, math.degrees(p))
            * axis_quat(1, 0, 0, math.degrees(r)))


def set_pose(prim, pose):
    x, y, z, r, p, yaw = pose
    xf = UsdGeom.Xformable(prim)
    xf.ClearXformOpOrder()
    xf.AddTranslateOp().Set(Gf.Vec3d(x, y, z))
    xf.AddOrientOp(UsdGeom.XformOp.PrecisionDouble).Set(rpy_to_quat(r, p, yaw))


def convert_mesh(src):
    """dae/stl -> usd (cached by mtime)."""
    import asyncio
    import omni.kit.asset_converter as conv
    os.makedirs(USD_CACHE, exist_ok=True)
    dst = os.path.join(USD_CACHE, os.path.basename(src).rsplit(".", 1)[0]
                       + "_" + src.rsplit(".", 1)[1] + ".usd")
    if os.path.exists(dst) and os.path.getmtime(dst) > os.path.getmtime(src):
        return dst

    async def run():
        task = conv.get_instance().create_converter_task(src, dst, None, None)
        return await task.wait_until_finished()

    ok = asyncio.get_event_loop().run_until_complete(run())
    if not ok:
        raise RuntimeError(f"asset convert failed: {src}")
    return dst


def add_reference(stage, path, usd_file):
    prim = stage.DefinePrim(path, "Xform")
    prim.GetReferences().AddReference(usd_file)
    # The asset converter may emit Y-up layers (DAE sources are Z-up but the
    # converted stage is Y-up); referencing those into this Z-up world lays
    # models on their side. Correct per-layer by the layer's own metadata.
    layer = Usd.Stage.Open(usd_file)
    if UsdGeom.GetStageUpAxis(layer) == UsdGeom.Tokens.y:
        UsdGeom.Xformable(prim).AddRotateXOp().Set(90.0)
    return prim


def make_double_sided(prim):
    """Gazebo renders mesh backfaces; USD meshes default to single-sided, which
    turned the shelf front black in the D405 view. Match Gazebo."""
    for p in Usd.PrimRange(prim):
        if p.IsA(UsdGeom.Mesh):
            UsdGeom.Mesh(p).CreateDoubleSidedAttr(True)


def make_collider(stage, path, usd_file, visible=False):
    prim = add_reference(stage, path, usd_file)
    for p in Usd.PrimRange(prim):
        if p.IsA(UsdGeom.Mesh):
            UsdPhysics.CollisionAPI.Apply(p)
    if not visible:
        UsdGeom.Imageable(prim).MakeInvisible()
    return prim


def add_tag_quad(stage, path, world_pose, texture):
    """Textured quad, +Z normal in local frame (matches the SDF thin-box face)."""
    mesh = UsdGeom.Mesh.Define(stage, path)
    h = TAG_SIZE / 2
    mesh.CreatePointsAttr([(-h, -h, 0), (h, -h, 0), (h, h, 0), (-h, h, 0)])
    mesh.CreateFaceVertexCountsAttr([4])
    mesh.CreateFaceVertexIndicesAttr([0, 1, 2, 3])
    mesh.CreateDoubleSidedAttr(True)
    st = UsdGeom.PrimvarsAPI(mesh).CreatePrimvar(
        "st", Sdf.ValueTypeNames.TexCoord2fArray, UsdGeom.Tokens.faceVarying)
    st.Set([(0, 0), (1, 0), (1, 1), (0, 1)])
    set_pose(mesh.GetPrim(), world_pose)

    mat = UsdShade.Material.Define(stage, path + "_mat")
    sh = UsdShade.Shader.Define(stage, path + "_mat/shader")
    sh.CreateIdAttr("UsdPreviewSurface")
    sh.CreateInput("roughness", Sdf.ValueTypeNames.Float).Set(1.0)
    tex = UsdShade.Shader.Define(stage, path + "_mat/tex")
    tex.CreateIdAttr("UsdUVTexture")
    tex.CreateInput("file", Sdf.ValueTypeNames.Asset).Set(texture)
    tex.CreateInput("wrapS", Sdf.ValueTypeNames.Token).Set("clamp")
    tex.CreateInput("wrapT", Sdf.ValueTypeNames.Token).Set("clamp")
    reader = UsdShade.Shader.Define(stage, path + "_mat/st")
    reader.CreateIdAttr("UsdPrimvarReader_float2")
    reader.CreateInput("varname", Sdf.ValueTypeNames.Token).Set("st")
    tex.CreateInput("st", Sdf.ValueTypeNames.Float2).ConnectToSource(
        reader.ConnectableAPI(), "result")
    sh.CreateInput("diffuseColor", Sdf.ValueTypeNames.Color3f).ConnectToSource(
        tex.ConnectableAPI(), "rgb")
    mat.CreateSurfaceOutput().ConnectToSource(sh.ConnectableAPI(), "surface")
    UsdShade.MaterialBindingAPI.Apply(mesh.GetPrim()).Bind(mat)


def compose(model_pose, local_pose):
    """world = model_pose * local_pose (both x y z r p y). Model yaw only here."""
    mx, my, mz, _, _, myaw = model_pose
    lx, ly, lz, lr, lp, lyaw = local_pose
    c, s = math.cos(myaw), math.sin(myaw)
    return (mx + c * lx - s * ly, my + s * lx + c * ly, mz + lz, lr, lp, lyaw + myaw)


# ---------------------------------------------------------------- world build
world = World(stage_units_in_meters=1.0, physics_dt=1.0 / 60.0, rendering_dt=1.0 / 60.0)
stage = omni.usd.get_context().get_stage()

world.scene.add_default_ground_plane()
light = UsdLux.DistantLight.Define(stage, "/World/sun")
light.CreateIntensityAttr(3000.0)

# Robot from URDF (base fixed: missions run with the AGV parked; see ponytail note).
status, import_cfg = omni.kit.commands.execute("URDFCreateImportConfig")
import_cfg.merge_fixed_joints = False
import_cfg.fix_base = True  # ponytail: AGV parked; kinematic base drive when teleop is needed
import_cfg.self_collision = False
import_cfg.convex_decomp = False
import_cfg.import_inertia_tensor = True
import_cfg.default_drive_strength = 1.0e5
import_cfg.default_position_drive_damping = 1.0e4
import_cfg.distance_scale = 1.0
status, robot_path = omni.kit.commands.execute(
    "URDFParseAndImportFile", urdf_path=URDF, import_config=import_cfg,
    get_articulation_root=True)
robot_prim = stage.GetPrimAtPath(robot_path)
# Root transform: prim may carry importer ops; overwrite with the park pose.
set_pose(stage.GetPrimAtPath(f"/{ROBOT_NAME}") if stage.GetPrimAtPath(f"/{ROBOT_NAME}")
         else robot_prim, (ROBOT_XY[0], ROBOT_XY[1], 0, 0, 0, 0))

# Static models
for name, (pose, visual, collisions) in STATIC_MODELS.items():
    root = f"/World/models/{name}"
    stage.DefinePrim(root, "Xform")
    set_pose(stage.GetPrimAtPath(root), pose)
    make_double_sided(add_reference(stage, root + "/visual", convert_mesh(visual)))
    for i, cm in enumerate(collisions):
        make_collider(stage, root + f"/col_{i}", convert_mesh(cm))

# Tag plates
for i, (model, local, texture) in enumerate(TAGS):
    add_tag_quad(stage, f"/World/tags/tag_{i}",
                 compose(STATIC_MODELS[model][0], local), texture)

# Boxes (dynamic)
box_usd = convert_mesh(f"{BLENDER}/box/meshes/box.dae")
for name, pose in BOX_POSES.items():
    root = f"/World/models/{name}"
    prim = stage.DefinePrim(root, "Xform")
    set_pose(prim, pose)
    UsdPhysics.RigidBodyAPI.Apply(prim)
    mass = UsdPhysics.MassAPI.Apply(prim)
    mass.CreateMassAttr(0.3)
    make_double_sided(add_reference(stage, root + "/visual", box_usd))
    cube = UsdGeom.Cube.Define(stage, root + "/collision")
    cube.CreateSizeAttr(1.0)
    UsdGeom.Xformable(cube).AddScaleOp().Set(Gf.Vec3f(0.236, 0.081, 0.14))
    UsdPhysics.CollisionAPI.Apply(cube.GetPrim())
    UsdGeom.Imageable(cube).MakeInvisible()

# ------------------------------------------------------------------- camera
# d405_link prim lives somewhere under the robot; find it by name.
def find_prim_named(root_prim, name):
    for p in Usd.PrimRange(root_prim):
        if p.GetName() == name:
            return p
    return None

robot_root = stage.GetPrimAtPath(f"/{ROBOT_NAME}")
if not robot_root or not robot_root.IsValid():
    robot_root = robot_prim.GetParent() if robot_prim.GetName() != ROBOT_NAME else robot_prim
    for p in stage.Traverse():
        if p.GetName() == ROBOT_NAME:
            robot_root = p
            break
d405_prim = find_prim_named(robot_root, "d405_link")
assert d405_prim, "d405_link not found under robot"

if PHYS_GRASP:
    # Gazebo's grip contact params: mu 1.2 on the gripper pads, high-mu box.
    grip_mat = UsdShade.Material.Define(stage, "/World/grip_mat")
    mat_api = UsdPhysics.MaterialAPI.Apply(grip_mat.GetPrim())
    mat_api.CreateStaticFrictionAttr(1.2)
    mat_api.CreateDynamicFrictionAttr(1.2)
    mat_api.CreateRestitutionAttr(0.0)

    def bind_grip_mat(prim):
        for p in Usd.PrimRange(prim):
            if p.HasAPI(UsdPhysics.CollisionAPI):
                UsdShade.MaterialBindingAPI.Apply(p).Bind(
                    grip_mat, materialPurpose="physics")

    for name in BOX_POSES:
        bind_grip_mat(stage.GetPrimAtPath(f"/World/models/{name}"))
    for link in ("gripper_base_link", "gripper_finger_link"):
        p = find_prim_named(robot_root, link)
        if p:
            bind_grip_mat(p)
    # Force-limited gripper drive (URDF effort 60 N): position target past the box
    # face then squeezes with at most 60 N instead of unbounded drive force.
    jp = find_prim_named(robot_root, "gripper_finger_joint")
    if jp:
        drive = UsdPhysics.DriveAPI.Apply(jp, "linear")
        drive.CreateMaxForceAttr(60.0)

# Gravity off on every robot link, mirroring the <gazebo><gravity>false</gravity>
# tags: the arm is position-controlled and static load against the drives left a
# ~0.2 rad/s velocity jitter at rest, which broke the vision node's arm-still
# gate (needs < 0.02).
from pxr import PhysxSchema

for p in Usd.PrimRange(robot_root):
    if p.HasAPI(UsdPhysics.RigidBodyAPI):
        PhysxSchema.PhysxRigidBodyAPI.Apply(p).CreateDisableGravityAttr(True)

cam = UsdGeom.Camera.Define(stage, str(d405_prim.GetPath()) + "/d405_cam")
# optical frame (rpy -90,0,-90 from d405_link) then a local Rx(180): ROS optical
# is +Z forward / +Y down, USD cameras look -Z with +Y up.
opt_q = rpy_to_quat(-math.pi / 2, 0, -math.pi / 2) * axis_quat(1, 0, 0, 180)
xf = UsdGeom.Xformable(cam)
xf.ClearXformOpOrder()
xf.AddTranslateOp().Set(Gf.Vec3d(0, 0, 0))
xf.AddOrientOp(UsdGeom.XformOp.PrecisionDouble).Set(opt_q)
# 87 deg horizontal FOV at 848x480 (D405 native mode used by the Gazebo sensor).
cam.CreateFocalLengthAttr(18.0)
cam.CreateHorizontalApertureAttr(2 * 18.0 * math.tan(1.5184 / 2))
cam.CreateVerticalApertureAttr(2 * 18.0 * math.tan(1.5184 / 2) * 480.0 / 848.0)
cam.CreateClippingRangeAttr(Gf.Vec2f(0.07, 5.0))

import omni.replicator.core as rep
render_product = rep.create.render_product(str(cam.GetPath()), (848, 480))

# ------------------------------------------------------------- ROS 2 graphs
og.Controller.edit(
    {"graph_path": "/World/ros_graph", "evaluator_name": "execution"},
    {
        og.Controller.Keys.CREATE_NODES: [
            ("tick", "omni.graph.action.OnPlaybackTick"),
            ("ctx", "isaacsim.ros2.bridge.ROS2Context"),
            ("clock", "isaacsim.ros2.bridge.ROS2PublishClock"),
            ("read_time", "isaacsim.core.nodes.IsaacReadSimulationTime"),
            ("pub_js", "isaacsim.ros2.bridge.ROS2PublishJointState"),
            ("sub_jc", "isaacsim.ros2.bridge.ROS2SubscribeJointState"),
            ("art_ctrl", "isaacsim.core.nodes.IsaacArticulationController"),
            ("cam_rgb", "isaacsim.ros2.bridge.ROS2CameraHelper"),
            ("cam_info", "isaacsim.ros2.bridge.ROS2CameraInfoHelper"),
            ("cam_depth", "isaacsim.ros2.bridge.ROS2CameraHelper"),
        ],
        og.Controller.Keys.SET_VALUES: [
            ("clock.inputs:topicName", "/clock"),
            ("pub_js.inputs:topicName", "/isaac_joint_states"),
            ("pub_js.inputs:targetPrim", [Sdf.Path(robot_path)]),
            ("sub_jc.inputs:topicName", "/isaac_joint_commands"),
            ("art_ctrl.inputs:targetPrim", [Sdf.Path(robot_path)]),
            ("cam_rgb.inputs:type", "rgb"),
            ("cam_rgb.inputs:topicName", "/camera/d405/color/image_raw"),
            ("cam_rgb.inputs:frameId", "d405_optical_frame"),
            ("cam_rgb.inputs:renderProductPath", render_product.path),
            ("cam_info.inputs:topicName", "/camera/d405/color/camera_info"),
            ("cam_info.inputs:frameId", "d405_optical_frame"),
            ("cam_info.inputs:renderProductPath", render_product.path),
            ("cam_depth.inputs:type", "depth"),
            ("cam_depth.inputs:topicName",
             "/camera/d405/aligned_depth_to_color/image_raw"),
            ("cam_depth.inputs:frameId", "d405_optical_frame"),
            ("cam_depth.inputs:renderProductPath", render_product.path),
        ],
        og.Controller.Keys.CONNECT: [
            ("tick.outputs:tick", "clock.inputs:execIn"),
            ("read_time.outputs:simulationTime", "clock.inputs:timeStamp"),
            ("tick.outputs:tick", "pub_js.inputs:execIn"),
            ("read_time.outputs:simulationTime", "pub_js.inputs:timeStamp"),
            ("tick.outputs:tick", "sub_jc.inputs:execIn"),
            ("tick.outputs:tick", "cam_rgb.inputs:execIn"),
            ("tick.outputs:tick", "cam_info.inputs:execIn"),
            ("tick.outputs:tick", "cam_depth.inputs:execIn"),
            ("ctx.outputs:context", "clock.inputs:context"),
            ("ctx.outputs:context", "pub_js.inputs:context"),
            ("ctx.outputs:context", "sub_jc.inputs:context"),
            ("sub_jc.outputs:jointNames", "art_ctrl.inputs:jointNames"),
            ("sub_jc.outputs:positionCommand", "art_ctrl.inputs:positionCommand"),
            ("sub_jc.outputs:velocityCommand", "art_ctrl.inputs:velocityCommand"),
            ("sub_jc.outputs:effortCommand", "art_ctrl.inputs:effortCommand"),
            ("sub_jc.outputs:execOut", "art_ctrl.inputs:execIn"),
        ],
    },
)

# ------------------------------------------- Gazebo-compat bridge (rclpy)
import rclpy
from gazebo_msgs.msg import ModelStates
from geometry_msgs.msg import Pose, TransformStamped
from linkattacher_msgs.srv import AttachLink, DetachLink
from rclpy.node import Node as RosNode
from tf2_ros import TransformBroadcaster

MODEL_NAMES = [ROBOT_NAME] + list(STATIC_MODELS) + list(BOX_POSES)
_pose_cache = {}          # name -> (pos, quat) refreshed on the main loop
_sim_time = 0.0            # world time, refreshed on the main loop (stamps TF)
_attach_q = queue.Queue()  # (kind, request, done_event, result_holder)
_attached = {}             # box model name -> joint path


class GazeboCompat(RosNode):
    def __init__(self):
        super().__init__("isaac_gazebo_compat")
        self.pub = self.create_publisher(ModelStates, "/gazebo/model_states", 10)
        self.create_timer(0.5, self.publish_states)  # 2 Hz, like gazebo_ros_state
        self.create_service(AttachLink, "/ATTACHLINK", self.on_attach)
        self.create_service(DetachLink, "/DETACHLINK", self.on_detach)
        # odom -> mpo_base_link TF: gazebo_ros_planar_move published this in the
        # Gazebo sim; the flows treat odom == world. Stamped with sim time to
        # match the /clock the consumers run on.
        self.tf = TransformBroadcaster(self)
        self.create_timer(0.05, self.publish_odom_tf)

    def publish_odom_tf(self):
        entry = _pose_cache.get(ROBOT_NAME)
        if entry is None:
            return
        (x, y, z), (qw, qx, qy, qz) = entry
        t = TransformStamped()
        t.header.stamp.sec = int(_sim_time)
        t.header.stamp.nanosec = int((_sim_time % 1.0) * 1e9)
        t.header.frame_id = "odom"
        t.child_frame_id = "mpo_base_link"
        t.transform.translation.x = x
        t.transform.translation.y = y
        t.transform.translation.z = z
        t.transform.rotation.w = qw
        t.transform.rotation.x = qx
        t.transform.rotation.y = qy
        t.transform.rotation.z = qz
        self.tf.sendTransform(t)

    def publish_states(self):
        msg = ModelStates()
        for name, (pos, quat) in list(_pose_cache.items()):
            p = Pose()
            p.position.x, p.position.y, p.position.z = pos
            p.orientation.w, p.orientation.x, p.orientation.y, p.orientation.z = quat
            msg.name.append(name)
            msg.pose.append(p)
        self.pub.publish(msg)

    def _run_on_main(self, kind, req):
        done, holder = threading.Event(), {}
        _attach_q.put((kind, req, done, holder))
        ok = done.wait(timeout=5.0) and holder.get("ok", False)
        return ok, holder.get("msg", "timeout")

    def on_attach(self, req, res):
        ok, msg = self._run_on_main("attach", req)
        res.success, res.message = ok, msg
        return res

    def on_detach(self, req, res):
        ok, msg = self._run_on_main("detach", req)
        res.success, res.message = ok, msg
        return res


def process_attach_queue():
    while not _attach_q.empty():
        kind, req, done, holder = _attach_q.get()
        try:
            box = req.model2_name
            box_path = f"/World/models/{box}"
            if kind == "attach":
                link_prim = find_prim_named(robot_root, req.link1_name)
                box_prim = stage.GetPrimAtPath(box_path)
                if not link_prim or not box_prim.IsValid():
                    raise ValueError(f"unknown link/model {req.link1_name}/{box}")
                if PHYS_GRASP:
                    # No weld: physics holds the box. Report success only if the
                    # box centre is actually at the jaws, so a missed grasp still
                    # fails the sequence the way a failed weld would.
                    pad = find_prim_named(robot_root, "gripper_finger_link")
                    d = (omni.usd.get_world_transform_matrix(pad).ExtractTranslation()
                         - omni.usd.get_world_transform_matrix(box_prim).ExtractTranslation())
                    dist = math.sqrt(d[0] ** 2 + d[1] ** 2 + d[2] ** 2)
                    if dist > 0.20:
                        raise ValueError(f"{box} not at gripper (d={dist:.3f} m)")
                    holder["ok"], holder["msg"] = True, f"friction grasp {box} (d={dist:.3f})"
                else:
                    joint = physx_utils.createJoint(stage, "Fixed", link_prim, box_prim)
                    _attached[box] = str(joint.GetPath())
                    holder["ok"], holder["msg"] = True, f"attached {box}"
            else:
                jp = _attached.pop(box, None)
                if jp:
                    stage.RemovePrim(jp)
                holder["ok"], holder["msg"] = True, f"detached {box}"
        except Exception as e:  # noqa: BLE001 - report failure to the caller
            holder["ok"], holder["msg"] = False, str(e)
        done.set()


def refresh_pose_cache():
    for name in MODEL_NAMES:
        path = f"/{ROBOT_NAME}" if name == ROBOT_NAME else f"/World/models/{name}"
        prim = stage.GetPrimAtPath(path)
        if not prim.IsValid():
            prim = robot_root if name == ROBOT_NAME else prim
        if not prim or not prim.IsValid():
            continue
        m = omni.usd.get_world_transform_matrix(prim)
        t = m.ExtractTranslation()
        q = m.ExtractRotationQuat()
        im = q.GetImaginary()
        _pose_cache[name] = ((t[0], t[1], t[2]),
                             (q.GetReal(), im[0], im[1], im[2]))


# ---------------------------------------------------------------------- run
world.reset()

# Initial joint positions from the URDF ros2_control initial_value params
# (same values gazebo_ros2_control used).
initials = {}
for j in ET.parse(URDF).getroot().iter("joint"):
    name = j.get("name")
    for p in j.iter("param"):
        if p.get("name") == "initial_value":
            initials[name] = float(p.text)
art = SingleArticulation(robot_path)
art.initialize()
positions = art.get_joint_positions()
for jname, val in initials.items():
    idx = art.get_dof_index(jname)
    if idx is not None and idx >= 0:
        positions[idx] = val
art.set_joint_positions(positions)

rclpy.init()
compat = GazeboCompat()
spin_thread = threading.Thread(
    target=rclpy.spin, args=(compat,), daemon=True)
spin_thread.start()

print("[isaac_sim] world up:", MODEL_NAMES)
frame = 0
while app.is_running():
    process_attach_queue()
    _sim_time = world.current_time
    if frame % 30 == 0:  # 2 Hz pose refresh feeds /gazebo/model_states + odom TF
        refresh_pose_cache()
    world.step(render=True)
    frame += 1

rclpy.shutdown()
app.close()
