# TODO

## Goal
Build an in-process mission DISPATCHER (`main.py`, minimal FSM) that
routes an MCS/AMR "stop + location(id)" message by location TYPE to a vision
preamble + the matching pick-place flow:
  - shelf id     -> vision (ArUco shelf frame + AI magazine detect, FUTURE)
                    -> hub_pick_place flow
  - wirebonder id-> vision (ArUco tag -> slot coords, DONE & validated)
                    -> wirebonder_pick_place flow
First make the wirebonder branch work END-TO-END (the validated AprilTag vision
drives the transfer, replacing hand-measured absolute coords; no precise AMR
parking). orchestrator.py is REFERENCE only -- its async/call_async model is for
later, once AMR/MCS bridge nodes exist. Build now: wirebonder branch + dispatcher.
Defer: shelf ArUco+AI vision, AMR/MCS bridges, real-robot arm action boundary
(action_move_server) + ToolDO gripper.

## Decisions (settled)
- Markers are AprilTag **36h11**, IDs 0/1, 30 mm. Use only **ID 0** (left
  column, above slots A/B). One tag gives full 6-DoF pose.
- Detection via `cv2.aruco` (`DICT_APRILTAG_36h11`) on `/d405/color/image_raw`,
  intrinsics from `/d405/color/camera_info`. No new dependency.
- Eye-in-hand: tag pose in `d405_optical_frame` -> `base_link` via existing TF.
- Capture viewpoint = the existing `go_to_hub()` pose (tag is in FOV when the
  AMR is parked). No new arm pose.
