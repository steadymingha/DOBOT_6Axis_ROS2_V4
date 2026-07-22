#!/usr/bin/env python3
"""Pick a box off shelf tier-1 and place it on the AGV base, in the MuJoCo scene.

A minimal self-contained demo -- no ROS, no pinocchio, no motion planner. It
uses MuJoCo's own Jacobian IK to move the gripper through waypoints, and a weld
equality (toggled at grasp/release) as the "gripper attach", the same idea as
Gazebo's link-attacher. Runs gravity-off so the free box stays put until welded.

    ./run_pick_place.sh              # animate in the viewer
    ./run_pick_place.sh --no-view    # headless, just verify the box moved

This is a starting point for layering real planning (cbirrt/OMPL) on top: swap
the straight-line waypoints for a planned joint path and keep the same execution
loop.
"""
import sys
import time
from pathlib import Path

import numpy as np
import mujoco

HERE = Path(__file__).resolve().parent
SCENE = HERE / "scene.xml"

PICK_BOX = "box_3_0"          # shelf tier-1 box at (0.71, 0.50, 1.29)
EE_BODY = "gripper_base_link"

# Startup camera for the viewer (tweak to taste).
CAM = dict(lookat=(0.60, 0.30, 1.05), distance=2.4, azimuth=150, elevation=-25)

# The gripper hangs below the wrist: its flange axis is local +z, so fingers
# point down when local +z stays world-up (yaw-only orientation -- flipping z
# put the fingers UP). Grasp yaw must be 0 (pad closing axis = world x): the
# pads open ~113mm/close 73mm, so they can only wrap the box's 81mm x-width,
# not its 140mm y-width. At yaw 0 the pads land at x 0.664/0.750 -- exactly
# straddling box_3_0's faces (0.669..0.750).
DOWN_QUAT = np.array([1.0, 0.0, 0.0, 0.0])
# Grasp center (between the finger pads) in the gripper frame is ~(0.143, 0,
# 0.08) (pads ~0.14 off the tool axis, blender measurement). At yaw 0 the
# world offset is the same vector.
GRASP_C = np.array([0.143, 0.0, 0.08])


def ee_for(gx, gy, gz):
    """EE target that puts the grasp center at world (gx,gy,gz), gripper down."""
    return [gx - GRASP_C[0], gy - GRASP_C[1], gz - GRASP_C[2]]


# Waypoints are grasp-center points. box_3_0: center z=1.29, top z=1.408.
# Pick mirrors sequences/shelf_pick_place.py: approach the shelf with the jaw
# UNtwisted (along the insertion axis), slide in above the box, then a pure-J6
# twist swings the jaw over the box, then descend + close + grasp. Return backs
# out twisted, like the real twisted replay.
TWIST = np.pi / 2                                # J6 in-gap twist (GRIPPER_YAW_TWIST)
PREGRASP_BACK = 0.25                             # start this far in front (-y)
BOX_Z = 1.29                                     # pick box center height
GRASP_GZ = 1.331     # pads wrap ~120mm below the box top; jaw bottom stays
                     # 16mm above the tier-1 shelf plate (1.172), so this is
                     # about as deep as the shelf allows
HOVER = ee_for(0.71, 0.50, 1.466)                # over the box, insert height
GRASP = ee_for(0.71, 0.50, GRASP_GZ)
# The box mesh origin is NOT its centroid: the mesh bottom is 70mm below the
# body origin (measured by drop test), so flush-on-pad center = 0.963 + 0.070.
BOX_BOTTOM = 0.070
BOX_PLACE = np.array([0.498, 0.185, 0.963 + BOX_BOTTOM])   # resting on the pad
# The pad is 236x80mm with its long axis along x, the box footprint is 81x140mm,
# so the box goes down rotated 90 deg from its shelf orientation (the real
# sequence's PLACE_YAW). Carry/place therefore use a +90 yaw EE orientation,
# which also rotates the grasp-center offset to +y.
PLACE_QUAT = np.array([0.70710678, 0.0, 0.0, 0.70710678])


def ee_for_pad(gx, gy, gz):
    """EE target for the +90-yaw place orientation (offset spun to +y)."""
    return [gx, gy - GRASP_C[0], gz - GRASP_C[2]]


