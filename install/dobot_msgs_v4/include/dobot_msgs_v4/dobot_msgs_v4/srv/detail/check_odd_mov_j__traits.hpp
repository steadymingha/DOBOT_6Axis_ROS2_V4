// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from dobot_msgs_v4:srv/CheckOddMovJ.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__CHECK_ODD_MOV_J__TRAITS_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__CHECK_ODD_MOV_J__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "dobot_msgs_v4/srv/detail/check_odd_mov_j__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace dobot_msgs_v4
{

namespace srv
{

inline void to_flow_style_yaml(
  const CheckOddMovJ_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: point1_j1
  {
    out << "point1_j1: ";
    rosidl_generator_traits::value_to_yaml(msg.point1_j1, out);
    out << ", ";
  }

  // member: point1_j2
  {
    out << "point1_j2: ";
    rosidl_generator_traits::value_to_yaml(msg.point1_j2, out);
    out << ", ";
  }

  // member: point1_j3
  {
    out << "point1_j3: ";
    rosidl_generator_traits::value_to_yaml(msg.point1_j3, out);
    out << ", ";
  }

  // member: point1_j4
  {
    out << "point1_j4: ";
    rosidl_generator_traits::value_to_yaml(msg.point1_j4, out);
    out << ", ";
  }

  // member: point1_j5
  {
    out << "point1_j5: ";
    rosidl_generator_traits::value_to_yaml(msg.point1_j5, out);
    out << ", ";
  }

  // member: point1_j6
  {
    out << "point1_j6: ";
    rosidl_generator_traits::value_to_yaml(msg.point1_j6, out);
    out << ", ";
  }

  // member: point2_j1
  {
    out << "point2_j1: ";
    rosidl_generator_traits::value_to_yaml(msg.point2_j1, out);
    out << ", ";
  }

  // member: point2_j2
  {
    out << "point2_j2: ";
    rosidl_generator_traits::value_to_yaml(msg.point2_j2, out);
    out << ", ";
  }

  // member: point2_j3
  {
    out << "point2_j3: ";
    rosidl_generator_traits::value_to_yaml(msg.point2_j3, out);
    out << ", ";
  }

  // member: point2_j4
  {
    out << "point2_j4: ";
    rosidl_generator_traits::value_to_yaml(msg.point2_j4, out);
    out << ", ";
  }

  // member: point2_j5
  {
    out << "point2_j5: ";
    rosidl_generator_traits::value_to_yaml(msg.point2_j5, out);
    out << ", ";
  }

  // member: point2_j6
  {
    out << "point2_j6: ";
    rosidl_generator_traits::value_to_yaml(msg.point2_j6, out);
    out << ", ";
  }

  // member: param_value
  {
    if (msg.param_value.size() == 0) {
      out << "param_value: []";
    } else {
      out << "param_value: [";
      size_t pending_items = msg.param_value.size();
      for (auto item : msg.param_value) {
        rosidl_generator_traits::value_to_yaml(item, out);
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
  const CheckOddMovJ_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: point1_j1
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "point1_j1: ";
    rosidl_generator_traits::value_to_yaml(msg.point1_j1, out);
    out << "\n";
  }

  // member: point1_j2
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "point1_j2: ";
    rosidl_generator_traits::value_to_yaml(msg.point1_j2, out);
    out << "\n";
  }

  // member: point1_j3
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "point1_j3: ";
    rosidl_generator_traits::value_to_yaml(msg.point1_j3, out);
    out << "\n";
  }

  // member: point1_j4
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "point1_j4: ";
    rosidl_generator_traits::value_to_yaml(msg.point1_j4, out);
    out << "\n";
  }

  // member: point1_j5
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "point1_j5: ";
    rosidl_generator_traits::value_to_yaml(msg.point1_j5, out);
    out << "\n";
  }

  // member: point1_j6
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "point1_j6: ";
    rosidl_generator_traits::value_to_yaml(msg.point1_j6, out);
    out << "\n";
  }

  // member: point2_j1
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "point2_j1: ";
    rosidl_generator_traits::value_to_yaml(msg.point2_j1, out);
    out << "\n";
  }

  // member: point2_j2
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "point2_j2: ";
    rosidl_generator_traits::value_to_yaml(msg.point2_j2, out);
    out << "\n";
  }

  // member: point2_j3
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "point2_j3: ";
    rosidl_generator_traits::value_to_yaml(msg.point2_j3, out);
    out << "\n";
  }

  // member: point2_j4
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "point2_j4: ";
    rosidl_generator_traits::value_to_yaml(msg.point2_j4, out);
    out << "\n";
  }

  // member: point2_j5
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "point2_j5: ";
    rosidl_generator_traits::value_to_yaml(msg.point2_j5, out);
    out << "\n";
  }

  // member: point2_j6
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "point2_j6: ";
    rosidl_generator_traits::value_to_yaml(msg.point2_j6, out);
    out << "\n";
  }

  // member: param_value
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.param_value.size() == 0) {
      out << "param_value: []\n";
    } else {
      out << "param_value:\n";
      for (auto item : msg.param_value) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "- ";
        rosidl_generator_traits::value_to_yaml(item, out);
        out << "\n";
      }
    }
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const CheckOddMovJ_Request & msg, bool use_flow_style = false)
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
  const dobot_msgs_v4::srv::CheckOddMovJ_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  dobot_msgs_v4::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use dobot_msgs_v4::srv::to_yaml() instead")]]
