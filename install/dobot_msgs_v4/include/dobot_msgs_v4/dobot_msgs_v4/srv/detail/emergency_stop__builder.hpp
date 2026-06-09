// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/EmergencyStop.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__EMERGENCY_STOP__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__EMERGENCY_STOP__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/emergency_stop__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_EmergencyStop_Request_value
{
public:
  Init_EmergencyStop_Request_value()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::EmergencyStop_Request value(::dobot_msgs_v4::srv::EmergencyStop_Request::_value_type arg)
  {
    msg_.value = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::EmergencyStop_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::EmergencyStop_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_EmergencyStop_Request_value();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_EmergencyStop_Response_res
{
public:
  Init_EmergencyStop_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::EmergencyStop_Response res(::dobot_msgs_v4::srv::EmergencyStop_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::EmergencyStop_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::EmergencyStop_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_EmergencyStop_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__EMERGENCY_STOP__BUILDER_HPP_
