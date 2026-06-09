// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/FCSetDeviation.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__FC_SET_DEVIATION__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__FC_SET_DEVIATION__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/fc_set_deviation__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_FCSetDeviation_Request_controltype
{
public:
  explicit Init_FCSetDeviation_Request_controltype(::dobot_msgs_v4::srv::FCSetDeviation_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::FCSetDeviation_Request controltype(::dobot_msgs_v4::srv::FCSetDeviation_Request::_controltype_type arg)
  {
    msg_.controltype = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCSetDeviation_Request msg_;
};

class Init_FCSetDeviation_Request_rz
{
public:
  explicit Init_FCSetDeviation_Request_rz(::dobot_msgs_v4::srv::FCSetDeviation_Request & msg)
  : msg_(msg)
  {}
  Init_FCSetDeviation_Request_controltype rz(::dobot_msgs_v4::srv::FCSetDeviation_Request::_rz_type arg)
  {
    msg_.rz = std::move(arg);
    return Init_FCSetDeviation_Request_controltype(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCSetDeviation_Request msg_;
};

class Init_FCSetDeviation_Request_ry
{
public:
  explicit Init_FCSetDeviation_Request_ry(::dobot_msgs_v4::srv::FCSetDeviation_Request & msg)
  : msg_(msg)
  {}
  Init_FCSetDeviation_Request_rz ry(::dobot_msgs_v4::srv::FCSetDeviation_Request::_ry_type arg)
  {
    msg_.ry = std::move(arg);
    return Init_FCSetDeviation_Request_rz(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCSetDeviation_Request msg_;
};

class Init_FCSetDeviation_Request_rx
{
public:
  explicit Init_FCSetDeviation_Request_rx(::dobot_msgs_v4::srv::FCSetDeviation_Request & msg)
  : msg_(msg)
  {}
  Init_FCSetDeviation_Request_ry rx(::dobot_msgs_v4::srv::FCSetDeviation_Request::_rx_type arg)
  {
    msg_.rx = std::move(arg);
    return Init_FCSetDeviation_Request_ry(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCSetDeviation_Request msg_;
};

class Init_FCSetDeviation_Request_z
{
public:
  explicit Init_FCSetDeviation_Request_z(::dobot_msgs_v4::srv::FCSetDeviation_Request & msg)
  : msg_(msg)
  {}
  Init_FCSetDeviation_Request_rx z(::dobot_msgs_v4::srv::FCSetDeviation_Request::_z_type arg)
  {
    msg_.z = std::move(arg);
    return Init_FCSetDeviation_Request_rx(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCSetDeviation_Request msg_;
};

class Init_FCSetDeviation_Request_y
{
public:
  explicit Init_FCSetDeviation_Request_y(::dobot_msgs_v4::srv::FCSetDeviation_Request & msg)
  : msg_(msg)
  {}
  Init_FCSetDeviation_Request_z y(::dobot_msgs_v4::srv::FCSetDeviation_Request::_y_type arg)
  {
    msg_.y = std::move(arg);
    return Init_FCSetDeviation_Request_z(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCSetDeviation_Request msg_;
};

class Init_FCSetDeviation_Request_x
{
public:
  Init_FCSetDeviation_Request_x()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_FCSetDeviation_Request_y x(::dobot_msgs_v4::srv::FCSetDeviation_Request::_x_type arg)
  {
    msg_.x = std::move(arg);
    return Init_FCSetDeviation_Request_y(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCSetDeviation_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::FCSetDeviation_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_FCSetDeviation_Request_x();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_FCSetDeviation_Response_res
{
public:
  Init_FCSetDeviation_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::FCSetDeviation_Response res(::dobot_msgs_v4::srv::FCSetDeviation_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::FCSetDeviation_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::FCSetDeviation_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_FCSetDeviation_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__FC_SET_DEVIATION__BUILDER_HPP_
