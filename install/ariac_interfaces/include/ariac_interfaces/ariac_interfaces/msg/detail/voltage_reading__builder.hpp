// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:msg/VoltageReading.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__VOLTAGE_READING__BUILDER_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__VOLTAGE_READING__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/msg/detail/voltage_reading__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace msg
{

namespace builder
{

class Init_VoltageReading_operation_status
{
public:
  explicit Init_VoltageReading_operation_status(::ariac_interfaces::msg::VoltageReading & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::msg::VoltageReading operation_status(::ariac_interfaces::msg::VoltageReading::_operation_status_type arg)
  {
    msg_.operation_status = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::msg::VoltageReading msg_;
};

class Init_VoltageReading_voltage
{
public:
  Init_VoltageReading_voltage()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_VoltageReading_operation_status voltage(::ariac_interfaces::msg::VoltageReading::_voltage_type arg)
  {
    msg_.voltage = std::move(arg);
    return Init_VoltageReading_operation_status(msg_);
  }

private:
  ::ariac_interfaces::msg::VoltageReading msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::msg::VoltageReading>()
{
  return ariac_interfaces::msg::builder::Init_VoltageReading_voltage();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__VOLTAGE_READING__BUILDER_HPP_
