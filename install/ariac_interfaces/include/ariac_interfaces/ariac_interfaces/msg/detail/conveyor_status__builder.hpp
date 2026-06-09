// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:msg/ConveyorStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__BUILDER_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/msg/detail/conveyor_status__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace msg
{

namespace builder
{

class Init_ConveyorStatus_operating_status
{
public:
  explicit Init_ConveyorStatus_operating_status(::ariac_interfaces::msg::ConveyorStatus & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::msg::ConveyorStatus operating_status(::ariac_interfaces::msg::ConveyorStatus::_operating_status_type arg)
  {
    msg_.operating_status = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::msg::ConveyorStatus msg_;
};

class Init_ConveyorStatus_speed
{
public:
  explicit Init_ConveyorStatus_speed(::ariac_interfaces::msg::ConveyorStatus & msg)
  : msg_(msg)
  {}
  Init_ConveyorStatus_operating_status speed(::ariac_interfaces::msg::ConveyorStatus::_speed_type arg)
  {
    msg_.speed = std::move(arg);
    return Init_ConveyorStatus_operating_status(msg_);
  }

private:
  ::ariac_interfaces::msg::ConveyorStatus msg_;
};

class Init_ConveyorStatus_direction
{
public:
  Init_ConveyorStatus_direction()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ConveyorStatus_speed direction(::ariac_interfaces::msg::ConveyorStatus::_direction_type arg)
  {
    msg_.direction = std::move(arg);
    return Init_ConveyorStatus_speed(msg_);
  }

private:
  ::ariac_interfaces::msg::ConveyorStatus msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::msg::ConveyorStatus>()
{
  return ariac_interfaces::msg::builder::Init_ConveyorStatus_direction();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__BUILDER_HPP_
