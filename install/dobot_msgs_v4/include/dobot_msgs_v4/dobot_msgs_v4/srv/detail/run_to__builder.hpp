// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/RunTo.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__RUN_TO__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__RUN_TO__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/run_to__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_RunTo_Request_v
{
public:
  explicit Init_RunTo_Request_v(::dobot_msgs_v4::srv::RunTo_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::RunTo_Request v(::dobot_msgs_v4::srv::RunTo_Request::_v_type arg)
  {
    msg_.v = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::RunTo_Request msg_;
};

class Init_RunTo_Request_a
{
public:
  explicit Init_RunTo_Request_a(::dobot_msgs_v4::srv::RunTo_Request & msg)
  : msg_(msg)
  {}
  Init_RunTo_Request_v a(::dobot_msgs_v4::srv::RunTo_Request::_a_type arg)
  {
    msg_.a = std::move(arg);
    return Init_RunTo_Request_v(msg_);
  }

private:
  ::dobot_msgs_v4::srv::RunTo_Request msg_;
};

class Init_RunTo_Request_tool
{
public:
  explicit Init_RunTo_Request_tool(::dobot_msgs_v4::srv::RunTo_Request & msg)
  : msg_(msg)
  {}
  Init_RunTo_Request_a tool(::dobot_msgs_v4::srv::RunTo_Request::_tool_type arg)
  {
    msg_.tool = std::move(arg);
    return Init_RunTo_Request_a(msg_);
  }

private:
  ::dobot_msgs_v4::srv::RunTo_Request msg_;
};

class Init_RunTo_Request_user
{
public:
  explicit Init_RunTo_Request_user(::dobot_msgs_v4::srv::RunTo_Request & msg)
  : msg_(msg)
  {}
  Init_RunTo_Request_tool user(::dobot_msgs_v4::srv::RunTo_Request::_user_type arg)
  {
    msg_.user = std::move(arg);
    return Init_RunTo_Request_tool(msg_);
  }

private:
  ::dobot_msgs_v4::srv::RunTo_Request msg_;
};

class Init_RunTo_Request_move_type
{
public:
  explicit Init_RunTo_Request_move_type(::dobot_msgs_v4::srv::RunTo_Request & msg)
  : msg_(msg)
  {}
  Init_RunTo_Request_user move_type(::dobot_msgs_v4::srv::RunTo_Request::_move_type_type arg)
  {
    msg_.move_type = std::move(arg);
    return Init_RunTo_Request_user(msg_);
  }

private:
  ::dobot_msgs_v4::srv::RunTo_Request msg_;
};

class Init_RunTo_Request_f1
{
public:
  explicit Init_RunTo_Request_f1(::dobot_msgs_v4::srv::RunTo_Request & msg)
  : msg_(msg)
  {}
  Init_RunTo_Request_move_type f1(::dobot_msgs_v4::srv::RunTo_Request::_f1_type arg)
  {
    msg_.f1 = std::move(arg);
    return Init_RunTo_Request_move_type(msg_);
  }

private:
  ::dobot_msgs_v4::srv::RunTo_Request msg_;
};

class Init_RunTo_Request_e1
{
public:
  explicit Init_RunTo_Request_e1(::dobot_msgs_v4::srv::RunTo_Request & msg)
  : msg_(msg)
  {}
  Init_RunTo_Request_f1 e1(::dobot_msgs_v4::srv::RunTo_Request::_e1_type arg)
  {
    msg_.e1 = std::move(arg);
    return Init_RunTo_Request_f1(msg_);
  }

private:
  ::dobot_msgs_v4::srv::RunTo_Request msg_;
};

class Init_RunTo_Request_d1
{
public:
  explicit Init_RunTo_Request_d1(::dobot_msgs_v4::srv::RunTo_Request & msg)
  : msg_(msg)
  {}
  Init_RunTo_Request_e1 d1(::dobot_msgs_v4::srv::RunTo_Request::_d1_type arg)
  {
    msg_.d1 = std::move(arg);
    return Init_RunTo_Request_e1(msg_);
  }

private:
  ::dobot_msgs_v4::srv::RunTo_Request msg_;
};

class Init_RunTo_Request_c1
{
public:
  explicit Init_RunTo_Request_c1(::dobot_msgs_v4::srv::RunTo_Request & msg)
  : msg_(msg)
  {}
  Init_RunTo_Request_d1 c1(::dobot_msgs_v4::srv::RunTo_Request::_c1_type arg)
  {
    msg_.c1 = std::move(arg);
    return Init_RunTo_Request_d1(msg_);
  }

private:
  ::dobot_msgs_v4::srv::RunTo_Request msg_;
};

class Init_RunTo_Request_b1
{
public:
  explicit Init_RunTo_Request_b1(::dobot_msgs_v4::srv::RunTo_Request & msg)
  : msg_(msg)
  {}
  Init_RunTo_Request_c1 b1(::dobot_msgs_v4::srv::RunTo_Request::_b1_type arg)
  {
    msg_.b1 = std::move(arg);
    return Init_RunTo_Request_c1(msg_);
  }

private:
  ::dobot_msgs_v4::srv::RunTo_Request msg_;
};

class Init_RunTo_Request_a1
{
public:
  Init_RunTo_Request_a1()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_RunTo_Request_b1 a1(::dobot_msgs_v4::srv::RunTo_Request::_a1_type arg)
  {
    msg_.a1 = std::move(arg);
    return Init_RunTo_Request_b1(msg_);
  }

private:
  ::dobot_msgs_v4::srv::RunTo_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::RunTo_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_RunTo_Request_a1();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_RunTo_Response_res
{
public:
  Init_RunTo_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::RunTo_Response res(::dobot_msgs_v4::srv::RunTo_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::RunTo_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::RunTo_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_RunTo_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__RUN_TO__BUILDER_HPP_
