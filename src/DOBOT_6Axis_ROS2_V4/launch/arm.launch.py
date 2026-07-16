"""Bring up the vision node + mission dispatcher together, both under the SYSTEM
python (unified env: apt ros-humble-pinocchio + system OpenCV 4.5.4). The MCS bridge
is the comms team's node and starts separately.

    ros2 launch src/DOBOT_6Axis_ROS2_V4/launch/arm.launch.py

main.py has no TTY under launch, so its debug keyboard auto-disables and it runs
purely on /mcs/command + /mcs/stop. ros2 launch stops both on Ctrl-C.
"""
import os

from launch import LaunchDescription
from launch.actions import ExecuteProcess

PKG = os.path.expanduser('~/dobot_ws/src/DOBOT_6Axis_ROS2_V4')
PY = '/usr/bin/python3'   # cv2 4.5.4 + apt pinocchio both live here


def generate_launch_description():
    return LaunchDescription([
        ExecuteProcess(
            cmd=[PY, os.path.join(PKG, 'vision', 'tag_vision_node.py')],
            name='tag_vision', output='screen'),
        ExecuteProcess(
            cmd=[PY, os.path.join(PKG, 'main.py')],
            name='mcs_dispatcher', output='screen'),
    ])
