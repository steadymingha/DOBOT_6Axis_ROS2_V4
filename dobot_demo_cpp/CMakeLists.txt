cmake_minimum_required(VERSION 3.8)
project(dobot_demo_cpp)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(ament_cmake REQUIRED)
find_package(rclcpp REQUIRED)
find_package(rclcpp_action REQUIRED)
find_package(geometry_msgs REQUIRED)
find_package(moveit_msgs REQUIRED)
find_package(control_msgs REQUIRED)
find_package(trajectory_msgs REQUIRED)
find_package(sensor_msgs REQUIRED)

add_executable(gemini_demo src/gemini_demo.cpp)

target_compile_features(gemini_demo PUBLIC cxx_std_17)

ament_target_dependencies(gemini_demo
  rclcpp
  rclcpp_action
  geometry_msgs
  moveit_msgs
  control_msgs
  trajectory_msgs
  sensor_msgs
)

install(TARGETS gemini_demo
  DESTINATION lib/${PROJECT_NAME}
)

ament_package()
