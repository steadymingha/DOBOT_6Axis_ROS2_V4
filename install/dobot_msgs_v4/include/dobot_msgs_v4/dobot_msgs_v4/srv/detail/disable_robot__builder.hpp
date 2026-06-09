// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/DisableRobot.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__DISABLE_ROBOT__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__DISABLE_ROBOT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/disable_robot__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{


}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::DisableRobot_Request>()
{
  return ::dobot_msgs_v4::srv::DisableRobot_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_DisableRobot_Response_res
{
public:
  explicit Init_DisableRobot_Response_res(::dobot_msgs_v4::srv::DisableRobot_Response & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::DisableRobot_Response res(::dobot_msgs_v4::srv::DisableRobot_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::DisableRobot_Response msg_;
};

class Init_DisableRobot_Response_robot_return
{
public:
  Init_DisableRobot_Response_robot_return()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DisableRobot_Response_res robot_return(::dobot_msgs_v4::srv::DisableRobot_Response::_robot_return_type arg)
  {
    msg_.robot_return = std::move(arg);
    return Init_DisableRobot_Response_res(msg_);
  }

private:
  ::dobot_msgs_v4::srv::DisableRobot_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::DisableRobot_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_DisableRobot_Response_robot_return();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__DISABLE_ROBOT__BUILDER_HPP_
