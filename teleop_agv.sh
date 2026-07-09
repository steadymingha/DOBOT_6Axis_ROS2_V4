#!/bin/bash
# Low-latency keyboard teleop for the MPO-700 AGV.
# Thin wrapper: source ROS, then run the rclpy teleop node (keeps one publisher
# alive so key -> publish is immediate, no per-key node spawn delay).
#
# Keep THIS terminal focused while pressing keys (not the Gazebo window).

source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/local_setup.bash

exec uv run ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/tools/teleop_agv.py
