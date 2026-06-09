// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ariac_interfaces:msg/CompetitionStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATUS__TRAITS_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATUS__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ariac_interfaces/msg/detail/competition_status__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'time'
#include "ariac_interfaces/msg/detail/competition_time__traits.hpp"

namespace ariac_interfaces
{

namespace msg
{

inline void to_flow_style_yaml(
  const CompetitionStatus & msg,
  std::ostream & out)
{
  out << "{";
  // member: competition_state
  {
    out << "competition_state: ";
    rosidl_generator_traits::value_to_yaml(msg.competition_state, out);
    out << ", ";
  }

  // member: num_kits
  {
    out << "num_kits: ";
    rosidl_generator_traits::value_to_yaml(msg.num_kits, out);
    out << ", ";
  }

  // member: num_modules
  {
    out << "num_modules: ";
    rosidl_generator_traits::value_to_yaml(msg.num_modules, out);
    out << ", ";
  }

  // member: num_kits_remaining
  {
    out << "num_kits_remaining: ";
    rosidl_generator_traits::value_to_yaml(msg.num_kits_remaining, out);
    out << ", ";
  }

  // member: num_modules_remaining
  {
    out << "num_modules_remaining: ";
    rosidl_generator_traits::value_to_yaml(msg.num_modules_remaining, out);
    out << ", ";
  }

  // member: time
  {
    out << "time: ";
    to_flow_style_yaml(msg.time, out);
    out << ", ";
  }

  // member: run_id
  {
    out << "run_id: ";
    rosidl_generator_traits::value_to_yaml(msg.run_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const CompetitionStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: competition_state
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "competition_state: ";
    rosidl_generator_traits::value_to_yaml(msg.competition_state, out);
    out << "\n";
  }

  // member: num_kits
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "num_kits: ";
    rosidl_generator_traits::value_to_yaml(msg.num_kits, out);
    out << "\n";
  }

  // member: num_modules
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "num_modules: ";
    rosidl_generator_traits::value_to_yaml(msg.num_modules, out);
    out << "\n";
  }

  // member: num_kits_remaining
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "num_kits_remaining: ";
    rosidl_generator_traits::value_to_yaml(msg.num_kits_remaining, out);
    out << "\n";
  }

  // member: num_modules_remaining
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "num_modules_remaining: ";
    rosidl_generator_traits::value_to_yaml(msg.num_modules_remaining, out);
    out << "\n";
  }

  // member: time
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "time:\n";
    to_block_style_yaml(msg.time, out, indentation + 2);
  }

  // member: run_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "run_id: ";
    rosidl_generator_traits::value_to_yaml(msg.run_id, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const CompetitionStatus & msg, bool use_flow_style = false)
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
  const ariac_interfaces::msg::CompetitionStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::msg::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::msg::CompetitionStatus & msg)
{
  return ariac_interfaces::msg::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::msg::CompetitionStatus>()
{
  return "ariac_interfaces::msg::CompetitionStatus";
}

template<>
inline const char * name<ariac_interfaces::msg::CompetitionStatus>()
{
  return "ariac_interfaces/msg/CompetitionStatus";
}

template<>
struct has_fixed_size<ariac_interfaces::msg::CompetitionStatus>
  : std::integral_constant<bool, has_fixed_size<ariac_interfaces::msg::CompetitionTime>::value> {};

template<>
struct has_bounded_size<ariac_interfaces::msg::CompetitionStatus>
  : std::integral_constant<bool, has_bounded_size<ariac_interfaces::msg::CompetitionTime>::value> {};

template<>
struct is_message<ariac_interfaces::msg::CompetitionStatus>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATUS__TRAITS_HPP_
