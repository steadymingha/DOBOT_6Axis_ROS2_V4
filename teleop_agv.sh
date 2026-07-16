#!/bin/bash
# Low-latency keyboard teleop for the MPO-700 AGV.
# Thin wrapper: source ROS, then run the rclpy teleop node (keeps one publisher
# alive so key -> publish is immediate, no per-key node spawn delay).
#
# Keep THIS terminal focused while pressing keys (not the Gazebo window).

source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/local_setup.bash
export ROS_LOCALHOST_ONLY=1   # single-machine cell; see run_mpo700_cr7.sh note
export FASTRTPS_DEFAULT_PROFILES_FILE=$HOME/dobot_ws/fastdds_localhost.xml

# NO exec: exec replaces this shell with the teleop process, so quitting the
# teleop (q / Ctrl-C) killed the whole terminal. Run it as a child instead --
# quitting returns to the prompt. stty sane is a belt in case the raw-mode
# restore was skipped (e.g. the process was SIGKILLed).
uv run ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/tools/teleop_agv.py
stty sane 2>/dev/null
echo "teleop stopped -- terminal is yours again"
