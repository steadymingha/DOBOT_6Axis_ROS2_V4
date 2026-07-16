#!/bin/bash
# Swappable Dobot arm on the MPO-700 AGV in the cr.world (shelf/wirebonder/post_wb),
# Gazebo Classic only -- NO MoveIt. Used to eyeball arm reachability with jog_tcp.py.
#
# Swap the arm by changing ARM_TYPE below (cr10/cr12/cr16/cr20). Each needs its
# urdf/<arm>_arm.xacro to exist (see arm_on_mpo700.urdf.xacro); rebuild cra_description
# after adding one. The CR7 run (run_mpo700_cr7.sh) is untouched by this script.
#
# jog: ARM_TYPE is exported, so cr7_pnp/geometry.py loads the matching arm model:
#   ARM_TYPE=cr10 ~/dobot_ws/.venv/bin/python3 ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/tools/jog_tcp.py

export ARM_TYPE=cr10          # <-- change to cr12 / cr16 / cr20 to swap the arm

export DISPLAY=:0
export __GLX_VENDOR_LIBRARY_NAME=nvidia
export GAZEBO_MODEL_PATH=$GAZEBO_MODEL_PATH:/home/user/dobot_ws/src/blender
export ROS_LOCALHOST_ONLY=1   # single-machine cell; see run_mpo700_cr7.sh note
export FASTRTPS_DEFAULT_PROFILES_FILE=$HOME/dobot_ws/fastdds_localhost.xml

# ponytail: strip conda from PATH so python3 -> /usr/bin/python3 (conda base shadows
# ROS's pyyaml -> xacro dies -> empty robot_description). Don't 'exit' if sourced.
if [ -n "$CONDA_PREFIX" ]; then
    echo "conda active ($CONDA_PREFIX); removing it from PATH for this run"
    PATH=$(echo "$PATH" | tr ':' '\n' | grep -v "$CONDA_PREFIX" | paste -sd:)
    export PATH
    unset CONDA_PREFIX
fi

source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/local_setup.bash

echo "=== Gazebo + controllers starting (${ARM_TYPE} on MPO-700, no MoveIt) ==="
ros2 launch dobot_gazebo gazebo_mpo700_arm.launch.py &
GAZEBO_PID=$!
echo "Gazebo PID: $GAZEBO_PID"

echo "Started. jog with:"
echo "  ARM_TYPE=${ARM_TYPE} ~/dobot_ws/.venv/bin/python3 ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/tools/jog_tcp.py"
echo "Press Ctrl+C to exit"
wait
