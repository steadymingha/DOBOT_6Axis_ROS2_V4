// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:msg/InspectionReport.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__INSPECTION_REPORT__BUILDER_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__INSPECTION_REPORT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/msg/detail/inspection_report__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace msg
{

namespace builder
{

class Init_InspectionReport_defects
{
public:
  explicit Init_InspectionReport_defects(::ariac_interfaces::msg::InspectionReport & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::msg::InspectionReport defects(::ariac_interfaces::msg::InspectionReport::_defects_type arg)
  {
    msg_.defects = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::msg::InspectionReport msg_;
};

class Init_InspectionReport_passed
{
public:
  Init_InspectionReport_passed()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_InspectionReport_defects passed(::ariac_interfaces::msg::InspectionReport::_passed_type arg)
  {
    msg_.passed = std::move(arg);
    return Init_InspectionReport_defects(msg_);
  }

private:
  ::ariac_interfaces::msg::InspectionReport msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::msg::InspectionReport>()
{
  return ariac_interfaces::msg::builder::Init_InspectionReport_passed();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__INSPECTION_REPORT__BUILDER_HPP_
