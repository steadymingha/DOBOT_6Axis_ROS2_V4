#!/bin/bash
# CR7 on MPO-700 AGV simulation launch script (Gazebo Classic + MoveIt).
# Step 1 brings up Gazebo with the combined AGV+CR7 and the ros2_control
# controllers (joint_state_broadcaster, cr7_group_controller, gripper_controller).
# Step 2 brings up MoveIt move_group + RViz so cbirrt_pick_place.py can plan
# (/compute_ik, /check_state_validity). Drive the AGV with teleop_agv.sh.

export DISPLAY=:0
export DOBOT_TYPE=cr7
export GAZEBO_MODEL_PATH=$GAZEBO_MODEL_PATH:/home/user/dobot_ws/src/blender

source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/local_setup.bash

echo "=== [1/2] Gazebo + controllers starting (CR7 on MPO-700) ==="
ros2 launch dobot_gazebo gazebo_mpo700_cr7.launch.py &
GAZEBO_PID=$!
echo "Gazebo PID: $GAZEBO_PID"

echo "Gazebo initialization waiting (10 seconds)..."
sleep 10

echo "=== [2/2] MoveIt + RViz starting ==="
ros2 launch dobot_moveit moveit_gazebo.launch.py &
MOVEIT_PID=$!
echo "MoveIt PID: $MOVEIT_PID"

echo "All started. Press Ctrl+C to exit"
wait
