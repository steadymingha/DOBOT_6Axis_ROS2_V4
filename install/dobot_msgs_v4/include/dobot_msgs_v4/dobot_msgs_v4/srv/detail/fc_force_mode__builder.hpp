// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/FCForceMode.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__FC_FORCE_MODE__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__FC_FORCE_MODE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/fc_force_mode__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_FCForceMode_Request_tool
{
public:
  explicit Init_FCForceMode_Request_tool(::dobot_msgs_v4::srv::FCForceMode_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::FCForceMode_Request tool(::dobot_msgs_v4::srv::FCForceMode_Request::_tool_type arg)
  {
    msg_.tool = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

class Init_FCForceMode_Request_user
{
public:
  explicit Init_FCForceMode_Request_user(::dobot_msgs_v4::srv::FCForceMode_Request & msg)
  : msg_(msg)
  {}
  Init_FCForceMode_Request_tool user(::dobot_msgs_v4::srv::FCForceMode_Request::_user_type arg)
  {
    msg_.user = std::move(arg);
    return Init_FCForceMode_Request_tool(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

class Init_FCForceMode_Request_reference
{
public:
  explicit Init_FCForceMode_Request_reference(::dobot_msgs_v4::srv::FCForceMode_Request & msg)
  : msg_(msg)
  {}
  Init_FCForceMode_Request_user reference(::dobot_msgs_v4::srv::FCForceMode_Request::_reference_type arg)
  {
    msg_.reference = std::move(arg);
    return Init_FCForceMode_Request_user(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

class Init_FCForceMode_Request_frz
{
public:
  explicit Init_FCForceMode_Request_frz(::dobot_msgs_v4::srv::FCForceMode_Request & msg)
  : msg_(msg)
  {}
  Init_FCForceMode_Request_reference frz(::dobot_msgs_v4::srv::FCForceMode_Request::_frz_type arg)
  {
    msg_.frz = std::move(arg);
    return Init_FCForceMode_Request_reference(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

class Init_FCForceMode_Request_fry
{
public:
  explicit Init_FCForceMode_Request_fry(::dobot_msgs_v4::srv::FCForceMode_Request & msg)
  : msg_(msg)
  {}
  Init_FCForceMode_Request_frz fry(::dobot_msgs_v4::srv::FCForceMode_Request::_fry_type arg)
  {
    msg_.fry = std::move(arg);
    return Init_FCForceMode_Request_frz(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

class Init_FCForceMode_Request_frx
{
public:
  explicit Init_FCForceMode_Request_frx(::dobot_msgs_v4::srv::FCForceMode_Request & msg)
  : msg_(msg)
  {}
  Init_FCForceMode_Request_fry frx(::dobot_msgs_v4::srv::FCForceMode_Request::_frx_type arg)
  {
    msg_.frx = std::move(arg);
    return Init_FCForceMode_Request_fry(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

class Init_FCForceMode_Request_fz
{
public:
  explicit Init_FCForceMode_Request_fz(::dobot_msgs_v4::srv::FCForceMode_Request & msg)
  : msg_(msg)
  {}
  Init_FCForceMode_Request_frx fz(::dobot_msgs_v4::srv::FCForceMode_Request::_fz_type arg)
  {
    msg_.fz = std::move(arg);
    return Init_FCForceMode_Request_frx(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

class Init_FCForceMode_Request_fy
{
public:
  explicit Init_FCForceMode_Request_fy(::dobot_msgs_v4::srv::FCForceMode_Request & msg)
  : msg_(msg)
  {}
  Init_FCForceMode_Request_fz fy(::dobot_msgs_v4::srv::FCForceMode_Request::_fy_type arg)
  {
    msg_.fy = std::move(arg);
    return Init_FCForceMode_Request_fz(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

class Init_FCForceMode_Request_fx
{
public:
  explicit Init_FCForceMode_Request_fx(::dobot_msgs_v4::srv::FCForceMode_Request & msg)
  : msg_(msg)
  {}
  Init_FCForceMode_Request_fy fx(::dobot_msgs_v4::srv::FCForceMode_Request::_fx_type arg)
  {
    msg_.fx = std::move(arg);
    return Init_FCForceMode_Request_fy(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

class Init_FCForceMode_Request_rz
{
public:
  explicit Init_FCForceMode_Request_rz(::dobot_msgs_v4::srv::FCForceMode_Request & msg)
  : msg_(msg)
  {}
  Init_FCForceMode_Request_fx rz(::dobot_msgs_v4::srv::FCForceMode_Request::_rz_type arg)
  {
    msg_.rz = std::move(arg);
    return Init_FCForceMode_Request_fx(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

class Init_FCForceMode_Request_ry
{
public:
  explicit Init_FCForceMode_Request_ry(::dobot_msgs_v4::srv::FCForceMode_Request & msg)
  : msg_(msg)
  {}
  Init_FCForceMode_Request_rz ry(::dobot_msgs_v4::srv::FCForceMode_Request::_ry_type arg)
  {
    msg_.ry = std::move(arg);
    return Init_FCForceMode_Request_rz(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

class Init_FCForceMode_Request_rx
{
public:
  explicit Init_FCForceMode_Request_rx(::dobot_msgs_v4::srv::FCForceMode_Request & msg)
  : msg_(msg)
  {}
  Init_FCForceMode_Request_ry rx(::dobot_msgs_v4::srv::FCForceMode_Request::_rx_type arg)
  {
    msg_.rx = std::move(arg);
    return Init_FCForceMode_Request_ry(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

class Init_FCForceMode_Request_z
{
public:
  explicit Init_FCForceMode_Request_z(::dobot_msgs_v4::srv::FCForceMode_Request & msg)
  : msg_(msg)
  {}
  Init_FCForceMode_Request_rx z(::dobot_msgs_v4::srv::FCForceMode_Request::_z_type arg)
  {
    msg_.z = std::move(arg);
    return Init_FCForceMode_Request_rx(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

class Init_FCForceMode_Request_y
{
public:
  explicit Init_FCForceMode_Request_y(::dobot_msgs_v4::srv::FCForceMode_Request & msg)
  : msg_(msg)
  {}
  Init_FCForceMode_Request_z y(::dobot_msgs_v4::srv::FCForceMode_Request::_y_type arg)
  {
    msg_.y = std::move(arg);
    return Init_FCForceMode_Request_z(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

class Init_FCForceMode_Request_x
{
public:
  Init_FCForceMode_Request_x()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_FCForceMode_Request_y x(::dobot_msgs_v4::srv::FCForceMode_Request::_x_type arg)
  {
    msg_.x = std::move(arg);
    return Init_FCForceMode_Request_y(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::FCForceMode_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_FCForceMode_Request_x();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_FCForceMode_Response_res
{
public:
  Init_FCForceMode_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::FCForceMode_Response res(::dobot_msgs_v4::srv::FCForceMode_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCForceMode_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::FCForceMode_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_FCForceMode_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__FC_FORCE_MODE__BUILDER_HPP_
