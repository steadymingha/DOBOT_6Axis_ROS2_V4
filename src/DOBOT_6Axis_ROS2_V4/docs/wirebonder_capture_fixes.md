# Wirebonder vision capture & base-pick — problems and fixes

Debug log for the `base -> slot A` transfer (`sequences/wirebonder_pick_place.py`
+ `vision/wirebonder_vision*.py`). Each entry: **symptom → root cause → fix**.
Ordered from the original single-tag capture through to a working full cycle.

---

## 1. Single-tag pose flip (27° yaw → 300 mm error)

**Symptom.** With a single AprilTag read, the slot approach drove sideways under
the device and collided. Vision reported the device yaw as `-0.465 rad (-27°)`.

**Root cause.** A single small (30 mm) planar tag has **two** almost-equal
IPPE pose solutions. Near fronto-parallel OpenCV silently picks the *flipped*
one (the "ambiguity flip"). The tag sits ~1 m above the low slots, so a 27° yaw
error is levered into a ~300 mm error at the approach waypoint. `median` over
frames can't help — the flip is consistent, not noise.

**Fix.** `vision/wirebonder_vision.py`:
- `detect_tag` now returns **both** IPPE_SQUARE solutions (via `solvePnPGeneric`).
- `device_pose_in_base` keeps the **upright** one (model z-axis most aligned with
  base z, `T[2,2]` max) — the device is known vertical, so the tilted twin is
  rejected. The reflection is about the camera line-of-sight plane, not vertical,
  so exactly one solution is upright (as long as the view isn't purely horizontal;
  the capture pitch guarantees a vertical component).

Result: yaw dropped `-0.465 → -0.136` rad, then to ~0 after the steps below.

---

## 2. Range error (parked farther → placed short)

**Symptom.** After the flip fix, moving the AGV farther from the device made the
place fall short. Error changed with parking **distance**.

**Root cause.** Monocular scale is the weak axis: a single tag's estimated range
∝ apparent size, poorly conditioned for a 30 mm tag. This is a geometry
weakness, not noise — in the noiseless sim it is deterministic.

