// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:msg/CellDefect.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__CELL_DEFECT__BUILDER_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__CELL_DEFECT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/msg/detail/cell_defect__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace msg
{

namespace builder
{

class Init_CellDefect_z
{
public:
  explicit Init_CellDefect_z(::ariac_interfaces::msg::CellDefect & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::msg::CellDefect z(::ariac_interfaces::msg::CellDefect::_z_type arg)
  {
    msg_.z = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::msg::CellDefect msg_;
};

class Init_CellDefect_theta
{
public:
  explicit Init_CellDefect_theta(::ariac_interfaces::msg::CellDefect & msg)
  : msg_(msg)
  {}
  Init_CellDefect_z theta(::ariac_interfaces::msg::CellDefect::_theta_type arg)
  {
    msg_.theta = std::move(arg);
    return Init_CellDefect_z(msg_);
  }

private:
  ::ariac_interfaces::msg::CellDefect msg_;
};

class Init_CellDefect_defect_type
{
public:
  Init_CellDefect_defect_type()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_CellDefect_theta defect_type(::ariac_interfaces::msg::CellDefect::_defect_type_type arg)
  {
    msg_.defect_type = std::move(arg);
    return Init_CellDefect_theta(msg_);
  }

private:
  ::ariac_interfaces::msg::CellDefect msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::msg::CellDefect>()
{
  return ariac_interfaces::msg::builder::Init_CellDefect_defect_type();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__CELL_DEFECT__BUILDER_HPP_
