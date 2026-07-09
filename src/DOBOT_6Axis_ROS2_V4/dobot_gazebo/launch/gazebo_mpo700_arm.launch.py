import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch_ros.actions import Node
from launch.actions import ExecuteProcess, IncludeLaunchDescription, RegisterEventHandler
from launch.event_handlers import OnProcessExit
from launch.launch_description_sources import PythonLaunchDescriptionSource
import xacro


def generate_launch_description():
    # Swappable Dobot arm on the MPO-700 AGV, no MoveIt. The arm is chosen by the
    # ARM_TYPE env var (default cr10) which selects urdf/<arm>_arm.xacro through the
    # parametric arm_on_mpo700.urdf.xacro. cr12/16/20 work once their <arm>_arm.xacro
    # exists -- no change here. This is a sibling of gazebo_mpo700_cr7.launch.py; the
    # CR7 launch is left untouched.
    arm_type = os.getenv('ARM_TYPE', 'cr10')
    robot_name_in_model = f'{arm_type}_on_mpo700'
    package_name = 'cra_description'
    urdf_name = 'arm_on_mpo700.urdf.xacro'

    world_path = os.path.join(
        get_package_share_directory('dobot_gazebo'),
        'worlds',
        'cr.world'
    )
    gazebo = IncludeLaunchDescription(
        PythonLaunchDescriptionSource([os.path.join(
            get_package_share_directory('gazebo_ros'), 'launch'), '/gazebo.launch.py']),
        launch_arguments={'world': world_path}.items(),
    )

    cra_description_path = get_package_share_directory(package_name)
    xacro_file = os.path.join(cra_description_path, 'urdf', urdf_name)

    # Pass arm_type so the right <arm>_arm.xacro is included.
    doc = xacro.process_file(xacro_file, mappings={'arm_type': arm_type})
    robot_description_config = doc.toxml()
    # Strip XML declaration: gazebo_ros2_control (Humble 0.4.x) passes robot_description
    # to its internal controller_manager via rcl --param, and the '<?xml ?>' preamble
    # trips the rcl argument parser, preventing controller_manager from starting.
    if robot_description_config.startswith('<?xml'):
        robot_description_config = robot_description_config[
            robot_description_config.index('?>') + 2:].lstrip()
    robot_description = {'robot_description': robot_description_config}

    node_robot_state_publisher = Node(
        package='robot_state_publisher',
        executable='robot_state_publisher',
        output='screen',
        parameters=[robot_description]
    )

    # Spawn the combined robot at the CR7 "sweet spot" in front of the shelf.
    spawn_entity = Node(package='gazebo_ros', executable='spawn_entity.py',
                        arguments=['-topic', 'robot_description',
                                   '-entity', robot_name_in_model,
                                   '-x', '2.16', '-y', '-1.08', '-z', '-0.005'],
                        output='screen')

    # Controllers come from cr7_moveit/config/ros2_controllers.yaml (referenced by the
    # xacro); joint names (joint1..6) are identical across all CR arms, so the same
    # controller names work for any arm_type.
    load_joint_state_controller = ExecuteProcess(
        cmd=['ros2', 'control', 'load_controller', '--set-state', 'active',
             'joint_state_broadcaster'],
        output='screen'
    )
    load_joint_trajectory_controller = ExecuteProcess(
        cmd=['ros2', 'control', 'load_controller', '--set-state', 'active',
             'cr7_group_controller'],
        output='screen'
    )
    load_gripper_controller = ExecuteProcess(
        cmd=['ros2', 'control', 'load_controller', '--set-state', 'active',
             'gripper_controller'],
        output='screen'
    )

    close_evt1 = RegisterEventHandler(
        event_handler=OnProcessExit(
            target_action=spawn_entity,
            on_exit=[load_joint_state_controller],
        )
    )
    close_evt2 = RegisterEventHandler(
        event_handler=OnProcessExit(
            target_action=load_joint_state_controller,
            on_exit=[load_joint_trajectory_controller],
        )
    )
    close_evt3 = RegisterEventHandler(
        event_handler=OnProcessExit(
            target_action=load_joint_trajectory_controller,
            on_exit=[load_gripper_controller],
        )
    )

    return LaunchDescription([
        close_evt1,
        close_evt2,
        close_evt3,
        gazebo,
        node_robot_state_publisher,
        spawn_entity
    ])
