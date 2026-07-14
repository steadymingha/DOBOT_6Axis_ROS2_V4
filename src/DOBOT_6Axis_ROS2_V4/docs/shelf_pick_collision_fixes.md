# Shelf pick & place — collision + last-box fixes (2026-07-13)

`sequences/shelf_pick_place.py` had two failure modes that commit `4aa55d5`
(`hub_pick_place.py`, same sequence logic) did not show:

1. The gripper **brushed the target box on the approach** and **knocked over
   resting boxes on the return** to the hub.
2. After fixing (1), the **last box (d, outer flank) aborted in pre-flight**:
   `insert servo infeasible: reaches 246 of 250 mm -> singular(sigma_min=0.0001)`
   — from a pose only 0.53 m from the base, nowhere near full extension.

All five root causes and their fixes below. Verified in sim: full 4-box
tier-1 cycle, all boxes upright in pockets, no resting box moved.

---

## Why 4aa55d5 was clean (and why that was luck, not safety)

At `4aa55d5` the collision environment contained **only the 3 shelf boards** —
no resting-box phantoms at all. The run was clean because the geometry left
margin, not because planning avoided boxes:

| | 4aa55d5 | now |
|---|---|---|
| fixed-pad inner face `JAW_FIXED_PAD_X` | 0.1632 | 0.1832 (+20 mm) |
| carried-box lateral hang `GRASP_LATERAL_M` | ~120 mm | ~140 mm |
| tier-1 top relative to arm base | 0.21 m | 0.27 m (shelf +0.38, base +0.32) |

The +20 mm jaw (and the box hanging 20 mm further off the tool axis) consumed
the clearance the old run survived on. Box pitch (0.181 m) and box size were
unchanged, so the free gap between resting boxes stayed ~100 mm while the
swept volume grew.

---

## Cause 1 — collision-model finger frozen 30 mm smaller than the real jaw

The planning model reduced (`pin.buildReducedModel`) the finger joint at
**neutral q=0** (pad gap 81 mm), but every travel move runs with the gripper
**OPEN** (gap 111 mm). The real moving jaw therefore swung 30 mm outside the
volume the planner checked — paths the planner cleared brushed the target box.

**Fix** (`cr7_pnp/model.py`): freeze the finger at `FINGER_OPEN_M = 0.03`
(the widest opening any sequence commands) instead of neutral, and clamp
`GRIPPER_OPEN = [FINGER_OPEN_M]` (`cr7_pnp/geometry.py`) so the real jaw can
never be commanded outside the model. Verified by building the reduced model:
the finger geometry sits exactly −0.03 m along the opening axis.

## Cause 2 — resting-stock phantoms shrunk 10 mm per side

The shelf-stock phantoms were built 10 mm/side smaller than the real boxes,
so "collision-free" plans could graze real boxes by that much.

**Fix** (`cr7_pnp/node.py`, `_add_shelf_stock`): `STOCK_SHRINK = 0.0` — the
avoidance phantom is full-size. Rule of thumb: avoidance obstacles are never
shrunk; on the real robot they are **inflated** by the vision/tracking
uncertainty.

## Cause 3 — the twisted return was a different, never-validated path

The return to the hub replays the approach spoke with **J6 offset +90°**
(`offset_j6(rev(P1), twist_delta)`) because the box is grasped in the twisted
azimuth and un-twisting next to the shelf sweeps the neighbours. But P1 was
planned/validated at the **untwisted** wrist: same TCP line, rotated sweep
volume (the carried box is 236 mm long and hangs 140 mm off the tool axis).
Reverse-replay guarantees safety **only for the identical joint path** — a J6
offset breaks the guarantee. Worse, the runtime had a fallback that executed
the colliding replay with just a warning ("may brush the stock").

**Fix** (`sequences/shelf_pick_place.py`, pre-flight section): validate the
twisted replay waypoint-by-waypoint **before any motion**, under the
box-attached + box-vs-stock model with the target's phantom parked. If it
collides, pre-plan a replacement hub spoke under the same model; if that also
fails, abort with **no motion**. The runtime replan/fallback is deleted — the
executed return is always a pre-validated path. The twist itself is driven
with `move_single_joint` to exactly `+GRIPPER_YAW_TWIST` (not `rotate_j6`,
which may flip sign at runtime and would invalidate the pre-flighted return).

## Cause 4 — carried-box vs resting-stock pairs did not exist

Pinocchio only checks **registered collision pairs**. The carried-box phantom
vs shelf-stock pairs had been removed wholesale (at the untwisted wrist the
phantom lands off-centre over a neighbouring box and false-collides), so the
object most likely to hit a neighbour on the return — the box in the gripper —
was invisible to every sweep. A stale comment claimed the return sweep covered
it; it could not.

**Fix** (`cr7_pnp/node.py`): the pairs live in a separate list
`_box_stock_pairs` (8 pairs, 2 tiers x 4) toggled by
`set_box_stock_collision(on)` — ON only while validating configs where the box
is actually in the gripper in its grasped (twisted) azimuth, OFF otherwise.
Contextual toggle instead of permanent removal: full-time ON false-positives,
full-time OFF is blind.

## Cause 5 — last box: the place_ref-nearest IK branch hit a wrist singularity

Box d's pre-flight died with `singular(sigma_min=0.0001)` 4 mm before the end
of the straight-line insert. Not a reach problem (box 0.53 m from base; both
twist signs solvable — measured offline): the sequence picked the single IK
branch nearest the pocket family (`ik_nearest(pregrasp, place_ref)`), and for
this box that branch's straight-line insert passes through a wrist
singularity. Other branches of the same pose servo cleanly.

**Fix**: the pre-flight now sorts **all** IK candidates by distance to
`place_ref` and takes the first branch whose insert + descend servos pre-flight
clean (`sequences/shelf_pick_place.py`), then pins the approach spoke to that
exact branch via `plan_spoke(..., goal_q=...)` (`cr7_pnp/node.py`). The pin
matters because IK is stochastic — re-solving inside `plan_spoke` could land
on a different (unvetted) branch. The target's phantom stays solid for the
insert checks and P1 planning; it is parked per-candidate for the descend
check and re-parked once a branch is locked in.

---

## Principles worth keeping

- **Reverse-replay is only guaranteed for the identical joint path.** Any
  offset (a held J6 twist) makes it a new path that must be re-validated.
- **Collision checks only see pairs that exist.** A phantom without pairs is
  transparent. If a pair false-positives in one context, toggle it per
  context; never delete it.
- **Leave no runtime freedom between validation and execution.** A primitive
  that picks a direction/sign/branch at execution time (`rotate_j6`,
  re-solved IK goals) invalidates whatever was pre-flighted. Pin the vetted
  choice and execute exactly that.
- **Move every check before the first motion.** With full pre-flight the worst
  outcome is "no motion + clear log", never "stranded mid-carry" or "knocked a
  box over on the way back".
- **The planning model must bound the real robot.** Freeze articulated parts
  at their widest commanded state; never shrink obstacle phantoms — inflate
  them by the measurement uncertainty instead.
