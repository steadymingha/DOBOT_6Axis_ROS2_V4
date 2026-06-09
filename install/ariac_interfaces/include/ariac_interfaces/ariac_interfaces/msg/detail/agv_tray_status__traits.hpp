// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ariac_interfaces:msg/AgvTrayStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__AGV_TRAY_STATUS__TRAITS_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__AGV_TRAY_STATUS__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ariac_interfaces/msg/detail/agv_tray_status__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace ariac_interfaces
{

namespace msg
{

inline void to_flow_style_yaml(
  const AgvTrayStatus & msg,
  std::ostream & out)
{
  out << "{";
  // member: slot_1_occupied
  {
    out << "slot_1_occupied: ";
    rosidl_generator_traits::value_to_yaml(msg.slot_1_occupied, out);
    out << ", ";
  }

  // member: slot_2_occupied
  {
    out << "slot_2_occupied: ";
    rosidl_generator_traits::value_to_yaml(msg.slot_2_occupied, out);
    out << ", ";
  }

  // member: slot_3_occupied
  {
    out << "slot_3_occupied: ";
    rosidl_generator_traits::value_to_yaml(msg.slot_3_occupied, out);
    out << ", ";
  }

  // member: slot_4_occupied
  {
    out << "slot_4_occupied: ";
    rosidl_generator_traits::value_to_yaml(msg.slot_4_occupied, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const AgvTrayStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: slot_1_occupied
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "slot_1_occupied: ";
    rosidl_generator_traits::value_to_yaml(msg.slot_1_occupied, out);
    out << "\n";
  }

  // member: slot_2_occupied
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "slot_2_occupied: ";
    rosidl_generator_traits::value_to_yaml(msg.slot_2_occupied, out);
    out << "\n";
  }

  // member: slot_3_occupied
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "slot_3_occupied: ";
    rosidl_generator_traits::value_to_yaml(msg.slot_3_occupied, out);
    out << "\n";
  }

  // member: slot_4_occupied
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "slot_4_occupied: ";
    rosidl_generator_traits::value_to_yaml(msg.slot_4_occupied, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const AgvTrayStatus & msg, bool use_flow_style = false)
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
  const ariac_interfaces::msg::AgvTrayStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::msg::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::msg::AgvTrayStatus & msg)
{
  return ariac_interfaces::msg::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::msg::AgvTrayStatus>()
{
  return "ariac_interfaces::msg::AgvTrayStatus";
}

template<>
inline const char * name<ariac_interfaces::msg::AgvTrayStatus>()
{
  return "ariac_interfaces/msg/AgvTrayStatus";
}

template<>
struct has_fixed_size<ariac_interfaces::msg::AgvTrayStatus>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<ariac_interfaces::msg::AgvTrayStatus>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<ariac_interfaces::msg::AgvTrayStatus>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // ARIAC_INTERFACES__MSG__DETAIL__AGV_TRAY_STATUS__TRAITS_HPP_
