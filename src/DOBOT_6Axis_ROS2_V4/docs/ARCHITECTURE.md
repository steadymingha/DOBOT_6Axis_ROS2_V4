# Architecture Overview — CR7 Pick-and-Place

> **Scope**: `cr7_pnp/` library + sequence scripts (`shelf_pick_place.py`, `cbirrt_pick_place.py`, device sequences)
> **Robot**: DOBOT CR7 on MPO-700 AGV, Blender fixed-jaw gripper (mounted on Link6)
> **Stack**: ROS 2 Humble · pinocchio · coal (HPP-FCL) · Gazebo Classic + link-attacher
>
> **No MoveIt at runtime.** IK and collision are solved in-process with pinocchio
> (`cr7_pnp/model.py`), so `/compute_ik` and `/check_state_validity` are not used.

---

## 1. Component Map

```
┌─────────────────────────────────────────────────────────────────────┐
│  cr7_pnp/   (self-contained motion library — no external script deps) │
│                                                                       │
│  node.py     CR7Node ─► CBiRRTPickPlace ─► HubPickPlace  (ROS 2 node) │
│     ├─ /joint_states subscriber                                       │
│     ├─ FollowJointTrajectory action client  (arm + gripper)           │
│     ├─ /ATTACHLINK · /DETACHLINK  (link-attacher)                     │
│     ├─ TF (odom→base_link, gripper, AGV root)                         │
│     ├─ pinocchio IK + collision         ◄── model.py                  │
│     └─ CBiRRT plan + Cartesian servo    ◄── cbirrt.py                 │
│                                                                       │
│  model.py    ReachabilityModel  — pinocchio FK / IK / self+scene coll │
│  cbirrt.py   ConstrainedPlanner — CBiRRT plan() + linear_path() servo │
│  geometry.py pure helpers (quat, pose_at, trigger) + tuned constants  │
└─────────────────────────────────────────────────────────────────────┘
        ▲ import only
        │
┌───────┴───────────────── sequence scripts (main + config) ───────────┐
│  shelf_pick_place.py     shelf → base pockets (hub-and-spoke, CBiRRT)   │
│  cbirrt_pick_place.py  segmented shelf→base DEMO (pre-hub reference)   │
│  wirebonder_pick_place.py  base pocket → wirebonder H_L (linear+RRT) │
│  spawn_device_markers.py  Gazebo markers at device magazine slots     │
└──────────────────────────────────────────────────────────────────────┘

External processes (must be running):
  Gazebo Classic ──► /joint_states, trajectory + gripper controllers,
                     /ATTACHLINK, /DETACHLINK, /spawn_entity
  robot_state_publisher ──► TF (base_link, odom, gripper_base_link)
```

The library copied the runtime parts of the former `test_w_gripper.py`,
`constrained_cbirrt.py` and `reachability_map.py` and now stands alone; those
first two files were deleted, `reachability_map.py` survives only as the offline
map builder used by `deploy_optimizer.py`.

---

## 2. Class Hierarchy (`cr7_pnp/node.py`)

```
rclpy.Node
  └── CR7Node                      ROS plumbing: joint states, gripper,
        │                          link-attacher, FollowJointTrajectory,
        │                          plan_rrt() (free joint-space RRT)
        │
        └── CBiRRTPickPlace        pinocchio engines + motion primitives
              │   setup_planner()        → ConstrainedPlanner + 2× ReachabilityModel
              │   compute_ik_ordered()   → nearest-branch, collision-gated IK
              │   is_state_valid()       → pinocchio whole-robot collision
              │   move_constrained()     → tilt-held CBiRRT carry
              │   move_to_pose_ref()     → free RRT to a chosen IK branch
              │   linear_servo()         → straight Cartesian-Jacobian servo
              │   rotate_j6() / move_single_joint()  → single-joint moves
              │   update_shelf_collision()           → shelf boards via TF
              │
              └── HubPickPlace     hub routing + carrying
                    init_hub() / go_to_hub()         → tool-down standby waypoint
                    attach_box_collision()/detach... → carried-box phantom
                    plan_spoke() / preflight_linear()→ CBiRRT spoke + no-motion check
                    capture()/replay_reverse()/rev() → record & reverse-replay
                    gripper_x_in_base_fk()           → predict jaw axis by FK
                    attach_box_to_magazine()         → fix placed box to the AGV
```

