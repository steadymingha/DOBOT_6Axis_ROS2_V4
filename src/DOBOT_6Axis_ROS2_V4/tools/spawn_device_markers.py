"""Spawn 4 magazine-footprint markers in Gazebo behind the wirebonder rails.

The 4 thin bar cubes (Cube_H_L/H_R/G_L/G_R, 256x15x13 mm) are the front rails of
the magazine slots. A magazine (236 x 81 mm footprint, 140 mm tall) is placed
BEHIND each rail (+y, into the device). This spawns a translucent box of the
magazine size at each slot's place centre (world frame) so the fit can be
eyeballed in the Gazebo GUI, then prints the coordinates.

Place centre = rail centre shifted +y by (rail_half_thick + magazine_half_depth
+ gap) so the magazine front face sits flush behind the rail, and +z by --up so
its vertical centre matches the support surface (tune in sim).

Run (sim must be up):
    source /opt/ros/humble/setup.bash
    source ~/dobot_ws/install/setup.bash
    ~/dobot_ws/.venv/bin/python3 spawn_device_markers.py             # flush behind rail
    ~/dobot_ws/.venv/bin/python3 spawn_device_markers.py --gap 0.01 --up 0.05
    ~/dobot_ws/.venv/bin/python3 spawn_device_markers.py --delete    # remove them
"""

import argparse

import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Pose
from gazebo_msgs.srv import SpawnEntity, DeleteEntity

# Magazine box (the shelf box): footprint 236 x 81 mm, 140 mm tall. The 236 mm
# side aligns with the rail width (x), 81 mm is the slot depth (y).
MAG = (0.236, 0.081, 0.14)   # (x, y, z) metres
RAIL_HALF_Y = 0.0075         # rail half-thickness in y (15 mm bar)

# Rail centres in world (= wirebonder spawn 2.35,0.5,0 + each cube's baked AABB
# centre). (name, rail_world_xyz, rgb, tag)  -- tags are the user's in/out labels.
RAILS = [
    ('marker_H_L', (2.002, 0.388, 0.896), (1.0, 0.1, 0.1), 'H_L back(in)'),
    ('marker_H_R', (2.698, 0.388, 0.896), (0.1, 1.0, 0.1), 'H_R back(out)'),
    ('marker_G_L', (2.002, 0.388, 1.281), (0.2, 0.4, 1.0), 'G_L back(out)'),
    ('marker_G_R', (2.698, 0.388, 1.281), (1.0, 0.9, 0.1), 'G_R back(in)'),
]


def box_sdf(name, rgb):
    r, g, b = rgb
    return f"""<?xml version='1.0'?>
<sdf version='1.7'>
  <model name='{name}'>
    <static>true</static>
    <link name='link'>
      <visual name='visual'>
        <geometry><box><size>{MAG[0]} {MAG[1]} {MAG[2]}</size></box></geometry>
        <material>
          <ambient>{r} {g} {b} 0.5</ambient>
          <diffuse>{r} {g} {b} 0.5</diffuse>
        </material>
        <transparency>0.5</transparency>
      </visual>
    </link>
  </model>
</sdf>"""


def call(client, req, node):
    if not client.wait_for_service(timeout_sec=5.0):
        node.get_logger().error(f"{client.srv_name} not available (is Gazebo up?)")
        return None
    fut = client.call_async(req)
    rclpy.spin_until_future_complete(node, fut, timeout_sec=10.0)
    return fut.result()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--gap', type=float, default=0.0,
                    help='extra +y gap between rail back face and magazine front (m)')
    ap.add_argument('--up', type=float, default=0.0,
                    help='+z shift of the magazine centre vs the rail height (m)')
    ap.add_argument('--delete', action='store_true', help='remove the markers instead')
    args = ap.parse_args()

    rclpy.init()
    node = rclpy.create_node('device_marker_spawner')

    if args.delete:
        cli = node.create_client(DeleteEntity, '/delete_entity')
        for name, _, _, _ in RAILS:
            res = call(cli, DeleteEntity.Request(name=name), node)
            node.get_logger().info(f"delete {name}: {getattr(res, 'success', '?')}")
        node.destroy_node(); rclpy.shutdown(); return

    # Magazine centre behind the rail: flush behind = rail half-thickness +
    # magazine half-depth, plus any extra gap.
    back = RAIL_HALF_Y + MAG[1] / 2.0 + args.gap

    cli = node.create_client(SpawnEntity, '/spawn_entity')
    print(f"{'marker':12} {'magazine centre world (x, y, z)':32} tag   "
          f"(gap={args.gap}, up={args.up})")
    for name, (rx, ry, rz), rgb, tag in RAILS:
        cx, cy, cz = rx, ry + back, rz + args.up
        pose = Pose()
        pose.position.x, pose.position.y, pose.position.z = cx, cy, cz
        req = SpawnEntity.Request(
            name=name, xml=box_sdf(name, rgb), reference_frame='world')
        req.initial_pose = pose
        res = call(cli, req, node)
        ok = getattr(res, 'success', False)
        print(f"{name:12} ({cx:+.3f}, {cy:+.3f}, {cz:+.3f})        {tag:14} -> "
              f"{'spawned' if ok else 'FAILED'}")

    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
