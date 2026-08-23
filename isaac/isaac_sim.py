#!/usr/bin/env python3
"""Isaac Sim replacement for the Gazebo sim (gazebo_mpo700_cr7.launch.py + cr.world).

Run with the Isaac Sim python:  ~/isaacsim/python.sh isaac_sim.py [--headless]
(source ROS 2 humble + the workspace first; run_mpo700_cr7_isaac.sh does this).

Provides the exact external interfaces the Gazebo sim provided, so
main.py / MoveIt / tag_vision_node.py run unchanged:
  /isaac_joint_states, /isaac_joint_commands  (topic_based_ros2_control)
  /clock
  /camera/d405/color/image_raw + camera_info, aligned_depth_to_color/image_raw
  /camera/canonical/image_raw                 (fixed agent-view cam, dataset)
  /gazebo/model_states                        (pocket occupancy + model_at)
  /ATTACHLINK, /DETACHLINK                    (grasp fixed-joint attach)
  /gazebo/set_entity_state                    (teleport: AGV re-park + box reset;
                                               robot teleports carry pocket boxes)
Not provided: /gazebo/get_entity_state (node.level_base is a Gazebo-drift
repair; the base is fixed here so the failure class does not exist).
"""

import math
import os
import queue
import sys
import threading
import time
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
from pxr import Gf, PhysxSchema, Sdf, Usd, UsdGeom, UsdLux, UsdPhysics, UsdShade

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
# Shelf boxes are generated from the SAME model-frame x offsets as
# cr7_pnp/geometry.py SHELF_BOX_XS (letters = pick order) -- keep in sync.
# 10 per tier (extended 2026-07-24 for diffusion-policy dataset collection).
SHELF_BOX_XS = (-0.0905, +0.0905, -0.2715, +0.2715, -0.4525, +0.4525,
                -0.6335, +0.6335, -0.8145, +0.8145)
SHELF_XY = (0.8, 0.5)                     # shelf model origin (world)
SHELF_TIER_BOX_Z = {1: 1.29, 2: 1.79}     # tier top + box_h/2
BOX_POSES = {
    f"box_t{tier}{'abcdefghij'[i]}":
        (SHELF_XY[0] + bx, SHELF_XY[1], z, 0, 0, 1.5708)
    for tier, z in SHELF_TIER_BOX_Z.items() for i, bx in enumerate(SHELF_BOX_XS)
}
BOX_POSES.update({
    "box_l2a": (2.002, 0.441, 1.355, 0, 0, 0),
    "box_l2b": (2.698, 0.441, 0.896, 0, 0, 0),
})
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
# --beacon: photo/preview layout. Swaps the wirebonder for the signal-tower version and
# lays out a two-row "fab line" for screenshots (extra clones added after the model loop).
# Guarded so dataset runs (no flag) are unchanged. Same wirebonder name/pose keeps the
# AprilTag quads and appearance overrides working; the extra Beacon_* nodes keep dae colors.
BEACON = "--beacon" in sys.argv
if BEACON:
    _wbb = f"{BLENDER}/wirebonder_beacon"
    STATIC_MODELS["wirebonder"] = (STATIC_MODELS["wirebonder"][0],
                                   f"{_wbb}/meshes/wirebonder.dae",
                                   [f"{_wbb}/collision/{n}" for n in sorted(
                                       os.listdir(f"{_wbb}/collision")) if n.endswith(".stl")])
    del STATIC_MODELS["post_wb"]                 # post_wb dropped from the photo scene
    # robot parked between the shelf (x=0.8) and the first equipment (x=2.35), in the aisle
    ROBOT_XY = (1.5, -0.45)
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
# Physics at 240 Hz: 60 Hz physics left the stiff position drives ringing
# during moves and at stop. Rendering at 30 Hz (was 60): the offscreen RTX
# renders for the two dataset cameras dominate wall-clock cost even headless,
# and the collector samples at 10 Hz -- halving the render rate speeds up
# collection without touching the recorded data (matches the real D405's
# 30 fps, too).
world = World(stage_units_in_meters=1.0, physics_dt=1.0 / 240.0, rendering_dt=1.0 / 30.0)
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

# --beacon photo layout: two facing rows of wirebonders across the aisle. Visual-only
# clones (no colliders) -- this is a screenshot scene, not a mission, so 20 extra bodies
# stay cheap. Row A faces -y toward the aisle; row B is rotated 180 to face it back.
if BEACON:
    _wb_usd = convert_mesh(f"{BLENDER}/wirebonder_beacon/meshes/wirebonder.dae")
    _PITCH = 1.08                              # body is 0.95 m wide -> ~0.13 m gap
    _X0 = STATIC_MODELS["wirebonder"][0][0]    # original wirebonder x (2.35)
    _ROWS = [(0.5, 0.0), (-1.40, math.pi)]     # (y, yaw): row A / facing row B (aisle 1.38 m)

    def _clone(nm, usd, pose, scale=None):
        croot = f"/World/models/{nm}"
        stage.DefinePrim(croot, "Xform")
        prim = stage.GetPrimAtPath(croot)
        set_pose(prim, pose)
        if scale is not None:                   # appended after T,O -> squashes local geometry
            UsdGeom.Xformable(prim).AddScaleOp().Set(Gf.Vec3f(*scale))
        make_double_sided(add_reference(stage, croot + "/visual", usd))

    for _k in range(1, 11):                     # 10 beside the original in each row
        _cx = _X0 + _k * _PITCH
        _clone(f"wirebonder_a{_k}", _wb_usd, (_cx, _ROWS[0][0], 0, 0, 0, _ROWS[0][1]))
        _clone(f"wirebonder_b{_k}", _wb_usd, (_cx, _ROWS[1][0], 0, 0, 0, _ROWS[1][1]))
    # one extra shelf at the east end of the shelf line (y=0.5), squashed a bit shorter.
    # Uses the raw shelf mesh (no wire-mesh board swap / leg extensions -- background prop).
    _sh_usd = convert_mesh(STATIC_MODELS["shelf"][1])
    _sh_pose = STATIC_MODELS["shelf"][0]
    _clone("shelf_end", _sh_usd, (14.9, _sh_pose[1], _sh_pose[2], 0, 0, 0), scale=(1, 1, 0.75))
    print("[beacon] photo layout: original + 2x10 wirebonder clones, end shelf, robot mid-aisle")

