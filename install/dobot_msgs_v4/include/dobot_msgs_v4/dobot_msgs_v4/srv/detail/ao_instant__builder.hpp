// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/AOInstant.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__AO_INSTANT__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__AO_INSTANT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/ao_instant__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_AOInstant_Request_value
{
public:
  explicit Init_AOInstant_Request_value(::dobot_msgs_v4::srv::AOInstant_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::AOInstant_Request value(::dobot_msgs_v4::srv::AOInstant_Request::_value_type arg)
  {
    msg_.value = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::AOInstant_Request msg_;
};

class Init_AOInstant_Request_index
{
public:
  Init_AOInstant_Request_index()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_AOInstant_Request_value index(::dobot_msgs_v4::srv::AOInstant_Request::_index_type arg)
  {
    msg_.index = std::move(arg);
    return Init_AOInstant_Request_value(msg_);
  }

private:
  ::dobot_msgs_v4::srv::AOInstant_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::AOInstant_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_AOInstant_Request_index();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_AOInstant_Response_res
{
public:
  Init_AOInstant_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::AOInstant_Response res(::dobot_msgs_v4::srv::AOInstant_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::AOInstant_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::AOInstant_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_AOInstant_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__AO_INSTANT__BUILDER_HPP_
