// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:srv/AttachTool.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__ATTACH_TOOL__BUILDER_HPP_
#define ARIAC_INTERFACES__SRV__DETAIL__ATTACH_TOOL__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/srv/detail/attach_tool__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_AttachTool_Request_tool
{
public:
  Init_AttachTool_Request_tool()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ariac_interfaces::srv::AttachTool_Request tool(::ariac_interfaces::srv::AttachTool_Request::_tool_type arg)
  {
    msg_.tool = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::AttachTool_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::AttachTool_Request>()
{
  return ariac_interfaces::srv::builder::Init_AttachTool_Request_tool();
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_AttachTool_Response_message
{
public:
  explicit Init_AttachTool_Response_message(::ariac_interfaces::srv::AttachTool_Response & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::srv::AttachTool_Response message(::ariac_interfaces::srv::AttachTool_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::AttachTool_Response msg_;
};

class Init_AttachTool_Response_success
{
public:
  Init_AttachTool_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_AttachTool_Response_message success(::ariac_interfaces::srv::AttachTool_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_AttachTool_Response_message(msg_);
  }

private:
  ::ariac_interfaces::srv::AttachTool_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::AttachTool_Response>()
{
  return ariac_interfaces::srv::builder::Init_AttachTool_Response_success();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__SRV__DETAIL__ATTACH_TOOL__BUILDER_HPP_