# --- appearance overrides (visual only; physics colliders untouched) --------
def _solid_mat(name, color, rough=0.5, metallic=0.0):
    m = UsdShade.Material.Define(stage, "/World/mats/" + name)
    s = UsdShade.Shader.Define(stage, "/World/mats/" + name + "/shader")
    s.CreateIdAttr("UsdPreviewSurface")
    s.CreateInput("diffuseColor", Sdf.ValueTypeNames.Color3f).Set(Gf.Vec3f(*color))
    s.CreateInput("roughness", Sdf.ValueTypeNames.Float).Set(rough)
    s.CreateInput("metallic", Sdf.ValueTypeNames.Float).Set(metallic)
    m.CreateSurfaceOutput().ConnectToSource(s.ConnectableAPI(), "surface")
    return m


def _bind_strong(prim, mat):
    UsdShade.MaterialBindingAPI.Apply(prim).Bind(
        mat, bindingStrength=UsdShade.Tokens.strongerThanDescendants)


# wirebonder: ivory body stays; C / G_L / G_R panels dark grey
_wb_dark = _solid_mat("wb_dark", (0.28, 0.29, 0.30), 0.45)
_wb_dark_steel = _solid_mat("wb_dark_steel", (0.28, 0.29, 0.30), 0.40, metallic=0.7)
_wb_overrides = {"Cube_C": _wb_dark, "Cube_G_L": _wb_dark, "Cube_G_R": _wb_dark,
                 "Cube_D": _wb_dark_steel}
_wb_found = []
for p in Usd.PrimRange(stage.GetPrimAtPath("/World/models/wirebonder/visual")):
    for _k, _m in _wb_overrides.items():
        if p.GetName().startswith(_k):
            _bind_strong(p, _m)
            _wb_found.append(p.GetName())
print("[appearance] wirebonder overrides:", _wb_found or "NONE MATCHED")

# shelf boards -> wire-mesh steel. The converted USD has no UVs, so the DAE
# boards are hidden and rebuilt as UV-mapped boxes using the wire_mesh.png
# alpha-cutout texture. Replacement boards take the world bbox of each original
# board (measured BEFORE hiding), so they land exactly where the DAE rendered.
_board_bounds = []
_bbc = UsdGeom.BBoxCache(Usd.TimeCode.Default(), ["default", "render"])
for p in Usd.PrimRange(stage.GetPrimAtPath("/World/models/shelf/visual")):
    if p.GetName().startswith("board_"):
        _r = _bbc.ComputeWorldBound(p).ComputeAlignedRange()
        _lo, _hi = _r.GetMin(), _r.GetMax()
        _board_bounds.append((tuple((_lo[i] + _hi[i]) / 2 for i in range(3)),
                              tuple(_hi[i] - _lo[i] for i in range(3))))
        UsdGeom.Imageable(p).MakeInvisible()
print("[appearance] shelf boards hidden: %d" % len(_board_bounds))

_wire_mat = UsdShade.Material.Define(stage, "/World/mats/mesh_steel")
_ws = UsdShade.Shader.Define(stage, "/World/mats/mesh_steel/shader")
_ws.CreateIdAttr("UsdPreviewSurface")
_ws.CreateInput("metallic", Sdf.ValueTypeNames.Float).Set(0.9)
_ws.CreateInput("roughness", Sdf.ValueTypeNames.Float).Set(0.30)
_ws.CreateInput("opacityThreshold", Sdf.ValueTypeNames.Float).Set(0.5)
_wt = UsdShade.Shader.Define(stage, "/World/mats/mesh_steel/tex")
_wt.CreateIdAttr("UsdUVTexture")
_wt.CreateInput("file", Sdf.ValueTypeNames.Asset).Set(f"{HOME}/dobot_ws/isaac/wire_mesh.png")
_wt.CreateInput("wrapS", Sdf.ValueTypeNames.Token).Set("repeat")
_wt.CreateInput("wrapT", Sdf.ValueTypeNames.Token).Set("repeat")
_wr = UsdShade.Shader.Define(stage, "/World/mats/mesh_steel/st")
_wr.CreateIdAttr("UsdPrimvarReader_float2")
_wr.CreateInput("varname", Sdf.ValueTypeNames.Token).Set("st")
_wt.CreateInput("st", Sdf.ValueTypeNames.Float2).ConnectToSource(
    _wr.ConnectableAPI(), "result")
_ws.CreateInput("diffuseColor", Sdf.ValueTypeNames.Color3f).ConnectToSource(
    _wt.ConnectableAPI(), "rgb")
_ws.CreateInput("opacity", Sdf.ValueTypeNames.Float).ConnectToSource(
    _wt.ConnectableAPI(), "a")
_wire_mat.CreateSurfaceOutput().ConnectToSource(_ws.ConnectableAPI(), "surface")


