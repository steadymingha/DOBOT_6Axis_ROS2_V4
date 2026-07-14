# Shelf pick & place — restored to the June-proven sequence

The two-tier + vision `sequences/shelf_pick_place.py` stopped picking reliably,
so it was **overwritten** with the older, simpler hub-and-spoke sequence from
commit `4aa55d5` (`hub_pick_place.py`, "shelf-base full cycle with long
gripper"), and the shelf was raised / the robot re-parked for the new layout.

## 2026-07-13 review — why it still collided, and the fix

The world/geometry side of this restore (sections 4–6) WAS on disk, but the
`shelf_pick_place.py` rewrite described in section 1 was **not** — the file was
still the HEAD version. Three consequences, first two = the observed crash
"collides on the way to the box":

1. **Stale box constants.** `SHELF_BOXES` was hardcoded to the OLD world
   (`box_l1a..d`, z = 0.97) while the raised world has `box_t1a..d` at
   z = 1.35. The sequence steered the gripper into the (now empty) 0.78–1.28
   gap, under the tier-1 board — and `ATTACHLINK` could never match the model
   name either.
2. **Spawn→hub RRT was shelf-blind.** `update_shelf_collision()` was first
   called only inside the pick cycle, so the initial spawn→hub motion planned
   with all shelf boards/stock phantoms still parked far away → free to swing
   through the real shelf.
3. **Target box phantom never parked.** The current `cr7_pnp` node auto-adds a
   resting-stock phantom per shelf box (the June node had none); without
   `set_shelf_stock_absent()` the pick's own grasp legs false-collide in
   pre-flight.

Fixed **only in `sequences/shelf_pick_place.py`** (cr7_pnp, main.py and the
wirebonder flow untouched):

- `SHELF_BOXES` now derived from `shelf_box_center(1, i)` /
  `shelf_box_model(1, i)` — tier height and names live in one place
  (`cr7_pnp/geometry.py`) and always match `cr.world`.
- New `bringup(node)`: `update_shelf_collision()` **before any motion**, then
  `init_hub` → `go_to_hub` → `clear_pocket_stowaway()`. Both the standalone
  `main()` and `main.py` (which already imported `shelf.bringup`) use it.
- `set_shelf_stock_absent(1, idx)` before each pick; restored on pick failure
  (box still on the shelf), left absent after success (box is in a pocket).
- `clear_pocket_stowaway(node)`: best-effort `/delete_entity` of `box_l2c` so
  all four pockets are free (with the shelf spawn park it falls to the floor
  near the wirebonder anyway — deleted to keep the sim clean).
- `MAGAZINE_ATTACH = False`: the AGV is parked in this flow, the placed box
  settles by gravity. Set `True` before driving the AGV with boxes aboard
  (a loose box slides in the pocket; see memory note on link-attach).

### Follow-up: brushing neighbours on approach, knocking them over on return

Observed after the restore: the arm brushed resting boxes on the way in and
knocked them over on the way back. Three causes, three fixes (the first is the
big one — the real gripper was wider than the planned one):

0. **The collision model froze the finger CLOSED.** `ReachabilityModel` locked
   non-arm joints at `pin.neutral` → finger q = 0 (gap 81 mm), but every
   approach runs with the gripper OPEN (q = 0.03, gap 111 mm): the real moving
   jaw travelled **30 mm outside the collision model**, hitting boxes the
   planner had legitimately cleared. Fixed in `cr7_pnp/model.py`: the finger is
   frozen at `FINGER_OPEN_M` (new single-source constant in
   `gripper_params.py`; `GRIPPER_OPEN` is tied to it so a wider open command
   can never silently outgrow the model). Verified: `gripper_finger_link`
   collision geoms now sit at x = −0.030 in the frozen model.

1. **Stock phantoms were 10 mm/side smaller than the real boxes** (legacy guard
   shrink), so the planner could legally graze 10 mm into a real box — and
   tracking error spent the rest. Now FULL SIZE (`STOCK_SHRINK = 0.0` in
   `cr7_pnp/node.py::_add_shelf_stock`): nothing legitimate ever needs to enter
   a resting box's volume, because the pick target's phantom is parked absent.
   On the REAL robot set it NEGATIVE (inflate) by the vision + tracking
   uncertainty — avoidance wants margin, not slack.
2. **The twist-held return replay executed even when its own validity check
   failed** ("warn and proceed"). The forward spoke is box-validated in the
   UNTWISTED azimuth; replaying it with J6 twisted +90° sweeps the long gripper
   and the carried box through different space.
   **SUPERSEDED later the same day**: the validation + re-plan moved into the
   pre-flight (no motion on failure), the blind-replay fallback was deleted,
   carried-box vs resting-stock pairs became a contextual toggle, and the twist
   is driven to the exact pre-flighted sign. Full write-up:
   `shelf_pick_collision_fixes.md` (causes 3-5).

Note on heights: the June-proven relative workspace was box-centre 0.29 m
above base_link (0.97 vs 0.68). The initial "tier-1 top = base + 0.28 m"
put it at 0.35 m (1.35 vs 1.00) — 6 cm higher relative to the arm than what
was proven. That margin proved too thin in practice: box d (outer flank) went
all-branch singular ~20 mm short of the insert whenever the base ratcheted a
few mm (see `level_base` in the sequence). **APPLIED 2026-07-14**: shelf pose
z 0.38 → 0.32 in `cr.world` (boxes t1 1.35 → 1.29, t2 1.85 → 1.79),
`SHELF_BOARD_TOPS`/`SHELF_TIER_TOPS` −0.06, leg stubs 0.38 → 0.32 — tier-1
top is now base + 0.22, the June-proven offset (tier-2 + 0.72 likewise).

## What changed

### 1. `sequences/shelf_pick_place.py` — replaced with the v1 sequence
Port of `4aa55d5:hub_pick_place.py`, adapted to the refactored `cr7_pnp`
package. **Tier 1 only, 4 boxes, no vision, no put-back.**

- Motion node + primitives are **imported** from `cr7_pnp` (they used to be an
  inline `HubPickPlace` subclass): `init_hub`, `go_to_hub`, `plan_spoke`,
  `ik_nearest`, `preflight_linear`, `gripper_x_in_base_fk`,
  `attach/detach_box_collision`, `capture`, `linear_servo`, `rotate_j6`,
  `move_single_joint`, `attach_box`, `replay_reverse`, `rev/join/offset_j6`.
- `init_hub(ref_pose, HUB_TCP, GRASP_LATERAL_M)` — current 3-arg signature.
- Box centres come from `shelf_box_center(1, i)` / `shelf_box_model(1, i)` so
  tier height lives in **one** place (`SHELF_TIER_TOPS`).
- `set_shelf_stock_absent(1, idx, ...)` around each pick — the current node
  auto-adds a resting-stock phantom per shelf box, so the target's own phantom
  is parked while its grasp legs run millimetres from it (the old node had none).
- `MAGAZINE_ATTACH = False`: AGV is parked (spawn-and-pick), box settles into
  its pocket by gravity — no link-attach that fights the pocket floor.
- Kept `bringup()` (hub init + move + pocket-stowaway clear) and
  `clear_pocket_stowaway()` so **`main.py` keeps working** with a minimal edit.

### 2. `main.py` — shelf branch simplified to the v1 API
- `locate()` shelf branch → **no-op `return True`** (no vision, no per-tier
  ArUco capture, no AGV driving; stowaway now cleared in `bringup`).
- `pick_place()` shelf branch → `pick_place_one_box(node, node.box_idx)` with
  `n = len(shelf.PLACE_ORDER_Y)` (was `tier_of` + `park_for_box` +
  `pick_place_one_box(node, tier, i)` across two tiers).
- Comments updated (no twist-safe re-pick, no shelf vision subscription).

### 3. `sequences/test_shelf_cycle.py` — **deleted**
It drove the old two-tier round-trip / put-back API, which no longer exists.

### 4. Shelf + boxes raised — `dobot_gazebo/worlds/cr.world`
Tier-1 top must sit 0.28 m above the robot base (~1.0 m) → **1.28 m**. Board
heights are baked into `shelf.dae`, so the whole shelf model is raised **+0.38 m**
via its pose instead of re-baking the mesh:

| entity        | old z | new z |
|---------------|-------|-------|
| `shelf` pose  | 0.00  | 0.38  |
| `box_t1a..d`  | 0.97  | 1.35  |
| `box_t2a..d`  | 1.47  | 1.85  |

Tier-1 top 1.28 m, tier-2 1.78 m (0.50 m spacing preserved). **Side effect:**
the shelf legs floated 0.38 m above the floor. Fixed without re-baking the
mesh: `shelf/model.sdf` adds four 0.38 m leg-extension stubs (visual +
collision) that continue the posts to the ground — keep the stub length equal
to the shelf pose z in `cr.world`.

### 5. Geometry constants — `cr7_pnp/geometry.py`
Mirror the raised world (both absolute world z):
- `SHELF_BOARD_TOPS`: `(0.40, 0.90, 1.40, 1.95)` → `(0.78, 1.28, 1.78, 2.33)`
- `SHELF_TIER_TOPS`: `{1: 0.90, 2: 1.40}` → `{1: 1.28, 2: 1.78}`

### 6. Robot spawn — `dobot_gazebo/launch/gazebo_mpo700_cr7.launch.py`
`-x 0.849 -y -0.072` → **`-x 0.683 -y 0.008`** (single park reaching all four
tier-1 boxes; no in-sequence AGV driving).

## Run

```bash
source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/setup.bash
cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
# restart Gazebo so the raised world + new spawn take effect
/usr/bin/python3 sequences/shelf_pick_place.py   # standalone, SPACE per box
# or via the dispatcher: /usr/bin/python3 main.py   (shelf location)
```

Stop the sim with `./kill_sim.sh` (isolate the call — it self-matches
`ros2`/pattern text).

## Verified vs. not

- **Verified:** byte-compile of `shelf_pick_place.py` + `main.py`,
  `main.py --selftest` passes, no dangling refs to removed symbols
  (`tier_of`, `TIERS`, `park_for_box`, `capture_shelf`, `put_back`, …),
  `shelf_box_center(1,0)` world z = 1.35, `GRASP_LATERAL_M` = 0.1397.
- **Since verified in sim (2026-07-13)**: full 4-box tier-1 cycle, all boxes
  upright in their pockets, no resting box disturbed — after the collision +
  last-box fixes documented in `shelf_pick_collision_fixes.md`. Tuning knobs
  if a leg ever fails again:
  - `HUB_TCP` z/x (hub IK fails or carried box collides at the hub),
  - `PLACE_TCP_ABOVE_HUB` (box pressed into / dropped above the pocket),
  - `GRASP_YAW_OFFSET`, `GRIPPER_YAW_TWIST` sign (jaw azimuth in the gap),
  - spawn `-x/-y` (a pre-grasp/pocket IK reports unreachable).
