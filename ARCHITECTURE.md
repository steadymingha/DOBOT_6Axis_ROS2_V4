# Architecture Overview — CBiRRT Pick-and-Place

> **Scope**: `cbirrt_pick_place.py` + `constrained_cbirrt.py`  
> **Robot**: DOBOT CR7, OnRobot 2FG7 gripper  
> **Stack**: ROS 2 Humble · pinocchio 4.0.0 · MoveIt 2 (IK / collision srv only)

---

## 1. Component Map

```
┌─────────────────────────────────────────────────────────────────────┐
│  cbirrt_pick_place.py                                               │
│                                                                     │
│  CBiRRTPickPlace (ROS 2 node)                                       │
│  ├─ inherits  CR7RRTPlanner  ◄─── test_w_gripper.py                │
│  │   ├─ /joint_states subscriber                                    │
│  │   ├─ /compute_ik  (MoveIt srv)                                   │
│  │   ├─ /check_state_validity  (MoveIt srv)                         │
│  │   ├─ FollowJointTrajectory action client                         │
│  │   ├─ gripper controller (position cmd)                           │
│  │   └─ link_attacher_plugin (attach / detach)                      │
│  │                                                                  │
│  └─ owns  ConstrainedPlanner  ◄─── constrained_cbirrt.py           │
│      ├─ pinocchio reduced model (gripper joints locked)             │
│      ├─ plan()        — CBiRRT in joint space                       │
│      └─ lift_path()   — Cartesian-Jacobian straight-line servo      │
│                                                                     │
│  main()  — 6-segment sequencer                                      │
└─────────────────────────────────────────────────────────────────────┘

External services (must be running):
  Gazebo  ──►  /joint_states, trajectory execution, gripper, attach
  MoveIt  ──►  /compute_ik, /check_state_validity
  robot_state_publisher  ──►  TF (base_link)
```

---

## 2. Class Hierarchy

```
rclpy.Node
  └── CR7RRTPlanner          (test_w_gripper.py)
        └── CBiRRTPickPlace  (cbirrt_pick_place.py)
              │   setup_planner()          → creates ConstrainedPlanner
              │   compute_ik_ordered()     → nearest-branch IK with multi-seed
              │   move_constrained()       → CBiRRT carry (segment 4)
              │   vertical_servo()         → Cartesian servo (segments 2,3,5)
              │   execute_path()           → JointTrajectory → action client
              │
              └── (owns) ConstrainedPlanner   (constrained_cbirrt.py)
                    set_reference(quat)     → stores grasp orientation R0
                    plan(start, goal, ...)  → CBiRRT path or None
                    lift_path(start, dz,..) → straight-line Cartesian path
                    _project(q)             → Newton projection onto manifold
                    _err_and_jac(q)         → orientation error + Jacobian
```

---

## 3. Motion Pipeline — 6 Segments

```
 [start: any pose]
       │
       ▼
 ┌─ Segment 1 ─────────────────────────────────────────────────────┐
 │  Approach (free joint-space RRT)                                │
 │  move_to_pose → pre-grasp (0.4, 0.0, z=0.30) gripper down      │
 │  → gripper OPEN                                                 │
 └─────────────────────────────────────────────────────────────────┘
       │
       ▼
 ┌─ Segment 2 ─────────────────────────────────────────────────────┐
 │  Vertical DESCEND  (Cartesian-Jacobian servo, no RRT)           │
 │  vertical_servo(dz = 0.24 - 0.30 = -0.06 m)                    │
 │  → gripper CLOSE + attach_box()                                 │
 └─────────────────────────────────────────────────────────────────┘
       │
       ▼
 ┌─ Segment 3 ─────────────────────────────────────────────────────┐
 │  Vertical LIFT  (Cartesian-Jacobian servo)                      │
 │  vertical_servo(dz = 0.30 - 0.24 = +0.06 m)                    │
 └─────────────────────────────────────────────────────────────────┘
       │
       ▼
 ┌─ Segment 4 ─────────────────────────────────────────────────────┐
 │  Horizontal CARRY  (CBiRRT, tilt-constrained)                   │
 │  move_constrained → above marker (0.2, 0.35, z=0.30)           │
 │  orientation held: gripper stays down, yaw free                 │
 └─────────────────────────────────────────────────────────────────┘
       │
       ▼
 ┌─ Segment 5 ─────────────────────────────────────────────────────┐
 │  Vertical DESCEND + release  (Cartesian-Jacobian servo)         │
 │  vertical_servo(dz = 0.25 - 0.30 = -0.05 m)                    │
 │  → detach_box() + gripper OPEN                                  │
 └─────────────────────────────────────────────────────────────────┘
       │
       ▼
 ┌─ Segment 6 ─────────────────────────────────────────────────────┐
 │  Retreat + overhead wait pose                                   │
 │  vertical_servo(+0.08 m) → move_to_joint_pose(overhead)        │
 └─────────────────────────────────────────────────────────────────┘
```

