#!/usr/bin/env python3
"""Regression check: preflight_transfer for every wirebonder sequence must pass,
WITHOUT moving the arm. Run after changing waypoints/geometry constants or the
device pose pipeline -- if a sequence prints FAIL here, the live transfer would
have refused at the hub with the same [preflight] error.

The live arm may be parked anywhere; current_joints is faked to hub_q (preflight
always evaluates from the hub, where every transfer starts).

    source /opt/ros/humble/setup.bash && source ~/dobot_ws/install/setup.bash
    /usr/bin/python3 tools/preflight_check.py     # sim up; exit 0 = all PASS
"""
import os
import sys
import threading
import time

import numpy as np
import rclpy
from rclpy.executors import MultiThreadedExecutor

_PKG = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, _PKG)
sys.path.insert(0, os.path.join(_PKG, 'sequences'))
sys.path.insert(0, os.path.join(_PKG, 'comms'))
from cr7_pnp import (HubPickPlace, pose_at, quat_mul, quat_about_z, DOWN,
                     GRASP_TCP_ABOVE, GRASP_LATERAL_M)
import wirebonder_pick_place as wb


class FakeHubNode(HubPickPlace):
    _fake_hub = False

    @property
    def current_joints(self):
        if self._fake_hub and self.hub_q is not None:
            return np.array(self.hub_q, float)
        return self._cj

    @current_joints.setter
    def current_joints(self, v):
        self._cj = v


def main():
    rclpy.init()
    node = FakeHubNode()
    node.setup_planner()
    node.add_wirebonder_meshes(os.path.join(os.path.dirname(_PKG),
                                            'blender', 'wirebonder', 'collision'))
    ex = MultiThreadedExecutor(); ex.add_node(node)
    threading.Thread(target=ex.spin, daemon=True).start()
    time.sleep(2)
    base = wb.base_loc()
    ref = pose_at([base.ref[0], base.ref[1], base.ref[2] + GRASP_TCP_ABOVE],
                  quat_mul(quat_about_z(base.yaw), DOWN))
    assert node.init_hub(ref, wb.HUB_TCP, GRASP_LATERAL_M)
    node.update_wirebonder_collision(wb.DEVICES['wb1'])
    node._fake_hub = True

    results = {}
    for key, (src, dst) in wb.SEQUENCES.items():
        direct = src.kind == 'slot' and dst.kind == 'slot'
        print(f"\n--- preflight seq {key}: {src.name} -> {dst.name} ---")
        results[key] = wb.preflight_transfer(node, src, dst, direct)
    print("\nRESULT:", {k: ('PASS' if v else 'FAIL') for k, v in results.items()})
    rclpy.shutdown()
    sys.exit(0 if all(results.values()) else 1)


if __name__ == '__main__':
    main()
