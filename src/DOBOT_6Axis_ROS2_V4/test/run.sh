#!/usr/bin/env bash
# Run cbirrt_p1p2_test.py inside the ros2_dobot container with the workspace
# sourced and the pinocchio venv picked, forwarding every argument.
#
# Run this on the JETSON HOST (it calls docker exec), from anywhere:
#     ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/test/run.sh --teach-table
#     ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/test/run.sh --teach p1
#     ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/test/run.sh --run --cycles 3
#
# Passing the arguments after the `_` matters: `bash -c 'script' arg` makes arg
# $0, not $1, so a bare `bash -c "..." --teach-table` silently drops the flag
# and argparse sees no mode at all.
set -eu

TTY_FLAGS=""
[ -t 0 ] && TTY_FLAGS="-it"          # a TTY only when there is one to hand over

exec docker exec $TTY_FLAGS ros2_dobot bash -lc '
source /opt/ros/humble/setup.bash
source /root/dobot_ws/install/setup.bash
export DOBOT_TYPE=cr7
cd /root/dobot_ws/src/DOBOT_6Axis_ROS2_V4
exec /root/dobot_ws/.venv/bin/python3 test/cbirrt_p1p2_test.py "$@"
' _ "$@"
