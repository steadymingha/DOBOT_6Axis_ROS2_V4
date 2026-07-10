# wirebonder_pick_place.py refactoring plan

> **STATUS (2026-07-10)**: the Phase-3 GOAL (full-transfer dry-run, refuse before
> any motion, MCS error) is now shipped as a standalone `preflight_transfer()`
> replacing `preflight_place()` -- see `wirebonder_transfer_hardening.md` for the
> measurements and the seq-1 high-transit place / seq-2 replay-return fixes that
> went with it. Section 1.3's coverage table is therefore obsolete. What remains
> of this plan is the STRUCTURAL part (Phase 1-2 Live/Dry executor unification,
> strategy tables, module hoisting) to remove the preflight<->live manual
> duplication, and Phase 4 (back-out on mid-motion failure).

Analysis of `sequences/wirebonder_pick_place.py` (2026-07-09) and a concrete,
phased refactoring proposal. Goals, as requested:

1. The three sequences share one structure instead of three ad-hoc shapes.
2. **Full-transfer dry-run**: if ANY motion leg of a transfer is infeasible,
   the arm does not move at all — for all sequences, not just one.
3. Reusable modules: hoist the generic parts out of the sequence script.

The current code WORKS; every asymmetry that is tuned physics is kept
(see "What stays asymmetric on purpose" below). This is a structure/coverage
refactor, not a behavior change.

