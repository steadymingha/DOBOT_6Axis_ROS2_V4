// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ariac_interfaces:msg/VoltageReading.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__VOLTAGE_READING__TRAITS_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__VOLTAGE_READING__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ariac_interfaces/msg/detail/voltage_reading__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace ariac_interfaces
{

namespace msg
{

inline void to_flow_style_yaml(
  const VoltageReading & msg,
  std::ostream & out)
{
  out << "{";
  // member: voltage
  {
    out << "voltage: ";
    rosidl_generator_traits::value_to_yaml(msg.voltage, out);
    out << ", ";
  }

  // member: operation_status
  {
    out << "operation_status: ";
    rosidl_generator_traits::value_to_yaml(msg.operation_status, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const VoltageReading & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: voltage
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "voltage: ";
    rosidl_generator_traits::value_to_yaml(msg.voltage, out);
    out << "\n";
  }

  // member: operation_status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "operation_status: ";
    rosidl_generator_traits::value_to_yaml(msg.operation_status, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const VoltageReading & msg, bool use_flow_style = false)
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
  const ariac_interfaces::msg::VoltageReading & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::msg::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::msg::VoltageReading & msg)
{
  return ariac_interfaces::msg::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::msg::VoltageReading>()
{
  return "ariac_interfaces::msg::VoltageReading";
}

template<>
inline const char * name<ariac_interfaces::msg::VoltageReading>()
{
  return "ariac_interfaces/msg/VoltageReading";
}

template<>
struct has_fixed_size<ariac_interfaces::msg::VoltageReading>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<ariac_interfaces::msg::VoltageReading>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<ariac_interfaces::msg::VoltageReading>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // ARIAC_INTERFACES__MSG__DETAIL__VOLTAGE_READING__TRAITS_HPP_
