// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/StartPath.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__START_PATH__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__START_PATH__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/start_path__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_StartPath_Request_param_value
{
public:
  explicit Init_StartPath_Request_param_value(::dobot_msgs_v4::srv::StartPath_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::StartPath_Request param_value(::dobot_msgs_v4::srv::StartPath_Request::_param_value_type arg)
  {
    msg_.param_value = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::StartPath_Request msg_;
};

class Init_StartPath_Request_trace_name
{
public:
  Init_StartPath_Request_trace_name()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_StartPath_Request_param_value trace_name(::dobot_msgs_v4::srv::StartPath_Request::_trace_name_type arg)
  {
    msg_.trace_name = std::move(arg);
    return Init_StartPath_Request_param_value(msg_);
  }

private:
  ::dobot_msgs_v4::srv::StartPath_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::StartPath_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_StartPath_Request_trace_name();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_StartPath_Response_res
{
public:
  Init_StartPath_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::StartPath_Response res(::dobot_msgs_v4::srv::StartPath_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::StartPath_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::StartPath_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_StartPath_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__START_PATH__BUILDER_HPP_
