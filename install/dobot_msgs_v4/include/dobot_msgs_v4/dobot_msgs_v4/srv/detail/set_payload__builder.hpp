// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/SetPayload.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__SET_PAYLOAD__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__SET_PAYLOAD__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/set_payload__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_SetPayload_Request_z
{
public:
  explicit Init_SetPayload_Request_z(::dobot_msgs_v4::srv::SetPayload_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::SetPayload_Request z(::dobot_msgs_v4::srv::SetPayload_Request::_z_type arg)
  {
    msg_.z = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetPayload_Request msg_;
};

class Init_SetPayload_Request_y
{
public:
  explicit Init_SetPayload_Request_y(::dobot_msgs_v4::srv::SetPayload_Request & msg)
  : msg_(msg)
  {}
  Init_SetPayload_Request_z y(::dobot_msgs_v4::srv::SetPayload_Request::_y_type arg)
  {
    msg_.y = std::move(arg);
    return Init_SetPayload_Request_z(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetPayload_Request msg_;
};

class Init_SetPayload_Request_x
{
public:
  explicit Init_SetPayload_Request_x(::dobot_msgs_v4::srv::SetPayload_Request & msg)
  : msg_(msg)
  {}
  Init_SetPayload_Request_y x(::dobot_msgs_v4::srv::SetPayload_Request::_x_type arg)
  {
    msg_.x = std::move(arg);
    return Init_SetPayload_Request_y(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetPayload_Request msg_;
};

class Init_SetPayload_Request_load
{
public:
  Init_SetPayload_Request_load()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetPayload_Request_x load(::dobot_msgs_v4::srv::SetPayload_Request::_load_type arg)
  {
    msg_.load = std::move(arg);
    return Init_SetPayload_Request_x(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetPayload_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::SetPayload_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_SetPayload_Request_load();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_SetPayload_Response_res
{
public:
  Init_SetPayload_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::SetPayload_Response res(::dobot_msgs_v4::srv::SetPayload_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetPayload_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::SetPayload_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_SetPayload_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__SET_PAYLOAD__BUILDER_HPP_
