// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:msg/CellFeederStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__CELL_FEEDER_STATUS__BUILDER_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__CELL_FEEDER_STATUS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/msg/detail/cell_feeder_status__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace msg
{

namespace builder
{

class Init_CellFeederStatus_feed_rate
{
public:
  explicit Init_CellFeederStatus_feed_rate(::ariac_interfaces::msg::CellFeederStatus & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::msg::CellFeederStatus feed_rate(::ariac_interfaces::msg::CellFeederStatus::_feed_rate_type arg)
  {
    msg_.feed_rate = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::msg::CellFeederStatus msg_;
};

class Init_CellFeederStatus_cell_type
{
public:
  Init_CellFeederStatus_cell_type()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_CellFeederStatus_feed_rate cell_type(::ariac_interfaces::msg::CellFeederStatus::_cell_type_type arg)
  {
    msg_.cell_type = std::move(arg);
    return Init_CellFeederStatus_feed_rate(msg_);
  }

private:
  ::ariac_interfaces::msg::CellFeederStatus msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::msg::CellFeederStatus>()
{
  return ariac_interfaces::msg::builder::Init_CellFeederStatus_cell_type();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__CELL_FEEDER_STATUS__BUILDER_HPP_