> **Status update (2026-07-10):** goal 2 (full-transfer dry-run) is now DONE,
> out of order: `preflight_transfer()` hand-mirrors every plannable leg of pick
> AND place for all location types and gates `transfer()` — `preflight_place()`
> was deleted as planned. That is Phase 3's outcome without Phases 1–2, at the
> cost of the leg lists existing twice (live + dry), acknowledged by a
> `ponytail:` comment in the code. The coverage matrix in 1.3 is therefore
> HISTORICAL. Phases 1, 2 and 4 remain open; Phase 2's motivation changes —
> see the amended phase notes below. New code the refactor must carry:
> `front_place_legs()` (shared high-transit leg list — already the "write the
> legs once" pattern in miniature), transit recording + reverse replay for the
> direct slot→slot return, and the single-view capture (view B dropped).

---

## 1. Current state

### 1.1 Strategy inventory

There are 4 pick strategies and 3 place strategies, spread over 5 functions
with the dispatch half inside `pick()`/`place()` and half in front of them:

| location type            | pick                                  | place                       |
|--------------------------|---------------------------------------|-----------------------------|
| base pocket              | `pick()` inline branch (l.666)        | `place()` inline branch (l.714) — guarded descend |
| front slot (generic)     | `pick()` inline branch (l.677)        | `place()` capture/replay part (l.747) |
| front slot in STAGE_JOINTS| `pick_slot_front_j1()` (l.581)        | (same as front)             |
| top slot                 | `pick_slot_top()` (l.531)             | `place_slot_top()` (l.561)  |

Dispatch today is a chain of `if loc.kind == 'slot' and slot_mode(loc) == 'top'`
/ `loc.ref[1] in STAGE_JOINTS` checks duplicated at the top of both `pick()` and
`place()`.

### 1.2 Copy-pasted blocks

- **Grasp block** (`control_gripper(CLOSE)` → set `object_model/link` →
  `attach_box()` w/ open-on-fail + `ATTACH_FAILED` + `sleep(0.5)`): verbatim
  ×3 — `pick_slot_top` l.547, `pick_slot_front_j1` l.640, `pick` l.687.
- **Release block** (`detach_box()` → `control_gripper(OPEN)` → `sleep(0.5)`):
  ×3 — `place_slot_top` l.573, `place` base l.740, `place` front l.780.
- **Hover→descend→act→ascend shape**: re-written 4× with small deltas.

### 1.3 Dry-run / preflight coverage — the real gap (HISTORICAL, fixed by `preflight_transfer`)

`preflight_place()` (l.788) only covers **hub→front-slot place** legs, and only
when the place starts from the hub. Actual coverage per sequence:

| seq | src → dst            | dry-run coverage today | worst uncovered failure |
|-----|----------------------|------------------------|-------------------------|
| 1   | base → slot A (front)| place approach+seat only | pick descend fails after approach moved (empty gripper, off-hub) |
| 2   | slot B → C (top→top) | **none** (direct → preflight skipped) | place hover servo fails → **box stranded in gripper mid-air** |
| 3   | slot D → base        | **none** (dst is base → preflight skipped) | staged forward fails mid-insert → arm stranded deep in the device front, partial path discarded |

So seq 1 is the only protected one, and even it only on the place side.
(Note: it *feels* like seq 3 is the protected one because its staged pick logs
every resolved target before moving — but nothing is validated up front there.)

Why full coverage is cheap: **every leg is already computable without moving.**

- Cartesian legs → `cbirrt.linear_path` is pure computation; `node.preflight_linear`
  already wraps it and returns the end config for chaining (shelf_pick_place.py
  already chains it this way, l.159–165).
- Joint legs (`joint_move`, stage configs) → interpolate + `is_state_valid`, pure.
- Free-RRT legs (`goto`, `go_to_hub`) → goal `compute_ik_ordered` is pure and
  catches the dominant failure ("goal unreachable / in collision"). The RRT
  search itself can still fail at runtime, but rarely when the goal is valid.
- Jaw-square J6 correction (`pick_slot_front_j1`) → predictable via
  `gripper_x_in_base_fk` (pure FK) + `transform_world_vector` (TF read, no motion).
- Grasp/attach and `guarded_descend` contact are *runtime* events — a dry-run
  cannot pre-validate them; they keep their existing runtime failure paths.

---

## 2. Target design

One idea: **write each strategy body once, run it twice** — first against a
`Dry` motion executor (chains virtual configs, no motion), then against the
`Live` one (the current behavior). `transfer()` refuses to move if the dry
pass fails anywhere.

```
transfer(src, dst)
 ├─ resolve strategies:  PICK[strategy(src)], PLACE[strategy(dst)]
 ├─ DRY pass : pick_x(node, Dry, src); place_y(node, Dry, dst)   ← no motion
 │             any leg infeasible → fail(UNREACHABLE, ...), arm never moved
 └─ LIVE pass: pick_x(node, Live, src); place_y(node, Live, dst)
```

### 2.1 Motion executor interface (the only new abstraction)

Two implementations, both thin. This is justified duplication removal, not a
speculative interface — dry and live genuinely differ.

```python
class Live:                            # delegates to node — current behavior
    servo_to(pose, label) -> bool      # node-level servo_to (see 3.3)
    servo(delta, label)   -> bool      # node.linear_servo
    joint_to(q, label)    -> bool      # node.joint_move
    goto(pose, label)     -> bool      # RRT via move_to_pose_ref
    phantom(on)                        # attach/detach_box_collision
    grasp(loc) / release()             # shared blocks from 1.2 (+ gripper, attach)
    guarded_descend(max_drop, label)   # node.guarded_descend
    q                                  # node.current_joints

class Dry:                             # never commands the arm
    servo_to / servo  -> preflight_linear chain, virtual q advances
    joint_to          -> interpolated is_state_valid sweep (like move_single_joint)
    goto              -> compute_ik_ordered(pose) is not None; virtual q = nearest IK
    phantom(on)       -> real toggle (collision model only), restored on exit
    grasp/release     -> no-op True (runtime-only events)
    guarded_descend   -> no-op (contact-seeking; cannot fail dry)
    q                 -> the virtual config
```

`Dry.phantom` must toggle the real collision phantom exactly like
`preflight_place` does today (approach carries the box, seat doesn't) and
**restore the entry state in a finally** — this also kills the current
phantom-leak class of bugs noted at `transfer()` l.840.

### 2.2 Strategy dispatch

```python
def strategy(loc):
    if loc.kind == 'base':
        return 'base'
    m = slot_mode(loc)                       # 'front' | 'top' | None
    if m == 'front' and loc.ref[1] in STAGE_JOINTS:
        return 'front_staged'
    return m                                 # None -> fail(NO coords measured)

PICK  = {'base': pick_base, 'front': pick_front,
         'front_staged': pick_front_staged, 'top': pick_top}
PLACE = {'base': place_base, 'front': place_front, 'top': place_top}
```

Sequences 1/2/3 stay pure data in `SEQUENCES`; adding a 4th transfer or a
2nd device is a table edit, no new code.

### 2.3 What stays asymmetric on purpose (do NOT unify)

These look like inconsistencies but are tuned physics, each documented in-file:

- base place uses `guarded_descend` (contact seating), slots use fixed drops —
  pocket heights vary, slot shelves don't.
- top pick descend is a *fixed* servo, not guarded — the phantom false-triggers
  on the front rail ~54 mm early (l.542 comment).
- front strategies use capture/reverse-replay for the return — deterministic,
  box-safe path vs. a fresh RRT.
- `SLOT_PICK_DROP` ≠ `SLOT_PLACE_DROP` — separate knobs (pads must plunge
  around the box on pick; ceiling documented at l.222).
- `pick_front` (generic, non-staged) looks dead for the current `SEQUENCES`
  (only slot D is picked from a front slot, and D is staged) — it is the
  documented fallback for front slots without a `STAGE_JOINTS` entry (slot A
  as a future src). Keep it; it becomes the `'front'` row of the PICK table.

---

## 3. Phased plan

Each phase is independently shippable and verified by running transfers 1, 2, 3
in sim under `--no-vision` (fully deterministic waypoints — the natural
regression harness; restart with `./kill_sim.sh` between runs).

### Phase 1 — code motion only (no behavior change) ✅ DONE (2026-07-10)

1. ✅ `HubPickPlace.grasp_object(model, link)` / `.release_object()` in
   `cr7_pnp/node.py` replace the ×3 grasp and ×3 release copies. `fail(...)`
   stays in the sequence via a one-line `grasp(node, loc)` wrapper.
2. ✅ `pick()`/`place()` split into `pick_base`/`pick_front`/`pick_front_staged`/
   `pick_top` and `place_base`/`place_front`/`place_top` (old `pick_slot_top`,
   `place_slot_top`, `pick_slot_front_j1` renamed).
3. ✅ `strategy()` + `PICK`/`PLACE` tables; `transfer()` and both
   `preflight_transfer` branches now dispatch on the SAME `strategy()`, so live
   and dry can't disagree about which body runs. `transfer()` refuses up front
   when a slot has no measured coords (`strategy() is None`).

Also folded in (from the ponytail review + Phase 4):

- ✅ `node.tcp_xyz(config=None)` and `node.servo_to(pose, label)` hoisted to
  `cr7_pnp/node.py`; the module-level `servo_to` and the ×4 inlined
  `ik_model.fk_tcp(ik_model.pin_q(...))` blocks are gone.
- ✅ `run_legs()` — the float-z-vs-pose leg walker, previously inlined in
  `place_front`.
- ✅ deleted: `CAPTURE_BASELINE`, `CAPTURE_B_JOINTS`, `node._cap_pub`, the
  `Int32` import (all dead since view B was dropped).
- ✅ **Phase 4 back-out** landed here since it was two lines: `pick_front_staged`
  and `place_front` now `replay_reverse(fwd)` on a failed `forward()` instead of
  discarding the partial path and stranding the arm inside the device.
- ✅ `sequences/test_wirebonder_dispatch.py` — runs without ROS/sim, fails if a
  location stops resolving to the body that used to run for it.

Verify: `python3 sequences/test_wirebonder_dispatch.py` (passes), then seq 1/2/3
in sim under `--no-vision` (motion must be identical; the logs are deterministic).

### Phase 2 — Live/Dry executors, strategies written against them

**Amended motivation (2026-07-10):** the dry-run already exists
(`preflight_transfer`), so this phase is no longer about *enabling* it — it is
about **deleting the hand-mirrored copy**. Today every strategy's leg list is
written twice (live body + `dry_pick`/`dry_place` branch) and the two can
drift; `front_place_legs()` already proves the fix locally (one leg list,
consumed by both the live place and the preflight). Phase 2 generalises that:
strategies written once against the executor, `preflight_transfer`'s ~130
lines collapse into `Dry` (~40 lines).

1. Add `Live` and `Dry` (≈60–80 lines total) to the sequence file (they are
   wirebonder-agnostic; if the shelf sequence later wants them, hoist to
   `cr7_pnp` then — not before). `Dry` absorbs `preflight_transfer`'s `dry` /
   `dry_to` chaining, its phantom toggling + `finally` restore, and the
   float-z "lift" leg handling (currently `isinstance(tgt, float)` in two
   places).
2. Thread the executor through the 7 strategy functions: every
   `node.linear_servo(...)` → `m.servo(...)`, `servo_to(node,...)` →
   `m.servo_to(...)`, gripper/attach → `m.grasp(loc)` / `m.release()`, etc.
   Pose/target *computation* (`slot_target`, `top_grasp_pose`,
   `slot_flange_seat`, `resolve`, `grasp_tcp_pose`, `front_place_legs`) stays
   as-is — it is already pure and shared by both passes.
3. The capture/reverse-replay wrapper in the front strategies applies only to
   `Live` (`Dry` records nothing): give the executor `m.capture(fn)` where the
   Dry version just calls `fn()`. Same for the `_pick_transit`/`_place_transit`
   recording in the top strategies (direct slot→slot return path).

Verify: seq 1/2/3 with `Live` only (dry not yet wired) — identical behavior.

### Phase 3 — transfer() dry-runs the whole transfer first ✅ DONE (2026-07-10, hand-rolled)

Implemented as `preflight_transfer()` gating `transfer()`; `preflight_place()`
deleted as planned. Differences from the sketch below (which assumed Phases
1–2 were in): the dry legs are hand-mirrored per strategy instead of reusing
the live bodies — that duplication is what the amended Phase 2 removes. The
J1-swing validity sweep, the goal-IK gate on the generic-front RRT approach,
and the phantom `finally` restore all landed as designed. The original sketch
is kept for reference:

```python
def transfer(node, src, dst):
    node.detach_box_collision()                    # leak guard, as today
    ...update_wirebonder_collision...
    direct = src.kind == 'slot' and dst.kind == 'slot'
    dry = Dry(node)
    try:
        ok = (PICK[strategy(src)](node, dry, src, to_hub=not direct)
              and PLACE[strategy(dst)](node, dry, dst))
    finally:
        dry.restore_phantom()
    if not ok:
        return fail(node, proto.ErrorCode.UNREACHABLE,
                    f"[preflight] {src.name}->{dst.name} infeasible; arm NOT moved")
    live = Live(node)
    return (PICK[strategy(src)](node, live, src, to_hub=not direct)
            and PLACE[strategy(dst)](node, live, dst))
```

- **Delete `preflight_place()`** — fully subsumed (its two legs are now two of
  the ~8–10 dry legs).
- Cost: one extra `linear_path`/IK solve per leg before moving — same order as
  today's preflight, well under a second total; negligible vs. arm motion time.
- Honest limits (state in the code where relevant): attach failures,
  `guarded_descend` no-contact, and RRT *search* failures on a valid goal can
  still fail at runtime mid-transfer — unchanged from today, just much rarer.

Verify: (a) seq 1/2/3 nominal — one extra "[preflight] ..." log, motion identical;
(b) inject infeasibility (e.g. temporary `SLOT_NUDGE` far into the device) →
transfer refuses with the arm never leaving the hub, for **each** of 1/2/3.

### Phase 4 — back-out on mid-motion failure ✅ DONE (2026-07-10, folded into Phase 1)

Today `pick_front_staged`/`place_front` discard the recorded partial path when
`forward()` fails, stranding the arm mid-device. Change:

```python
ok, fwd = node.capture(forward)
if not ok:
    node.replay_reverse(fwd)      # retrace the proven partial path out
    return False
```

The recorded prefix was just executed, so its reverse is safe by construction.
With Phase 3 in place this triggers rarely (runtime-only failures), which is
exactly when you want an automatic back-out.

### Phase 5 (optional — flagged YAGNI, skip unless it hurts)

Plan-cache execution: have the Dry pass keep every computed joint path and have
Live execute the cached paths instead of re-solving. Removes the double solve
and makes dry≡live exactly. Not worth it now: solves are fast, and caching adds
staleness rules (arm must not move between passes). Add only if the double
planning time ever becomes noticeable.

---

## 4. Module reuse (hoisting out of the sequence script)

Move now (small, already generic):

| what | from | to | why |
|------|------|----|-----|
| `_T_odom_model`, `_to_model`, `_to_odom`, `quat_yaw`, `quat_about_y` | sequence l.276–303, 870 | `cr7_pnp/geometry.py` | pure math next to `quat_about_z`/`pose_at` |
| grasp/release blocks | ×3 copies | `cr7_pnp/node.py` (`HubPickPlace`) | composes node-owned services (Phase 1) |
| `servo_to` | sequence l.493 | `cr7_pnp/node.py` | uses only `ik_model` + `linear_servo`; any sequence wants it |
| `fail()`, `wait_for_key()` | sequence | `sequences/common.py` (new, tiny) | shared by future sequence scripts; `fail` needs `mcs_protocol`, which node.py should not import |

Stays in the sequence file (wirebonder-specific, correctly so): `SLOT_OFFSET`,
`SLOT_WORLD`/`SLOT_LOCAL` anchoring, `DEVICES`, capture/vision gating,
`SEQUENCES`, all strategy functions, `Live`/`Dry` (until a 2nd consumer exists).

`shelf_pick_place.py` can adopt `grasp_object`/`release_object` and `servo_to`
later — do not touch it in this refactor beyond keeping imports compatible.

---

## 5. Expected outcome

- Each motion strategy exists exactly once; dispatch is a table; sequences are data.
- Any of 1/2/3 (and any future transfer) refuses to move unless every plannable
  leg of pick **and** place is feasible from the hub — the seq-2 "box stranded
  mid-air" and seq-3 "stranded mid-device" classes disappear at the planning level,
  and Phase 4 backs out of the residual runtime failures.
- ~40–60 fewer lines in the sequence file, but the real win is coverage and
  single-point-of-change, not line count.
- Regression harness: `--no-vision` runs of 1→2→3 after every phase
  (deterministic waypoints make "identical behavior" checkable from the logs).