`HubPickPlace` is **the** reusable node; every sequence instantiates it. A
linear-only device sequence simply uses `linear_servo` + `move_to_pose_ref` and
skips the CBiRRT spoke helpers.

---

## 3. IK & Collision — `cr7_pnp/model.py` (`ReachabilityModel`)

Two instances are built in `setup_planner()`:

| Instance | URDF | Purpose |
|----------|------|---------|
| `self.collision` | `cr7_on_mpo700` (arm + cube platform + MPO-700 AGV + gripper) | `is_collision_free()` — self, cube/AGV, shelf boards, carried-box phantom |
| `self.ik_model` | `cr7_robot` (arm only) | `inverse_kinematics()` — damped-least-squares CLIK at the TCP |

```
xacro → /tmp/cr7_*_model_<pid>.urdf
  └─ pin.buildModelFromUrdf() + buildGeomFromUrdf()
       └─ buildReducedModel(lock non-arm joints)   → 6-DOF arm, rest frozen
            └─ collision pairs = all − SRDF-disabled − colliding-at-neutral
               (arm_pairs_only: drop fixed↔fixed pairs)
```

- IK targets are the **gripper TCP** = Link6 origin + `TCP_OFFSET_M` (0.12005 m)
  along the tool z-axis. This is an abstract target convention shared with the
  reachability map, **not** the physical Blender pad (which bottoms out ~0.0821 m
  below the flange). Sequence constants compensate via `GRASP_TCP_ABOVE` etc.
- `compute_ik_ordered()` wraps the solver: seed near the current pose first, then
  random restarts, gate every candidate through the whole-robot collision model,
  return the branch nearest the current config (or all candidates).
- Joint limits are widened to the URDF hardware range (±6.27 rad) in
  `setup_planner()` — collision is enforced by pinocchio, so the conservative
  software clamps are unnecessary and blocked valid elbow/wrist branches.

---

## 4. CBiRRT & Servo — `cr7_pnp/cbirrt.py` (`ConstrainedPlanner`)

### 4a. Orientation constraint (tilt-only by default)

```
e = log3(R0ᵀ · R(q))        ∈ ℝ³   (SO3 log map)
J = Jlog3(R0ᵀ · R) · Jω_local      (3×6 chain-rule Jacobian)
```

- `lock_tilt_only=True` (default): uses `e[0:2]`, `J[0:2]` → 2-DOF constraint,
  4-DOF manifold. Tool z (approach axis) locked; yaw about it free → box stays
  level while carried.
- `lock_tilt_only=False`: full 3-DOF orientation lock.

### 4b. CBiRRT (`plan`)

```
Ta = [project(start)]   Tb = [project(goal)]
loop (max_iter or time_limit):
  q_rand ← project(random sample)        (or connect_bias → other tree's root)
  extend(Ta, q_rand): nearest → step → project → edge collision-check
  if extended: connect(Tb, q_new) greedily; if joined → trace both → path
  swap Ta ↔ Tb
```
`_project(q)` is Newton–Raphson: `dq = J⁺·e ; q ← q − dq` until `‖e‖ < tol`.

### 4c. Cartesian servo (`linear_path`)

Straight-line translate by `delta` (m, base_link) holding the current
orientation, seeded from `start_q` (no IK branch jump):

```
for dist in steps along delta:
  oMdes = SE3(R0_current, p0 + unit·dist)
  damped-least-squares IK:  v = -Jᵀ(JJᵀ + λI)⁻¹·log6(oMf⁻¹·oMdes) ;  q += 0.5·v
  stop if joint limit / collision / singular (reports σ_min)
returns (path, reached_metres, reason)
```
A short result is a **hard failure** at the sequence level: `linear_servo`/
`preflight_linear` abort rather than grasp/place from the wrong spot.

---

## 5. Sequences

