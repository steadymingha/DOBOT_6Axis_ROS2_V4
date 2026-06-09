// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:srv/ControlCellFeeder.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__CONTROL_CELL_FEEDER__BUILDER_HPP_
#define ARIAC_INTERFACES__SRV__DETAIL__CONTROL_CELL_FEEDER__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/srv/detail/control_cell_feeder__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_ControlCellFeeder_Request_cell_type
{
public:
  Init_ControlCellFeeder_Request_cell_type()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ariac_interfaces::srv::ControlCellFeeder_Request cell_type(::ariac_interfaces::srv::ControlCellFeeder_Request::_cell_type_type arg)
  {
    msg_.cell_type = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::ControlCellFeeder_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::ControlCellFeeder_Request>()
{
  return ariac_interfaces::srv::builder::Init_ControlCellFeeder_Request_cell_type();
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_ControlCellFeeder_Response_message
{
public:
  explicit Init_ControlCellFeeder_Response_message(::ariac_interfaces::srv::ControlCellFeeder_Response & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::srv::ControlCellFeeder_Response message(::ariac_interfaces::srv::ControlCellFeeder_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::ControlCellFeeder_Response msg_;
};

class Init_ControlCellFeeder_Response_success
{
public:
  Init_ControlCellFeeder_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ControlCellFeeder_Response_message success(::ariac_interfaces::srv::ControlCellFeeder_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_ControlCellFeeder_Response_message(msg_);
  }

private:
  ::ariac_interfaces::srv::ControlCellFeeder_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::ControlCellFeeder_Response>()
{
  return ariac_interfaces::srv::builder::Init_ControlCellFeeder_Response_success();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__SRV__DETAIL__CONTROL_CELL_FEEDER__BUILDER_HPP_
