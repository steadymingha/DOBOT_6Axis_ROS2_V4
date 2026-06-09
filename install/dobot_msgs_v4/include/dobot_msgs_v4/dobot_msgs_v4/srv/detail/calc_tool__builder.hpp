// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/CalcTool.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__CALC_TOOL__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__CALC_TOOL__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/calc_tool__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_CalcTool_Request_offset
{
public:
  explicit Init_CalcTool_Request_offset(::dobot_msgs_v4::srv::CalcTool_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::CalcTool_Request offset(::dobot_msgs_v4::srv::CalcTool_Request::_offset_type arg)
  {
    msg_.offset = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::CalcTool_Request msg_;
};

class Init_CalcTool_Request_matrix
{
public:
  explicit Init_CalcTool_Request_matrix(::dobot_msgs_v4::srv::CalcTool_Request & msg)
  : msg_(msg)
  {}
  Init_CalcTool_Request_offset matrix(::dobot_msgs_v4::srv::CalcTool_Request::_matrix_type arg)
  {
    msg_.matrix = std::move(arg);
    return Init_CalcTool_Request_offset(msg_);
  }

private:
  ::dobot_msgs_v4::srv::CalcTool_Request msg_;
};

class Init_CalcTool_Request_index
{
public:
  Init_CalcTool_Request_index()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_CalcTool_Request_matrix index(::dobot_msgs_v4::srv::CalcTool_Request::_index_type arg)
  {
    msg_.index = std::move(arg);
    return Init_CalcTool_Request_matrix(msg_);
  }

private:
  ::dobot_msgs_v4::srv::CalcTool_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::CalcTool_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_CalcTool_Request_index();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_CalcTool_Response_res
{
public:
  Init_CalcTool_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::CalcTool_Response res(::dobot_msgs_v4::srv::CalcTool_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::CalcTool_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::CalcTool_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_CalcTool_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__CALC_TOOL__BUILDER_HPP_