def _mesh_board(path, center, size, cell=0.018):
    """UV-mapped box so the wire-mesh cutout tiles at `cell` metres."""
    hx, hy, hz = size[0] / 2, size[1] / 2, size[2] / 2
    m = UsdGeom.Mesh.Define(stage, path)
    pts = [(-hx, -hy, -hz), (hx, -hy, -hz), (hx, hy, -hz), (-hx, hy, -hz),
           (-hx, -hy, hz), (hx, -hy, hz), (hx, hy, hz), (-hx, hy, hz)]
    m.CreatePointsAttr(pts)
    m.CreateFaceVertexCountsAttr([4] * 6)
    m.CreateFaceVertexIndicesAttr([0, 3, 2, 1,  4, 5, 6, 7,  0, 1, 5, 4,
                                   2, 3, 7, 6,  1, 2, 6, 5,  3, 0, 4, 7])
    m.CreateDoubleSidedAttr(True)
    nx, ny, nz = size[0] / cell, size[1] / cell, size[2] / cell
    st = []
    for a, b in ((nx, ny), (nx, ny), (nx, nz), (nx, nz), (ny, nz), (ny, nz)):
        st += [(0, 0), (a, 0), (a, b), (0, b)]
    UsdGeom.PrimvarsAPI(m).CreatePrimvar(
        "st", Sdf.ValueTypeNames.TexCoord2fArray, UsdGeom.Tokens.faceVarying).Set(st)
    UsdGeom.Xformable(m).AddTranslateOp().Set(Gf.Vec3d(*center))
    UsdShade.MaterialBindingAPI.Apply(m.GetPrim()).Bind(_wire_mat)


for _bi, (_bc, _bs) in enumerate(_board_bounds):
    # NOT under /World/models/shelf: that prim carries the shelf transform and
    # would double-apply it (bbox centers are already world coordinates).
    _mesh_board("/World/shelf_mesh_boards/board_%d" % _bi, _bc, _bs)
    print("[appearance] mesh board %d at (%.3f, %.3f, %.3f) size (%.3f, %.3f, %.3f)"
          % ((_bi,) + _bc + _bs))

# Shelf leg extensions: the post meshes end at model-local z=0 while the model
# sits at z=0.32, so the shelf floats. Continue the four 3x3 cm posts to the
# floor (positions from shelf.dae post nodes; shelf yaw is 0).
_sx, _sy, _sz = STATIC_MODELS["shelf"][0][:3]
for _i, (_lx, _ly) in enumerate(
        [(0.985, 0.135), (0.985, -0.135), (-0.985, 0.135), (-0.985, -0.135)]):
    _leg = UsdGeom.Cube.Define(stage, f"/World/models/shelf_leg_ext_{_i}")
    _leg.CreateSizeAttr(1.0)
    _leg.CreateDisplayColorAttr([(0.25, 0.25, 0.27)])
    _xf = UsdGeom.Xformable(_leg.GetPrim())
    _xf.AddTranslateOp().Set(Gf.Vec3d(_sx + _lx, _sy + _ly, _sz / 2))
    _xf.AddScaleOp().Set(Gf.Vec3f(0.03, 0.03, _sz))
    UsdPhysics.CollisionAPI.Apply(_leg.GetPrim())

# Background environment (VISUAL ONLY -- every collider in it is disabled so
# it cannot touch mission physics):  --env hospital|warehouse|office|simple_room|grid
# Needs the NVIDIA asset server; first load downloads the assets.
_ENVS = {
    "hospital": "/Isaac/Environments/Hospital/hospital.usd",
    "warehouse": "/Isaac/Environments/Simple_Warehouse/warehouse.usd",
    "office": "/Isaac/Environments/Office/office.usd",
    "simple_room": "/Isaac/Environments/Simple_Room/simple_room.usd",
    "grid": "/Isaac/Environments/Grid/gridroom_curved.usd",
}
def _build_fab_room():
    """Local fab clean-room look (no asset download), modeled on the real line
    photo: perforated raised-floor tiles (isaac/floor_tile.png, 0.6 m/tile),
    yellow aisle lines, white panel walls. Visual only."""
    # textured floor (room bounds: resize the whole shell here)
    _fl = UsdGeom.Mesh.Define(stage, "/World/room/floor")
    # --beacon extends the east wall past the equipment lines (end shelf) and pulls the north
    # wall in right behind the shelf-side row (Row A back is at y=1.28).
    _x0, _x1, _y0, _y1 = -6.0, (16.5 if BEACON else 14.0), -7.0, (1.6 if BEACON else 7.0)
    _fl.CreatePointsAttr([(_x0, _y0, 0.001), (_x1, _y0, 0.001),
                          (_x1, _y1, 0.001), (_x0, _y1, 0.001)])
    _fl.CreateFaceVertexCountsAttr([4])
    _fl.CreateFaceVertexIndicesAttr([0, 1, 2, 3])
    _nx, _ny = (_x1 - _x0) / 0.6, (_y1 - _y0) / 0.6  # 0.6 m tile pitch
    _stv = UsdGeom.PrimvarsAPI(_fl).CreatePrimvar(
        "st", Sdf.ValueTypeNames.TexCoord2fArray, UsdGeom.Tokens.faceVarying)
    _stv.Set([(0, 0), (_nx, 0), (_nx, _ny), (0, _ny)])
    _m = UsdShade.Material.Define(stage, "/World/room/floor_mat")
    _sh = UsdShade.Shader.Define(stage, "/World/room/floor_mat/shader")
    _sh.CreateIdAttr("UsdPreviewSurface")
    _sh.CreateInput("roughness", Sdf.ValueTypeNames.Float).Set(0.55)
    _tx = UsdShade.Shader.Define(stage, "/World/room/floor_mat/tex")
    _tx.CreateIdAttr("UsdUVTexture")
    _tx.CreateInput("file", Sdf.ValueTypeNames.Asset).Set(
        f"{HOME}/dobot_ws/isaac/floor_tile_dark.png" if BEACON
        else f"{HOME}/dobot_ws/isaac/floor_tile.png")
    _tx.CreateInput("wrapS", Sdf.ValueTypeNames.Token).Set("repeat")
    _tx.CreateInput("wrapT", Sdf.ValueTypeNames.Token).Set("repeat")
    _rd = UsdShade.Shader.Define(stage, "/World/room/floor_mat/st")
    _rd.CreateIdAttr("UsdPrimvarReader_float2")
    _rd.CreateInput("varname", Sdf.ValueTypeNames.Token).Set("st")
    _tx.CreateInput("st", Sdf.ValueTypeNames.Float2).ConnectToSource(
        _rd.ConnectableAPI(), "result")
    _sh.CreateInput("diffuseColor", Sdf.ValueTypeNames.Color3f).ConnectToSource(
        _tx.ConnectableAPI(), "rgb")
    _m.CreateSurfaceOutput().ConnectToSource(_sh.ConnectableAPI(), "surface")
    UsdShade.MaterialBindingAPI.Apply(_fl.GetPrim()).Bind(_m)
    # yellow aisle lines + white panel walls, derived from the bounds above
    _cx, _cy = (_x0 + _x1) / 2, (_y0 + _y1) / 2
    _lx, _ly, _h = _x1 - _x0, _y1 - _y0, 3.0
    _parts = [
        ("line_a", (_cx, -1.04 if BEACON else -1.5, 0.003), (_lx - 0.4, 0.08, 0.002), (0.93, 0.78, 0.05)),
        # full-length aisle line just in front of the wirebonder face (y=0)
        ("line_b", (_cx, 0.10, 0.003), (_lx - 0.4, 0.08, 0.002), (0.93, 0.78, 0.05)),
        ("wall_n", (_cx, _y1, _h / 2), (_lx, 0.05, _h), (0.92, 0.93, 0.95)),
        ("wall_s", (_cx, _y0, _h / 2), (_lx, 0.05, _h), (0.92, 0.93, 0.95)),
        ("wall_e", (_x1, _cy, _h / 2), (0.05, _ly, _h), (0.92, 0.93, 0.95)),
        ("wall_w", (_x0, _cy, _h / 2), (0.05, _ly, _h), (0.92, 0.93, 0.95)),
    ]
    for _nm, _c, _s, _col in _parts:
        _pr = UsdGeom.Cube.Define(stage, "/World/room/" + _nm)
        _pr.CreateSizeAttr(1.0)
        _pr.CreateDisplayColorAttr([_col])
        _xf = UsdGeom.Xformable(_pr.GetPrim())
        _xf.AddTranslateOp().Set(Gf.Vec3d(*_c))
        _xf.AddScaleOp().Set(Gf.Vec3f(*_s))
    _dm = UsdLux.DomeLight.Define(stage, "/World/room/dome")
    _dm.CreateIntensityAttr(400.0)
    print("[env] fab room built (tiled floor + aisle lines + walls)")


