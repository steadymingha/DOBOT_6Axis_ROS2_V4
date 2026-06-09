// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:srv/CheckKitQuality.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__CHECK_KIT_QUALITY__BUILDER_HPP_
#define ARIAC_INTERFACES__SRV__DETAIL__CHECK_KIT_QUALITY__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/srv/detail/check_kit_quality__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_CheckKitQuality_Request_cell_type
{
public:
  Init_CheckKitQuality_Request_cell_type()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ariac_interfaces::srv::CheckKitQuality_Request cell_type(::ariac_interfaces::srv::CheckKitQuality_Request::_cell_type_type arg)
  {
    msg_.cell_type = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::CheckKitQuality_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::CheckKitQuality_Request>()
{
  return ariac_interfaces::srv::builder::Init_CheckKitQuality_Request_cell_type();
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_CheckKitQuality_Response_message
{
public:
  explicit Init_CheckKitQuality_Response_message(::ariac_interfaces::srv::CheckKitQuality_Response & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::srv::CheckKitQuality_Response message(::ariac_interfaces::srv::CheckKitQuality_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::CheckKitQuality_Response msg_;
};

class Init_CheckKitQuality_Response_is_good
{
public:
  Init_CheckKitQuality_Response_is_good()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_CheckKitQuality_Response_message is_good(::ariac_interfaces::srv::CheckKitQuality_Response::_is_good_type arg)
  {
    msg_.is_good = std::move(arg);
    return Init_CheckKitQuality_Response_message(msg_);
  }

private:
  ::ariac_interfaces::srv::CheckKitQuality_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::CheckKitQuality_Response>()
{
  return ariac_interfaces::srv::builder::Init_CheckKitQuality_Response_is_good();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__SRV__DETAIL__CHECK_KIT_QUALITY__BUILDER_HPP_
