"""SIM ground-truth watchdog: catches physical accidents the sequence cannot see.

The runtime code only knows its own collision MODEL -- a real box that gets
clipped, tipped, or knocked to the floor, or the AGV riding up on something,
leaves no trace in the sequence log (the mission can even report DONE). This
script polls Gazebo ground truth for every shelf box + the robot and prints a
timestamped event line whenever:

    FLOOR   box z dropped below any legal surface   -> it fell off
    TIPPED  box roll/pitch beyond tolerance          -> it was knocked over
    MOVED   box shifted > MOVE_MM between sweeps     -> something touched it
            (the box currently being carried legitimately MOVEs -- correlate
             the timestamp with the sequence log to see WHICH box was in hand;
             any OTHER box moving = a strike)
    TILT    robot roll/pitch/z beyond tolerance      -> base pushed/beached

Timestamps are unix epoch, directly comparable to the [sec.nsec] stamps in the
rclpy sequence logs. Wall-clock polling via `gz model` (no plugins needed).

Run alongside any test (sim up):
    /usr/bin/python3 tools/world_watchdog.py [| tee watchdog.log]
Exit code 1 if any FLOOR/TIPPED/TILT event was seen (MOVED alone is neutral).
"""

import math
import os
import subprocess
import sys
import time

BOXES = [f'box_t{t}{c}' for t in (1, 2) for c in 'abcd']
ROBOT = 'cr7_on_mpo700'

MOVE_MM = 15.0        # per-sweep xy shift that counts as "touched"
BOX_TILT_RAD = 0.30   # resting and carried boxes both stay upright (tool-down)
FLOOR_Z = 0.50        # below every legal surface (boards 0.90/1.40, pockets ~0.96)
ROBOT_TILT_RAD = 0.08
ROBOT_Z_M = 0.05

ENV = dict(os.environ, GAZEBO_MASTER_URI=os.environ.get(
    'GAZEBO_MASTER_URI', 'http://localhost:11345'))


def pose(name):
    """(x, y, z, roll, pitch, yaw) from `gz model -p`, or None."""
    try:
        out = subprocess.run(['gz', 'model', '-m', name, '-p'], env=ENV,
                             capture_output=True, text=True, timeout=5).stdout
        vals = [float(v) for v in out.split()[:6]]
        return vals if len(vals) == 6 else None
    except Exception:
        return None


def main():
    prev = {}
    flagged = set()          # (name, kind) -> report state changes, not spam
    bad = False
    print(f"[watchdog] polling {len(BOXES)} boxes + robot; thresholds: "
          f"move>{MOVE_MM:.0f}mm tilt>{BOX_TILT_RAD:.2f}rad floor<{FLOOR_Z:.2f}m "
          f"robot tilt>{ROBOT_TILT_RAD:.2f}rad", flush=True)
    while True:
        t = time.time()
        for name in BOXES + [ROBOT]:
            p = pose(name)
            if p is None:
                continue
            x, y, z, r, pit, _ = p
            if name == ROBOT:
                if abs(r) > ROBOT_TILT_RAD or abs(pit) > ROBOT_TILT_RAD or abs(z) > ROBOT_Z_M:
                    if (name, 'TILT') not in flagged:
                        print(f"[{t:.1f}] TILT   {name} r={r:+.3f} p={pit:+.3f} "
                              f"z={z:+.3f}", flush=True)
                        flagged.add((name, 'TILT'))
                        bad = True
                else:
                    flagged.discard((name, 'TILT'))
            else:
                if z < FLOOR_Z:
                    if (name, 'FLOOR') not in flagged:
                        print(f"[{t:.1f}] FLOOR  {name} at ({x:+.3f},{y:+.3f},{z:+.3f})",
                              flush=True)
                        flagged.add((name, 'FLOOR'))
                        bad = True
                elif abs(r) > BOX_TILT_RAD or abs(pit) > BOX_TILT_RAD:
                    if (name, 'TIPPED') not in flagged:
                        print(f"[{t:.1f}] TIPPED {name} r={r:+.3f} p={pit:+.3f} "
                              f"at ({x:+.3f},{y:+.3f},{z:+.3f})", flush=True)
                        flagged.add((name, 'TIPPED'))
                        bad = True
                else:
                    flagged.discard((name, 'TIPPED'))
                    flagged.discard((name, 'FLOOR'))
                if name in prev:
                    dx, dy = x - prev[name][0], y - prev[name][1]
                    d_mm = math.hypot(dx, dy) * 1000.0
                    if d_mm > MOVE_MM:
                        print(f"[{t:.1f}] MOVED  {name} {d_mm:.0f} mm -> "
                              f"({x:+.3f},{y:+.3f},{z:+.3f})", flush=True)
            prev[name] = (x, y, z)
        # Also usable one-shot: --once prints current state and exits.
        if '--once' in sys.argv:
            sys.exit(1 if bad else 0)
        time.sleep(0.5)


if __name__ == '__main__':
    try:
        main()
    except KeyboardInterrupt:
        sys.exit(0)