def _load_asset_env(_env_name):
    try:
        from isaacsim.storage.native import get_assets_root_path
        _root = get_assets_root_path()
        assert _root, "asset root unavailable (no Nucleus/cloud access)"
        _env_prim = stage.DefinePrim("/World/env", "Xform")
        _env_prim.GetReferences().AddReference(_root + _ENVS[_env_name])
        # sink 2 mm so the env floor does not z-fight our ground plane
        # (reuse the translate op the referenced root may already author)
        _exf = UsdGeom.Xformable(_env_prim)
        _tops = [o for o in _exf.GetOrderedXformOps()
                 if o.GetOpType() == UsdGeom.XformOp.TypeTranslate]
        _tv = _tops[0].Get() if _tops else None
        (_tops[0] if _tops else _exf.AddTranslateOp()).Set(
            Gf.Vec3d(_tv[0], _tv[1], _tv[2] - 0.002) if _tv else Gf.Vec3d(0, 0, -0.002))
        _n_off = 0
        for p in Usd.PrimRange(_env_prim, Usd.TraverseInstanceProxies()):
            try:
                if p.HasAPI(UsdPhysics.CollisionAPI):
                    UsdPhysics.CollisionAPI(p).CreateCollisionEnabledAttr(False)
                    _n_off += 1
            except Exception:
                pass  # instance proxies cannot be edited; report the count below
        print("[env] %s loaded, %d colliders disabled" % (_env_name, _n_off))
    except Exception as e:
        print("[env] load failed (%s); continuing without background" % e)


if "--env" in sys.argv:
    _env_name = sys.argv[sys.argv.index("--env") + 1]
    if _env_name == "fab":
        _build_fab_room()
    else:
        _load_asset_env(_env_name)

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
    # Never sleep: a welded box below the sleep threshold during the slow guarded
    # place-descend froze mid-air (pose stuck until the detach resync woke it),
    # and a sleeping box also freezes its /gazebo/model_states pose.
    PhysxSchema.PhysxRigidBodyAPI.Apply(prim).CreateSleepThresholdAttr(0.0)
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

# Live robot pose source: the BASE LINK prim. Physics writes link transforms
# back to USD every frame; the root Xform only holds the spawn pose we authored
# and never updates, so reading it froze /gazebo/model_states after a teleport.
base_link_prim = find_prim_named(robot_root, "mpo_base_link")
assert base_link_prim, "mpo_base_link not found under robot"

# World-anchor root joint (fix_base=True import): the fixed joint whose body0
# side is NOT a rigid link (empty = world, or a plain Xform). Teleporting a
# fixed-base articulation = shifting THIS joint's anchor (localPose0); PhysX
# projects the whole articulation to the new anchor. (art.set_world_pose is
# silently ignored for fixed-base -- measured: model_states never moved.)
ROOT_JOINT = None
for _p in Usd.PrimRange(robot_root):
    if _p.IsA(UsdPhysics.FixedJoint):
        _tg = UsdPhysics.Joint(_p).GetBody0Rel().GetTargets()
        _pr = stage.GetPrimAtPath(_tg[0]) if _tg else None
        if _pr is None or not _pr.HasAPI(UsdPhysics.RigidBodyAPI):
            ROOT_JOINT = UsdPhysics.Joint(_p)
            break
