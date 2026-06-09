// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/FCCollisionSwitch.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__FC_COLLISION_SWITCH__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__FC_COLLISION_SWITCH__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/fc_collision_switch__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_FCCollisionSwitch_Request_enable
{
public:
  Init_FCCollisionSwitch_Request_enable()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::FCCollisionSwitch_Request enable(::dobot_msgs_v4::srv::FCCollisionSwitch_Request::_enable_type arg)
  {
    msg_.enable = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCCollisionSwitch_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::FCCollisionSwitch_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_FCCollisionSwitch_Request_enable();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_FCCollisionSwitch_Response_res
{
public:
  Init_FCCollisionSwitch_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::FCCollisionSwitch_Response res(::dobot_msgs_v4::srv::FCCollisionSwitch_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCCollisionSwitch_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::FCCollisionSwitch_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_FCCollisionSwitch_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__FC_COLLISION_SWITCH__BUILDER_HPP_
