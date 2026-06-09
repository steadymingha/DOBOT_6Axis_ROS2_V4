// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ariac_interfaces:msg/AgvStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__AGV_STATUS__TRAITS_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__AGV_STATUS__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ariac_interfaces/msg/detail/agv_status__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'pose'
#include "geometry_msgs/msg/detail/pose__traits.hpp"

namespace ariac_interfaces
{

namespace msg
{

inline void to_flow_style_yaml(
  const AgvStatus & msg,
  std::ostream & out)
{
  out << "{";
  // member: station_id
  {
    out << "station_id: ";
    rosidl_generator_traits::value_to_yaml(msg.station_id, out);
    out << ", ";
  }

  // member: pose
  {
    out << "pose: ";
    to_flow_style_yaml(msg.pose, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const AgvStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: station_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "station_id: ";
    rosidl_generator_traits::value_to_yaml(msg.station_id, out);
    out << "\n";
  }

  // member: pose
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pose:\n";
    to_block_style_yaml(msg.pose, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const AgvStatus & msg, bool use_flow_style = false)
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
  const ariac_interfaces::msg::AgvStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::msg::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::msg::AgvStatus & msg)
{
  return ariac_interfaces::msg::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::msg::AgvStatus>()
{
  return "ariac_interfaces::msg::AgvStatus";
}

template<>
inline const char * name<ariac_interfaces::msg::AgvStatus>()
{
  return "ariac_interfaces/msg/AgvStatus";
}

template<>
struct has_fixed_size<ariac_interfaces::msg::AgvStatus>
  : std::integral_constant<bool, has_fixed_size<geometry_msgs::msg::Pose>::value> {};

template<>
struct has_bounded_size<ariac_interfaces::msg::AgvStatus>
  : std::integral_constant<bool, has_bounded_size<geometry_msgs::msg::Pose>::value> {};

template<>
struct is_message<ariac_interfaces::msg::AgvStatus>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // ARIAC_INTERFACES__MSG__DETAIL__AGV_STATUS__TRAITS_HPP_
