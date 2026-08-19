#!/bin/bash
# REAL robot, one command (mirror of run_test.sh for the sim): the stub MCS bridge
# (TCP -> /mcs/command,/mcs/stop) + arm.launch.py profile:=real (static TF +
# vision_hover_node + main.py --profile real), all inside the ros2_dobot container.
#
# Run on the JETSON HOST:
#     ~/dobot_ws/run_real.sh                       # bridge -> 127.0.0.1:9100 (mcs_server stub)
#     ~/dobot_ws/run_real.sh --host 10.0.0.5 --port 9100   # real 관제
#
# Before this: bringup + dobot_joint.launch up (container), vision_runner up (host),
# real.json measured, obs taught -- see docs/manual.md 5. Without a 관제/stub the
# bridge just keeps retrying; a mission can still be injected by hand:
#     ros2 topic pub --once /mcs/command std_msgs/msg/String '{data: "{\"TargetID\": 0, \"Command\": 0}"}'
# When the comms team's node replaces the bridge, run the launch line alone.
TTY_FLAGS=""
[ -t 0 ] && TTY_FLAGS="-it"
exec docker exec $TTY_FLAGS ros2_dobot bash -lc '
source /opt/ros/humble/setup.bash
source /root/dobot_ws/install/setup.bash
export DOBOT_TYPE=cr7
PKG=/root/dobot_ws/src/DOBOT_6Axis_ROS2_V4
PY=/root/dobot_ws/.venv/bin/python3
$PY $PKG/comms/mcs_bridge.py "$@" &          # TCP -> /mcs/command,/mcs/stop
BRIDGE_PID=$!
trap "kill $BRIDGE_PID 2>/dev/null" EXIT     # stop bridge when launch exits
ros2 launch $PKG/launch/arm.launch.py profile:=real
' _ "$@"
