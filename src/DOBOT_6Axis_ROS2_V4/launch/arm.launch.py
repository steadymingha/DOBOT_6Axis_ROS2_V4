"""Bring up the vision node + mission dispatcher together, both under the SYSTEM
python (unified env: apt ros-humble-pinocchio + system OpenCV 4.5.4). The MCS bridge
is the comms team's node and starts separately.

    ros2 launch src/DOBOT_6Axis_ROS2_V4/launch/arm.launch.py                # sim
    ros2 launch src/DOBOT_6Axis_ROS2_V4/launch/arm.launch.py profile:=real  # real robot

ONE argument picks BOTH the vision node and main.py's profile, so "real profile
with the sim vision node up" cannot happen (docs/real_robot_pipeline_plan.md 4.1c).
profile:=real also exports DOBOT_ENV=real to main.py, so the collision constants
follow the same switch (main.py still cross-checks the two).

    sim  -> vision/tag_vision_node.py      (Gazebo tags)     + main.py --profile sim
    real -> vision/vision_hover_node.py    (vision_bridge)   + main.py --profile real

main.py has no TTY under launch, so its debug keyboard auto-disables and it runs
purely on /mcs/command + /mcs/stop. ros2 launch stops both on Ctrl-C.
"""
import os

from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument, ExecuteProcess, SetEnvironmentVariable
from launch.conditions import IfCondition, UnlessCondition
from launch.substitutions import LaunchConfiguration, PythonExpression
from launch_ros.actions import Node

PKG = os.path.expanduser('~/dobot_ws/src/DOBOT_6Axis_ROS2_V4')
PY = '/usr/bin/python3'   # sim PC: cv2 4.5.4 + apt pinocchio both live here
# Real robot (Jetson container ros2_dobot): pinocchio lives only in the venv;
# same interpreter test/run.sh and tools/run.sh use.
PY_REAL = os.path.expanduser('~/dobot_ws/.venv/bin/python3')


def generate_launch_description():
    profile = LaunchConfiguration('profile')
    is_sim = PythonExpression(["'", profile, "' == 'sim'"])
    return LaunchDescription([
        DeclareLaunchArgument('profile', default_value='sim', choices=['sim', 'real'],
                              description='sim (default) or real robot'),
        SetEnvironmentVariable('DOBOT_ENV', profile),
        ExecuteProcess(
            cmd=[PY, os.path.join(PKG, 'vision', 'tag_vision_node.py')],
            name='tag_vision', output='screen', condition=IfCondition(is_sim)),
        ExecuteProcess(
            cmd=[PY_REAL, os.path.join(PKG, 'vision', 'vision_hover_node.py')],
            name='vision_hover', output='screen', condition=UnlessCondition(is_sim)),
        # Real robot: nothing publishes odom (no AGV driver), and the arm's own
        # frame is base_link -- so odom == base_link, an identity static TF, and
        # every world<->base transform in the sequences becomes a pass-through
        # (plan 2-1). This bakes in "the AGV does not move during a mission";
        # re-run locate_shelf after re-parking. The collision model root on the
        # real Jetson is base_link too (arm-only model), so no mpo_base_link.
        Node(package='tf2_ros', executable='static_transform_publisher',
             name='odom_is_base_link', output='screen',
             arguments=['--frame-id', 'odom', '--child-frame-id', 'base_link'],
             condition=UnlessCondition(is_sim)),
        ExecuteProcess(
            cmd=[PY, os.path.join(PKG, 'main.py'), '--profile', 'sim'],
            name='mcs_dispatcher', output='screen', condition=IfCondition(is_sim)),
        ExecuteProcess(
            cmd=[PY_REAL, os.path.join(PKG, 'main.py'), '--profile', 'real'],
            name='mcs_dispatcher', output='screen', condition=UnlessCondition(is_sim)),
    ])
