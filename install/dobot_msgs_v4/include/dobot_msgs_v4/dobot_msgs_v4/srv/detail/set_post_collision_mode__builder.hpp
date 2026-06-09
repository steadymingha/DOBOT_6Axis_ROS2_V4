// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/SetPostCollisionMode.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__SET_POST_COLLISION_MODE__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__SET_POST_COLLISION_MODE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/set_post_collision_mode__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_SetPostCollisionMode_Request_mode
{
public:
  Init_SetPostCollisionMode_Request_mode()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::SetPostCollisionMode_Request mode(::dobot_msgs_v4::srv::SetPostCollisionMode_Request::_mode_type arg)
  {
    msg_.mode = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetPostCollisionMode_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::SetPostCollisionMode_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_SetPostCollisionMode_Request_mode();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_SetPostCollisionMode_Response_res
{
public:
  Init_SetPostCollisionMode_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::SetPostCollisionMode_Response res(::dobot_msgs_v4::srv::SetPostCollisionMode_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetPostCollisionMode_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::SetPostCollisionMode_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_SetPostCollisionMode_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__SET_POST_COLLISION_MODE__BUILDER_HPP_