### 5a. `shelf_pick_place.py` — shelf → base (production, hub-and-spoke)

Every motion routes through a tool-down **HUB** waypoint so the arm never crosses
shelf→pocket directly (that direct carry stalls when grasp and place fall in
different elbow/wrist families — see `docs/CARRY_BRANCH_STALL.md`).

```
hub ──(CBiRRT spoke)──► pre-grasp ─► insert ─► J6 twist ─► descend ─► grip+attach
hub ◄──(reverse-replay of the recorded forward path, twist held)──────────────┘
hub ──(CBiRRT spoke)──► pocket hover ─► descend ─► release+attach-to-AGV
hub ◄──(reverse-replay)───────────────────────────────────────────────────────┘
```

Return is guaranteed two ways: spokes are **pre-flighted** under the carried-box
collision model with no motion (infeasible → abort before moving), and forward
joint waypoints are **recorded and replayed in reverse** (a path just executed is
executable backwards). Four tier-1 boxes → four base pockets, one per SPACE.

### 5b. `cbirrt_pick_place.py` — segmented DEMO (pre-hub reference)

The original single-cycle flow: RRT approach → insert → J6 twist → jaw-align →
descend → grip → ascend → retreat → tilt-constrained carry → place-descend →
release. Kept as a runnable reference; the hub version supersedes it.

### 5c. `wirebonder_pick_place.py` — base pocket → wirebonder slot (linear + RRT)

Device sequences use **free joint-space RRT for transit and straight Cartesian
servos for the fine moves — not CBiRRT** — and route through the hub:

```
hub → above base pocket (RRT) → descend (servo) → grasp → ascend → hub
    → above H_L slot    (RRT) → descend (servo) → release → ascend → hub
```

The H_L target is a world-frame point on the static wirebonder, looked up in
base_link via TF each cycle (AGV must be parked facing the device). The grasped
box hangs ~`GRASP_LATERAL_M` (≈48 mm) off the tool axis toward the fixed jaw, so
`grasp_tcp_pose()` offsets the TCP (via `gripper_x_in_base_fk`) to centre the box.

### 5d. `spawn_device_markers.py` — slot visualisation

Spawns translucent magazine-sized boxes (236×81×140 mm) in Gazebo at the four
wirebonder magazine slots (behind rails `Cube_H_L/H_R/G_L/G_R`), `--gap`/`--up`
to tune, `--delete` to remove. Prints the slot-centre world coordinates.

---

## 6. Trigger Decoupling (toward the BT/FSM mission node)

`wait_for_spacebar()` (in `geometry.py`) is a **development stand-in** for the
AMR/MCS state signal. Sequence functions (`pick_place_one_box`, `base_to_hl`, …)
are trigger-agnostic: today a SPACE loop in `main()` calls them; later a BT/FSM
mission node fed by `/amr_status` (TCP/IP bridge) and MCS (MQTT) calls the same
functions unchanged. `cr7_pnp` is the Robot-Control layer of that architecture.

---

## 7. Key Constants (`cr7_pnp/geometry.py`)

| Constant | Value | Meaning |
|----------|-------|---------|
| `DOWN` | `(0.707, 0.707, 0, 0)` | gripper tool z → world −Z (straight down) |
| `TCP_OFFSET_M` | `0.12005 m` | Link6 → IK-target TCP along tool z (convention) |
| `BOX_SIZE` | `(0.081, 0.236, 0.14)` | magazine box (short, long, height) m |
| `GRASP_LATERAL_M` | ≈ 0.046 m | box hang off tool axis (jaw geometry, computed) |
| `GRIPPER_OPEN` / `GRIPPER_CLOSE` | `[0.03]` / `[0.0]` | finger joint command (m) |
| `GRASP_TCP_ABOVE` | `0.015 m` | TCP above box centre at grasp |
| `INSERT_TCP_ABOVE` | `0.105 m` | TCP above box while inside the shelf gap |
| `PREGRASP_BACK` | `0.25 m` | stand-off in front of the shelf |
| `POCKET_X` / `POCKET_Y` | `0.3705` / `[±0.177, ±0.059]` | base pocket centres (base_link) |
| `STANDBY_POSE_DEG` | `[-8,-39,-105,0,0,0]` | folded rest config |

