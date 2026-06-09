// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:msg/HighPriorityOrder.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__HIGH_PRIORITY_ORDER__BUILDER_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__HIGH_PRIORITY_ORDER__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/msg/detail/high_priority_order__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace msg
{

namespace builder
{

class Init_HighPriorityOrder_id
{
public:
  Init_HighPriorityOrder_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ariac_interfaces::msg::HighPriorityOrder id(::ariac_interfaces::msg::HighPriorityOrder::_id_type arg)
  {
    msg_.id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::msg::HighPriorityOrder msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::msg::HighPriorityOrder>()
{
  return ariac_interfaces::msg::builder::Init_HighPriorityOrder_id();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__HIGH_PRIORITY_ORDER__BUILDER_HPP_