# Held offset = grasp center above box center; +5mm so the box visibly drops
# onto the pad at release and gravity seats it.
PLACE = ee_for_pad(0.498, 0.185, BOX_PLACE[2] + (GRASP_GZ - BOX_Z) + 0.005)

# Finger cmd -> pad gap: 0.02 -> 106mm (clears the 81mm box), -0.008 -> 78mm
# (light pinch). The old -0.05 closed to 36mm, slicing 45mm INTO the box.
GRIP_OPEN, GRIP_CLOSED = 0.02, -0.008
SEED = np.array([0.0, 0.5, -1.0, 0.0, -0.8, 0.0])   # nominal elbow-down pose


def build_model():
    """Load the scene, make the pick box a free body, and add the grasp weld."""
    if not SCENE.exists():
        sys.exit("scene.xml missing -- run ./run.sh --no-view first")
    spec = mujoco.MjSpec.from_file(str(SCENE))
    spec.body(PICK_BOX).add_freejoint()        # box must be dynamic to be moved
    weld = spec.add_equality()
    weld.type = mujoco.mjtEq.mjEQ_WELD
    weld.objtype = mujoco.mjtObj.mjOBJ_BODY
    weld.name1 = EE_BODY
    weld.name2 = PICK_BOX
    weld.active = False                        # engaged at grasp

    # Real physics for the box: gravity on, but contacts ONLY as explicit pairs
    # against two support primitives. Blanket contacts are unusable here: MuJoCo
    # collides meshes by convex hull, and the one-piece shelf mesh's hull is a
    # solid block that would eject the box (same for the C-shaped jaw around it).
    spec.option.gravity = [0.0, 0.0, -9.81]
    for g in spec.geoms:
        g.contype = 0
        g.conaffinity = 0
    boxgeom = spec.body(PICK_BOX).geoms[0]
    boxgeom.name = "pick_box_geom"
    invisible = [0, 0, 0, 0]
    # Support tops match the box's resting height (origin 70mm above the mesh
    # bottom): tier-1 plate top 1.220 keeps the box exactly at its spawn height.
    spec.worldbody.add_geom(name="tier1_plate", type=mujoco.mjtGeom.mjGEOM_BOX,
                            pos=[0.80, 0.50, 1.218], size=[0.45, 0.12, 0.002],
                            rgba=invisible)                 # top z = 1.220
    spec.worldbody.add_geom(name="agv_pad", type=mujoco.mjtGeom.mjGEOM_BOX,
                            pos=[0.498, 0.185, 0.961], size=[0.13, 0.05, 0.002],
                            rgba=invisible)                 # top z = 0.963
    for support in ("tier1_plate", "agv_pad"):
        pair = spec.add_pair()
        pair.geomname1 = "pick_box_geom"
        pair.geomname2 = support
    return spec.compile()


