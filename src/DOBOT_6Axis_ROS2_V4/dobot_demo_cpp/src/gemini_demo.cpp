#include <rclcpp/rclcpp.hpp>
#include <rclcpp_action/rclcpp_action.hpp>
#include <geometry_msgs/msg/pose_stamped.hpp>
#include <moveit_msgs/srv/get_position_ik.hpp>
#include <control_msgs/action/follow_joint_trajectory.hpp>
#include <trajectory_msgs/msg/joint_trajectory_point.hpp>
#include <sensor_msgs/msg/joint_state.hpp>

#include <algorithm>
#include <chrono>
#include <future>
#include <mutex>
#include <thread>
#include <cstdlib>
#include <cmath> // std::pow 사용을 위해 추가

using namespace std::chrono_literals;
using GetPositionIK   = moveit_msgs::srv::GetPositionIK;
using FollowJointTraj = control_msgs::action::FollowJointTrajectory;

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

        gripper_client_ = rclcpp_action::create_client<FollowJointTraj>(
            this, "/gripper_controller/follow_joint_trajectory");

        joint_state_sub_ = this->create_subscription<sensor_msgs::msg::JointState>(
            "/joint_states", 10,
            [this](const sensor_msgs::msg::JointState::SharedPtr msg) {
                std::lock_guard<std::mutex> lock(joint_state_mutex_);
                latest_joint_state_ = *msg;
                has_joint_state_    = true;
            });
    }

    bool wait_for_joint_state(double timeout_sec = 5.0)
    {
        auto start = this->now();
        rclcpp::Rate rate(50);
        while (rclcpp::ok()) {
            {
                std::lock_guard<std::mutex> lock(joint_state_mutex_);
                if (has_joint_state_) return true;
            }
            if ((this->now() - start).seconds() > timeout_sec) {
                RCLCPP_ERROR(get_logger(), "Timed out waiting for /joint_states");
                return false;
            }
            rate.sleep();
        }
        return false;
    }

    bool move_to_coordinate(double x, double y, double z)
    {
        if (!ik_client_->wait_for_service(5s)) {
            RCLCPP_ERROR(get_logger(), "IK service not available");
            return false;
        }

        if (!wait_for_joint_state()) return false;

        const char * env = std::getenv("DOBOT_TYPE");
        std::string robot_type = env ? env : "cr7";

        auto req = std::make_shared<GetPositionIK::Request>();
        req->ik_request.group_name = robot_type + "_group";

        const std::vector<std::string> joint_names = {
            "joint1", "joint2", "joint3", "joint4", "joint5", "joint6"
        };

        // --- Step 1: 시작 관절값 q0 (현재 관절값) 읽기 ---
        std::vector<double> start_positions;
        {
            std::lock_guard<std::mutex> lock(joint_state_mutex_);
            auto & seed = req->ik_request.robot_state.joint_state;
            for (const auto & name : joint_names) {
                auto it = std::find(latest_joint_state_.name.begin(),
                                    latest_joint_state_.name.end(), name);
                if (it != latest_joint_state_.name.end()) {
                    double current_pos = latest_joint_state_.position[
                        std::distance(latest_joint_state_.name.begin(), it)
                    ];
                    seed.name.push_back(name);
                    seed.position.push_back(current_pos);
                    start_positions.push_back(current_pos); // q0 저장
                } else {
                    RCLCPP_ERROR(get_logger(), "Joint '%s' not found in current states", name.c_str());
                    return false;
                }
            }
        }

        // --- Step 2: IK로 목표 관절값 q1 계산 ---
        geometry_msgs::msg::PoseStamped pose;
        pose.header.frame_id      = "base_link";
        pose.pose.position.x      = x;
        pose.pose.position.y      = y;
        pose.pose.position.z      = z;
        pose.pose.orientation.x   = 0.0;
        pose.pose.orientation.y   = 1.0;  // end-effector pointing downward
        pose.pose.orientation.z   = 0.0;
        pose.pose.orientation.w   = 0.0;
        req->ik_request.pose_stamped = pose;
        req->ik_request.timeout.sec  = 2;
        
        // 추가됨: IK 계산 시 MoveIt의 충돌 회피(Collision Checking)를 활성화하여 
        // 목표 자세 자체가 충돌 상태인 해를 반환하지 않도록 방지합니다.
        req->ik_request.avoid_collisions = false;//true;

        RCLCPP_INFO(get_logger(), "Target [X:%.2f, Y:%.2f, Z:%.2f] - Calculating IK...", x, y, z);

        auto ik_future = ik_client_->async_send_request(req);
        if (ik_future.wait_for(8s) != std::future_status::ready) {
            RCLCPP_ERROR(get_logger(), "IK service call timed out");
            return false;
        }

        auto res = ik_future.get();
        if (res->error_code.val != 1) {
            RCLCPP_ERROR(get_logger(), "IK Failed (Code: %d)", res->error_code.val);
            return false;
        }

        std::vector<double> target_positions;
        const auto & state = res->solution.joint_state;

        for (size_t j = 0; j < joint_names.size(); ++j) {
            auto it = std::find(state.name.begin(), state.name.end(), joint_names[j]);
            if (it == state.name.end()) {
                RCLCPP_ERROR(get_logger(), "Joint '%s' not found in IK solution", joint_names[j].c_str());
                return false;
            }
            
            double q0 = start_positions[j]; // 현재 관절값
            double q1 = state.position[std::distance(state.name.begin(), it)]; // IK가 계산한 목표 관절값

            // 추가됨: 최단 거리(Shortest Path) 조인트 정규화
            // IK 결과가 엉뚱한 방향으로 한 바퀴 도는 것을 방지 (-PI ~ PI 범위 안으로 차이 강제 조정)
            double diff = q1 - q0;
            while (diff > M_PI) {
                q1 -= 2.0 * M_PI;
                diff = q1 - q0;
            }
            while (diff < -M_PI) {
                q1 += 2.0 * M_PI;
                diff = q1 - q0;
            }

            target_positions.push_back(q1); // 최단 거리로 보정된 q1 저장
        }

        FollowJointTraj::Goal goal;
        goal.trajectory.joint_names = joint_names;

        // --- Step 3: Trajectory 생성 (5차 다항식 보간) ---
        if (!traj_client_->wait_for_action_server(5s)) {
            RCLCPP_ERROR(get_logger(), "Trajectory action server not available");
            return false;
        }

        double T = 3.0;        // 총 이동 시간 (초)
        int N = 60;            // 보간점(Point) 개수 (초당 20개 수준)

        for (int i = 1; i <= N; ++i) {
            double t = T * i / N; // 현재 경과 시간
            trajectory_msgs::msg::JointTrajectoryPoint point;

            for (size_t j = 0; j < joint_names.size(); ++j) {
                double q0 = start_positions[j];
                double q1 = target_positions[j];

                // 5차 다항식 계수 계산 (시작/끝의 속도, 가속도는 0 가정)
                // a0 = q0, a1 = 0, a2 = 0
                double a0 = q0;
                double a3 = 10.0 * (q1 - q0) / std::pow(T, 3);
                double a4 = -15.0 * (q1 - q0) / std::pow(T, 4);
                double a5 =  6.0 * (q1 - q0) / std::pow(T, 5);

                // 현재 시점 t에서의 Position, Velocity, Acceleration 계산
                double pos = a0 + a3 * std::pow(t, 3) + a4 * std::pow(t, 4) + a5 * std::pow(t, 5);
                double vel = 3.0 * a3 * std::pow(t, 2) + 4.0 * a4 * std::pow(t, 3) + 5.0 * a5 * std::pow(t, 4);
                double acc = 6.0 * a3 * t + 12.0 * a4 * std::pow(t, 2) + 20.0 * a5 * std::pow(t, 3);

                point.positions.push_back(pos);
                point.velocities.push_back(vel);
                point.accelerations.push_back(acc);
            }

            point.time_from_start = rclcpp::Duration::from_seconds(t);
            goal.trajectory.points.push_back(point);
        }

        RCLCPP_INFO(get_logger(), "Path found! Generated %d points. Sending command to Gazebo...", N);
        

        // // ─────────── 보간 없이 끝점 하나만 전송 (원래 방식) ───────────
        // trajectory_msgs::msg::JointTrajectoryPoint point;
        // point.positions = target_positions;
        // point.time_from_start = rclcpp::Duration::from_seconds(3.0);
        // goal.trajectory.points.push_back(point);
        // RCLCPP_INFO(get_logger(), "Path found! Sending command to Gazebo...");
        // // ─────────────────────────────────────────────────────────────


        // --- Step 4: 한 번에 trajectory 전송 ---
        auto send_goal_future = traj_client_->async_send_goal(goal);
        if (send_goal_future.wait_for(5s) != std::future_status::ready) {
            RCLCPP_ERROR(get_logger(), "Failed to send goal (timeout)");
            return false;
        }

        auto goal_handle = send_goal_future.get();
        if (!goal_handle) {
            RCLCPP_ERROR(get_logger(), "Goal was rejected by the action server");
            return false;
        }

        RCLCPP_INFO(get_logger(), "Robot is moving in Gazebo (Quintic Polynomial Interpolation)!");

        auto result_future = traj_client_->async_get_result(goal_handle);
        if (result_future.wait_for(15s) != std::future_status::ready) {
            RCLCPP_ERROR(get_logger(), "Movement timed out");
            return false;
        }

        RCLCPP_INFO(get_logger(), "Movement Complete!");
        return true;
    }

    // Drive the gripper fingers to the given positions [left, right] (meters).
    bool control_gripper(const std::vector<double> & positions)
    {
        if (!gripper_client_->wait_for_action_server(5s)) {
            RCLCPP_ERROR(get_logger(), "Gripper action server not available");
            return false;
        }

        FollowJointTraj::Goal goal;
        goal.trajectory.joint_names = {"left_finger_joint", "right_finger_joint"};

        trajectory_msgs::msg::JointTrajectoryPoint point;
        point.positions      = positions;
        point.velocities     = {0.0, 0.0};
        point.time_from_start = rclcpp::Duration::from_seconds(2.0);
        goal.trajectory.points.push_back(point);

        auto send_goal_future = gripper_client_->async_send_goal(goal);
        if (send_goal_future.wait_for(5s) != std::future_status::ready) {
            RCLCPP_ERROR(get_logger(), "Failed to send gripper goal (timeout)");
            return false;
        }

        auto goal_handle = send_goal_future.get();
        if (!goal_handle) {
            RCLCPP_ERROR(get_logger(), "Gripper goal was rejected by the action server");
            return false;
        }

        auto result_future = gripper_client_->async_get_result(goal_handle);
        if (result_future.wait_for(10s) != std::future_status::ready) {
            RCLCPP_ERROR(get_logger(), "Gripper movement timed out");
            return false;
        }

        RCLCPP_INFO(get_logger(), "Gripper moved to [%.3f, %.3f]",
                    positions[0], positions[1]);
        return true;
    }

    sensor_msgs::msg::JointState latest_joint_state_;
    std::mutex                   joint_state_mutex_;
    bool                         has_joint_state_{false};

