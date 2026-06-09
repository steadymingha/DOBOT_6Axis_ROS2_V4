// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ariac_interfaces:msg/InspectionReport.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__INSPECTION_REPORT__TRAITS_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__INSPECTION_REPORT__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ariac_interfaces/msg/detail/inspection_report__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'defects'
#include "ariac_interfaces/msg/detail/cell_defect__traits.hpp"

namespace ariac_interfaces
{

namespace msg
{

inline void to_flow_style_yaml(
  const InspectionReport & msg,
  std::ostream & out)
{
  out << "{";
  // member: passed
  {
    out << "passed: ";
    rosidl_generator_traits::value_to_yaml(msg.passed, out);
    out << ", ";
  }

  // member: defects
  {
    if (msg.defects.size() == 0) {
      out << "defects: []";
    } else {
      out << "defects: [";
      size_t pending_items = msg.defects.size();
      for (auto item : msg.defects) {
        to_flow_style_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const InspectionReport & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: passed
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "passed: ";
    rosidl_generator_traits::value_to_yaml(msg.passed, out);
    out << "\n";
  }

  // member: defects
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.defects.size() == 0) {
      out << "defects: []\n";
    } else {
      out << "defects:\n";
      for (auto item : msg.defects) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "-\n";
        to_block_style_yaml(item, out, indentation + 2);
      }
    }
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const InspectionReport & msg, bool use_flow_style = false)
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
  const ariac_interfaces::msg::InspectionReport & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::msg::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::msg::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::msg::InspectionReport & msg)
{
  return ariac_interfaces::msg::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::msg::InspectionReport>()
{
  return "ariac_interfaces::msg::InspectionReport";
}

template<>
inline const char * name<ariac_interfaces::msg::InspectionReport>()
{
  return "ariac_interfaces/msg/InspectionReport";
}

template<>
struct has_fixed_size<ariac_interfaces::msg::InspectionReport>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<ariac_interfaces::msg::InspectionReport>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<ariac_interfaces::msg::InspectionReport>
  : std::true_type {};

}  // namespace rosidl_generator_traits

#endif  // ARIAC_INTERFACES__MSG__DETAIL__INSPECTION_REPORT__TRAITS_HPP_