print("[teleport] world-anchor root joint:",
      ROOT_JOINT.GetPrim().GetPath() if ROOT_JOINT else "NOT FOUND (AGV teleport disabled)")

# --beacon appearance: match the real-line photo (visual only; guarded so dataset runs
# keep their look).
if BEACON:
    _arm_mat = _solid_mat("arm_dark", (0.16, 0.16, 0.17), 0.5)
    _base_mat = _solid_mat("robot_cream", (0.90, 0.86, 0.74), 1.0)  # matte
    _agv_mat = _solid_mat("agv_orange", (0.92, 0.45, 0.06), 0.5)

    # slotted silver magazine material (box.dae has UVs)
    def _tex_mat(name, png, rough=0.4, metallic=0.7):
        m = UsdShade.Material.Define(stage, "/World/mats/" + name)
        s = UsdShade.Shader.Define(stage, "/World/mats/" + name + "/shader")
        s.CreateIdAttr("UsdPreviewSurface")
        s.CreateInput("roughness", Sdf.ValueTypeNames.Float).Set(rough)
        s.CreateInput("metallic", Sdf.ValueTypeNames.Float).Set(metallic)
        t = UsdShade.Shader.Define(stage, "/World/mats/" + name + "/tex")
        t.CreateIdAttr("UsdUVTexture")
        t.CreateInput("file", Sdf.ValueTypeNames.Asset).Set(png)
        t.CreateInput("wrapS", Sdf.ValueTypeNames.Token).Set("repeat")
        t.CreateInput("wrapT", Sdf.ValueTypeNames.Token).Set("repeat")
        r = UsdShade.Shader.Define(stage, "/World/mats/" + name + "/st")
        r.CreateIdAttr("UsdPrimvarReader_float2")
        r.CreateInput("varname", Sdf.ValueTypeNames.Token).Set("st")
        t.CreateInput("st", Sdf.ValueTypeNames.Float2).ConnectToSource(r.ConnectableAPI(), "result")
        s.CreateInput("diffuseColor", Sdf.ValueTypeNames.Color3f).ConnectToSource(
            t.ConnectableAPI(), "rgb")
        m.CreateSurfaceOutput().ConnectToSource(s.ConnectableAPI(), "surface")
        return m
    _mag_mat = _tex_mat("magazine", f"{HOME}/dobot_ws/isaac/magazine_slots.png")

    # robot: Isaac's URDF importer puts visual meshes under a separate `visuals` scope, NOT
    # under the physics link prims -- binding the link Xform did nothing. De-instance the
    # whole robot, then bind every mesh whose prim path contains the link name (the link is
    # a path ancestor in the visuals scope). Leading slash keeps base_link != mpo_base_link.
    for _ in range(5):
        _inst = [p for p in Usd.PrimRange(robot_root) if p.IsInstanceable()]
        for p in _inst:
            p.SetInstanceable(False)
        if not _inst:
            break
    _robot_meshes = [p for p in Usd.PrimRange(robot_root) if p.IsA(UsdGeom.Mesh)]

    def _paint_link(link_name, mat):
        n = 0
        for p in _robot_meshes:
            if f"/{link_name}" in str(p.GetPath()):
                _bind_strong(p, mat); n += 1
        print(f"[beacon] {link_name}: painted {n} meshes")

    _paint_link("mpo_base_link", _agv_mat)                       # AGV -> orange
    for _ln in ("base_link", "cube_link"):                       # arm base / pedestal -> cream (matte)
        _paint_link(_ln, _base_mat)
    for _ln in ("Link1", "Link2", "Link3", "Link4", "Link5", "Link6"):
        _paint_link(_ln, _arm_mat)                               # arm links -> very dark grey

    # magazine per-face: big faces (perp to shortest box axis) = slotted grille; the two
    # end faces (perp to longest axis) = open board slots; top/bottom (middle axis) = solid.
    # Grouping by extent rank is axis-permutation safe (converter up-axis may differ).
    _sil_mat = _solid_mat("mag_solid", (0.72, 0.73, 0.75), 0.35, metallic=0.8)
    _open_mat = _solid_mat("mag_open", (0.07, 0.07, 0.08), 0.6)

    def _face_groups(mesh):
        pts = mesh.GetPointsAttr().Get()
        counts = mesh.GetFaceVertexCountsAttr().Get()
        idx = mesh.GetFaceVertexIndicesAttr().Get()
        ext = [max(p[i] for p in pts) - min(p[i] for p in pts) for i in range(3)]
        order = sorted(range(3), key=lambda i: ext[i])       # smallest .. largest axis
        axis_group = {order[0]: "grille", order[1]: "solid", order[2]: "open"}
        g = {"solid": [], "grille": [], "open": []}
        o = 0
        for fi, c in enumerate(counts):
            f = [idx[o + k] for k in range(c)]; o += c
            n = ((Gf.Vec3f(pts[f[1]]) - Gf.Vec3f(pts[f[0]]))
                 ^ (Gf.Vec3f(pts[f[2]]) - Gf.Vec3f(pts[f[0]])))
            ax = max(range(3), key=lambda i: abs(n[i]))
            g[axis_group[ax]].append(fi)
        return g

    def _magazine(box_name):
        vis = stage.GetPrimAtPath(f"/World/models/{box_name}/visual")
        mesh = next((UsdGeom.Mesh(p) for p in Usd.PrimRange(vis) if p.IsA(UsdGeom.Mesh)), None)
        if not mesh:
            return
        g = _face_groups(mesh)
        for gname, mat in (("solid", _sil_mat), ("grille", _mag_mat), ("open", _open_mat)):
            if not g[gname]:
                continue
            sub = UsdGeom.Subset.Define(stage, mesh.GetPath().AppendChild("mb_" + gname))
            sub.CreateElementTypeAttr(UsdGeom.Tokens.face)
            sub.CreateFamilyNameAttr("materialBind")
            sub.CreateIndicesAttr(g[gname])
            UsdShade.MaterialBindingAPI.Apply(sub.GetPrim()).Bind(mat)

    for _bn in BOX_POSES:
        if _bn in ("box_l2a", "box_l2b"):
            continue
        _magazine(_bn)
    for _bn in ("box_l2a", "box_l2b"):   # buried machine boxes hidden for the photo
        UsdGeom.Imageable(stage.GetPrimAtPath(f"/World/models/{_bn}")).MakeInvisible()
    print("[beacon] appearance applied")

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
for p in Usd.PrimRange(robot_root):
    if p.HasAPI(UsdPhysics.RigidBodyAPI):
        PhysxSchema.PhysxRigidBodyAPI.Apply(p).CreateDisableGravityAttr(True)

