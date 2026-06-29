#!/bin/bash
# Kill all leftover Gazebo / ROS2 simulation processes.
# Use before re-launching to avoid stale robot_state_publisher / gzserver clashes.

PATTERNS="gzserver gzclient spawn_entity robot_state_publisher move_group ros2_control_node controller_manager rqt_image_view image_view view_d405 ros2 launch"

echo "=== Before ==="
ps aux | grep -E "gzserver|gzclient|spawn_entity|robot_state_publisher|move_group|ros2_control_node|controller_manager|rqt_image_view|image_view|view_d405|ros2 launch" | grep -v grep | wc -l

for p in $PATTERNS; do
  pkill -9 -f "$p" 2>/dev/null
done

# Fallback: kill by PID in case pattern matching missed anything
ps aux | grep -E "gzserver|gzclient|spawn_entity|robot_state_publisher|move_group|ros2_control_node|controller_manager|rqt_image_view|image_view|view_d405" \
  | grep -v grep | awk '{print $2}' | xargs -r kill -9 2>/dev/null

sleep 1
REMAIN=$(ps aux | grep -E "gzserver|gzclient|spawn_entity|robot_state_publisher|move_group|ros2_control_node|controller_manager|rqt_image_view|image_view|view_d405" | grep -v grep | wc -l)
echo "=== After: $REMAIN remaining ==="
if [ "$REMAIN" -eq 0 ]; then
  echo "All simulation processes cleaned up."
else
  echo "WARNING: $REMAIN process(es) still alive; check manually with 'ps aux | grep gazebo'."
fi