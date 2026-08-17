#!/usr/bin/env bash
# Bring the real CR7 up and hand over the jog, in one command.
#
#   terminal A   dobot_bringup_ros2   robot TCP/IP connection
#   terminal B   dobot_joint          FollowJointTrajectory server + /joint_states
#   foreground   tools/jog_action.py  simultaneous multi-joint jog (needs A and B)
#
# See docs/real_robot_jetson_bringup.md 8.3/8.4/8.5 for what each one is.
# Run this on the JETSON HOST -- it calls docker exec.
#
# A and B are started DETACHED and left running when the jog exits: the teach
# commands (test/cbirrt_p1p2_test.py --teach ...) read /joint_states from B, so
# tearing them down after every jog would just mean starting them again. Whatever
# is already up is reused, never restarted, so running this twice is harmless.
#
#   jog_bringup.sh                 bring up what is missing, then jog
#   jog_bringup.sh --real          jog with tools/jog_real.py instead (see below)
#   jog_bringup.sh --enable        ... and enable the robot first (see below)
#   jog_bringup.sh --speed 5       ... other args are passed to the jog tool
#   jog_bringup.sh --enable-only   bring up + enable, then exit without jogging
#   jog_bringup.sh --status        report what is running AND the robot's state
#   jog_bringup.sh --clear-error   clear a collision trip / alarm, then exit
#   jog_bringup.sh --kill-jog      kill a leftover jog, leaving A and B alone
#   jog_bringup.sh --restart       stop A/B first, then bring up and jog
#   jog_bringup.sh --stop          stop A, B and any jog, then exit
#
# The controller comes up DISABLED after a power cycle and rejects motion until
# something calls EnableRobot once. --enable hands that to jog_action.py at
# startup; --enable-only does it without starting a jog, for when the next step
# is a run rather than a jog. Enabling an already-enabled robot is harmless.
#
# A jerky jog can trip the controller's own joint-torque collision detection.
# It then sits in ROBOT_MODE_COLLISION / ERROR and refuses to move -- looking
# exactly like a hang. --status names that state, --clear-error clears it
# (ClearError is documented to clear the collision state; it moves nothing).
# If it keeps tripping, jog slower: the jog streams absolute ServoJ targets, so
# a big jump in one tick reads as a tracking error.
#
# WHICH JOG
# --real runs tools/jog_real.py: one axis at a time, driven by MoveJog, so the
# CONTROLLER does the velocity planning. It also jogs in Cartesian (X/Y/Z/Rx/Ry/
# Rz) as well as joints, and needs terminal A only.
# The default runs tools/jog_action.py: up to four joints at once from a gamepad,
# streamed as absolute ServoJ targets, so the PC does the planning. Joints only,
# needs A and B. Prefer --real unless the simultaneous multi-joint feel is what
# you are after.
#
# Keys are the same layout in both: w/s a/d r/f and u/j i/k o/l, 'e' enables,
# 'x' disables, 'c' clears errors, 'q' quits ('m' toggles TCP/JOINT in --real).
# With a gamepad on jog_action.py, hold L1 (button 4) and use both sticks.
set -euo pipefail

CONTAINER=ros2_dobot
PKG=/root/dobot_ws/src/DOBOT_6Axis_ROS2_V4
LOG_DIR=/root/dobot_ws/log/jog_bringup          # == ~/dobot_ws/log/jog_bringup on the host
ROS_ENV='source /opt/ros/humble/setup.bash; source /root/dobot_ws/install/setup.bash; export DOBOT_TYPE=cr7;'

NODE_A=/dobot_bringup_ros2
NODE_B=/dobot_group_controller

action=jog
jog_tool=jog_action.py
jog_args=()
while [ $# -gt 0 ]; do
    case "$1" in
        --real)        jog_tool=jog_real.py; shift ;;
        --status)      action=status;  shift ;;
        --stop)        action=stop;    shift ;;
        --restart)     action=restart; shift ;;
        --enable-only) action=enable;   shift ;;
        --kill-jog)    action=killjog; shift ;;
        --clear-error) action=clear;   shift ;;
        -h|--help)
            # Print the leading comment block, so this never drifts out of sync
            # with a line-number range.
            awk 'NR>1 && /^#/ {sub(/^# ?/, ""); print; next} NR>1 {exit}' "$0"
            exit 0 ;;
        *)             jog_args+=("$1"); shift ;;   # --enable, --speed N, ...
    esac
done

dexec() { docker exec "$CONTAINER" bash -lc "$1"; }

# One `ros2 node list` costs a couple of seconds, so take a snapshot and query it
# rather than shelling out per node.
NODES=""
refresh_nodes() { NODES="$(dexec "$ROS_ENV timeout 10 ros2 node list 2>/dev/null" || true)"; }
has_node() { printf '%s\n' "$NODES" | grep -qx "$1"; }
node_count() { printf '%s\n' "$NODES" | grep -cx "$1" || true; }