# Gripper collision: replace ALL imported gripper colliders with exact
# primitive boxes measured from the collision STLs (link-local, meters).
# The importer convex-hulls each mesh: the L-shaped fixed jaw becomes a brick
# spanning the whole pad-to-pad gap (x -21..201 mm) so boxes cannot seat, and
# VHACD decomposition still bulges a few mm into the ~3 mm descend clearance.
# Primitives are exact and deterministic (jaw_fixed = arm + finger boxes).
_GRIP_BOXES = {
    "gripper_base_link": (
        ("flange",     (0.0,     0, 0.1336), (0.0820, 0.0820, 0.0130)),
        ("body",       (0.0050,  0, 0.1210), (0.1315, 0.0911, 0.0121)),
        ("jaw_arm",    (0.0901,  0, 0.1100), (0.2216, 0.0912, 0.0100)),
        ("jaw_finger", (0.19455, 0, 0.0850), (0.0127, 0.0912, 0.0600)),
        ("pad_fixed",  (0.1857,  0, 0.0800), (0.0050, 0.0800, 0.0440)),
    ),
    "gripper_finger_link": (
        ("jaw_moving", (0.0862,  0, 0.0811), (0.0220, 0.0700, 0.0500)),
        ("pad_moving", (0.0997,  0, 0.0810), (0.0050, 0.0600, 0.0380)),
    ),
}
for _link, _boxes in _GRIP_BOXES.items():
    _lp = find_prim_named(robot_root, _link)
    if not _lp:
        print("[gripper-collision] LINK NOT FOUND: %s" % _link)
        continue
    _col = _lp.GetChild("collisions")
    if _col:
        _col.SetActive(False)
        print("[gripper-collision] %s/collisions deactivated" % _link)
    else:
        # unknown layout: dump children so the log shows what to target, and
        # try disabling any collider prims found by traversal
        print("[gripper-collision] %s children: %s" % (
            _link, [(k.GetName(), str(k.GetTypeName())) for k in _lp.GetChildren()]))
        for p in Usd.PrimRange(_lp):
            try:
                if p.HasAPI(UsdPhysics.CollisionAPI) or p.IsA(UsdGeom.Mesh):
                    UsdPhysics.CollisionAPI.Apply(p).CreateCollisionEnabledAttr(False)
            except Exception as _e:
                print("[gripper-collision] disable failed on %s: %s" % (p.GetPath(), _e))
    for _nm, _c, _s in _boxes:
        _cb = UsdGeom.Cube.Define(stage, str(_lp.GetPath()) + "/col_" + _nm)
        _cb.CreateSizeAttr(1.0)
        _xf = UsdGeom.Xformable(_cb.GetPrim())
        _xf.AddTranslateOp().Set(Gf.Vec3d(*_c))
        _xf.AddScaleOp().Set(Gf.Vec3f(*_s))
        UsdPhysics.CollisionAPI.Apply(_cb.GetPrim())
        UsdGeom.Imageable(_cb.GetPrim()).MakeInvisible()
    print("[gripper-collision] %s: %d exact boxes added" % (_link, len(_boxes)))

# Solver iterations: PhysX default (4) cannot resolve the stiff position drives
# (1e5) -- symptom was velocity jitter at rest (masked by gravity-off above) and
# visible shaking during fast moves.
for p in Usd.PrimRange(robot_root):
    if p.HasAPI(UsdPhysics.ArticulationRootAPI):
        art = PhysxSchema.PhysxArticulationAPI.Apply(p)
        art.CreateSolverPositionIterationCountAttr(64)
        art.CreateSolverVelocityIterationCountAttr(4)

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

# Canonical (agent-view) camera for the diffusion-policy dataset: mounted on
# the AGV BASE (rigid to mpo_base_link, like the d405 on the wrist), framing
# the arm + base pockets + the in-reach shelf section CLOSE UP. Robot-relative
# (not world-fixed) keeps the robot at the same place in frame at every park --
# consistent with the BASE-frame obs/actions -- and the shelf shifting in frame
# IS the park-variation signal. Poses below are in the mpo_base_link frame.
# USD cameras look down -Z with +Y up; orientation built from eye -> target.
CANON_EYE, CANON_TARGET = (-1.15, -1.25, 1.85), (0.30, 0.45, 1.15)


def look_at_quat(eye, target):
    f = Gf.Vec3d(*target) - Gf.Vec3d(*eye)
    f = f.GetNormalized()
    x = Gf.Cross(f, Gf.Vec3d(0, 0, 1)).GetNormalized()   # camera right
    z = -f                                               # camera backward
    y = Gf.Cross(z, x)                                   # camera up
    return Gf.Matrix3d(x[0], x[1], x[2], y[0], y[1], y[2],
                       z[0], z[1], z[2]).ExtractRotation().GetQuat()


