// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/SetFCCollision.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__SET_FC_COLLISION__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__SET_FC_COLLISION__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/set_fc_collision__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_SetFCCollision_Request_torque
{
public:
  explicit Init_SetFCCollision_Request_torque(::dobot_msgs_v4::srv::SetFCCollision_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::SetFCCollision_Request torque(::dobot_msgs_v4::srv::SetFCCollision_Request::_torque_type arg)
  {
    msg_.torque = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetFCCollision_Request msg_;
};

class Init_SetFCCollision_Request_force
{
public:
  Init_SetFCCollision_Request_force()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetFCCollision_Request_torque force(::dobot_msgs_v4::srv::SetFCCollision_Request::_force_type arg)
  {
    msg_.force = std::move(arg);
    return Init_SetFCCollision_Request_torque(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetFCCollision_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::SetFCCollision_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_SetFCCollision_Request_force();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_SetFCCollision_Response_res
{
public:
  Init_SetFCCollision_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::SetFCCollision_Response res(::dobot_msgs_v4::srv::SetFCCollision_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetFCCollision_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::SetFCCollision_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_SetFCCollision_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__SET_FC_COLLISION__BUILDER_HPP_
