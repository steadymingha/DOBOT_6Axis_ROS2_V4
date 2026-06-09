// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/GetInputFloat.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__GET_INPUT_FLOAT__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__GET_INPUT_FLOAT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/get_input_float__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_GetInputFloat_Request_address
{
public:
  Init_GetInputFloat_Request_address()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::GetInputFloat_Request address(::dobot_msgs_v4::srv::GetInputFloat_Request::_address_type arg)
  {
    msg_.address = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::GetInputFloat_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::GetInputFloat_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_GetInputFloat_Request_address();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_GetInputFloat_Response_res
{
public:
  explicit Init_GetInputFloat_Response_res(::dobot_msgs_v4::srv::GetInputFloat_Response & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::GetInputFloat_Response res(::dobot_msgs_v4::srv::GetInputFloat_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::GetInputFloat_Response msg_;
};

class Init_GetInputFloat_Response_robot_return
{
public:
  Init_GetInputFloat_Response_robot_return()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GetInputFloat_Response_res robot_return(::dobot_msgs_v4::srv::GetInputFloat_Response::_robot_return_type arg)
  {
    msg_.robot_return = std::move(arg);
    return Init_GetInputFloat_Response_res(msg_);
  }

private:
  ::dobot_msgs_v4::srv::GetInputFloat_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::GetInputFloat_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_GetInputFloat_Response_robot_return();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__GET_INPUT_FLOAT__BUILDER_HPP_