canon_cam = UsdGeom.Camera.Define(
    stage, str(base_link_prim.GetPath()) + "/canonical_cam")
canon_cam.CreateFocalLengthAttr(18.0)
canon_cam.CreateHorizontalApertureAttr(20.955)   # ~60 deg horizontal FOV
canon_cam.CreateVerticalApertureAttr(20.955 * 3.0 / 4.0)
canon_cam.CreateClippingRangeAttr(Gf.Vec2f(0.1, 20.0))
_cxf = UsdGeom.Xformable(canon_cam)
_cxf.AddTranslateOp().Set(Gf.Vec3d(*CANON_EYE))
_cxf.AddOrientOp(UsdGeom.XformOp.PrecisionDouble).Set(
    look_at_quat(CANON_EYE, CANON_TARGET))
canon_product = rep.create.render_product(str(canon_cam.GetPath()), (640, 480))
# (Decision 2026-07-24: base-mounted close-up chosen over the world-fixed "GUI
# view" candidate -- side-by-side samples showed the GUI framing leaves boxes
# ~15 px and the pockets occluded. The comparison gui_view camera was removed.)

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
            ("cam_canon", "isaacsim.ros2.bridge.ROS2CameraHelper"),
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
            ("cam_canon.inputs:type", "rgb"),
            ("cam_canon.inputs:topicName", "/camera/canonical/image_raw"),
            ("cam_canon.inputs:frameId", "canonical_cam"),
            ("cam_canon.inputs:renderProductPath", canon_product.path),
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
            ("tick.outputs:tick", "cam_canon.inputs:execIn"),
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
from gazebo_msgs.srv import SetEntityState
from geometry_msgs.msg import Pose, TransformStamped
from linkattacher_msgs.srv import AttachLink, DetachLink
from rclpy.node import Node as RosNode
from tf2_ros import TransformBroadcaster

MODEL_NAMES = [ROBOT_NAME] + list(STATIC_MODELS) + list(BOX_POSES)
_pose_cache = {}          # name -> (pos, quat) refreshed on the main loop
_sim_time = 0.0            # world time, refreshed on the main loop (stamps TF)
_attach_q = queue.Queue()  # (kind, request, done_event, result_holder)
_attached = {}             # box model name -> joint path
_teleport_checks = []      # [frames_left, target_x, target_y] robot teleports
_sim_paused = False        # /sim_pause gate: freezes world.step (and thus /clock,
                           # controllers, cameras) for stepped policy evaluation


class GazeboCompat(RosNode):
    def __init__(self):
        super().__init__("isaac_gazebo_compat")
        self.pub = self.create_publisher(ModelStates, "/gazebo/model_states", 10)
        self.create_timer(0.5, self.publish_states)  # 2 Hz, like gazebo_ros_state
        self.create_service(AttachLink, "/ATTACHLINK", self.on_attach)
        self.create_service(DetachLink, "/DETACHLINK", self.on_detach)
        self.create_service(SetEntityState, "/gazebo/set_entity_state",
                            self.on_set_state)
        # Stepped-eval support: pausing stops world.step in the main loop, which
        # freezes sim time coherently (JTC, TF stamps, cameras). robomimic-style
        # semantics: the world does not advance while the policy thinks.
        from std_srvs.srv import SetBool
        self.create_service(SetBool, "/sim_pause", self.on_sim_pause)
        # odom -> mpo_base_link TF: gazebo_ros_planar_move published this in the
        # Gazebo sim; the flows treat odom == world. Stamped with sim time to
        # match the /clock the consumers run on.
        self.tf = TransformBroadcaster(self)
        self.create_timer(0.05, self.publish_odom_tf)

    def on_sim_pause(self, req, res):
        global _sim_paused
        _sim_paused = bool(req.data)
        res.success = True
        res.message = "paused" if _sim_paused else "running"
        return res

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

    def on_set_state(self, req, res):
        ok, msg = self._run_on_main("set_state", req)
        res.success = ok
        if not ok:
            self.get_logger().error(f"[set_entity_state] {msg}")
        return res


def _teleport_prim(prim, pos, quat=None):
    """Move a spawned rigid body: reuse the translate/orient ops set_pose authored
    (physics writes back through the same ops) and zero its velocities."""
    for op in UsdGeom.Xformable(prim).GetOrderedXformOps():
        if op.GetOpType() == UsdGeom.XformOp.TypeTranslate:
            op.Set(Gf.Vec3d(*pos))
        elif quat is not None and op.GetOpType() == UsdGeom.XformOp.TypeOrient:
            op.Set(quat)
    rb = UsdPhysics.RigidBodyAPI(prim)
    rb.CreateVelocityAttr().Set(Gf.Vec3f(0, 0, 0))
    rb.CreateAngularVelocityAttr().Set(Gf.Vec3f(0, 0, 0))


