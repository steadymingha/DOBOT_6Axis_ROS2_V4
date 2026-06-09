// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:srv/Trigger.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__TRIGGER__BUILDER_HPP_
#define ARIAC_INTERFACES__SRV__DETAIL__TRIGGER__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/srv/detail/trigger__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace srv
{


}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::Trigger_Request>()
{
  return ::ariac_interfaces::srv::Trigger_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_Trigger_Response_message
{
public:
  explicit Init_Trigger_Response_message(::ariac_interfaces::srv::Trigger_Response & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::srv::Trigger_Response message(::ariac_interfaces::srv::Trigger_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::Trigger_Response msg_;
};

class Init_Trigger_Response_success
{
public:
  Init_Trigger_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Trigger_Response_message success(::ariac_interfaces::srv::Trigger_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_Trigger_Response_message(msg_);
  }

private:
  ::ariac_interfaces::srv::Trigger_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::Trigger_Response>()
{
  return ariac_interfaces::srv::builder::Init_Trigger_Response_success();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__SRV__DETAIL__TRIGGER__BUILDER_HPP_