inline std::string to_yaml(const dobot_msgs_v4::srv::CheckOddMovJ_Request & msg)
{
  return dobot_msgs_v4::srv::to_yaml(msg);
}

template<>
inline const char * data_type<dobot_msgs_v4::srv::CheckOddMovJ_Request>()
{
  return "dobot_msgs_v4::srv::CheckOddMovJ_Request";
}

template<>
inline const char * name<dobot_msgs_v4::srv::CheckOddMovJ_Request>()
{
  return "dobot_msgs_v4/srv/CheckOddMovJ_Request";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::CheckOddMovJ_Request>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::CheckOddMovJ_Request>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<dobot_msgs_v4::srv::CheckOddMovJ_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace dobot_msgs_v4
{

namespace srv
{

inline void to_flow_style_yaml(
  const CheckOddMovJ_Response & msg,
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
  const CheckOddMovJ_Response & msg,
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

inline std::string to_yaml(const CheckOddMovJ_Response & msg, bool use_flow_style = false)
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
  const dobot_msgs_v4::srv::CheckOddMovJ_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  dobot_msgs_v4::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use dobot_msgs_v4::srv::to_yaml() instead")]]
inline std::string to_yaml(const dobot_msgs_v4::srv::CheckOddMovJ_Response & msg)
{
  return dobot_msgs_v4::srv::to_yaml(msg);
}

template<>
inline const char * data_type<dobot_msgs_v4::srv::CheckOddMovJ_Response>()
{
  return "dobot_msgs_v4::srv::CheckOddMovJ_Response";
}

template<>
inline const char * name<dobot_msgs_v4::srv::CheckOddMovJ_Response>()
{
  return "dobot_msgs_v4/srv/CheckOddMovJ_Response";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::CheckOddMovJ_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::CheckOddMovJ_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<dobot_msgs_v4::srv::CheckOddMovJ_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<dobot_msgs_v4::srv::CheckOddMovJ>()
{
  return "dobot_msgs_v4::srv::CheckOddMovJ";
}

template<>
inline const char * name<dobot_msgs_v4::srv::CheckOddMovJ>()
{
  return "dobot_msgs_v4/srv/CheckOddMovJ";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::CheckOddMovJ>
  : std::integral_constant<
    bool,
    has_fixed_size<dobot_msgs_v4::srv::CheckOddMovJ_Request>::value &&
    has_fixed_size<dobot_msgs_v4::srv::CheckOddMovJ_Response>::value
  >
{
};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::CheckOddMovJ>
  : std::integral_constant<
    bool,
    has_bounded_size<dobot_msgs_v4::srv::CheckOddMovJ_Request>::value &&
    has_bounded_size<dobot_msgs_v4::srv::CheckOddMovJ_Response>::value
  >
{
};

template<>
struct is_service<dobot_msgs_v4::srv::CheckOddMovJ>
  : std::true_type
{
};

template<>
struct is_service_request<dobot_msgs_v4::srv::CheckOddMovJ_Request>
  : std::true_type
{
};

template<>
struct is_service_response<dobot_msgs_v4::srv::CheckOddMovJ_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__CHECK_ODD_MOV_J__TRAITS_HPP_
