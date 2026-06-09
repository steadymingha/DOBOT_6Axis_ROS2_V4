// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/SetTool485.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__SET_TOOL485__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__SET_TOOL485__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/set_tool485__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_SetTool485_Request_identify
{
public:
  explicit Init_SetTool485_Request_identify(::dobot_msgs_v4::srv::SetTool485_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::SetTool485_Request identify(::dobot_msgs_v4::srv::SetTool485_Request::_identify_type arg)
  {
    msg_.identify = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetTool485_Request msg_;
};

class Init_SetTool485_Request_stop
{
public:
  explicit Init_SetTool485_Request_stop(::dobot_msgs_v4::srv::SetTool485_Request & msg)
  : msg_(msg)
  {}
  Init_SetTool485_Request_identify stop(::dobot_msgs_v4::srv::SetTool485_Request::_stop_type arg)
  {
    msg_.stop = std::move(arg);
    return Init_SetTool485_Request_identify(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetTool485_Request msg_;
};

class Init_SetTool485_Request_parity
{
public:
  explicit Init_SetTool485_Request_parity(::dobot_msgs_v4::srv::SetTool485_Request & msg)
  : msg_(msg)
  {}
  Init_SetTool485_Request_stop parity(::dobot_msgs_v4::srv::SetTool485_Request::_parity_type arg)
  {
    msg_.parity = std::move(arg);
    return Init_SetTool485_Request_stop(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetTool485_Request msg_;
};

class Init_SetTool485_Request_baudrate
{
public:
  Init_SetTool485_Request_baudrate()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SetTool485_Request_parity baudrate(::dobot_msgs_v4::srv::SetTool485_Request::_baudrate_type arg)
  {
    msg_.baudrate = std::move(arg);
    return Init_SetTool485_Request_parity(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetTool485_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::SetTool485_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_SetTool485_Request_baudrate();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_SetTool485_Response_res
{
public:
  Init_SetTool485_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::SetTool485_Response res(::dobot_msgs_v4::srv::SetTool485_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::SetTool485_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::SetTool485_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_SetTool485_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__SET_TOOL485__BUILDER_HPP_
