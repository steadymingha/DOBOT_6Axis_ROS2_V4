#!/bin/bash
# Kill all leftover Gazebo / ROS2 simulation processes.
# Use before re-launching to avoid stale robot_state_publisher / gzserver clashes.

# action_move_server / dobot_moveit joint_states are the REAL-ROBOT bridge
# (dobot_joint.launch.py): the bridge serves the SAME
# /<type>_group_controller/follow_joint_trajectory action as the sim controller
# -- if it survives, every client sees TWO action servers and results cross.
PATTERNS="gzserver gzclient spawn_entity robot_state_publisher move_group ros2_control_node controller_manager rqt_image_view image_view view_d405 action_move_server dobot_moveit/joint_states ros2 launch"

# Arm-side nodes: stale duplicates here publish conflicting /vision/device_pose (two
# vision nodes) or double-consume commands, which silently corrupts a capture. Kill
# them too. main.py pattern is path-qualified so it can't match unrelated main.py.
ARM_PATTERNS="wirebonder_vision_node wirebonder_pick_place shelf_pick_place DOBOT_6Axis_ROS2_V4/main.py mcs_bridge mcs_server"

echo "=== Before ==="
ps aux | grep -E "gzserver|gzclient|spawn_entity|robot_state_publisher|move_group|ros2_control_node|controller_manager|rqt_image_view|image_view|view_d405|ros2 launch" | grep -v grep | wc -l

for p in $PATTERNS $ARM_PATTERNS; do
  pkill -9 -f "$p" 2>/dev/null
done

# Fallback: kill by PID in case pattern matching missed anything
ps aux | grep -E "gzserver|gzclient|spawn_entity|robot_state_publisher|move_group|ros2_control_node|controller_manager|rqt_image_view|image_view|view_d405|action_move_server|dobot_moveit/joint_states" \
  | grep -v grep | awk '{print $2}' | xargs -r kill -9 2>/dev/null

sleep 1
REMAIN=$(ps aux | grep -E "gzserver|gzclient|spawn_entity|robot_state_publisher|move_group|ros2_control_node|controller_manager|rqt_image_view|image_view|view_d405|action_move_server|dobot_moveit/joint_states" | grep -v grep | wc -l)
echo "=== After: $REMAIN remaining ==="
if [ "$REMAIN" -eq 0 ]; then
  echo "All simulation processes cleaned up."
else
  echo "WARNING: $REMAIN process(es) still alive; check manually with 'ps aux | grep gazebo'."
fi

# Verify no stale arm nodes remain -- a lingering 2nd vision node is what silently
# poisons /vision/device_pose (correct capture, wrong pose read by the transfer).
ARM_REMAIN=$(pgrep -af "wirebonder_vision_node|wirebonder_pick_place|shelf_pick_place|DOBOT_6Axis_ROS2_V4/main.py|mcs_bridge|mcs_server" | grep -vE "pgrep|bash -c" | wc -l)
echo "=== Arm nodes: $ARM_REMAIN remaining ==="
if [ "$ARM_REMAIN" -ne 0 ]; then
  echo "WARNING: stale arm node(s) alive -- 'ros2 topic info /vision/device_pose' should show Publisher count: 1"
fi