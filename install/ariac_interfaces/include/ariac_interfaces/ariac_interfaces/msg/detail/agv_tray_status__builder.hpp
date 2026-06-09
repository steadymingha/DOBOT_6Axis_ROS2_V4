// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:msg/AgvTrayStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__AGV_TRAY_STATUS__BUILDER_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__AGV_TRAY_STATUS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/msg/detail/agv_tray_status__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace msg
{

namespace builder
{

class Init_AgvTrayStatus_slot_4_occupied
{
public:
  explicit Init_AgvTrayStatus_slot_4_occupied(::ariac_interfaces::msg::AgvTrayStatus & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::msg::AgvTrayStatus slot_4_occupied(::ariac_interfaces::msg::AgvTrayStatus::_slot_4_occupied_type arg)
  {
    msg_.slot_4_occupied = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::msg::AgvTrayStatus msg_;
};

class Init_AgvTrayStatus_slot_3_occupied
{
public:
  explicit Init_AgvTrayStatus_slot_3_occupied(::ariac_interfaces::msg::AgvTrayStatus & msg)
  : msg_(msg)
  {}
  Init_AgvTrayStatus_slot_4_occupied slot_3_occupied(::ariac_interfaces::msg::AgvTrayStatus::_slot_3_occupied_type arg)
  {
    msg_.slot_3_occupied = std::move(arg);
    return Init_AgvTrayStatus_slot_4_occupied(msg_);
  }

private:
  ::ariac_interfaces::msg::AgvTrayStatus msg_;
};

class Init_AgvTrayStatus_slot_2_occupied
{
public:
  explicit Init_AgvTrayStatus_slot_2_occupied(::ariac_interfaces::msg::AgvTrayStatus & msg)
  : msg_(msg)
  {}
  Init_AgvTrayStatus_slot_3_occupied slot_2_occupied(::ariac_interfaces::msg::AgvTrayStatus::_slot_2_occupied_type arg)
  {
    msg_.slot_2_occupied = std::move(arg);
    return Init_AgvTrayStatus_slot_3_occupied(msg_);
  }

private:
  ::ariac_interfaces::msg::AgvTrayStatus msg_;
};

class Init_AgvTrayStatus_slot_1_occupied
{
public:
  Init_AgvTrayStatus_slot_1_occupied()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_AgvTrayStatus_slot_2_occupied slot_1_occupied(::ariac_interfaces::msg::AgvTrayStatus::_slot_1_occupied_type arg)
  {
    msg_.slot_1_occupied = std::move(arg);
    return Init_AgvTrayStatus_slot_2_occupied(msg_);
  }

private:
  ::ariac_interfaces::msg::AgvTrayStatus msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::msg::AgvTrayStatus>()
{
  return ariac_interfaces::msg::builder::Init_AgvTrayStatus_slot_1_occupied();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__AGV_TRAY_STATUS__BUILDER_HPP_
