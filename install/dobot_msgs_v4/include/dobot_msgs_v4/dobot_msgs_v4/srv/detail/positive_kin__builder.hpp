// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/PositiveKin.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__POSITIVE_KIN__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__POSITIVE_KIN__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/positive_kin__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_PositiveKin_Request_tool
{
public:
  explicit Init_PositiveKin_Request_tool(::dobot_msgs_v4::srv::PositiveKin_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::PositiveKin_Request tool(::dobot_msgs_v4::srv::PositiveKin_Request::_tool_type arg)
  {
    msg_.tool = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::PositiveKin_Request msg_;
};

class Init_PositiveKin_Request_user
{
public:
  explicit Init_PositiveKin_Request_user(::dobot_msgs_v4::srv::PositiveKin_Request & msg)
  : msg_(msg)
  {}
  Init_PositiveKin_Request_tool user(::dobot_msgs_v4::srv::PositiveKin_Request::_user_type arg)
  {
    msg_.user = std::move(arg);
    return Init_PositiveKin_Request_tool(msg_);
  }

private:
  ::dobot_msgs_v4::srv::PositiveKin_Request msg_;
};

class Init_PositiveKin_Request_j6
{
public:
  explicit Init_PositiveKin_Request_j6(::dobot_msgs_v4::srv::PositiveKin_Request & msg)
  : msg_(msg)
  {}
  Init_PositiveKin_Request_user j6(::dobot_msgs_v4::srv::PositiveKin_Request::_j6_type arg)
  {
    msg_.j6 = std::move(arg);
    return Init_PositiveKin_Request_user(msg_);
  }

private:
  ::dobot_msgs_v4::srv::PositiveKin_Request msg_;
};

class Init_PositiveKin_Request_j5
{
public:
  explicit Init_PositiveKin_Request_j5(::dobot_msgs_v4::srv::PositiveKin_Request & msg)
  : msg_(msg)
  {}
  Init_PositiveKin_Request_j6 j5(::dobot_msgs_v4::srv::PositiveKin_Request::_j5_type arg)
  {
    msg_.j5 = std::move(arg);
    return Init_PositiveKin_Request_j6(msg_);
  }

private:
  ::dobot_msgs_v4::srv::PositiveKin_Request msg_;
};

class Init_PositiveKin_Request_j4
{
public:
  explicit Init_PositiveKin_Request_j4(::dobot_msgs_v4::srv::PositiveKin_Request & msg)
  : msg_(msg)
  {}
  Init_PositiveKin_Request_j5 j4(::dobot_msgs_v4::srv::PositiveKin_Request::_j4_type arg)
  {
    msg_.j4 = std::move(arg);
    return Init_PositiveKin_Request_j5(msg_);
  }

private:
  ::dobot_msgs_v4::srv::PositiveKin_Request msg_;
};

class Init_PositiveKin_Request_j3
{
public:
  explicit Init_PositiveKin_Request_j3(::dobot_msgs_v4::srv::PositiveKin_Request & msg)
  : msg_(msg)
  {}
  Init_PositiveKin_Request_j4 j3(::dobot_msgs_v4::srv::PositiveKin_Request::_j3_type arg)
  {
    msg_.j3 = std::move(arg);
    return Init_PositiveKin_Request_j4(msg_);
  }

private:
  ::dobot_msgs_v4::srv::PositiveKin_Request msg_;
};

class Init_PositiveKin_Request_j2
{
public:
  explicit Init_PositiveKin_Request_j2(::dobot_msgs_v4::srv::PositiveKin_Request & msg)
  : msg_(msg)
  {}
  Init_PositiveKin_Request_j3 j2(::dobot_msgs_v4::srv::PositiveKin_Request::_j2_type arg)
  {
    msg_.j2 = std::move(arg);
    return Init_PositiveKin_Request_j3(msg_);
  }

private:
  ::dobot_msgs_v4::srv::PositiveKin_Request msg_;
};

class Init_PositiveKin_Request_j1
{
public:
  Init_PositiveKin_Request_j1()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_PositiveKin_Request_j2 j1(::dobot_msgs_v4::srv::PositiveKin_Request::_j1_type arg)
  {
    msg_.j1 = std::move(arg);
    return Init_PositiveKin_Request_j2(msg_);
  }

private:
  ::dobot_msgs_v4::srv::PositiveKin_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::PositiveKin_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_PositiveKin_Request_j1();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_PositiveKin_Response_res
{
public:
  explicit Init_PositiveKin_Response_res(::dobot_msgs_v4::srv::PositiveKin_Response & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::PositiveKin_Response res(::dobot_msgs_v4::srv::PositiveKin_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::PositiveKin_Response msg_;
};

class Init_PositiveKin_Response_robot_return
{
public:
  Init_PositiveKin_Response_robot_return()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_PositiveKin_Response_res robot_return(::dobot_msgs_v4::srv::PositiveKin_Response::_robot_return_type arg)
  {
    msg_.robot_return = std::move(arg);
    return Init_PositiveKin_Response_res(msg_);
  }

private:
  ::dobot_msgs_v4::srv::PositiveKin_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::PositiveKin_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_PositiveKin_Response_robot_return();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__POSITIVE_KIN__BUILDER_HPP_
