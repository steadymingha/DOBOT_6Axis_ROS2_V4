#!/bin/bash
# Local test wiring in one command: the stub bridge + arm.launch.py (vision +
# dispatcher). Run the MCS server (관제 stub) SEPARATELY in its own terminal so you
# can type commands and watch its output:
#
#     /usr/bin/python3 ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/comms/mcs_server.py
#
# Then type 'A START' there -> bridge receives it over TCP -> /mcs/command -> main.py
# runs the transfer. The bridge retries until the server is up, so start order is free.
#
# For real operation the 관제 system + comms team's node replace the server + bridge;
# use run_arm.sh (arm side only) then.

source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/local_setup.bash
PKG=~/dobot_ws/src/DOBOT_6Axis_ROS2_V4

/usr/bin/python3 "$PKG/comms/mcs_bridge.py" &      # TCP -> /mcs/command,/mcs/stop
BRIDGE_PID=$!
trap 'kill $BRIDGE_PID 2>/dev/null' EXIT           # stop bridge when launch exits

ros2 launch "$PKG/launch/arm.launch.py"            # vision + dispatcher (foreground)
