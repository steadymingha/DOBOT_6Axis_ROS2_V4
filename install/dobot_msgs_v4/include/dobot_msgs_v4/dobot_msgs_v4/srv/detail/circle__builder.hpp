// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/Circle.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__CIRCLE__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__CIRCLE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/circle__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_Circle_Request_param_value
{
public:
  explicit Init_Circle_Request_param_value(::dobot_msgs_v4::srv::Circle_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::Circle_Request param_value(::dobot_msgs_v4::srv::Circle_Request::_param_value_type arg)
  {
    msg_.param_value = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

class Init_Circle_Request_count
{
public:
  explicit Init_Circle_Request_count(::dobot_msgs_v4::srv::Circle_Request & msg)
  : msg_(msg)
  {}
  Init_Circle_Request_param_value count(::dobot_msgs_v4::srv::Circle_Request::_count_type arg)
  {
    msg_.count = std::move(arg);
    return Init_Circle_Request_param_value(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

class Init_Circle_Request_f2
{
public:
  explicit Init_Circle_Request_f2(::dobot_msgs_v4::srv::Circle_Request & msg)
  : msg_(msg)
  {}
  Init_Circle_Request_count f2(::dobot_msgs_v4::srv::Circle_Request::_f2_type arg)
  {
    msg_.f2 = std::move(arg);
    return Init_Circle_Request_count(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

class Init_Circle_Request_e2
{
public:
  explicit Init_Circle_Request_e2(::dobot_msgs_v4::srv::Circle_Request & msg)
  : msg_(msg)
  {}
  Init_Circle_Request_f2 e2(::dobot_msgs_v4::srv::Circle_Request::_e2_type arg)
  {
    msg_.e2 = std::move(arg);
    return Init_Circle_Request_f2(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

class Init_Circle_Request_d2
{
public:
  explicit Init_Circle_Request_d2(::dobot_msgs_v4::srv::Circle_Request & msg)
  : msg_(msg)
  {}
  Init_Circle_Request_e2 d2(::dobot_msgs_v4::srv::Circle_Request::_d2_type arg)
  {
    msg_.d2 = std::move(arg);
    return Init_Circle_Request_e2(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

class Init_Circle_Request_c2
{
public:
  explicit Init_Circle_Request_c2(::dobot_msgs_v4::srv::Circle_Request & msg)
  : msg_(msg)
  {}
  Init_Circle_Request_d2 c2(::dobot_msgs_v4::srv::Circle_Request::_c2_type arg)
  {
    msg_.c2 = std::move(arg);
    return Init_Circle_Request_d2(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

class Init_Circle_Request_b2
{
public:
  explicit Init_Circle_Request_b2(::dobot_msgs_v4::srv::Circle_Request & msg)
  : msg_(msg)
  {}
  Init_Circle_Request_c2 b2(::dobot_msgs_v4::srv::Circle_Request::_b2_type arg)
  {
    msg_.b2 = std::move(arg);
    return Init_Circle_Request_c2(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

class Init_Circle_Request_a2
{
public:
  explicit Init_Circle_Request_a2(::dobot_msgs_v4::srv::Circle_Request & msg)
  : msg_(msg)
  {}
  Init_Circle_Request_b2 a2(::dobot_msgs_v4::srv::Circle_Request::_a2_type arg)
  {
    msg_.a2 = std::move(arg);
    return Init_Circle_Request_b2(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

class Init_Circle_Request_f
{
public:
  explicit Init_Circle_Request_f(::dobot_msgs_v4::srv::Circle_Request & msg)
  : msg_(msg)
  {}
  Init_Circle_Request_a2 f(::dobot_msgs_v4::srv::Circle_Request::_f_type arg)
  {
    msg_.f = std::move(arg);
    return Init_Circle_Request_a2(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

class Init_Circle_Request_e
{
public:
  explicit Init_Circle_Request_e(::dobot_msgs_v4::srv::Circle_Request & msg)
  : msg_(msg)
  {}
  Init_Circle_Request_f e(::dobot_msgs_v4::srv::Circle_Request::_e_type arg)
  {
    msg_.e = std::move(arg);
    return Init_Circle_Request_f(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

class Init_Circle_Request_d
{
public:
  explicit Init_Circle_Request_d(::dobot_msgs_v4::srv::Circle_Request & msg)
  : msg_(msg)
  {}
  Init_Circle_Request_e d(::dobot_msgs_v4::srv::Circle_Request::_d_type arg)
  {
    msg_.d = std::move(arg);
    return Init_Circle_Request_e(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

class Init_Circle_Request_c
{
public:
  explicit Init_Circle_Request_c(::dobot_msgs_v4::srv::Circle_Request & msg)
  : msg_(msg)
  {}
  Init_Circle_Request_d c(::dobot_msgs_v4::srv::Circle_Request::_c_type arg)
  {
    msg_.c = std::move(arg);
    return Init_Circle_Request_d(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

class Init_Circle_Request_b
{
public:
  explicit Init_Circle_Request_b(::dobot_msgs_v4::srv::Circle_Request & msg)
  : msg_(msg)
  {}
  Init_Circle_Request_c b(::dobot_msgs_v4::srv::Circle_Request::_b_type arg)
  {
    msg_.b = std::move(arg);
    return Init_Circle_Request_c(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

class Init_Circle_Request_a
{
public:
  explicit Init_Circle_Request_a(::dobot_msgs_v4::srv::Circle_Request & msg)
  : msg_(msg)
  {}
  Init_Circle_Request_b a(::dobot_msgs_v4::srv::Circle_Request::_a_type arg)
  {
    msg_.a = std::move(arg);
    return Init_Circle_Request_b(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

class Init_Circle_Request_mode
{
public:
  Init_Circle_Request_mode()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Circle_Request_a mode(::dobot_msgs_v4::srv::Circle_Request::_mode_type arg)
  {
    msg_.mode = std::move(arg);
    return Init_Circle_Request_a(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::Circle_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_Circle_Request_mode();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_Circle_Response_res
{
public:
  Init_Circle_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::Circle_Response res(::dobot_msgs_v4::srv::Circle_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::Circle_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::Circle_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_Circle_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__CIRCLE__BUILDER_HPP_
