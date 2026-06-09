// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ariac_interfaces:msg/CompetitionTime.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_TIME__TRAITS_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_TIME__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ariac_interfaces/msg/detail/competition_time__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'start'
#include "builtin_interfaces/msg/detail/time__traits.hpp"
// Member 'elapsed'
// Member 'remaining'
#include "builtin_interfaces/msg/detail/duration__traits.hpp"

namespace ariac_interfaces
{

namespace msg
{

inline void to_flow_style_yaml(
  const CompetitionTime & msg,
  std::ostream & out)
{
  out << "{";
  // member: start
  {
    out << "start: ";
    to_flow_style_yaml(msg.start, out);
    out << ", ";
  }

  // member: elapsed
  {
    out << "elapsed: ";
    to_flow_style_yaml(msg.elapsed, out);
    out << ", ";
  }

  // member: remaining
  {
    out << "remaining: ";
    to_flow_style_yaml(msg.remaining, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const CompetitionTime & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: start
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "start:\n";
    to_block_style_yaml(msg.start, out, indentation + 2);
  }

  // member: elapsed
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "elapsed:\n";
    to_block_style_yaml(msg.elapsed, out, indentation + 2);
  }

  // member: remaining
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "remaining:\n";
    to_block_style_yaml(msg.remaining, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const CompetitionTime & msg, bool use_flow_style = false)
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
  const ariac_interfaces::msg::CompetitionTime & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::msg::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::msg::CompetitionTime & msg)
{
  return ariac_interfaces::msg::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::msg::CompetitionTime>()
{
  return "ariac_interfaces::msg::CompetitionTime";
}

template<>
inline const char * name<ariac_interfaces::msg::CompetitionTime>()
{
  return "ariac_interfaces/msg/CompetitionTime";
}

template<>
struct has_fixed_size<ariac_interfaces::msg::CompetitionTime>
  : std::integral_constant<bool, has_fixed_size<builtin_interfaces::msg::Duration>::value && has_fixed_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct has_bounded_size<ariac_interfaces::msg::CompetitionTime>
  : std::integral_constant<bool, has_bounded_size<builtin_interfaces::msg::Duration>::value && has_bounded_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct is_message<ariac_interfaces::msg::CompetitionTime>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_TIME__TRAITS_HPP_
