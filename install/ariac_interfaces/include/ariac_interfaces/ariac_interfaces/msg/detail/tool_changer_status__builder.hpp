// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:msg/ToolChangerStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__TOOL_CHANGER_STATUS__BUILDER_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__TOOL_CHANGER_STATUS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/msg/detail/tool_changer_status__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace msg
{

namespace builder
{

class Init_ToolChangerStatus_attached_tool
{
public:
  Init_ToolChangerStatus_attached_tool()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ariac_interfaces::msg::ToolChangerStatus attached_tool(::ariac_interfaces::msg::ToolChangerStatus::_attached_tool_type arg)
  {
    msg_.attached_tool = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::msg::ToolChangerStatus msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::msg::ToolChangerStatus>()
{
  return ariac_interfaces::msg::builder::Init_ToolChangerStatus_attached_tool();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__TOOL_CHANGER_STATUS__BUILDER_HPP_
