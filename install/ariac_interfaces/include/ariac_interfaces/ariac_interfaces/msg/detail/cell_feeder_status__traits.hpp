// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ariac_interfaces:msg/CellFeederStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__CELL_FEEDER_STATUS__TRAITS_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__CELL_FEEDER_STATUS__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ariac_interfaces/msg/detail/cell_feeder_status__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace ariac_interfaces
{

namespace msg
{

inline void to_flow_style_yaml(
  const CellFeederStatus & msg,
  std::ostream & out)
{
  out << "{";
  // member: cell_type
  {
    out << "cell_type: ";
    rosidl_generator_traits::value_to_yaml(msg.cell_type, out);
    out << ", ";
  }

  // member: feed_rate
  {
    out << "feed_rate: ";
    rosidl_generator_traits::value_to_yaml(msg.feed_rate, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const CellFeederStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: cell_type
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "cell_type: ";
    rosidl_generator_traits::value_to_yaml(msg.cell_type, out);
    out << "\n";
  }

  // member: feed_rate
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "feed_rate: ";
    rosidl_generator_traits::value_to_yaml(msg.feed_rate, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const CellFeederStatus & msg, bool use_flow_style = false)
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
  const ariac_interfaces::msg::CellFeederStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::msg::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::msg::CellFeederStatus & msg)
{
  return ariac_interfaces::msg::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::msg::CellFeederStatus>()
{
  return "ariac_interfaces::msg::CellFeederStatus";
}

template<>
inline const char * name<ariac_interfaces::msg::CellFeederStatus>()
{
  return "ariac_interfaces/msg/CellFeederStatus";
}

template<>
struct has_fixed_size<ariac_interfaces::msg::CellFeederStatus>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<ariac_interfaces::msg::CellFeederStatus>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<ariac_interfaces::msg::CellFeederStatus>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // ARIAC_INTERFACES__MSG__DETAIL__CELL_FEEDER_STATUS__TRAITS_HPP_
