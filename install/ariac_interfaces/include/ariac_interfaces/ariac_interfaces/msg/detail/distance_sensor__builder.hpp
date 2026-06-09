// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:msg/DistanceSensor.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__DISTANCE_SENSOR__BUILDER_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__DISTANCE_SENSOR__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/msg/detail/distance_sensor__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace msg
{

namespace builder
{

class Init_DistanceSensor_distance
{
public:
  explicit Init_DistanceSensor_distance(::ariac_interfaces::msg::DistanceSensor & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::msg::DistanceSensor distance(::ariac_interfaces::msg::DistanceSensor::_distance_type arg)
  {
    msg_.distance = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::msg::DistanceSensor msg_;
};

class Init_DistanceSensor_header
{
public:
  Init_DistanceSensor_header()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_DistanceSensor_distance header(::ariac_interfaces::msg::DistanceSensor::_header_type arg)
  {
    msg_.header = std::move(arg);
    return Init_DistanceSensor_distance(msg_);
  }

private:
  ::ariac_interfaces::msg::DistanceSensor msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::msg::DistanceSensor>()
{
  return ariac_interfaces::msg::builder::Init_DistanceSensor_header();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__DISTANCE_SENSOR__BUILDER_HPP_
