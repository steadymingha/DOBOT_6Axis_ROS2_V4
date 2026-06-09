// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/InverseSolution.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__INVERSE_SOLUTION__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__INVERSE_SOLUTION__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/inverse_solution__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_InverseSolution_Request_parameter
{
public:
  Init_InverseSolution_Request_parameter()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::InverseSolution_Request parameter(::dobot_msgs_v4::srv::InverseSolution_Request::_parameter_type arg)
  {
    msg_.parameter = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::InverseSolution_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::InverseSolution_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_InverseSolution_Request_parameter();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_InverseSolution_Response_res
{
public:
  Init_InverseSolution_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::InverseSolution_Response res(::dobot_msgs_v4::srv::InverseSolution_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::InverseSolution_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::InverseSolution_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_InverseSolution_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__INVERSE_SOLUTION__BUILDER_HPP_
