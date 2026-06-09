// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:srv/EndCompetition.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__END_COMPETITION__BUILDER_HPP_
#define ARIAC_INTERFACES__SRV__DETAIL__END_COMPETITION__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/srv/detail/end_competition__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_EndCompetition_Request_shutdown_gazebo
{
public:
  Init_EndCompetition_Request_shutdown_gazebo()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ariac_interfaces::srv::EndCompetition_Request shutdown_gazebo(::ariac_interfaces::srv::EndCompetition_Request::_shutdown_gazebo_type arg)
  {
    msg_.shutdown_gazebo = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::EndCompetition_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::EndCompetition_Request>()
{
  return ariac_interfaces::srv::builder::Init_EndCompetition_Request_shutdown_gazebo();
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_EndCompetition_Response_message
{
public:
  explicit Init_EndCompetition_Response_message(::ariac_interfaces::srv::EndCompetition_Response & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::srv::EndCompetition_Response message(::ariac_interfaces::srv::EndCompetition_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::EndCompetition_Response msg_;
};

class Init_EndCompetition_Response_success
{
public:
  Init_EndCompetition_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_EndCompetition_Response_message success(::ariac_interfaces::srv::EndCompetition_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_EndCompetition_Response_message(msg_);
  }

private:
  ::ariac_interfaces::srv::EndCompetition_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::EndCompetition_Response>()
{
  return ariac_interfaces::srv::builder::Init_EndCompetition_Response_success();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__SRV__DETAIL__END_COMPETITION__BUILDER_HPP_
