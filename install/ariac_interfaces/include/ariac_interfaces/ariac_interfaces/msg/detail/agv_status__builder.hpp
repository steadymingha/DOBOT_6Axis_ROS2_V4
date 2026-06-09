// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:msg/AgvStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__AGV_STATUS__BUILDER_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__AGV_STATUS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/msg/detail/agv_status__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace msg
{

namespace builder
{

class Init_AgvStatus_pose
{
public:
  explicit Init_AgvStatus_pose(::ariac_interfaces::msg::AgvStatus & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::msg::AgvStatus pose(::ariac_interfaces::msg::AgvStatus::_pose_type arg)
  {
    msg_.pose = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::msg::AgvStatus msg_;
};

class Init_AgvStatus_station_id
{
public:
  Init_AgvStatus_station_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_AgvStatus_pose station_id(::ariac_interfaces::msg::AgvStatus::_station_id_type arg)
  {
    msg_.station_id = std::move(arg);
    return Init_AgvStatus_pose(msg_);
  }

private:
  ::ariac_interfaces::msg::AgvStatus msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::msg::AgvStatus>()
{
  return ariac_interfaces::msg::builder::Init_AgvStatus_station_id();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__AGV_STATUS__BUILDER_HPP_
