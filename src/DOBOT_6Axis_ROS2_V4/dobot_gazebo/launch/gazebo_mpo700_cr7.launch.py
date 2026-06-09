import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch_ros.actions import Node
from launch.actions import ExecuteProcess, IncludeLaunchDescription, RegisterEventHandler
from launch.event_handlers import OnProcessExit
from launch.launch_description_sources import PythonLaunchDescriptionSource
import xacro


def generate_launch_description():
    # CR7 mounted on the MPO-700 AGV (single combined URDF).
    robot_name_in_model = 'cr7_on_mpo700'
    package_name = 'cra_description'
    urdf_name = 'cr7_on_mpo700.urdf.xacro'

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

    doc = xacro.parse(open(xacro_file))
    xacro.process_doc(doc)
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

    # Spawn the combined robot. MPO-700 wheel bottom is ~mpo_base_link+0.01,
    # so z=0 lets the wheels rest on the ground. Change -x/-y to place the AGV
    # somewhere else (e.g. in front of the shelf).
    spawn_entity = Node(package='gazebo_ros', executable='spawn_entity.py',
                        arguments=['-topic', 'robot_description',
                                   '-entity', robot_name_in_model,
                                   '-x', '0', '-y', '0', '-z', '0.0'],
                        output='screen')

    # Joint state broadcaster
    load_joint_state_controller = ExecuteProcess(
        cmd=['ros2', 'control', 'load_controller', '--set-state', 'active',
             'joint_state_broadcaster'],
        output='screen'
    )

    # CR7 arm trajectory controller
    load_joint_trajectory_controller = ExecuteProcess(
        cmd=['ros2', 'control', 'load_controller', '--set-state', 'active',
             'cr7_group_controller'],
        output='screen'
    )

    # Gripper position controller
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
