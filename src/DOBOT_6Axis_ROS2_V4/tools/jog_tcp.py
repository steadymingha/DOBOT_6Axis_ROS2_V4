"""Keyboard TCP jogger -- a throwaway tuning aid.

Jog the arm end-effector along base_link axes with single keypresses (each press
runs ONE collision-gated linear servo of the current step), then hit SPACE to
print the TCP coordinates in BOTH base_link and world. Copy the world xyz into
SLOT_NUDGE / target constants once it looks right.

Orientation is held fixed (linear_servo), so this only finds positions, not
azimuths -- park the arm at roughly the right pose first (e.g. run a transfer up
to the failing approach, or start from the hub).

Keys:
    w/s : +x / -x      a/d : +y / -y      r/f : +z / -z   (base_link, metres)
    [/] : step  x0.5 / x2        space : print TCP        q/Esc : quit
    c   : toggle the carried-box phantom  (jog box-free, then check at a pose)
    v   : collision check the CURRENT pose + list any colliding geometry pairs

With the phantom ON the jog itself refuses to enter a colliding pose, so the
usual flow is: jog box-free to the target, press c (phantom on), press v -- if it
reports a collision the phantom/clearance is too conservative; if it is clear the
problem is goal-IK, not the box.

Run (sim up, arm spawned):
    source /opt/ros/humble/setup.bash
    source ~/dobot_ws/install/setup.bash
    cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
    ~/dobot_ws/.venv/bin/python3 jog_tcp.py
"""

import os
import sys
import time
import threading

import numpy as np
import pinocchio as pin
import rclpy
from rclpy.duration import Duration
from rclpy.executors import MultiThreadedExecutor

# tools/ lives one level below the package root; add the root so cr7_pnp imports.
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from cr7_pnp import HubPickPlace  # noqa: E402
from cr7_pnp.geometry import quat_to_R  # noqa: E402

# key -> base_link unit jog direction
JOG = {
    'w': (+1, 0, 0), 's': (-1, 0, 0),
    'a': (0, +1, 0), 'd': (0, -1, 0),
    'r': (0, 0, +1), 'f': (0, 0, -1),
}


def read_key():
    import termios, tty
    fd = sys.stdin.fileno()
    old = termios.tcgetattr(fd)
    try:
        tty.setraw(fd)
        ch = sys.stdin.read(1)
    finally:
        termios.tcsetattr(fd, termios.TCSADRAIN, old)
    return ch


def tcp_base(node):
    """Current TCP position in base_link, via FK of the live joints."""
    q = node.current_joints.tolist()
    pos, _ = node.ik_model.fk_tcp(node.ik_model.pin_q(q))
    return np.asarray(pos)


def base_to_world(node, p_base):
    """Map a base_link point into the world frame via live TF, or None."""
    try:
        tf = node.tf_buffer.lookup_transform(
            node.world_frame, 'base_link', rclpy.time.Time(),
            timeout=Duration(seconds=2.0))
    except Exception as e:
        node.get_logger().error(f"[TF] base_link->{node.world_frame} failed: {e}")
        return None
    t, r = tf.transform.translation, tf.transform.rotation
    R = quat_to_R(r.x, r.y, r.z, r.w)
    return R @ np.asarray(p_base) + np.array([t.x, t.y, t.z])


def check_collision(node):
    """Run the whole-robot collision check at the current config and print any
    colliding geometry-name pairs (same model is_state_valid uses)."""
    cm = node.collision
    qp = cm.pin_q(node.current_joints.tolist())
    pin.computeCollisions(cm.model, cm.data, cm.geom, cm.geom_data, qp, False)
    objs = cm.geom.geometryObjects
    hits = [(objs[cp.first].name, objs[cp.second].name)
            for i, cp in enumerate(cm.geom.collisionPairs)
            if cm.geom_data.collisionResults[i].isCollision()]
    if not hits:
        print("  collision: NONE (state valid)")
    else:
        print(f"  collision: {len(hits)} pair(s):")
        for a, b in hits:
            print(f"    {a}  <->  {b}")


def main(args=None):
    rclpy.init(args=args)
    node = HubPickPlace()
    node.setup_planner()
    executor = MultiThreadedExecutor()
    executor.add_node(node)
    threading.Thread(target=executor.spin, daemon=True).start()
    time.sleep(2)  # wait for joint states

    step = 0.01
    phantom = False
    print("\n" + "=" * 60)
    print(" TCP jogger.  w/s=x a/d=y r/f=z  [/]=step  space=print")
    print("              c=toggle box phantom  v=collision check  q=quit")
    print("=" * 60)

    def show():
        b = tcp_base(node)
        w = base_to_world(node, b)
        wt = f"x={w[0]:.3f} y={w[1]:.3f} z={w[2]:.3f}" if w is not None else "n/a"
        print(f"\nTCP  base_link: x={b[0]:.3f} y={b[1]:.3f} z={b[2]:.3f}"
              f"   world: {wt}   step={step*1000:.0f}mm")

    show()
    try:
        while rclpy.ok():
            ch = read_key()
            if ch in ('q', '\x1b', '\x03'):
                break
            if ch == '[':
                step = max(step / 2, 0.0005); show(); continue
            if ch == ']':
                step = min(step * 2, 0.2); show(); continue
            if ch == ' ':
                show(); continue
            if ch == 'c':
                phantom = not phantom
                (node.attach_box_collision if phantom else node.detach_box_collision)()
                print(f"\nbox phantom: {'ON' if phantom else 'OFF'}")
                continue
            if ch == 'v':
                show(); check_collision(node); continue
            if ch in JOG:
                node.linear_servo(step * np.array(JOG[ch], dtype=float), label=ch)
                show()
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
