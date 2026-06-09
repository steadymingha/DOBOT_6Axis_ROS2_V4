// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ariac_interfaces:msg/ToolChangerStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__TOOL_CHANGER_STATUS__TRAITS_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__TOOL_CHANGER_STATUS__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ariac_interfaces/msg/detail/tool_changer_status__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace ariac_interfaces
{

namespace msg
{

inline void to_flow_style_yaml(
  const ToolChangerStatus & msg,
  std::ostream & out)
{
  out << "{";
  // member: attached_tool
  {
    out << "attached_tool: ";
    rosidl_generator_traits::value_to_yaml(msg.attached_tool, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ToolChangerStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: attached_tool
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "attached_tool: ";
    rosidl_generator_traits::value_to_yaml(msg.attached_tool, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ToolChangerStatus & msg, bool use_flow_style = false)
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
  const ariac_interfaces::msg::ToolChangerStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::msg::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::msg::ToolChangerStatus & msg)
{
  return ariac_interfaces::msg::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::msg::ToolChangerStatus>()
{
  return "ariac_interfaces::msg::ToolChangerStatus";
}

template<>
inline const char * name<ariac_interfaces::msg::ToolChangerStatus>()
{
  return "ariac_interfaces/msg/ToolChangerStatus";
}

template<>
struct has_fixed_size<ariac_interfaces::msg::ToolChangerStatus>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<ariac_interfaces::msg::ToolChangerStatus>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<ariac_interfaces::msg::ToolChangerStatus>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // ARIAC_INTERFACES__MSG__DETAIL__TOOL_CHANGER_STATUS__TRAITS_HPP_
