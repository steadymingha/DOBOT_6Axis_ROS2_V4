// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/DOGroupDEC.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__DO_GROUP_DEC__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__DO_GROUP_DEC__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/do_group_dec__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_DOGroupDEC_Request_value
{
public:
  explicit Init_DOGroupDEC_Request_value(::dobot_msgs_v4::srv::DOGroupDEC_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::DOGroupDEC_Request value(::dobot_msgs_v4::srv::DOGroupDEC_Request::_value_type arg)
  {
    msg_.value = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::DOGroupDEC_Request msg_;
};

class Init_DOGroupDEC_Request_group
{
public:
  Init_DOGroupDEC_Request_group()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DOGroupDEC_Request_value group(::dobot_msgs_v4::srv::DOGroupDEC_Request::_group_type arg)
  {
    msg_.group = std::move(arg);
    return Init_DOGroupDEC_Request_value(msg_);
  }

private:
  ::dobot_msgs_v4::srv::DOGroupDEC_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::DOGroupDEC_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_DOGroupDEC_Request_group();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_DOGroupDEC_Response_res
{
public:
  Init_DOGroupDEC_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::DOGroupDEC_Response res(::dobot_msgs_v4::srv::DOGroupDEC_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::DOGroupDEC_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::DOGroupDEC_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_DOGroupDEC_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__DO_GROUP_DEC__BUILDER_HPP_
