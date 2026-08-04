#!/bin/bash
# Real-robot jogger. Needs only terminal A (dobot_bringup_ros2.launch.py) up.
source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/setup.bash
exec python3 ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/tools/jog_real.py "$@"