**Why two planners for vertical vs horizontal?**

| Motion | Planner | Reason |
|--------|---------|--------|
| Vertical (segments 2,3,5) | Cartesian-Jacobian servo (`lift_path`) | Deterministic, no branch jump, fast convergence on a straight line |
| Horizontal carry (segment 4) | CBiRRT | Must route through joint space while holding tilt — needs a sampling planner |
| Approach (segment 1) | Free RRT (MoveIt/CR7RRTPlanner) | No orientation constraint yet; just needs to reach pre-grasp |

---

## 4. constrained_cbirrt.py — Internal Flow

### 4a. Model setup

```
xacro → /tmp/cr7_cbirrt_model.urdf
  └─ pin.buildModelFromUrdf()
       └─ pin.buildReducedModel(lock gripper joints)
            → 6-DOF arm model, gripper frozen at neutral
               (same reduced-model pattern as reachability_map.py)
```

### 4b. Orientation constraint

The constraint is the **orientation error** between the current EE rotation and the reference grasp rotation `R0`:

```
e = log3(R0ᵀ · R(q))        ∈ ℝ³   (SO3 log map)
J = Jlog3(R0ᵀ · R) · Jω_local      (3×6 chain-rule Jacobian)
```

- `lock_tilt_only=True` (default): uses only `e[0:2]` and `J[0:2]` → 2-DOF constraint, 4-DOF manifold.  
  The approach axis (tool z) is locked; yaw about that axis is free → box stays level.
- `lock_tilt_only=False`: uses all 3 components → full 3-DOF orientation lock.

### 4c. CBiRRT algorithm

```
Ta = [start_projected]    Tb = [goal_projected]

loop (max_iter or time_limit):
  q_rand ← random sample in [lo, hi]  (or connect_bias → Tb root)
  q_rand ← _project(q_rand)           Newton projection onto manifold
  extend(Ta, q_rand)
    nearest node in Ta → step toward q_rand → project → collision check
  if extended:
    connect(Tb, q_new)                 repeated extend until reach or stuck
    if connected → trace both trees → full path
  swap Ta ↔ Tb
```

`_project(q)` is a **Newton–Raphson** iterator:
```
dq = J⁺ · e    (lstsq pseudo-inverse)
q  ← q - dq
```
stops when `‖e‖ < tol` or `max_iters` reached.

### 4d. Cartesian servo (`lift_path`)

```
target_z_k = p0.z + step * k * sign(dz)    for k = 1..N
  oMdes = SE3(R0_current, [p0.x, p0.y, target_z_k])
  IK via damped Jacobian:
    err = log6(oMf⁻¹ · oMdes)
    J   = -Jlog6 · computeFrameJacobian
    v   = -Jᵀ (J Jᵀ + λI)⁻¹ err        (damped least-squares)
    q  += 0.5 · v
  stop if: joint limit, collision, IK diverges
```

---

## 5. Data Flow Diagram

```
 /joint_states ──────────────────► current_joints (np.array[6])
                                          │
                    ┌─────────────────────▼──────────────────────┐
                    │          CBiRRTPickPlace                    │
                    │                                             │
 target_pose ──────►│ compute_ik_ordered()                        │
 (PoseStamped)      │   MoveIt /compute_ik (120 seeds)            │
                    │   → goal_q (nearest IK branch)             │
                    │                                             │
                    │ move_constrained(target_pose)               │
                    │   set_reference(orientation)                │
                    │   ConstrainedPlanner.plan(start, goal)      │
                    │   → path [[q0..q5], ...]                    │
                    │   execute_path(path)                        │
                    │   → FollowJointTrajectory goal ─────────────┼──► Gazebo
                    │                                             │
                    │ vertical_servo(dz)                          │
                    │   ConstrainedPlanner.lift_path(start, dz)   │
                    │   → path [[q0..q5], ...]                    │
                    │   execute_path(path) ───────────────────────┼──► Gazebo
                    └─────────────────────────────────────────────┘
```

