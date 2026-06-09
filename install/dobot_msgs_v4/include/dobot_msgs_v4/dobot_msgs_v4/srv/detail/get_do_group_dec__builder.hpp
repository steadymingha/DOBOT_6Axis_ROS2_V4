// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/GetDOGroupDEC.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__GET_DO_GROUP_DEC__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__GET_DO_GROUP_DEC__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/get_do_group_dec__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_GetDOGroupDEC_Request_value
{
public:
  explicit Init_GetDOGroupDEC_Request_value(::dobot_msgs_v4::srv::GetDOGroupDEC_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::GetDOGroupDEC_Request value(::dobot_msgs_v4::srv::GetDOGroupDEC_Request::_value_type arg)
  {
    msg_.value = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::GetDOGroupDEC_Request msg_;
};

class Init_GetDOGroupDEC_Request_group
{
public:
  Init_GetDOGroupDEC_Request_group()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GetDOGroupDEC_Request_value group(::dobot_msgs_v4::srv::GetDOGroupDEC_Request::_group_type arg)
  {
    msg_.group = std::move(arg);
    return Init_GetDOGroupDEC_Request_value(msg_);
  }

private:
  ::dobot_msgs_v4::srv::GetDOGroupDEC_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::GetDOGroupDEC_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_GetDOGroupDEC_Request_group();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_GetDOGroupDEC_Response_res
{
public:
  explicit Init_GetDOGroupDEC_Response_res(::dobot_msgs_v4::srv::GetDOGroupDEC_Response & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::GetDOGroupDEC_Response res(::dobot_msgs_v4::srv::GetDOGroupDEC_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::GetDOGroupDEC_Response msg_;
};

class Init_GetDOGroupDEC_Response_robot_return
{
public:
  Init_GetDOGroupDEC_Response_robot_return()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GetDOGroupDEC_Response_res robot_return(::dobot_msgs_v4::srv::GetDOGroupDEC_Response::_robot_return_type arg)
  {
    msg_.robot_return = std::move(arg);
    return Init_GetDOGroupDEC_Response_res(msg_);
  }

private:
  ::dobot_msgs_v4::srv::GetDOGroupDEC_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::GetDOGroupDEC_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_GetDOGroupDEC_Response_robot_return();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__GET_DO_GROUP_DEC__BUILDER_HPP_
