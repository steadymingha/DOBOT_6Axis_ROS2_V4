// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:srv/SubmitHighPriorityOrder.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_HIGH_PRIORITY_ORDER__BUILDER_HPP_
#define ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_HIGH_PRIORITY_ORDER__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/srv/detail/submit_high_priority_order__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_SubmitHighPriorityOrder_Request_id
{
public:
  Init_SubmitHighPriorityOrder_Request_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ariac_interfaces::srv::SubmitHighPriorityOrder_Request id(::ariac_interfaces::srv::SubmitHighPriorityOrder_Request::_id_type arg)
  {
    msg_.id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::SubmitHighPriorityOrder_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::SubmitHighPriorityOrder_Request>()
{
  return ariac_interfaces::srv::builder::Init_SubmitHighPriorityOrder_Request_id();
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_SubmitHighPriorityOrder_Response_message
{
public:
  explicit Init_SubmitHighPriorityOrder_Response_message(::ariac_interfaces::srv::SubmitHighPriorityOrder_Response & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::srv::SubmitHighPriorityOrder_Response message(::ariac_interfaces::srv::SubmitHighPriorityOrder_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::SubmitHighPriorityOrder_Response msg_;
};

class Init_SubmitHighPriorityOrder_Response_success
{
public:
  Init_SubmitHighPriorityOrder_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SubmitHighPriorityOrder_Response_message success(::ariac_interfaces::srv::SubmitHighPriorityOrder_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_SubmitHighPriorityOrder_Response_message(msg_);
  }

private:
  ::ariac_interfaces::srv::SubmitHighPriorityOrder_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::SubmitHighPriorityOrder_Response>()
{
  return ariac_interfaces::srv::builder::Init_SubmitHighPriorityOrder_Response_success();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_HIGH_PRIORITY_ORDER__BUILDER_HPP_