class Arm:
    """Jacobian (damped least-squares) IK + servo command helpers."""

    def __init__(self, model, data):
        self.m, self.d = model, data
        self.ee = mujoco.mj_name2id(model, mujoco.mjtObj.mjOBJ_BODY, EE_BODY)
        jid = [mujoco.mj_name2id(model, mujoco.mjtObj.mjOBJ_JOINT, f"joint{k}") for k in range(1, 7)]
        self.qadr = [model.jnt_qposadr[j] for j in jid]
        self.dof = [model.jnt_dofadr[j] for j in jid]
        # Clamp to +-180 deg for clean, human-looking poses.
        self.lo = np.maximum([model.jnt_range[j][0] for j in jid], -np.pi)
        self.hi = np.minimum([model.jnt_range[j][1] for j in jid], np.pi)
        self.grip = mujoco.mj_name2id(model, mujoco.mjtObj.mjOBJ_ACTUATOR, "gripper_finger_joint")
        self.act = [mujoco.mj_name2id(model, mujoco.mjtObj.mjOBJ_ACTUATOR, f"joint{k}") for k in range(1, 7)]
        self.weld = 0  # single equality
        self.box = mujoco.mj_name2id(model, mujoco.mjtObj.mjOBJ_BODY, PICK_BOX)

    def engage_weld(self):
        """Freeze the box at its CURRENT pose relative to the gripper, then weld.
        Without setting relpose, the weld would snap the box to a default frame."""
        if self.d.eq_active[self.weld]:
            return
        mujoco.mj_forward(self.m, self.d)
        R1 = self.d.xmat[self.ee].reshape(3, 3)
        relpos = R1.T @ (self.d.xpos[self.box] - self.d.xpos[self.ee])
        q1inv = np.zeros(4); mujoco.mju_negQuat(q1inv, self.d.xquat[self.ee])
        relquat = np.zeros(4); mujoco.mju_mulQuat(relquat, q1inv, self.d.xquat[self.box])
        self.m.eq_data[self.weld][:3] = 0.0          # anchor at box origin
        self.m.eq_data[self.weld][3:6] = relpos
        self.m.eq_data[self.weld][6:10] = relquat
        self.m.eq_data[self.weld][10] = 1.0          # torquescale
        self.d.eq_active[self.weld] = 1

    def release_weld(self):
        self.d.eq_active[self.weld] = 0
        # Park the box: with gravity/contacts off nothing else stops the residual
        # velocity it picked up while welded, so it would drift away forever.
        adr = self.m.body_dofadr[self.box]
        self.d.qvel[adr:adr + 6] = 0.0

    def fk(self, q):
        """EE (pos, quat) at joints q. Restores qpos afterwards."""
        qpos0 = self.d.qpos.copy()
        for i, a in enumerate(self.qadr):
            self.d.qpos[a] = q[i]
        mujoco.mj_forward(self.m, self.d)
        pos, quat = self.d.xpos[self.ee].copy(), self.d.xquat[self.ee].copy()
        self.d.qpos[:] = qpos0
        mujoco.mj_forward(self.m, self.d)
        return pos, quat

    def ik(self, target, seed, quat=None, iters=1000, tol=8e-4):
        """6D DLS IK: EE body origin at target, orientation quat (DOWN_QUAT).
        Restores qpos afterwards -- the iteration scratches the live state, and
        leaving it would teleport the arm to the goal before the animation."""
        want = DOWN_QUAT if quat is None else quat
        qpos0 = self.d.qpos.copy()
        q = seed.copy()
        J = np.zeros((6, self.m.nv))
        neg, qerr, rerr = np.zeros(4), np.zeros(4), np.zeros(3)
        for _ in range(iters):
            for i, a in enumerate(self.qadr):
                self.d.qpos[a] = q[i]
            mujoco.mj_forward(self.m, self.d)
            perr = np.asarray(target) - self.d.xpos[self.ee]
            mujoco.mju_negQuat(neg, self.d.xquat[self.ee])
            mujoco.mju_mulQuat(qerr, want, neg)
            mujoco.mju_quat2Vel(rerr, qerr, 1.0)     # world-frame rotation error
            if np.linalg.norm(perr) < tol and np.linalg.norm(rerr) < 0.02:
                break
            mujoco.mj_jacBody(self.m, self.d, J[:3], J[3:], self.ee)
            Jr = J[:, self.dof]
            err = np.concatenate([perr, rerr])
            dq = Jr.T @ np.linalg.solve(Jr @ Jr.T + 1e-2 * np.eye(6), err)
            q = np.clip(q + np.clip(dq, -0.1, 0.1), self.lo, self.hi)
        self.d.qpos[:] = qpos0
        mujoco.mj_forward(self.m, self.d)
        return q, float(np.linalg.norm(perr))

    def command(self, q_arm, grip_closed):
        for i, a in enumerate(self.act):
            self.d.ctrl[a] = q_arm[i]
        self.d.ctrl[self.grip] = GRIP_CLOSED if grip_closed else GRIP_OPEN