---

## 6. Key Constants (cbirrt_pick_place.py `main()`)

| Constant | Value | Meaning |
|----------|-------|---------|
| `OBJECT_XY` | `(0.4, 0.0)` | Pick box XY position |
| `MARKER_XY` | `(0.2, 0.35)` | Place marker XY position |
| `Z_PREGRASP` | `0.30 m` | Link6 height for approach |
| `Z_GRASP` | `0.24 m` | Link6 height to close gripper |
| `Z_CARRY` | `0.30 m` | Transport height (max reachable at MARKER_XY with gripper-down) |
| `Z_PLACE` | `0.25 m` | Link6 height to release |
| `DOWN` | `(0.707, 0.707, 0, 0)` | Quaternion: gripper local z → world −Z |
| `GRIPPER_OPEN` | `[0.09]` | Finger gap (m) |
| `GRIPPER_CLOSE` | `[0.036]` | Finger gap at grasp |

> **Note on Z heights**: these are Link6 (wrist flange) positions.  
> The OnRobot 2FG7 TCP is **0.12005 m below Link6** when in the gripper-down orientation.  
> → Actual TCP heights: approach=0.18, grasp=0.12, carry=0.18, place=0.13 m.

---

## 7. Directory Map

```
~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/
│
├── cbirrt_pick_place.py          ← Main executable: 6-segment pick-and-place
├── cbirrt_pick_place_draft.py    ← Earlier draft (single-segment version)
├── constrained_cbirrt.py         ← CBiRRT planner + Cartesian servo (pure Python)
├── reachability_map.py           ← Monte-Carlo FK reachability map (pinocchio)
├── test_w_gripper.py             ← Base node: CR7RRTPlanner (MoveIt RRT + helpers)
│
├── cr7_moveit/
│   └── config/
│       ├── cr7_robot.srdf        ← Disabled collision pairs (Adjacent / Never)
│       ├── kinematics.yaml       ← KDL IK solver config
│       ├── joint_limits.yaml     ← Velocity / accel limits for MoveIt
│       ├── moveit_controllers.yaml
│       └── ros2_controllers.yaml
│
├── cra_description/
│   └── urdf/
│       ├── cr7_robot.xacro       ← Top-level robot XACRO (arm + gripper)
│       ├── gripper.xacro         ← Simple gripper geometry definition
│       └── cr*.xacro             ← Other DOBOT arm variants
│
├── onrobot_2fg7_description/
│   └── urdf/
│       └── onrobot_2fg7.xacro    ← OnRobot 2FG7 gripper (TCP offset: 0.12005 m)
│
├── TODO_log/                     ← Completed task logs (1.TODO.md, 2.TODO.md, …)
├── reachability_out/             ← CSV / PCD / JSON outputs from reachability_map.py
├── ARCHITECTURE.md               ← This file
└── TODO.md                       ← Active task tracking
```

---

## 8. Runtime Dependencies

```
Python env: .venv  (uv-managed, ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/.venv)
  pinocchio==4.0.0   (FK, Jacobian, SE3 log/exp, reduced model)
  hpp-fcl            (collision geometry backend for reachability_map.py)
  xacro              (XACRO → URDF at runtime)
  rclpy              (ROS 2 Python client)
  numpy

ROS 2 services / actions required at runtime:
  /compute_ik               (moveit_msgs/srv/GetPositionIK)
  /check_state_validity     (moveit_msgs/srv/GetStateValidity)
  /cr7_joint_controller/follow_joint_trajectory  (control_msgs/action)
  /gripper_controller/...   (position command)
  /ATTACHERSERVICE          (link_attacher_plugin)
  /joint_states             (sensor_msgs/msg/JointState)
```

---

## 9. How to Run

```bash
# Terminal 1 — simulator
ros2 launch dobot_gazebo cr7_gazebo.launch.py

# Terminal 2 — MoveIt
ros2 launch cr7_moveit cr7_moveit.launch.py

# Terminal 3 — pick-and-place
source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/setup.bash
cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
.venv/bin/python3 cbirrt_pick_place.py

# Optional: reachability map
.venv/bin/python3 reachability_map.py --samples 300000 --seed 1
# or
uv run reachability_map.py --samples 300000 --seed 1
```
