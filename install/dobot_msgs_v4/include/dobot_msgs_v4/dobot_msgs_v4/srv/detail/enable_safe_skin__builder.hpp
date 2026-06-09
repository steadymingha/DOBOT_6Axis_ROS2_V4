// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/EnableSafeSkin.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__ENABLE_SAFE_SKIN__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__ENABLE_SAFE_SKIN__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/enable_safe_skin__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_EnableSafeSkin_Request_status
{
public:
  Init_EnableSafeSkin_Request_status()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::EnableSafeSkin_Request status(::dobot_msgs_v4::srv::EnableSafeSkin_Request::_status_type arg)
  {
    msg_.status = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::EnableSafeSkin_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::EnableSafeSkin_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_EnableSafeSkin_Request_status();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_EnableSafeSkin_Response_res
{
public:
  Init_EnableSafeSkin_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::EnableSafeSkin_Response res(::dobot_msgs_v4::srv::EnableSafeSkin_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::EnableSafeSkin_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::EnableSafeSkin_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_EnableSafeSkin_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__ENABLE_SAFE_SKIN__BUILDER_HPP_
