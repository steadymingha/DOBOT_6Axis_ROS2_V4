// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/ServoJ.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__SERVO_J__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__SERVO_J__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/servo_j__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_ServoJ_Request_param_value
{
public:
  explicit Init_ServoJ_Request_param_value(::dobot_msgs_v4::srv::ServoJ_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::ServoJ_Request param_value(::dobot_msgs_v4::srv::ServoJ_Request::_param_value_type arg)
  {
    msg_.param_value = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ServoJ_Request msg_;
};

class Init_ServoJ_Request_f
{
public:
  explicit Init_ServoJ_Request_f(::dobot_msgs_v4::srv::ServoJ_Request & msg)
  : msg_(msg)
  {}
  Init_ServoJ_Request_param_value f(::dobot_msgs_v4::srv::ServoJ_Request::_f_type arg)
  {
    msg_.f = std::move(arg);
    return Init_ServoJ_Request_param_value(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ServoJ_Request msg_;
};

class Init_ServoJ_Request_e
{
public:
  explicit Init_ServoJ_Request_e(::dobot_msgs_v4::srv::ServoJ_Request & msg)
  : msg_(msg)
  {}
  Init_ServoJ_Request_f e(::dobot_msgs_v4::srv::ServoJ_Request::_e_type arg)
  {
    msg_.e = std::move(arg);
    return Init_ServoJ_Request_f(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ServoJ_Request msg_;
};

class Init_ServoJ_Request_d
{
public:
  explicit Init_ServoJ_Request_d(::dobot_msgs_v4::srv::ServoJ_Request & msg)
  : msg_(msg)
  {}
  Init_ServoJ_Request_e d(::dobot_msgs_v4::srv::ServoJ_Request::_d_type arg)
  {
    msg_.d = std::move(arg);
    return Init_ServoJ_Request_e(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ServoJ_Request msg_;
};

class Init_ServoJ_Request_c
{
public:
  explicit Init_ServoJ_Request_c(::dobot_msgs_v4::srv::ServoJ_Request & msg)
  : msg_(msg)
  {}
  Init_ServoJ_Request_d c(::dobot_msgs_v4::srv::ServoJ_Request::_c_type arg)
  {
    msg_.c = std::move(arg);
    return Init_ServoJ_Request_d(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ServoJ_Request msg_;
};

class Init_ServoJ_Request_b
{
public:
  explicit Init_ServoJ_Request_b(::dobot_msgs_v4::srv::ServoJ_Request & msg)
  : msg_(msg)
  {}
  Init_ServoJ_Request_c b(::dobot_msgs_v4::srv::ServoJ_Request::_b_type arg)
  {
    msg_.b = std::move(arg);
    return Init_ServoJ_Request_c(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ServoJ_Request msg_;
};

class Init_ServoJ_Request_a
{
public:
  Init_ServoJ_Request_a()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ServoJ_Request_b a(::dobot_msgs_v4::srv::ServoJ_Request::_a_type arg)
  {
    msg_.a = std::move(arg);
    return Init_ServoJ_Request_b(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ServoJ_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::ServoJ_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_ServoJ_Request_a();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_ServoJ_Response_res
{
public:
  Init_ServoJ_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::ServoJ_Response res(::dobot_msgs_v4::srv::ServoJ_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ServoJ_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::ServoJ_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_ServoJ_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__SERVO_J__BUILDER_HPP_