# A SECOND action server is the nastiest failure here: both answer the same goal
# and each streams its own ServoJ targets to the arm, so the two fight and the
# arm lurches until the controller trips on following error. It looks exactly
# like a jog bug. Refuse to jog into that.
check_duplicates() {
    local bad=0 n c
    for n in "$NODE_A" "$NODE_B"; do
        c=$(node_count "$n")
        if [ "$c" -gt 1 ]; then
            echo "  !! $n is running $c TIMES -- an old launch was never stopped" >&2
            bad=1
        fi
    done
    return "$bad"
}

# The PROCESS, not the node list, decides whether a jog is running: a killed
# node lingers in DDS discovery for a while, and a jog that is really gone must
# not keep blocking the next one.
jog_procs() { dexec "pgrep -af 'jog_action\.py|jog_real\.py'" 2>/dev/null || true; }

require_container() {
    if ! docker ps --format '{{.Names}}' | grep -qx "$CONTAINER"; then
        echo "container '$CONTAINER' is not running. Start it first:" >&2
        echo "  docker start $CONTAINER" >&2
        exit 1
    fi
}

start_detached() {   # label, node-to-wait-for, logfile, launch command
    local label=$1 want=$2 log=$3 cmd=$4
    echo "starting $label ..."
    docker exec -d "$CONTAINER" bash -lc \
        "$ROS_ENV mkdir -p $LOG_DIR; exec $cmd > $LOG_DIR/$log 2>&1"
    for _ in $(seq 20); do
        sleep 2
        refresh_nodes
        if has_node "$want"; then
            echo "  $label up ($want)"
            return 0
        fi
    done
    echo "  $label did NOT come up in 40 s. Log: ~/dobot_ws/log/jog_bringup/$log" >&2
    dexec "tail -20 $LOG_DIR/$log" >&2 || true
    return 1
}

ensure_up() {
    refresh_nodes
    if has_node "$NODE_A"; then
        echo "terminal A already up ($NODE_A), reusing it"
    else
        start_detached "terminal A (dobot_bringup_ros2)" "$NODE_A" bringup.log \
            "ros2 launch cr_robot_ros2 dobot_bringup_ros2.launch.py"
    fi
    if has_node "$NODE_B"; then
        echo "terminal B already up ($NODE_B), reusing it"
    else
        start_detached "terminal B (dobot_joint)" "$NODE_B" joint.log \
            "ros2 launch dobot_moveit dobot_joint.launch.py"
    fi
}

enable_robot() {
    # EnableRobot is served by terminal A, which owns the robot's 29999 dashboard
    # socket -- there is no second client allowed, so this has to go through the
    # service rather than a socket of its own.
    echo "enabling the robot ..."
    dexec "$ROS_ENV timeout 25 ros2 service call \
        /dobot_bringup_ros2/srv/EnableRobot dobot_msgs_v4/srv/EnableRobot '{}'" \
        | tail -2
}

