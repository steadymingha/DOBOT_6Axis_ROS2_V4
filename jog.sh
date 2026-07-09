source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/local_setup.bash

# Selects which arm model jog_tcp loads for IK/collision (cr7_pnp/geometry.py).
# Must match the running sim's arm. Set cr7 (or unset) when jogging the CR7 sim.
export ARM_TYPE=cr10

exec uv run ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/tools/jog_tcp.py
