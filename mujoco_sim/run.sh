#!/bin/bash
# View the cr.world scene (CR7 on MPO-700 AGV + shelf/wirebonder/boxes) in
# MuJoCo. Visualization only. See README.md.
#
# Deps (once):  uv venv .venv && uv pip install --python .venv/bin/python mujoco trimesh pycollada
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
PY="$HERE/.venv/bin/python"
[ -x "$PY" ] || { echo "venv missing -- run: cd $HERE && uv venv .venv && uv pip install --python .venv/bin/python mujoco trimesh pycollada"; exit 1; }

# Strip conda so `xacro` uses ROS's pyyaml (same reason as run_mpo700_cr7.sh).
if [ -n "$CONDA_PREFIX" ]; then
    echo "conda active ($CONDA_PREFIX); removing it from PATH for this run"
    PATH=$(echo "$PATH" | tr ':' '\n' | grep -v "$CONDA_PREFIX" | paste -sd:)
    export PATH
    unset CONDA_PREFIX
fi

source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/local_setup.bash

# mujoco/trimesh live in the .venv; xacro/ros2 are called as subprocesses and
# use ROS's own python, so the two don't collide.
exec "$PY" "$HERE/build_scene.py" "$@"