def _do_set_state(req, holder):
    st = req.state
    p, o = st.pose.position, st.pose.orientation
    if st.name == ROBOT_NAME:
        # AGV re-park: keep z and yaw (planar park, yaw always 0 in the
        # missions). Implemented by shifting the world-anchor root joint by the
        # x/y delta from the LIVE base-link pose -- works for the fixed-base
        # articulation where set_world_pose does not. Boxes riding the magazine
        # pockets (loose, below tier-1 and near the base) are carried along,
        # or a teleport strands them mid-air.
        cur = omni.usd.get_world_transform_matrix(base_link_prim).ExtractTranslation()
        dx, dy = p.x - cur[0], p.y - cur[1]
        if ROOT_JOINT is not None:
            lp0 = ROOT_JOINT.GetLocalPos0Attr().Get() or Gf.Vec3f(0.0)
            ROOT_JOINT.GetLocalPos0Attr().Set(
                Gf.Vec3f(lp0[0] + dx, lp0[1] + dy, lp0[2]))
        # Belt and braces: ALSO set the articulation root through the tensor API.
        # Which of the two paths PhysX honours depends on the build; both express
        # the SAME absolute target, so applying both stays consistent. The result
        # is measured a few frames later and printed as OK/FAILED (main loop).
        try:
            art.set_world_pose(position=np.array([p.x, p.y, cur[2]]))
        except Exception as e:
            print(f"[set_entity_state] set_world_pose raised: {e}")
        _teleport_checks.append([10, p.x, p.y])
        carried = []
        for bname in BOX_POSES:
            bprim = stage.GetPrimAtPath(f"/World/models/{bname}")
            t = omni.usd.get_world_transform_matrix(bprim).ExtractTranslation()
            if (bname not in _attached and abs(t[0] - cur[0]) < 0.55
                    and abs(t[1] - cur[1]) < 0.45 and 0.80 < t[2] < 1.15):
                _teleport_prim(bprim, (t[0] + dx, t[1] + dy, t[2]))
                carried.append(bname)
        holder["ok"] = True
        holder["msg"] = f"robot -> ({p.x:.3f}, {p.y:.3f}), carried {carried}"
    else:
        prim = stage.GetPrimAtPath(f"/World/models/{st.name}")
        if not prim.IsValid():
            raise ValueError(f"unknown model {st.name}")
        jp = _attached.pop(st.name, None)   # teleport implies force-detach
        if jp:
            stage.RemovePrim(jp)
        _teleport_prim(prim, (p.x, p.y, p.z), Gf.Quatd(o.w, Gf.Vec3d(o.x, o.y, o.z)))
        holder["ok"], holder["msg"] = True, f"{st.name} teleported"
    print(f"[set_entity_state] {holder['msg']}")


def process_attach_queue():
    while not _attach_q.empty():
        kind, req, done, holder = _attach_q.get()
        try:
            if kind == "set_state":
                _do_set_state(req, holder)
                done.set()
                continue
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
        if name == ROBOT_NAME:
            prim = base_link_prim   # live (physics-written); root Xform is stale
        else:
            prim = stage.GetPrimAtPath(f"/World/models/{name}")
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

# GUI viewpoint: same viewing direction as the hand-framed 2026-07-22 pose, but
# pulled back and recentred so the FULL 2 m shelf row + the AGV station band
# (x ~ -0.1..1.5) stay in frame while the base teleports between stations.
if not HEADLESS:
    from isaacsim.core.utils.viewports import set_camera_view
    set_camera_view(eye=[-2.75, -2.35, 2.85], target=[0.8, 0.5, 0.8])

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

# Per-joint drive gains from the mass matrix at the spawn pose. The importer's
# uniform 1e5 Nm/rad stiffness saturates the 1000 Nm force clamp on ~0.01 rad
# errors: bang-bang chatter, seen as wrist joints dragged at ~1 rad/s during
# fast moves and as grasp-seating variance that jams the pocket descend.
# Target 10 Hz bandwidth, zeta=2 (overdamped; velocity FF supplies the speed).
# Gains are Nm per RADIAN: a pi/180 "per-degree" conversion tried on 2026-07-22
# left the arm sagging 0.04-0.09 rad under load (57x too soft). Skip with
# --no-drive-tune to fall back to the imported uniform gains.
if "--no-drive-tune" not in sys.argv:
    try:
        view = art._articulation_view
        getM = (getattr(view, "get_mass_matrices", None)
                or getattr(view, "get_generalized_mass_matrices"))
        M = np.asarray(getM())[0]
        omega, zeta = 2 * math.pi * 10.0, 2.0
        for i_j in range(1, 7):
            jn = "joint%d" % i_j
            di = art.get_dof_index(jn)
            mii = float(M[di, di])
            drv = UsdPhysics.DriveAPI.Get(find_prim_named(robot_root, jn), "angular")
            drv.CreateStiffnessAttr(omega ** 2 * mii)
            drv.CreateDampingAttr(2 * zeta * omega * mii)
            print("[drive-tune] %s: Mii=%.3f kgm^2 -> k=%.0f Nm/rad c=%.0f Nms/rad"
                  % (jn, mii, omega ** 2 * mii, 2 * zeta * omega * mii))
    except Exception as e:  # keep imported gains rather than die at bring-up
        print("[drive-tune] skipped (%s); imported uniform gains kept" % e)

rclpy.init()
compat = GazeboCompat()
spin_thread = threading.Thread(
    target=rclpy.spin, args=(compat,), daemon=True)
spin_thread.start()

print("[isaac_sim] world up:", MODEL_NAMES)
frame = 0
while app.is_running():
    process_attach_queue()
    if _sim_paused:
        time.sleep(0.005)   # world frozen: no physics step, no clock advance
        continue
    _sim_time = world.current_time
    if frame % 15 == 0:  # 2 Hz pose refresh (30 fps loop) feeds model_states + odom TF
        refresh_pose_cache()
    world.step(render=True)
    for c in list(_teleport_checks):   # verify robot teleports a few frames on
        c[0] -= 1
        if c[0] <= 0:
            t = omni.usd.get_world_transform_matrix(base_link_prim).ExtractTranslation()
            ok = abs(t[0] - c[1]) < 0.02 and abs(t[1] - c[2]) < 0.02
            print(f"[teleport] base at ({t[0]:.3f}, {t[1]:.3f}), target "
                  f"({c[1]:.3f}, {c[2]:.3f}) -> {'OK' if ok else 'FAILED'}")
            _teleport_checks.remove(c)
    frame += 1

rclpy.shutdown()
app.close()
