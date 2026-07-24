#!/bin/bash
# Generate the Isaac Sim URDF from the same combined xacro the Gazebo sim uses.
# use_gazebo:=false drops the Gazebo-only includes (planar move, lidars, wheel
# friction); the ros2_control hardware plugin is swapped for
# topic_based_ros2_control so a standalone controller_manager talks to the
# Isaac ROS 2 bridge topics instead of gazebo_ros2_control.
set -e
source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/local_setup.bash
OUT=~/dobot_ws/isaac/cr7_on_mpo700_isaac.urdf
xacro "$(ros2 pkg prefix cra_description)/share/cra_description/urdf/cr7_on_mpo700.urdf.xacro" use_gazebo:=false > "$OUT"
python3 - "$OUT" <<'EOF'
import os
import re
import subprocess
import sys

p = sys.argv[1]
src = open(p).read()
src = src.replace(
    "<plugin>gazebo_ros2_control/GazeboSystem</plugin>",
    "<plugin>topic_based_ros2_control/TopicBasedSystem</plugin>\n"
    '        <param name="joint_commands_topic">/isaac_joint_commands</param>\n'
    '        <param name="joint_states_topic">/isaac_joint_states</param>')
assert "TopicBasedSystem" in src, "hardware plugin swap failed"

# Velocity command interface on the six arm joints: velocity feed-forward into
# the PhysX drive damping term (without it the 60 Hz stepped position targets
# judder). Isaac-only -- the Gazebo variant must stay position-only, which is
# also why isaac/ros2_controllers.yaml is a separate copy.
old = "</command_interface>\n      <state_interface"
new = ("</command_interface>\n"
       '      <command_interface name="velocity"/>\n'
       "      <state_interface")
assert src.count(old) == 7, "expected 6 arm joints + gripper, got %d" % src.count(old)
src = src.replace(old, new, 6)

# Resolve package:// mesh URIs to absolute file:// paths (the Isaac URDF
# importer names mesh prims after the file; hyphenated names like
# MPO-700-BODY.dae are invalid USD prim names, so route those through
# an underscore symlink).
share = {}
def resolve(m):
    pkg, rel = m.group(1), m.group(2)
    if pkg not in share:
        share[pkg] = subprocess.check_output(
            ["ros2", "pkg", "prefix", "--share", pkg], text=True).strip()
    path = os.path.join(share[pkg], rel)
    base = os.path.basename(path)
    if "-" in base:
        safe = os.path.join(os.path.dirname(os.path.realpath(path)),
                            base.replace("-", "_"))
        if not os.path.exists(safe):
            os.symlink(os.path.realpath(path), safe)
        path = safe
    return "file://" + path

src = re.sub(r"package://([^/]+)/([^\"]+)", resolve, src)
open(p, "w").write(src)
print(f"wrote {p}")
EOF
