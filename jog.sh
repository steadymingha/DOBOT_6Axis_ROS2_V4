source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/local_setup.bash

exec uv run ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/jog_tcp.py
