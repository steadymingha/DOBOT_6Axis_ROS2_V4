#!/usr/bin/env bash
# Run any tools/*.py (or main.py) inside the ros2_dobot container with the workspace
# sourced, the pinocchio venv python and DOBOT_ENV=real. Same shape as test/run.sh.
#
# Run this on the JETSON HOST, from anywhere:
#     ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/tools/run.sh check_real_robot.py
#     ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/tools/run.sh teach_env.py --teach-shelf-yaw
#     ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/tools/run.sh teach_env.py --teach-surface z-
#     ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/tools/run.sh main.py --profile real --preflight
set -eu

TTY_FLAGS=""
[ -t 0 ] && TTY_FLAGS="-it"          # a TTY only when there is one to hand over

exec docker exec $TTY_FLAGS ros2_dobot bash -lc '
source /opt/ros/humble/setup.bash
source /root/dobot_ws/install/setup.bash
export DOBOT_TYPE=cr7
export DOBOT_ENV=${DOBOT_ENV:-real}
cd /root/dobot_ws/src/DOBOT_6Axis_ROS2_V4
script="$1"; shift
case "$script" in */*) ;; main.py) ;; *) script="tools/$script" ;; esac
exec /root/dobot_ws/.venv/bin/python3 "$script" "$@"
' _ "$@"
