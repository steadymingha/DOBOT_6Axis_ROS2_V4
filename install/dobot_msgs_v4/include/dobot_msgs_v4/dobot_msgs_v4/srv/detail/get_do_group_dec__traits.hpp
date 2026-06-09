// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from dobot_msgs_v4:srv/GetDOGroupDEC.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__GET_DO_GROUP_DEC__TRAITS_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__GET_DO_GROUP_DEC__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "dobot_msgs_v4/srv/detail/get_do_group_dec__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace dobot_msgs_v4
{

namespace srv
{

inline void to_flow_style_yaml(
  const GetDOGroupDEC_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: group
  {
    out << "group: ";
    rosidl_generator_traits::value_to_yaml(msg.group, out);
    out << ", ";
  }

  // member: value
  {
    out << "value: ";
    rosidl_generator_traits::value_to_yaml(msg.value, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const GetDOGroupDEC_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: group
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "group: ";
    rosidl_generator_traits::value_to_yaml(msg.group, out);
    out << "\n";
  }

  // member: value
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "value: ";
    rosidl_generator_traits::value_to_yaml(msg.value, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const GetDOGroupDEC_Request & msg, bool use_flow_style = false)
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
  const dobot_msgs_v4::srv::GetDOGroupDEC_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  dobot_msgs_v4::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use dobot_msgs_v4::srv::to_yaml() instead")]]
inline std::string to_yaml(const dobot_msgs_v4::srv::GetDOGroupDEC_Request & msg)
{
  return dobot_msgs_v4::srv::to_yaml(msg);
}

template<>
inline const char * data_type<dobot_msgs_v4::srv::GetDOGroupDEC_Request>()
{
  return "dobot_msgs_v4::srv::GetDOGroupDEC_Request";
}

template<>
inline const char * name<dobot_msgs_v4::srv::GetDOGroupDEC_Request>()
{
  return "dobot_msgs_v4/srv/GetDOGroupDEC_Request";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::GetDOGroupDEC_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::GetDOGroupDEC_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<dobot_msgs_v4::srv::GetDOGroupDEC_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace dobot_msgs_v4
{

namespace srv
{

inline void to_flow_style_yaml(
  const GetDOGroupDEC_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: robot_return
  {
    out << "robot_return: ";
    rosidl_generator_traits::value_to_yaml(msg.robot_return, out);
    out << ", ";
  }

  // member: res
  {
    out << "res: ";
    rosidl_generator_traits::value_to_yaml(msg.res, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const GetDOGroupDEC_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: robot_return
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "robot_return: ";
    rosidl_generator_traits::value_to_yaml(msg.robot_return, out);
    out << "\n";
  }

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

inline std::string to_yaml(const GetDOGroupDEC_Response & msg, bool use_flow_style = false)
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
  const dobot_msgs_v4::srv::GetDOGroupDEC_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  dobot_msgs_v4::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use dobot_msgs_v4::srv::to_yaml() instead")]]
inline std::string to_yaml(const dobot_msgs_v4::srv::GetDOGroupDEC_Response & msg)
{
  return dobot_msgs_v4::srv::to_yaml(msg);
}

template<>
inline const char * data_type<dobot_msgs_v4::srv::GetDOGroupDEC_Response>()
{
  return "dobot_msgs_v4::srv::GetDOGroupDEC_Response";
}

template<>
inline const char * name<dobot_msgs_v4::srv::GetDOGroupDEC_Response>()
{
  return "dobot_msgs_v4/srv/GetDOGroupDEC_Response";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::GetDOGroupDEC_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::GetDOGroupDEC_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<dobot_msgs_v4::srv::GetDOGroupDEC_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<dobot_msgs_v4::srv::GetDOGroupDEC>()
{
  return "dobot_msgs_v4::srv::GetDOGroupDEC";
}

template<>
inline const char * name<dobot_msgs_v4::srv::GetDOGroupDEC>()
{
  return "dobot_msgs_v4/srv/GetDOGroupDEC";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::GetDOGroupDEC>
  : std::integral_constant<
    bool,
    has_fixed_size<dobot_msgs_v4::srv::GetDOGroupDEC_Request>::value &&
    has_fixed_size<dobot_msgs_v4::srv::GetDOGroupDEC_Response>::value
  >
{
};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::GetDOGroupDEC>
  : std::integral_constant<
    bool,
    has_bounded_size<dobot_msgs_v4::srv::GetDOGroupDEC_Request>::value &&
    has_bounded_size<dobot_msgs_v4::srv::GetDOGroupDEC_Response>::value
  >
{
};

template<>
struct is_service<dobot_msgs_v4::srv::GetDOGroupDEC>
  : std::true_type
{
};

template<>
struct is_service_request<dobot_msgs_v4::srv::GetDOGroupDEC_Request>
  : std::true_type
{
};

template<>
struct is_service_response<dobot_msgs_v4::srv::GetDOGroupDEC_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__GET_DO_GROUP_DEC__TRAITS_HPP_
