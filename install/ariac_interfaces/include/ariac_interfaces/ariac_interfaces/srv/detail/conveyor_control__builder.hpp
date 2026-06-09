// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:srv/ConveyorControl.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__CONVEYOR_CONTROL__BUILDER_HPP_
#define ARIAC_INTERFACES__SRV__DETAIL__CONVEYOR_CONTROL__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/srv/detail/conveyor_control__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_ConveyorControl_Request_speed
{
public:
  Init_ConveyorControl_Request_speed()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ariac_interfaces::srv::ConveyorControl_Request speed(::ariac_interfaces::srv::ConveyorControl_Request::_speed_type arg)
  {
    msg_.speed = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::ConveyorControl_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::ConveyorControl_Request>()
{
  return ariac_interfaces::srv::builder::Init_ConveyorControl_Request_speed();
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_ConveyorControl_Response_success
{
public:
  Init_ConveyorControl_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ariac_interfaces::srv::ConveyorControl_Response success(::ariac_interfaces::srv::ConveyorControl_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::ConveyorControl_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::ConveyorControl_Response>()
{
  return ariac_interfaces::srv::builder::Init_ConveyorControl_Response_success();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__SRV__DETAIL__CONVEYOR_CONTROL__BUILDER_HPP_
