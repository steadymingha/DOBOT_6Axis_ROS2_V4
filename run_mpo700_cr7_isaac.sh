#!/bin/bash
# CR7 on MPO-700 simulation launch script -- Isaac Sim edition.
# Mirrors run_mpo700_cr7.sh but replaces Gazebo with Isaac Sim:
#   step 1  Isaac Sim (isaac/isaac_sim.py): physics, camera, /clock,
#           /isaac_joint_states|commands, /gazebo/model_states, /ATTACHLINK
#   step 2  ros2_control stack (isaac/controllers.launch.py)
#   step 3  MoveIt move_group + RViz (unchanged launch)
#   step 4  D405 viewer (unchanged)
# main.py / tag_vision_node.py run unchanged on top of this.

export DISPLAY=:0
export DOBOT_TYPE=cr7
export ROS_LOCALHOST_ONLY=1
export FASTRTPS_DEFAULT_PROFILES_FILE=$HOME/dobot_ws/fastdds_localhost.xml

# conda guard (same reason as run_mpo700_cr7.sh: conda python shadows ROS pyyaml)
if [ -n "$CONDA_PREFIX" ]; then
    echo "conda active ($CONDA_PREFIX); removing it from PATH for this run"
    PATH=$(echo "$PATH" | tr ':' '\n' | grep -v "$CONDA_PREFIX" | paste -sd:)
    export PATH
    unset CONDA_PREFIX
fi

source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/local_setup.bash

echo "=== [1/4] Isaac Sim starting (CR7 on MPO-700) ==="

~/isaacsim-venv/bin/python3 ~/dobot_ws/isaac/isaac_sim.py "$@" &
ISAAC_PID=$!
echo "Isaac PID: $ISAAC_PID"

echo "Waiting for /isaac_joint_states (Isaac bring-up)..."
until ros2 topic list 2>/dev/null | grep -q '^/isaac_joint_states$'; do
    sleep 2
    kill -0 $ISAAC_PID 2>/dev/null || { echo "Isaac Sim died during bring-up"; exit 1; }
done

echo "=== [2/4] ros2_control (topic_based) starting ==="
ros2 launch ~/dobot_ws/isaac/controllers.launch.py &
CONTROL_PID=$!

# Spawners intermittently fail while Isaac loads (controller left unconfigured);
# re-activate until all three are active.
(
  for i in $(seq 1 30); do
    sleep 5
    n=$(ros2 control list_controllers 2>/dev/null | grep -c active)
    [ "$n" = "3" ] && echo "[controllers] all active" && break
    for c in joint_state_broadcaster cr7_group_controller gripper_controller; do
      ros2 control list_controllers 2>/dev/null | grep "$c" | grep -q active && continue
      if ros2 control list_controllers 2>/dev/null | grep -q "$c"; then
        ros2 control set_controller_state "$c" inactive >/dev/null 2>&1
        ros2 control set_controller_state "$c" active >/dev/null 2>&1
      else
        # spawner died during a slow bring-up (e.g. first --env asset download)
        ros2 run controller_manager spawner "$c" >/dev/null 2>&1
      fi
    done
  done
) &

echo "=== [3/4] MoveIt + RViz starting ==="
ros2 launch dobot_moveit moveit_gazebo.launch.py &
MOVEIT_PID=$!

echo "=== [4/4] D405 eye-in-hand camera view ==="
sleep 8
python3 ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/debug/view_d405.py /camera/d405/color/image_raw &
VIEW_PID=$!
## headless mode is not supported in Isaac Sim, so we don't skip the D405 viewer. The following code is commented out for reference.
# if [[ " $* " == *" --headless "* ]]; then
#     echo "=== [4/4] D405 viewer skipped (--headless) ==="
#     VIEW_PID=
# else
#     echo "=== [4/4] D405 eye-in-hand camera view ==="
#     sleep 8
#     python3 ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/debug/view_d405.py /camera/d405/color/image_raw &
#     VIEW_PID=$!
# fi

echo "All started. Press Ctrl+C to exit"
trap "kill $ISAAC_PID $CONTROL_PID $MOVEIT_PID $VIEW_PID 2>/dev/null" EXIT
wait
