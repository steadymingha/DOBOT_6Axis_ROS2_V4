#include <rclcpp/rclcpp.hpp>
#include <rclcpp_action/rclcpp_action.hpp>
#include <geometry_msgs/msg/pose_stamped.hpp>
#include <moveit_msgs/srv/get_position_ik.hpp>
#include <control_msgs/action/follow_joint_trajectory.hpp>
#include <trajectory_msgs/msg/joint_trajectory_point.hpp>

#include <algorithm>
#include <chrono>
#include <thread>
#include <cstdlib>

using namespace std::chrono_literals;
using GetPositionIK      = moveit_msgs::srv::GetPositionIK;
using FollowJointTraj    = control_msgs::action::FollowJointTrajectory;

class DobotSimController : public rclcpp::Node
{
public:
    DobotSimController() : Node("dobot_sim_controller")
    {
        ik_client_ = this->create_client<GetPositionIK>("/compute_ik");

        const char * env = std::getenv("DOBOT_TYPE");
        std::string robot_type = env ? env : "cr7";
        std::string action_name = "/" + robot_type + "_group_controller/follow_joint_trajectory";
        traj_client_ = rclcpp_action::create_client<FollowJointTraj>(this, action_name);
    }

    bool move_to_coordinate(double x, double y, double z)
    {
        // --- Step 1: IK ---
        if (!ik_client_->wait_for_service(5s)) {
            RCLCPP_ERROR(get_logger(), "IK service not available");
            return false;
        }

        const char * env = std::getenv("DOBOT_TYPE");
        std::string robot_type = env ? env : "cr7";

        auto req = std::make_shared<GetPositionIK::Request>();
        req->ik_request.group_name = robot_type + "_group";

        geometry_msgs::msg::PoseStamped pose;
        pose.header.frame_id = "base_link";
        pose.pose.position.x = x;
        pose.pose.position.y = y;
        pose.pose.position.z = z;
        pose.pose.orientation.x = 0.0;
        pose.pose.orientation.y = 1.0;  // end-effector pointing downward
        pose.pose.orientation.z = 0.0;
        pose.pose.orientation.w = 0.0;
        req->ik_request.pose_stamped = pose;
        req->ik_request.timeout.sec  = 2;

        RCLCPP_INFO(get_logger(), "Target [X:%.2f, Y:%.2f, Z:%.2f] - Calculating IK...", x, y, z);

        auto ik_future = ik_client_->async_send_request(req);
        if (rclcpp::spin_until_future_complete(shared_from_this(), ik_future) !=
            rclcpp::FutureReturnCode::SUCCESS)
        {
            RCLCPP_ERROR(get_logger(), "IK service call failed");
            return false;
        }

        auto res = ik_future.get();
        if (res->error_code.val != 1) {
            RCLCPP_ERROR(get_logger(), "IK Failed (Code: %d)", res->error_code.val);
            return false;
        }

        // Extract the 6 joint angles computed by MoveIt
        const std::vector<std::string> joint_names = {
            "joint1", "joint2", "joint3", "joint4", "joint5", "joint6"
        };
        std::vector<double> target_positions;
        const auto & state = res->solution.joint_state;

        for (const auto & name : joint_names) {
            auto it = std::find(state.name.begin(), state.name.end(), name);
            if (it == state.name.end()) {
                RCLCPP_ERROR(get_logger(), "Joint '%s' not found in IK solution", name.c_str());
                return false;
            }
            target_positions.push_back(state.position[std::distance(state.name.begin(), it)]);
        }

        // --- Step 2: Send Trajectory ---
        if (!traj_client_->wait_for_action_server(5s)) {
            RCLCPP_ERROR(get_logger(), "Trajectory action server not available");
            return false;
        }

        FollowJointTraj::Goal goal;
        goal.trajectory.joint_names = joint_names;

        trajectory_msgs::msg::JointTrajectoryPoint point;
        point.positions        = target_positions;
        point.time_from_start.sec = 3;  // smoothly reach the target over 3 seconds
        goal.trajectory.points.push_back(point);

        RCLCPP_INFO(get_logger(), "Path found! Sending command to Gazebo...");

        auto send_goal_future = traj_client_->async_send_goal(goal);
        if (rclcpp::spin_until_future_complete(shared_from_this(), send_goal_future) !=
            rclcpp::FutureReturnCode::SUCCESS)
        {
            RCLCPP_ERROR(get_logger(), "Failed to send goal");
            return false;
        }

        auto goal_handle = send_goal_future.get();
        if (!goal_handle) {
            RCLCPP_ERROR(get_logger(), "Goal was rejected by the action server");
            return false;
        }

        RCLCPP_INFO(get_logger(), "Robot is moving in Gazebo!");

        auto result_future = traj_client_->async_get_result(goal_handle);
        if (rclcpp::spin_until_future_complete(shared_from_this(), result_future) !=
            rclcpp::FutureReturnCode::SUCCESS)
        {
            RCLCPP_ERROR(get_logger(), "Failed to get result");
            return false;
        }

        RCLCPP_INFO(get_logger(), "Movement Complete!");
        return true;
    }

private:
    rclcpp::Client<GetPositionIK>::SharedPtr            ik_client_;
    rclcpp_action::Client<FollowJointTraj>::SharedPtr   traj_client_;
};

int main(int argc, char * argv[])
{
    rclcpp::init(argc, argv);
    auto node = std::make_shared<DobotSimController>();

    std::cout << "\n[STEP 1] Moving to Target...\n";
    node->move_to_coordinate(0.4, 0.0, 0.3);

    std::cout << "[WAIT] Picking up object (2 seconds)...\n";
    std::this_thread::sleep_for(2s);

    std::cout << "[STEP 2] Returning to Home...\n";
    node->move_to_coordinate(0.2, 0.0, 0.5);

    std::cout << "\nAll tasks completed successfully!\n";

    rclcpp::shutdown();
    return 0;
}
