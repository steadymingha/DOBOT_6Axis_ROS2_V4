// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ariac_interfaces:srv/SubmitHighPriorityOrder.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_HIGH_PRIORITY_ORDER__TRAITS_HPP_
#define ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_HIGH_PRIORITY_ORDER__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ariac_interfaces/srv/detail/submit_high_priority_order__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace ariac_interfaces
{

namespace srv
{

inline void to_flow_style_yaml(
  const SubmitHighPriorityOrder_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: id
  {
    out << "id: ";
    rosidl_generator_traits::value_to_yaml(msg.id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const SubmitHighPriorityOrder_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "id: ";
    rosidl_generator_traits::value_to_yaml(msg.id, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const SubmitHighPriorityOrder_Request & msg, bool use_flow_style = false)
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
  const ariac_interfaces::srv::SubmitHighPriorityOrder_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::srv::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::srv::SubmitHighPriorityOrder_Request & msg)
{
  return ariac_interfaces::srv::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::srv::SubmitHighPriorityOrder_Request>()
{
  return "ariac_interfaces::srv::SubmitHighPriorityOrder_Request";
}

template<>
inline const char * name<ariac_interfaces::srv::SubmitHighPriorityOrder_Request>()
{
  return "ariac_interfaces/srv/SubmitHighPriorityOrder_Request";
}

template<>
struct has_fixed_size<ariac_interfaces::srv::SubmitHighPriorityOrder_Request>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<ariac_interfaces::srv::SubmitHighPriorityOrder_Request>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<ariac_interfaces::srv::SubmitHighPriorityOrder_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace ariac_interfaces
{

namespace srv
{

inline void to_flow_style_yaml(
  const SubmitHighPriorityOrder_Response & msg,
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
  const SubmitHighPriorityOrder_Response & msg,
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

inline std::string to_yaml(const SubmitHighPriorityOrder_Response & msg, bool use_flow_style = false)
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
  const ariac_interfaces::srv::SubmitHighPriorityOrder_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::srv::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::srv::SubmitHighPriorityOrder_Response & msg)
{
  return ariac_interfaces::srv::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::srv::SubmitHighPriorityOrder_Response>()
{
  return "ariac_interfaces::srv::SubmitHighPriorityOrder_Response";
}

template<>
inline const char * name<ariac_interfaces::srv::SubmitHighPriorityOrder_Response>()
{
  return "ariac_interfaces/srv/SubmitHighPriorityOrder_Response";
}

template<>
struct has_fixed_size<ariac_interfaces::srv::SubmitHighPriorityOrder_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<ariac_interfaces::srv::SubmitHighPriorityOrder_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<ariac_interfaces::srv::SubmitHighPriorityOrder_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<ariac_interfaces::srv::SubmitHighPriorityOrder>()
{
  return "ariac_interfaces::srv::SubmitHighPriorityOrder";
}

template<>
inline const char * name<ariac_interfaces::srv::SubmitHighPriorityOrder>()
{
  return "ariac_interfaces/srv/SubmitHighPriorityOrder";
}

template<>
struct has_fixed_size<ariac_interfaces::srv::SubmitHighPriorityOrder>
  : std::integral_constant<
    bool,
    has_fixed_size<ariac_interfaces::srv::SubmitHighPriorityOrder_Request>::value &&
    has_fixed_size<ariac_interfaces::srv::SubmitHighPriorityOrder_Response>::value
  >
{
};

template<>
struct has_bounded_size<ariac_interfaces::srv::SubmitHighPriorityOrder>
  : std::integral_constant<
    bool,
    has_bounded_size<ariac_interfaces::srv::SubmitHighPriorityOrder_Request>::value &&
    has_bounded_size<ariac_interfaces::srv::SubmitHighPriorityOrder_Response>::value
  >
{
};

template<>
struct is_service<ariac_interfaces::srv::SubmitHighPriorityOrder>
  : std::true_type
{
};

template<>
struct is_service_request<ariac_interfaces::srv::SubmitHighPriorityOrder_Request>
  : std::true_type
{
};

template<>
struct is_service_response<ariac_interfaces::srv::SubmitHighPriorityOrder_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_HIGH_PRIORITY_ORDER__TRAITS_HPP_
