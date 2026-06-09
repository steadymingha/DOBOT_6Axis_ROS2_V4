// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ariac_interfaces:srv/SubmitInspectionReport.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_INSPECTION_REPORT__TRAITS_HPP_
#define ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_INSPECTION_REPORT__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ariac_interfaces/srv/detail/submit_inspection_report__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'timestamp'
#include "builtin_interfaces/msg/detail/time__traits.hpp"
// Member 'report'
#include "ariac_interfaces/msg/detail/inspection_report__traits.hpp"

namespace ariac_interfaces
{

namespace srv
{

inline void to_flow_style_yaml(
  const SubmitInspectionReport_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: timestamp
  {
    out << "timestamp: ";
    to_flow_style_yaml(msg.timestamp, out);
    out << ", ";
  }

  // member: report
  {
    out << "report: ";
    to_flow_style_yaml(msg.report, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const SubmitInspectionReport_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: timestamp
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "timestamp:\n";
    to_block_style_yaml(msg.timestamp, out, indentation + 2);
  }

  // member: report
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "report:\n";
    to_block_style_yaml(msg.report, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const SubmitInspectionReport_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace ariac_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use ariac_interfaces::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ariac_interfaces::srv::SubmitInspectionReport_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::srv::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::srv::SubmitInspectionReport_Request & msg)
{
  return ariac_interfaces::srv::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::srv::SubmitInspectionReport_Request>()
{
  return "ariac_interfaces::srv::SubmitInspectionReport_Request";
}

template<>
inline const char * name<ariac_interfaces::srv::SubmitInspectionReport_Request>()
{
  return "ariac_interfaces/srv/SubmitInspectionReport_Request";
}

template<>
struct has_fixed_size<ariac_interfaces::srv::SubmitInspectionReport_Request>
  : std::integral_constant<bool, has_fixed_size<ariac_interfaces::msg::InspectionReport>::value && has_fixed_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct has_bounded_size<ariac_interfaces::srv::SubmitInspectionReport_Request>
  : std::integral_constant<bool, has_bounded_size<ariac_interfaces::msg::InspectionReport>::value && has_bounded_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct is_message<ariac_interfaces::srv::SubmitInspectionReport_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace ariac_interfaces
{

namespace srv
{

inline void to_flow_style_yaml(
  const SubmitInspectionReport_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: success
  {
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
    out << ", ";
  }

  // member: message
  {
    out << "message: ";
    rosidl_generator_traits::value_to_yaml(msg.message, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const SubmitInspectionReport_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: success
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
    out << "\n";
  }

  // member: message
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "message: ";
    rosidl_generator_traits::value_to_yaml(msg.message, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const SubmitInspectionReport_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace srv

}  // namespace ariac_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use ariac_interfaces::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ariac_interfaces::srv::SubmitInspectionReport_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::srv::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::srv::SubmitInspectionReport_Response & msg)
{
  return ariac_interfaces::srv::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::srv::SubmitInspectionReport_Response>()
{
  return "ariac_interfaces::srv::SubmitInspectionReport_Response";
}

template<>
inline const char * name<ariac_interfaces::srv::SubmitInspectionReport_Response>()
{
  return "ariac_interfaces/srv/SubmitInspectionReport_Response";
}

template<>
struct has_fixed_size<ariac_interfaces::srv::SubmitInspectionReport_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<ariac_interfaces::srv::SubmitInspectionReport_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<ariac_interfaces::srv::SubmitInspectionReport_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<ariac_interfaces::srv::SubmitInspectionReport>()
{
  return "ariac_interfaces::srv::SubmitInspectionReport";
}

template<>
inline const char * name<ariac_interfaces::srv::SubmitInspectionReport>()
{
  return "ariac_interfaces/srv/SubmitInspectionReport";
}

template<>
struct has_fixed_size<ariac_interfaces::srv::SubmitInspectionReport>
  : std::integral_constant<
    bool,
    has_fixed_size<ariac_interfaces::srv::SubmitInspectionReport_Request>::value &&
    has_fixed_size<ariac_interfaces::srv::SubmitInspectionReport_Response>::value
  >
{
};

template<>
struct has_bounded_size<ariac_interfaces::srv::SubmitInspectionReport>
  : std::integral_constant<
    bool,
    has_bounded_size<ariac_interfaces::srv::SubmitInspectionReport_Request>::value &&
    has_bounded_size<ariac_interfaces::srv::SubmitInspectionReport_Response>::value
  >
{
};

template<>
struct is_service<ariac_interfaces::srv::SubmitInspectionReport>
  : std::true_type
{
};

template<>
struct is_service_request<ariac_interfaces::srv::SubmitInspectionReport_Request>
  : std::true_type
{
};

template<>
struct is_service_response<ariac_interfaces::srv::SubmitInspectionReport_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_INSPECTION_REPORT__TRAITS_HPP_