def build_plan(arm):
    """Joint-space plan (name, q_goal, grip, weld) mirroring shelf_pick_place:
    untwisted insert, in-gap J6 twist, descend, grasp, twisted back-out."""
    q_hover, r_hover = arm.ik(np.array(HOVER), SEED)     # twisted (grasp) yaw
    q_ins = q_hover.copy()
    q_ins[5] -= TWIST                # untwist: jaw along the insertion axis (-y)
    p_ins, quat_ins = arm.fk(q_ins)
    q_pre, r_pre = arm.ik(p_ins - [0, PREGRASP_BACK, 0], SEED, quat=quat_ins)
    q_grasp, r_grasp = arm.ik(np.array(GRASP), q_hover)
    q_back, r_back = arm.ik(np.asarray(HOVER) - [0, PREGRASP_BACK, 0], q_hover)
    q_carry, r_carry = arm.ik(np.array(ee_for_pad(0.55, 0.185, 1.40)), q_back,
                              quat=PLACE_QUAT)
    q_place, r_place = arm.ik(np.array(PLACE), q_carry, quat=PLACE_QUAT)
    q_ret, r_ret = arm.ik(np.array(ee_for_pad(0.498, 0.185, 1.40)), q_place,
                          quat=PLACE_QUAT)
    return [
        ("pre-grasp", q_pre,   r_pre,   False, False),
        ("insert",    q_ins,   0.0,     False, False),   # straight -> above the box
        ("twist",     q_hover, r_hover, False, False),   # pure J6, jaw over the box
        ("descend",   q_grasp, r_grasp, False, False),
        ("close",     q_grasp, 0.0,     True,  False),   # visible pinch first
        ("lift",      q_hover, 0.0,     True,  True),    # weld engages here

        ("back-out",  q_back,  r_back,  True,  True),    # twisted retreat, box in jaw
        ("carry",     q_carry, r_carry, True,  True),
        ("place",     q_place, r_place, True,  True),
        ("release",   q_place, 0.0,     False, False),   # open + unweld
        ("retract",   q_ret,   r_ret,   False, False),
    ]


def run(view=True):
    model = build_model()
    data = mujoco.MjData(model)
    arm = Arm(model, data)

    # Start settled at the seed pose.
    for i, a in enumerate(arm.qadr):
        data.qpos[a] = SEED[i]
    arm.command(SEED, False)
    mujoco.mj_forward(model, data)

    viewer = None
    if view:
        from mujoco import viewer as mj_viewer
        viewer = mj_viewer.launch_passive(model, data)
        viewer.cam.lookat[:] = CAM["lookat"]
        viewer.cam.distance = CAM["distance"]
        viewer.cam.azimuth = CAM["azimuth"]
        viewer.cam.elevation = CAM["elevation"]

    dt = model.opt.timestep

    def segment(q_goal, grip, weld_on, seconds=1.5):
        """Ramp the servo setpoint from the current arm pose to q_goal."""
        q_now = np.array([data.qpos[a] for a in arm.qadr])
        steps = max(1, int(seconds / dt))
        if weld_on:
            arm.engage_weld()
        else:
            arm.release_weld()
        for s in range(steps):
            q_cmd = q_now + (q_goal - q_now) * (s + 1) / steps
            arm.command(q_cmd, grip)
            mujoco.mj_step(model, data)
            if viewer is not None:
                viewer.sync()
                time.sleep(dt)

    box_bid = mujoco.mj_name2id(model, mujoco.mjtObj.mjOBJ_BODY, PICK_BOX)
    print(f"box start: {np.round(data.xpos[box_bid], 3)}")
    for name, q_goal, resid, grip, weld_on in build_plan(arm):
        print(f"[{name:9}] IK resid {resid*1000:4.1f}mm  weld={'on ' if weld_on else 'off'}")
        segment(q_goal, grip, weld_on)
    end = data.xpos[box_bid]
    # Tilt = angle of the box z-axis off vertical (yaw is not a tilt).
    tilt = np.degrees(np.arccos(np.clip(data.xmat[box_bid].reshape(3, 3)[2, 2], -1, 1)))
    print(f"box end:   {np.round(end, 3)}  tilt {tilt:.1f} deg  (target ~ {BOX_PLACE})")
    assert np.linalg.norm(end - BOX_PLACE) < 0.03, "box did not land on the AGV pad"
    assert tilt < 10, "box is tilted"

    if viewer is not None:
        print("done -- close the window to exit")
        while viewer.is_running():
            viewer.sync()
            time.sleep(0.02)


if __name__ == "__main__":
    run(view="--no-view" not in sys.argv)
