// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ariac_interfaces:msg/ConveyorStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__TRAITS_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ariac_interfaces/msg/detail/conveyor_status__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace ariac_interfaces
{

namespace msg
{

inline void to_flow_style_yaml(
  const ConveyorStatus & msg,
  std::ostream & out)
{
  out << "{";
  // member: direction
  {
    out << "direction: ";
    rosidl_generator_traits::value_to_yaml(msg.direction, out);
    out << ", ";
  }

  // member: speed
  {
    out << "speed: ";
    rosidl_generator_traits::value_to_yaml(msg.speed, out);
    out << ", ";
  }

  // member: operating_status
  {
    out << "operating_status: ";
    rosidl_generator_traits::value_to_yaml(msg.operating_status, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ConveyorStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: direction
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "direction: ";
    rosidl_generator_traits::value_to_yaml(msg.direction, out);
    out << "\n";
  }

  // member: speed
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "speed: ";
    rosidl_generator_traits::value_to_yaml(msg.speed, out);
    out << "\n";
  }

  // member: operating_status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "operating_status: ";
    rosidl_generator_traits::value_to_yaml(msg.operating_status, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ConveyorStatus & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace msg

}  // namespace ariac_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use ariac_interfaces::msg::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ariac_interfaces::msg::ConveyorStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::msg::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::msg::ConveyorStatus & msg)
{
  return ariac_interfaces::msg::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::msg::ConveyorStatus>()
{
  return "ariac_interfaces::msg::ConveyorStatus";
}

template<>
inline const char * name<ariac_interfaces::msg::ConveyorStatus>()
{
  return "ariac_interfaces/msg/ConveyorStatus";
}

template<>
struct has_fixed_size<ariac_interfaces::msg::ConveyorStatus>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<ariac_interfaces::msg::ConveyorStatus>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<ariac_interfaces::msg::ConveyorStatus>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__TRAITS_HPP_