Sequence-specific tunables (e.g. `HUB_TCP`, `PLACE_ORDER_Y`, `HL_SLOT_WORLD`,
`HOVER_ABOVE`) live in the sequence scripts, not the library.

---

## 8. Directory Map

```
~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/
│
├── cr7_pnp/                      ← motion library (self-contained)
│   ├── __init__.py               ← public API re-exports
│   ├── node.py                   ← CR7Node → CBiRRTPickPlace → HubPickPlace
│   ├── model.py                  ← ReachabilityModel (pinocchio IK + collision)
│   ├── cbirrt.py                 ← ConstrainedPlanner (CBiRRT + servo)
│   └── geometry.py               ← pure helpers + tuned constants
│
├── shelf_pick_place.py             ← shelf→base, hub-and-spoke (production)
├── cbirrt_pick_place.py          ← segmented shelf→base DEMO
├── wirebonder_pick_place.py      ← base→wirebonder H_L (linear + RRT)
├── spawn_device_markers.py       ← Gazebo markers at device slots
├── reachability_map.py           ← offline reachability-map builder (CLI)
├── deploy_optimizer.py           ← deployment-orientation study (uses ReachabilityModel)
│
├── cr7_moveit/config/            ← SRDF (collision pairs), kinematics/limits yaml
├── cra_description/urdf/         ← cr7_robot.xacro, cr7_on_mpo700.urdf.xacro
├── dobot_gazebo/worlds/cr.world  ← shelf + wirebonder + AGV spawn
├── ../blender/wirebonder/        ← device model.sdf + per-cube collision STLs
│
├── docs/                         ← this file + CARRY_BRANCH_STALL, REACHABILITY_MAP, …
├── reachability_out/             ← CSV / PCD / JSON from reachability_map.py
└── TODO.md / TODO_history/       ← active + archived task tracking
```

---

## 9. Runtime Dependencies

```
Python env: ~/dobot_ws/.venv   (has pinocchio; ROS msgs come from the sourced overlay)
  pinocchio   (FK, Jacobian, SE3 log/exp, reduced model, IK)
  coal        (collision geometry backend, imported as `coal`)
  xacro       (XACRO → URDF at runtime)
  rclpy, numpy

ROS 2 interfaces required at runtime (Gazebo side):
  /joint_states                                       sensor_msgs/JointState
  /cr7_group_controller/follow_joint_trajectory       control_msgs/action/FollowJointTrajectory
  /gripper_controller/follow_joint_trajectory         control_msgs/action (gripper_finger_joint)
  /ATTACHLINK · /DETACHLINK                           linkattacher_msgs/srv
  /spawn_entity · /delete_entity                      gazebo_msgs/srv  (markers only)
  TF: odom (world proxy) → base_link, gripper_base_link, mpo_base_link
```

MoveIt is **not** required at runtime (IK/collision are pinocchio).

---

## 10. How to Run

```bash
# Terminal 1 — simulator (Gazebo Classic, brings up controllers + link-attacher)
ros2 launch dobot_gazebo cr7_gazebo.launch.py

# Terminal 2 — a sequence (always source ROS + the workspace first)
source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/setup.bash
cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
~/dobot_ws/.venv/bin/python3 shelf_pick_place.py      # shelf → base
# or
~/dobot_ws/.venv/bin/python3 wirebonder_pick_place.py   # base → wirebonder H_L

# Visualise the device magazine slots
~/dobot_ws/.venv/bin/python3 tools/spawn_device_markers.py --gap 0.005
~/dobot_ws/.venv/bin/python3 tools/spawn_device_markers.py --delete

# Offline reachability / deployment study (no sim needed)
~/dobot_ws/.venv/bin/python3 tools/reachability_map.py --bounds 0,0.8,-0.8,0,-0.05,0.05
~/dobot_ws/.venv/bin/python3 tools/deploy_optimizer.py

# Stop the sim cleanly (avoids stale gzserver)
./kill_sim.sh
```
