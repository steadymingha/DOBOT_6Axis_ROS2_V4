// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from dobot_msgs_v4:srv/RunTo.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__RUN_TO__TRAITS_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__RUN_TO__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "dobot_msgs_v4/srv/detail/run_to__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace dobot_msgs_v4
{

namespace srv
{

inline void to_flow_style_yaml(
  const RunTo_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: a1
  {
    out << "a1: ";
    rosidl_generator_traits::value_to_yaml(msg.a1, out);
    out << ", ";
  }

  // member: b1
  {
    out << "b1: ";
    rosidl_generator_traits::value_to_yaml(msg.b1, out);
    out << ", ";
  }

  // member: c1
  {
    out << "c1: ";
    rosidl_generator_traits::value_to_yaml(msg.c1, out);
    out << ", ";
  }

  // member: d1
  {
    out << "d1: ";
    rosidl_generator_traits::value_to_yaml(msg.d1, out);
    out << ", ";
  }

  // member: e1
  {
    out << "e1: ";
    rosidl_generator_traits::value_to_yaml(msg.e1, out);
    out << ", ";
  }

  // member: f1
  {
    out << "f1: ";
    rosidl_generator_traits::value_to_yaml(msg.f1, out);
    out << ", ";
  }

  // member: move_type
  {
    out << "move_type: ";
    rosidl_generator_traits::value_to_yaml(msg.move_type, out);
    out << ", ";
  }

  // member: user
  {
    out << "user: ";
    rosidl_generator_traits::value_to_yaml(msg.user, out);
    out << ", ";
  }

  // member: tool
  {
    out << "tool: ";
    rosidl_generator_traits::value_to_yaml(msg.tool, out);
    out << ", ";
  }

  // member: a
  {
    out << "a: ";
    rosidl_generator_traits::value_to_yaml(msg.a, out);
    out << ", ";
  }

  // member: v
  {
    out << "v: ";
    rosidl_generator_traits::value_to_yaml(msg.v, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const RunTo_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: a1
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "a1: ";
    rosidl_generator_traits::value_to_yaml(msg.a1, out);
    out << "\n";
  }

  // member: b1
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "b1: ";
    rosidl_generator_traits::value_to_yaml(msg.b1, out);
    out << "\n";
  }

  // member: c1
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "c1: ";
    rosidl_generator_traits::value_to_yaml(msg.c1, out);
    out << "\n";
  }

  // member: d1
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "d1: ";
    rosidl_generator_traits::value_to_yaml(msg.d1, out);
    out << "\n";
  }

  // member: e1
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "e1: ";
    rosidl_generator_traits::value_to_yaml(msg.e1, out);
    out << "\n";
  }

  // member: f1
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "f1: ";
    rosidl_generator_traits::value_to_yaml(msg.f1, out);
    out << "\n";
  }

  // member: move_type
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "move_type: ";
    rosidl_generator_traits::value_to_yaml(msg.move_type, out);
    out << "\n";
  }

  // member: user
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "user: ";
    rosidl_generator_traits::value_to_yaml(msg.user, out);
    out << "\n";
  }

  // member: tool
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "tool: ";
    rosidl_generator_traits::value_to_yaml(msg.tool, out);
    out << "\n";
  }

  // member: a
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "a: ";
    rosidl_generator_traits::value_to_yaml(msg.a, out);
    out << "\n";
  }

  // member: v
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "v: ";
    rosidl_generator_traits::value_to_yaml(msg.v, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const RunTo_Request & msg, bool use_flow_style = false)
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
  const dobot_msgs_v4::srv::RunTo_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  dobot_msgs_v4::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use dobot_msgs_v4::srv::to_yaml() instead")]]
inline std::string to_yaml(const dobot_msgs_v4::srv::RunTo_Request & msg)
{
  return dobot_msgs_v4::srv::to_yaml(msg);
}

template<>
inline const char * data_type<dobot_msgs_v4::srv::RunTo_Request>()
{
  return "dobot_msgs_v4::srv::RunTo_Request";
}

template<>
inline const char * name<dobot_msgs_v4::srv::RunTo_Request>()
{
  return "dobot_msgs_v4/srv/RunTo_Request";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::RunTo_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::RunTo_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<dobot_msgs_v4::srv::RunTo_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace dobot_msgs_v4
{

namespace srv
{

inline void to_flow_style_yaml(
  const RunTo_Response & msg,
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
  const RunTo_Response & msg,
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

inline std::string to_yaml(const RunTo_Response & msg, bool use_flow_style = false)
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
  const dobot_msgs_v4::srv::RunTo_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  dobot_msgs_v4::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use dobot_msgs_v4::srv::to_yaml() instead")]]
inline std::string to_yaml(const dobot_msgs_v4::srv::RunTo_Response & msg)
{
  return dobot_msgs_v4::srv::to_yaml(msg);
}

template<>
inline const char * data_type<dobot_msgs_v4::srv::RunTo_Response>()
{
  return "dobot_msgs_v4::srv::RunTo_Response";
}

template<>
inline const char * name<dobot_msgs_v4::srv::RunTo_Response>()
{
  return "dobot_msgs_v4/srv/RunTo_Response";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::RunTo_Response>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::RunTo_Response>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<dobot_msgs_v4::srv::RunTo_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<dobot_msgs_v4::srv::RunTo>()
{
  return "dobot_msgs_v4::srv::RunTo";
}

template<>
inline const char * name<dobot_msgs_v4::srv::RunTo>()
{
  return "dobot_msgs_v4/srv/RunTo";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::RunTo>
  : std::integral_constant<
    bool,
    has_fixed_size<dobot_msgs_v4::srv::RunTo_Request>::value &&
    has_fixed_size<dobot_msgs_v4::srv::RunTo_Response>::value
  >
{
};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::RunTo>
  : std::integral_constant<
    bool,
    has_bounded_size<dobot_msgs_v4::srv::RunTo_Request>::value &&
    has_bounded_size<dobot_msgs_v4::srv::RunTo_Response>::value
  >
{
};

template<>
struct is_service<dobot_msgs_v4::srv::RunTo>
  : std::true_type
{
};

template<>
struct is_service_request<dobot_msgs_v4::srv::RunTo_Request>
  : std::true_type
{
};

template<>
struct is_service_response<dobot_msgs_v4::srv::RunTo_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__RUN_TO__TRAITS_HPP_