**Fix (motion stereo).** Take **two** views from different camera *positions* and
triangulate the tag corners with a known baseline (the arm's motion, from FK):
- `vision/wirebonder_vision.py`: `detect_tag_corners`, `triangulate_rays`,
  `_kabsch`, `device_pose_from_two_views` (triangulate the 4 corners across the
  two views, rigid-fit the tag frame, compose to the device pose).
- `vision/wirebonder_vision_node.py`: `/vision/capture` (Int32) — `0` resets,
  `1` grabs a view `(T_odom_optical, corners, K)`; on the 2nd grab it triangulates
  and republishes that one solved pose every tick.
- `sequences/wirebonder_pick_place.py`: `capture_device` drives two poses (view A,
  view B) and pings `/vision/capture` at each.

Note: the baseline must be **perpendicular** to the camera→tag line (move sideways
/ up, not toward/away) or there is no parallax.

---

## 3. View B move: singularity / jitter

**Symptom.** The straight servo from view A to a far view B aborted
`singular (sigma_min≈0)` at ~370 mm of a ~0.78 m diagonal, and the arm jittered
approaching it.

**Root cause.** A long straight Cartesian servo forces the arm through a
near-singular configuration (high joint velocities to hold the line). The jog had
reached that pose axis-by-axis, avoiding the singularity.

**Fix.** `cr7_pnp/node.py`: added `go_to_config(joints)` — a joint-space RRT to an
arbitrary config (like `go_to_hub`), smooth and singularity-free. `tools/jog_tcp.py`
now prints the tool quaternion **and** joints so a jogged view B can be captured as
a joint config (`CAPTURE_B_JOINTS`). Only the two endpoints need to frame the tag;
the path between is free.

---

## 4. Grab landed on a blank frame (`view dropped`)

**Symptom.** Both captures ran but the node logged `tag or TF missing; view
dropped` and the planner timed out `no /vision/device_pose in 6.0s`. Yet the live
detection print showed a clean tag (11–19 mm) once settled.

**Root cause.** The grab fired ~0.5 s after the move "finished", but the arm was
still settling (image showed the tag out of frame at that instant). The
`Trajectory execution finished` signal came early — see the environmental note
below.

**Fix.** `sequences/wirebonder_pick_place.py`: bumped the pre-grab settle
`0.5 → 2.0 s` (matching the observed ~2 s settle), so the grab lands on a
detected frame. Single-view detection is now diagnostic-print only; the published
pose is the two-view solve.

---

## 5. Constant vision bias → wrong waypoints (slot side)

**Symptom.** The two-view read was clean and deterministic
(`x=2.359 y=0.531 z=0.007 yaw=+0.017`, **spread 0/0/0 mm**) but the slot approach
still clipped. Offset vs ground truth `(2.35, 0.5, 0, 0)` was a constant
`(+9, +31, +7) mm, +1°`.

**Root cause.** `spread 0/0/0` (noiseless sim) means this is a **constant
calibration bias** (extrinsic/intrinsic), NOT noise or conditioning — so more
views / bigger tags cannot remove it. The re-anchoring mixed frames: the jogged
waypoints' reference `OLD_DEVICE_POSE` was the **true** pose while runtime used the
**vision** (biased) pose, leaving the ~31 mm bias uncancelled and shifting the
waypoints into the tight front-load slot.

**Fix.** `sequences/wirebonder_pick_place.py`: set `OLD_DEVICE_POSE` to the
**vision** reading `(2.359, 0.531, 0.007, 0.017)`, not the true pose. Re-anchoring
is `T(DEVICES_now) @ inv(T(OLD)) @ SLOT_WORLD`; with both ends in the same
(biased) measurement system it cancels, and the waypoints reproduce the hand-jogged
values exactly. Deterministic sim = the bias is identical at jog time and runtime.

---

## 6. Base pick landed beside the box (offset), only in vision mode

**Symptom.** `--no-vision` picked the base box correctly; in vision mode the
gripper approached offset in +y and never grasped. Suspected the bias change — it
was not.

**Root cause.** Base pick is **not** vision-driven (fixed base_link pocket coords;
it never reads `OLD_DEVICE_POSE`/`DEVICES`). Its approach `base_hover_delta` is a
**pure translation from the hub**, valid only if the arm starts at the exact
`hub_q`. `--no-vision` leaves the arm at the exact hub (from `main`'s
`go_to_hub`); vision runs the capture and `replay_reverse` only landed the arm
*near* the hub, so the residual became the pick offset.

**Fix.** `sequences/wirebonder_pick_place.py`: after `replay_reverse`, call
`node.go_to_hub()` to snap to the exact `hub_q` (a tiny, box-safe move) before the
menu. Vision mode now matches `--no-vision`.

---

## 7. Return from capture swept the base box

**Symptom.** Returning from view B to the hub, the arm hit `box_l2c` sitting on
the base pocket.

**Root cause.** The base box (and the device) are **not** in the pinocchio
planning scene (only arm + gripper + cube + AGV + shelf boards + the carried-box
phantom are). A fresh `go_to_hub` RRT is unaware of the base box and swings through
it.

**Fix.** Retrace the proven outbound path in reverse instead of re-planning:
- `cr7_pnp/node.py`: `execute_trajectory` now records joint waypoints while a
  `capture()` is active (previously only `execute_path` did), so RRT / `go_to_config`
  moves get recorded.
- `sequences/wirebonder_pick_place.py`: `capture_device` wraps the outbound
  `hub → A → B` in `node.capture(...)` and returns via `node.replay_reverse(fwd)`
  (`B → A → hub`). The outbound path was collision-free, so its reverse is too.
  (Same reverse-replay trick `place()` uses for the front-load slot.)

---

## Environmental note (not a code bug)

Logs showed `more than one action server for
'/cr7_group_controller/follow_joint_trajectory'`. Two action servers advertise the
same action (a duplicate/stale node or a second launch), which makes
`Trajectory execution finished` fire early and the arm twitch past it. Not from any
code change here. Check with
`ros2 action info /cr7_group_controller/follow_joint_trajectory -t`; kill the stale
server and relaunch once.

## Parked for later

Capture trajectory → all-`linear_servo` (deterministic, box-safe, exact hub
return). Feasible: the d405 looks **forward** along the gripper reach axis with a
tunable pitch (`d405_joint` in `cra_description/urdf/arm_on_mpo700.urdf.xacro`), so
a fixed-orientation capture can frame the tag — but view A and view B must then be
re-jogged as a set sharing that one orientation, with view B a short perpendicular
baseline (avoiding the singularity).
