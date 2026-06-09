// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/DO.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__DO__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__DO__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/do__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_DO_Request_time
{
public:
  explicit Init_DO_Request_time(::dobot_msgs_v4::srv::DO_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::DO_Request time(::dobot_msgs_v4::srv::DO_Request::_time_type arg)
  {
    msg_.time = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::DO_Request msg_;
};

class Init_DO_Request_status
{
public:
  explicit Init_DO_Request_status(::dobot_msgs_v4::srv::DO_Request & msg)
  : msg_(msg)
  {}
  Init_DO_Request_time status(::dobot_msgs_v4::srv::DO_Request::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_DO_Request_time(msg_);
  }

private:
  ::dobot_msgs_v4::srv::DO_Request msg_;
};

class Init_DO_Request_index
{
public:
  Init_DO_Request_index()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DO_Request_status index(::dobot_msgs_v4::srv::DO_Request::_index_type arg)
  {
    msg_.index = std::move(arg);
    return Init_DO_Request_status(msg_);
  }

private:
  ::dobot_msgs_v4::srv::DO_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::DO_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_DO_Request_index();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_DO_Response_res
{
public:
  Init_DO_Response_res()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::dobot_msgs_v4::srv::DO_Response res(::dobot_msgs_v4::srv::DO_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::DO_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::DO_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_DO_Response_res();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__DO__BUILDER_HPP_
