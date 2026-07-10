"""Self-check for the wirebonder strategy dispatch (no ROS, no sim needed).

The PICK/PLACE tables and strategy() replaced a chain of inline `if` checks; this
fails if a location stops resolving to the body that used to run for it, or if a
table gains a key the other lacks. Run:  python3 sequences/test_wirebonder_dispatch.py
"""

import importlib.util
import os
import sys
import types
from unittest import mock

_HERE = os.path.dirname(os.path.abspath(__file__))
_SEQ = os.path.join(_HERE, 'wirebonder_pick_place.py')


def load():
    """Exec the sequence module with its ROS / pinocchio deps stubbed out.

    numpy is stubbed too when absent: strategy() and the tables need no real math,
    only the module-level SLOT_LOCAL comprehension touches np (mocks satisfy it)."""
    for name in ('rclpy', 'rclpy.executors', 'geometry_msgs', 'geometry_msgs.msg',
                 'cr7_pnp', 'mcs_protocol'):
        sys.modules.setdefault(name, mock.MagicMock())
    if importlib.util.find_spec('numpy') is None:
        sys.modules['numpy'] = mock.MagicMock()
    cr7 = sys.modules['cr7_pnp']
    cr7.GRASP_TCP_ABOVE, cr7.GRASP_LATERAL_M = 0.0, 0.053
    cr7.POCKET_X, cr7.POCKET_Y = 0.5, [0.0, 0.059, 0.1, 0.15]
    cr7.POCKET_SURFACE_Z, cr7.BOX_SIZE = 0.5, (0.236, 0.081, 0.14)
    cr7.DOWN, cr7.GRIPPER_OPEN = (0, 0, 0, 1), [0.03]
    cr7.quat_mul = cr7.quat_about_z = lambda *a: (0, 0, 0, 1)
    cr7.pose_at, cr7.HubPickPlace = (lambda *a: None), object
    mod = types.ModuleType('wirebonder'); mod.__file__ = _SEQ
    exec(compile(open(_SEQ).read(), _SEQ, 'exec'), mod.__dict__)
    return mod


def main():
    w = load()

    # Both tables cover the same strategies; staging only aids the PICK, so a staged
    # front slot is PLACED by the plain front body.
    assert set(w.PICK) == {'base', 'front', 'front_staged', 'top'}, w.PICK
    assert set(w.PLACE) == set(w.PICK), w.PLACE
    assert w.PLACE['front_staged'] is w.PLACE['front']

    # Every shipped transfer resolves to a body in the table that indexes it.
    for key, (src, dst) in w.SEQUENCES.items():
        s_pick, s_place = w.strategy(src), w.strategy(dst)
        assert s_pick in w.PICK, (key, src.name, s_pick)
        assert s_place in w.PLACE, (key, dst.name, s_place)
        print(f"  seq {key}: {src.name:12s} ({s_pick:12s}) -> {dst.name:12s} ({s_place})")

    # The per-location rules the old `if` chain encoded.
    assert w.strategy(w.base_loc()) == 'base'
    assert w.strategy(w.slot_loc('wb1', 'D')) == 'front_staged'   # D has STAGE_JOINTS
    assert w.strategy(w.slot_loc('wb1', 'A')) == 'front'          # A does not
    assert w.strategy(w.slot_loc('wb1', 'B')) == 'top'
    assert w.strategy(w.slot_loc('wb1', 'Z')) is None             # unmeasured -> refused
    print("strategy dispatch OK")


if __name__ == '__main__':
    main()
