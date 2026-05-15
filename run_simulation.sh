#!/bin/bash
# DOBOT CR7 simulation launch script

export DISPLAY=:0
export DOBOT_TYPE=cr7

source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/local_setup.bash

echo "=== [1/2] Gazebo + controller starting ==="
ros2 launch dobot_gazebo gazebo_moveit.launch.py &
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