- Integration path **2 -> 3**: standalone diagnostic first, then import the
  same core function into `wirebonder_pick_place.py` (synchronous "look before
  you transfer"). ROS2 `/object_pose` topic deferred to the later
  object-detection stage.
- New module: `src/DOBOT_6Axis_ROS2_V4/wirebonder_vision.py`.
- Tag-0 model-frame pose (from `wirebonder/model.sdf`):
  `xyz=(-0.348,-0.1205,1.2)`, `rpy=(1.5708,0,0)`.

## Tasks

### 1. Vision core (pure, offline-testable)
- [x] `detect_tag(bgr, K, dist, tag_id=0, size=0.03) -> (rvec, tvec) | None`
      using `cv2.aruco` with `DICT_APRILTAG_36h11`.
- [x] Constant `T_MODEL_TAG` from the SDF tag-0 pose; `device_pose_in_base()`
      composing `T_base_model` from a detection + base<-optical TF.
- [x] `slots_in_base(T_base_model) -> {A,B,C,D: (xyz, quat)}` using `SLOT_OFFSET`
      (magazine centres) so the existing magazine-centre machinery can consume it.
- [x] `__main__` self-check (assert, no framework): SE3 round-trip, slot centres
      match the `DEVICES`+`SLOT_OFFSET` world poses, and a live AprilTag detection
      smoke. Passes.

### 2. Standalone diagnostic node (Approach 2 - prove the transform)
- [x] Minimal rclpy node (`wirebonder_vision_node.py`): subscribe image +
      camera_info, tf2 listener for `base_link <- d405_optical_frame` (+ `<- odom`).
- [x] 1 Hz tick: detect ID 0, `estimatePoseSingleMarkers`, TF to `base_link`,
      compute the 4 slot centres in `base_link`.
- [x] Print side-by-side the `DEVICES`+TF-derived (odom) slot centres with a
      per-slot mm distance, to validate the transform + tag frame convention by eye.
- [x] Pinned the frame convention: `R_CV_TO_SDF = Rz(-90)`, and fixed a
      composition bug (the tag's `TAG0_XYZ` offset within the model was dropped).
      Offline-verified against captured sim data: error 1000-1500 mm -> ~13 mm
      (residual is a constant ~13 mm depth bias = monocular-tag range noise).
- [x] Live re-run confirmed: all 4 slots 12-15 mm, stable; residual is the
      expected constant ~13 mm depth (y) bias, x/z within 1 mm.

### 3. Integrate into pick/place (TOPIC boundary, not in-process import)
- [x] ENV-FORCED DECISION: `.venv` has no pip/ensurepip and its numpy2 breaks the
      system cv2, so cv2 cannot run in `.venv`. -> vision (cv2) stays in the system
      env and PUBLISHES the device pose; the `.venv` planner SUBSCRIBES. This is the
      topic boundary (Approach 1), which matches system_architecture.md and is the
      same node the AI detector + dispatcher reuse. No `.venv` surgery.
- [x] `wirebonder_vision_node.py` publishes the device pose in ODOM on
      `/vision/device_pose` (PoseStamped), alongside the diagnostic print.
- [x] `wirebonder_pick_place.py` subscribes; `refresh_device_pose(node, device)`
      waits for a FRESH pose and updates `DEVICES[device]` (x,y,z,yaw). Returns
      False on timeout.
- [x] Re-anchored the measured `SLOT_WORLD` TCP waypoints into the device MODEL
      frame (`SLOT_LOCAL`, baked from `OLD_DEVICE_POSE`); `slot_target` composes
      them with the LIVE device pose. `slot_world`/`slot_flange_seat` already read
      `DEVICES`, so they follow vision automatically. Tuned nudges kept intact.
      Verified: round-trip exact, waypoints track device translation/yaw.
- [x] Fail safe: each transfer refreshes the device(s) at the hub viewpoint and
      ABORTS (skips) if no fresh `/vision/device_pose` arrives -- never runs on the
      stale placeholder.
- [x] Live run: vision node + planner, confirm a transfer completes on tag-derived
      poses with the AGV parked off the measured spot. CONFIRMED via main.py
      (`wb1` -> base->slotA).

### 4. Integrated mission dispatcher (`main.py`, minimal FSM)
- [x] Rename `hub_pick_place.py` -> `shelf_pick_place.py` (shelf->base flow);
      updated its own docstring + live cross-refs (cbirrt_pick_place.py comment,
      docs/ARCHITECTURE.md, gripper_change_checklist.md). Historical decision docs
      (request.md, moveit_vs_tcpip_decision.md) left untouched.
- [x] Location registry: `id -> (type, *params)`. One transfer per trigger.
      `wb1 -> ('wirebonder', 'wb1', SEQUENCES['1'])`, `shelf -> ('shelf',)`.
- [x] Dict-router FSM: `run_mission` logs IDLE -> LOCATE -> PICK/PLACE -> REPORT
      -> IDLE; routes by location type. In-process worker-thread model
      (MultiThreadedExecutor + spin thread), NOT orchestrator.py's async executor.
- [x] Stand-in trigger: `input()` location-id loop; the AMR/MCS bridge replaces
      that seam (marked in main()).
- [x] wirebonder branch: LOCATE = wb.capture_device (AprilTag refresh at the
      capture viewpoint), then wb.transfer(src, dst) for that device.
- [x] shelf branch: wired to shelf.pick_place_one_box (walks node.box_idx);
      LOCATE is a STUB returning the hardcoded shelf frame (AI detect deferred).
- [x] Imports wirebonder_pick_place + shelf_pick_place AS LIBRARIES (reuse their
      sequence funcs); one shared hub (same HUB_TCP + pocket-family seed). No dup.

### 5. Verify end-to-end
- [x] In sim, park the AMR roughly facing wb1 (NOT the exact measured spot), run
      `main.py`, trigger the wb1 location, confirm a transfer (base ->
      slot A) completes using the tag-derived poses. DONE.
- [ ] Trigger the shelf location, confirm shelf_pick_place runs (stub vision).
      PENDING: shelf raised in sim -> retune `SHELF_BOXES` z in shelf_pick_place.py
      to the new shelf height before this passes.

### 6. System structure doc (after Task 5)
- [ ] Write a markdown doc describing the overall system structure and the
      related files (vision layer, dispatcher, the two flows, cr7_pnp library,
      real-robot seams). Supersedes/updates docs/ARCHITECTURE.md as needed.

### 7. Real-robot path (DEFERRED -- each is a clean swap at a marked seam)
Sim drives the arm via a `FollowJointTrajectory` action to
`/cr7_group_controller/follow_joint_trajectory`; `action_move_server.py` already
advertises THAT SAME action (DOBOT_TYPE=cr7 -> ServoJ), so the arm boundary is
already in place. The remaining work:
- [ ] Arm backend: run `action_move_server.py` instead of the Gazebo controller
      (same action name, ZERO planner change). Then HARDEN action_move_server:
      honour trajectory timing/velocities (drop the fixed `sleep(0.18)`+`t=0.2`),
      AWAIT each `ServoJ` so "succeeded" means "arrived", and add
      EnableRobot/ClearError + mode handling in the execute path.
- [ ] Gripper: re-implement `cr7_pnp.control_gripper(positions)` to map
      OPEN/CLOSE onto the real tool (`SetToolMode` / `SetHoldRegs` / ToolDO),
      replacing the sim `/gripper_controller` joint-trajectory. One-function swap.
- [ ] Grasp attach: make `attach_box` / `attach_box_to_magazine` no-ops on real
      hardware (physical grasp); KEEP `attach_box_collision` (planner phantom).
- [ ] Vision: point `wirebonder_vision_node` at the physical D405 via
      `realsense2_camera` (same `/d405/color/*` topics, no node change). Add the
      two real knobs: D405 intrinsics+distortion calibration (the `dist` param is
      already the hook) and hand-eye extrinsic (camera->flange TF) calibration.
- [ ] AMR/MCS bridge: replace the `main.py` `input()` trigger with a node
      that receives the TCP/IP "stop + location(id)" message and calls
      `run_mission(node, loc_id)`. SERIALISE: enqueue commands, dispatch only in
      IDLE (single-arm/blocking planner -- never two missions at once). MCS report
      publishes at the `[REPORT]` seam.

## Notes / risks
- AprilTag frame convention (z out of face, x right, y down) vs. the SDF
  `rpy=(1.5708,0,0)` placement is the main source of sign errors - task #2's
  side-by-side print is the guard.
- Gazebo pinhole camera has zero distortion; `dist` is zeros in sim but keep the
  parameter so the real D405 calibration plugs in later (hardware calib knob).
- This becomes the Vision Layer's marker stage in `system_architecture.md`; the
  `/object_pose` topic is for the later continuous object-detection stage.
- `wirebonder_pick_place.py` slot flow currently mixes two coordinate sources
  (`SLOT_OFFSET`-derived vs. measured `SLOT_WORLD`); #3 unifies them onto the tag.
- Code comments / console output in English (CLAUDE.md). TODO.md text in English
  here for the build log; ask if you want it in Korean.
- Dispatcher is a minimal dict-router FSM (architecture doc concludes FSM > BT for
  this POC). orchestrator.py stays REFERENCE: its SingleThreadedExecutor +
  call_async model collides with the blocking in-process pinocchio planner, so it
  only fits once the arm/vision live behind action/service boundaries.
- DEFERRED (real-robot path, not now): broken out into **Task 7** above (arm
  backend via action_move_server, gripper services, attach no-op, real D405 +
  calibration, AMR/MCS bridge). Still deferred and not in Task 7: shelf ArUco+AI
  magazine detection (the shelf LOCATE stub). Each is a clean swap at a marked seam.
