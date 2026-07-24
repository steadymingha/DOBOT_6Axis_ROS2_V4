"""Standalone ros2_control stack for the Isaac Sim robot.

Replaces gazebo_ros2_control (which lived inside gzserver): a plain
controller_manager drives topic_based_ros2_control against
/isaac_joint_states + /isaac_joint_commands, with the same
ros2_controllers.yaml and controller set as the Gazebo sim.
Launch:  ros2 launch ~/dobot_ws/isaac/controllers.launch.py
"""
import os

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import RegisterEventHandler
from launch.event_handlers import OnProcessExit
from launch_ros.actions import Node

URDF = os.path.expanduser("~/dobot_ws/isaac/cr7_on_mpo700_isaac.urdf")


def generate_launch_description():
    robot_description = {"robot_description": open(URDF).read()}
    # Isaac's own copy of cr7_moveit's ros2_controllers.yaml: adds the velocity
    # command interface (feed-forward) that the Gazebo variant must not have.
    controllers_yaml = os.path.expanduser("~/dobot_ws/isaac/ros2_controllers.yaml")
    use_sim_time = {"use_sim_time": True}

    # Spawners chained sequentially (same pattern as gazebo_mpo700_cr7.launch.py):
    # parallel spawners raced the controller_manager during Isaac bring-up and
    # left controllers unconfigured.
    spawn_jsb = Node(package="controller_manager", executable="spawner",
                     arguments=["joint_state_broadcaster"], output="screen")
    spawn_arm = Node(package="controller_manager", executable="spawner",
                     arguments=["cr7_group_controller"], output="screen")
    spawn_grip = Node(package="controller_manager", executable="spawner",
                      arguments=["gripper_controller"], output="screen")

    return LaunchDescription([
        Node(
            package="robot_state_publisher",
            executable="robot_state_publisher",
            output="screen",
            parameters=[robot_description, use_sim_time],
        ),
        Node(
            package="controller_manager",
            executable="ros2_control_node",
            output="screen",
            parameters=[robot_description, controllers_yaml, use_sim_time],
        ),
        spawn_jsb,
        RegisterEventHandler(OnProcessExit(target_action=spawn_jsb,
                                           on_exit=[spawn_arm])),
        RegisterEventHandler(OnProcessExit(target_action=spawn_arm,
                                           on_exit=[spawn_grip])),
    ])