robot_state() {
    # Read the controller's real-time feed (30004) directly. It serves several
    # read-only clients at once, so this neither disturbs nor needs terminal A.
    docker exec "$CONTAINER" python3 -c "
import socket, struct
MODES = {1:'INIT', 2:'BRAKE_OPEN', 3:'POWEROFF', 4:'DISABLED', 5:'ENABLE (idle)',
         6:'BACKDRIVE (drag)', 7:'RUNNING', 8:'SINGLE_MOVE', 9:'ERROR',
         10:'PAUSE', 11:'COLLISION'}
try:
    s = socket.create_connection(('192.168.5.1', 30004), timeout=3)
except OSError as e:
    raise SystemExit(f'  robot unreachable: {e}')
buf = b''
while len(buf) < 1440:
    c = s.recv(1440 - len(buf))
    if not c:
        raise SystemExit('  feed closed early')
    buf += c
s.close()
b = lambda off: struct.unpack_from('<b', buf, off)[0]
mode = struct.unpack_from('<Q', buf, 24)[0]
print(f\"  robot_mode : {mode} {MODES.get(mode, '?')}\")
print(f'  enabled    : {b(1026)}   error: {b(1029)}   collision: {b(1038)}')
print('  joints deg : ' + ' '.join(f'{v:+7.1f}'
      for v in struct.unpack_from('<6d', buf, 432)))
if mode in (9, 11) or b(1029) or b(1038):
    print('  -> tripped/alarmed: clear it with --clear-error before jogging')
" || true
}

clear_error() {
    echo "clearing the controller alarm / collision state ..."
    dexec "$ROS_ENV timeout 25 ros2 service call \
        /dobot_bringup_ros2/srv/ClearError dobot_msgs_v4/srv/ClearError '{}'" \
        | tail -2
}

stop_all() {
    echo "stopping jog, terminal B and terminal A ..."
    # The launch file and the nodes it spawned are separate processes; killing
    # only `ros2 launch` can leave the nodes holding the robot's dashboard socket
    # -- and a leftover action server is worse than useless, because a second one
    # answers the same goals and streams its own ServoJ stream to the arm.
    #
    # Every pattern is bracketed ([j]og_action) so it cannot match the very shell
    # running these pkills: that shell's own command line contains all of them
    # verbatim, so an unbracketed `pkill -f jog_action.py` kills the shell first
    # and nothing after it ever runs.
    dexec "pkill -f '[j]og_action\.py'; \
           pkill -f '[j]og_real\.py'; \
           pkill -f '[d]obot_joint\.launch\.py'; \
           pkill -f '[d]obot_bringup_ros2\.launch\.py'; \
           pkill -f '[a]ction_move_server'; \
           pkill -f '[d]obot_joint_states'; \
           pkill -f '[d]obot_bringup_ros2'; \
           pkill -f '[c]r_robot_ros2_node'; \
           sleep 2; true"
}

require_container

case "$action" in
    status)
        refresh_nodes
        echo "container $CONTAINER: running"
        for n in "$NODE_A" "$NODE_B"; do
            c=$(node_count "$n")
            case "$c" in
                0) echo "  $n : down" ;;
                1) echo "  $n : UP" ;;
                *) echo "  $n : UP x$c   <<< DUPLICATE, run --stop then start once" ;;
            esac
        done
        running_jog="$(jog_procs)"
        if [ -n "$running_jog" ]; then
            echo "  jog tool      : UP"
            printf '    %s\n' "$running_jog"
        else
            echo "  jog tool      : down"
        fi
        echo "robot:"
        robot_state
        exit 0
        ;;
    clear)
        ensure_up
        clear_error
        echo "robot:"
        robot_state
        exit 0
        ;;
    stop)
        stop_all
        refresh_nodes
        for n in "$NODE_A" "$NODE_B"; do
            has_node "$n" && echo "  $n : still UP (kill it by hand)" || echo "  $n : stopped"
        done
        [ -n "$(jog_procs)" ] && echo "  jog tool      : still UP (kill it by hand)" \
                              || echo "  jog tool      : stopped"
        exit 0
        ;;
    killjog)
        # A jog started in a terminal that is gone cannot be quit with 'q', and
        # it keeps the guard below tripping. Kill just it: A and B are what the
        # teach commands need, so they stay.
        echo "killing any running jog tool ..."
        dexec "pkill -f '[j]og_action\.py'; pkill -f '[j]og_real\.py'; sleep 1; true"
        left="$(jog_procs)"
        if [ -n "$left" ]; then
            echo "  still running -- kill it by hand:" >&2
            printf '%s\n' "$left" >&2
            exit 1
        fi
        echo "  gone. A and B untouched."
        exit 0
        ;;
    enable)
        ensure_up
        enable_robot
        echo
        echo "robot enabled; A and B left running."
        echo "next: $PKG/test/run.sh --teach p1   (or --dry / --run)"
        exit 0
        ;;
    restart)
        stop_all
        ;;
esac

refresh_nodes
running_jog="$(jog_procs)"
if [ -n "$running_jog" ]; then
    # Two joggers would race, each streaming its own goals to the same arm.
    echo "a jog is ALREADY running:" >&2
    printf '%s\n\n' "$running_jog" >&2
    echo "press 'q' in its terminal, or:" >&2
    echo "  $0 --kill-jog       kill it, leave A and B up" >&2
    echo "  $0 --enable-only    just enable the robot, no jog" >&2
    exit 1
fi

ensure_up

refresh_nodes
if ! check_duplicates; then
    echo >&2
    echo "Two of the same launch are up. Both action servers answer every goal" >&2
    echo "and each drives the arm on its own -- that is a lurching arm, not a" >&2
    echo "jog bug. Clear it before jogging:" >&2
    echo "  $0 --stop     then start ONE set (or just: $0)" >&2
    exit 1
fi

jog_cmd="python3 $PKG/tools/$jog_tool"
for arg in ${jog_args[@]+"${jog_args[@]}"}; do
    jog_cmd+=" $(printf '%q' "$arg")"
done

echo
echo "handing over to $jog_tool -- 'q' quits the jog; A and B stay up"
echo "then teach with: $PKG/test/run.sh --teach p1   (etc.)"
echo
exec docker exec -it "$CONTAINER" bash -lc "$ROS_ENV exec $jog_cmd"