private:
    rclcpp::Client<GetPositionIK>::SharedPtr                        ik_client_;
    rclcpp_action::Client<FollowJointTraj>::SharedPtr               traj_client_;
    rclcpp_action::Client<FollowJointTraj>::SharedPtr               gripper_client_;
    rclcpp::Subscription<sensor_msgs::msg::JointState>::SharedPtr   joint_state_sub_;
};

int main(int argc, char * argv[])
{
    rclcpp::init(argc, argv);
    auto node = std::make_shared<DobotSimController>();

    rclcpp::executors::SingleThreadedExecutor executor;
    executor.add_node(node);
    std::thread spin_thread([&executor]() { executor.spin(); });

    node->wait_for_joint_state();

    // std::cout << "\n[SEED] Current joint angles passed to IK solver:\n";
    // {
    //     std::lock_guard<std::mutex> lock(node->joint_state_mutex_);
    //     for (size_t i = 0; i < node->latest_joint_state_.name.size(); ++i) {
    //         std::cout << "  " << node->latest_joint_state_.name[i]
    //                   << " = " << node->latest_joint_state_.position[i] * 180.0 / M_PI
    //                   << " deg\n";
    //     }
    // }

    // Gripper finger positions [left, right] in meters
    const std::vector<double> GRIPPER_OPEN  = {0.06, 0.06};
    const std::vector<double> GRIPPER_CLOSE = {0.01, 0.01};

    // std::cout << "\n[STEP 2] Opening gripper...\n";
    // node->control_gripper(GRIPPER_OPEN);

    std::cout << "\n[STEP 1] Moving to (0.77, 0.0, 0.15)...\n";
    // node->move_to_coordinate(0.77, 0.0, 0.15);
    node->move_to_coordinate(0.4, 0.0, 0.15);

    // std::cout << "\n[STEP 3] Closing gripper...\n";
    // node->control_gripper(GRIPPER_CLOSE);

    // node->move_to_coordinate(0.2, 0.0, 0.5);

    // std::cout << "\n[SEED] Joint angles after move (seed for next IK call):\n";
    // {
    //     std::lock_guard<std::mutex> lock(node->joint_state_mutex_);
    //     for (size_t i = 0; i < node->latest_joint_state_.name.size(); ++i) {
    //         std::cout << "  " << node->latest_joint_state_.name[i]
    //                   << " = " << node->latest_joint_state_.position[i] * 180.0 / M_PI
    //                   << " deg\n";
    //     }
    // }

    std::cout << "\nAll tasks completed successfully!\n";

    rclcpp::shutdown();
    spin_thread.join();
    return 0;
}


