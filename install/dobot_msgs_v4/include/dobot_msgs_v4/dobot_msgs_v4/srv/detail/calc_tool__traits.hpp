// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from dobot_msgs_v4:srv/CalcTool.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__CALC_TOOL__TRAITS_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__CALC_TOOL__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "dobot_msgs_v4/srv/detail/calc_tool__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace dobot_msgs_v4
{

namespace srv
{

inline void to_flow_style_yaml(
  const CalcTool_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: index
  {
    out << "index: ";
    rosidl_generator_traits::value_to_yaml(msg.index, out);
    out << ", ";
  }

  // member: matrix
  {
    out << "matrix: ";
    rosidl_generator_traits::value_to_yaml(msg.matrix, out);
    out << ", ";
  }

  // member: offset
  {
    out << "offset: ";
    rosidl_generator_traits::value_to_yaml(msg.offset, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const CalcTool_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: index
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "index: ";
    rosidl_generator_traits::value_to_yaml(msg.index, out);
    out << "\n";
  }

  // member: matrix
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "matrix: ";
    rosidl_generator_traits::value_to_yaml(msg.matrix, out);
    out << "\n";
  }

  // member: offset
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "offset: ";
    rosidl_generator_traits::value_to_yaml(msg.offset, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const CalcTool_Request & msg, bool use_flow_style = false)
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

}  // namespace dobot_msgs_v4

namespace rosidl_generator_traits
{

[[deprecated("use dobot_msgs_v4::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const dobot_msgs_v4::srv::CalcTool_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  dobot_msgs_v4::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use dobot_msgs_v4::srv::to_yaml() instead")]]
inline std::string to_yaml(const dobot_msgs_v4::srv::CalcTool_Request & msg)
{
  return dobot_msgs_v4::srv::to_yaml(msg);
}

template<>
inline const char * data_type<dobot_msgs_v4::srv::CalcTool_Request>()
{
  return "dobot_msgs_v4::srv::CalcTool_Request";
}

template<>
inline const char * name<dobot_msgs_v4::srv::CalcTool_Request>()
{
  return "dobot_msgs_v4/srv/CalcTool_Request";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::CalcTool_Request>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::CalcTool_Request>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<dobot_msgs_v4::srv::CalcTool_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace dobot_msgs_v4
{

namespace srv
{

inline void to_flow_style_yaml(
  const CalcTool_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: res
  {
    out << "res: ";
    rosidl_generator_traits::value_to_yaml(msg.res, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const CalcTool_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: res
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "res: ";
    rosidl_generator_traits::value_to_yaml(msg.res, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const CalcTool_Response & msg, bool use_flow_style = false)
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

}  // namespace dobot_msgs_v4

namespace rosidl_generator_traits
{

[[deprecated("use dobot_msgs_v4::srv::to_block_style_yaml() instead")]]
inline void to_yaml(
  const dobot_msgs_v4::srv::CalcTool_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  dobot_msgs_v4::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use dobot_msgs_v4::srv::to_yaml() instead")]]
inline std::string to_yaml(const dobot_msgs_v4::srv::CalcTool_Response & msg)
{
  return dobot_msgs_v4::srv::to_yaml(msg);
}

template<>
inline const char * data_type<dobot_msgs_v4::srv::CalcTool_Response>()
{
  return "dobot_msgs_v4::srv::CalcTool_Response";
}

template<>
inline const char * name<dobot_msgs_v4::srv::CalcTool_Response>()
{
  return "dobot_msgs_v4/srv/CalcTool_Response";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::CalcTool_Response>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::CalcTool_Response>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<dobot_msgs_v4::srv::CalcTool_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<dobot_msgs_v4::srv::CalcTool>()
{
  return "dobot_msgs_v4::srv::CalcTool";
}

template<>
inline const char * name<dobot_msgs_v4::srv::CalcTool>()
{
  return "dobot_msgs_v4/srv/CalcTool";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::CalcTool>
  : std::integral_constant<
    bool,
    has_fixed_size<dobot_msgs_v4::srv::CalcTool_Request>::value &&
    has_fixed_size<dobot_msgs_v4::srv::CalcTool_Response>::value
  >
{
};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::CalcTool>
  : std::integral_constant<
    bool,
    has_bounded_size<dobot_msgs_v4::srv::CalcTool_Request>::value &&
    has_bounded_size<dobot_msgs_v4::srv::CalcTool_Response>::value
  >
{
};

template<>
struct is_service<dobot_msgs_v4::srv::CalcTool>
  : std::true_type
{
};

template<>
struct is_service_request<dobot_msgs_v4::srv::CalcTool_Request>
  : std::true_type
{
};

template<>
struct is_service_response<dobot_msgs_v4::srv::CalcTool_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__CALC_TOOL__TRAITS_HPP_
