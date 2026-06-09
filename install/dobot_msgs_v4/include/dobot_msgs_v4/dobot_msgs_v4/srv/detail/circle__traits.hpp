// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from dobot_msgs_v4:srv/Circle.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__CIRCLE__TRAITS_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__CIRCLE__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "dobot_msgs_v4/srv/detail/circle__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace dobot_msgs_v4
{

namespace srv
{

inline void to_flow_style_yaml(
  const Circle_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: mode
  {
    out << "mode: ";
    rosidl_generator_traits::value_to_yaml(msg.mode, out);
    out << ", ";
  }

  // member: a
  {
    out << "a: ";
    rosidl_generator_traits::value_to_yaml(msg.a, out);
    out << ", ";
  }

  // member: b
  {
    out << "b: ";
    rosidl_generator_traits::value_to_yaml(msg.b, out);
    out << ", ";
  }

  // member: c
  {
    out << "c: ";
    rosidl_generator_traits::value_to_yaml(msg.c, out);
    out << ", ";
  }

  // member: d
  {
    out << "d: ";
    rosidl_generator_traits::value_to_yaml(msg.d, out);
    out << ", ";
  }

  // member: e
  {
    out << "e: ";
    rosidl_generator_traits::value_to_yaml(msg.e, out);
    out << ", ";
  }

  // member: f
  {
    out << "f: ";
    rosidl_generator_traits::value_to_yaml(msg.f, out);
    out << ", ";
  }

  // member: a2
  {
    out << "a2: ";
    rosidl_generator_traits::value_to_yaml(msg.a2, out);
    out << ", ";
  }

  // member: b2
  {
    out << "b2: ";
    rosidl_generator_traits::value_to_yaml(msg.b2, out);
    out << ", ";
  }

  // member: c2
  {
    out << "c2: ";
    rosidl_generator_traits::value_to_yaml(msg.c2, out);
    out << ", ";
  }

  // member: d2
  {
    out << "d2: ";
    rosidl_generator_traits::value_to_yaml(msg.d2, out);
    out << ", ";
  }

  // member: e2
  {
    out << "e2: ";
    rosidl_generator_traits::value_to_yaml(msg.e2, out);
    out << ", ";
  }

  // member: f2
  {
    out << "f2: ";
    rosidl_generator_traits::value_to_yaml(msg.f2, out);
    out << ", ";
  }

  // member: count
  {
    out << "count: ";
    rosidl_generator_traits::value_to_yaml(msg.count, out);
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
  const Circle_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: mode
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "mode: ";
    rosidl_generator_traits::value_to_yaml(msg.mode, out);
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

  // member: b
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "b: ";
    rosidl_generator_traits::value_to_yaml(msg.b, out);
    out << "\n";
  }

  // member: c
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "c: ";
    rosidl_generator_traits::value_to_yaml(msg.c, out);
    out << "\n";
  }

  // member: d
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "d: ";
    rosidl_generator_traits::value_to_yaml(msg.d, out);
    out << "\n";
  }

  // member: e
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "e: ";
    rosidl_generator_traits::value_to_yaml(msg.e, out);
    out << "\n";
  }

  // member: f
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "f: ";
    rosidl_generator_traits::value_to_yaml(msg.f, out);
    out << "\n";
  }

  // member: a2
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "a2: ";
    rosidl_generator_traits::value_to_yaml(msg.a2, out);
    out << "\n";
  }

  // member: b2
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "b2: ";
    rosidl_generator_traits::value_to_yaml(msg.b2, out);
    out << "\n";
  }

  // member: c2
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "c2: ";
    rosidl_generator_traits::value_to_yaml(msg.c2, out);
    out << "\n";
  }

  // member: d2
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "d2: ";
    rosidl_generator_traits::value_to_yaml(msg.d2, out);
    out << "\n";
  }

  // member: e2
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "e2: ";
    rosidl_generator_traits::value_to_yaml(msg.e2, out);
    out << "\n";
  }

  // member: f2
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "f2: ";
    rosidl_generator_traits::value_to_yaml(msg.f2, out);
    out << "\n";
  }

  // member: count
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "count: ";
    rosidl_generator_traits::value_to_yaml(msg.count, out);
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

inline std::string to_yaml(const Circle_Request & msg, bool use_flow_style = false)
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
  const dobot_msgs_v4::srv::Circle_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  dobot_msgs_v4::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use dobot_msgs_v4::srv::to_yaml() instead")]]
inline std::string to_yaml(const dobot_msgs_v4::srv::Circle_Request & msg)
{
  return dobot_msgs_v4::srv::to_yaml(msg);
}

template<>
inline const char * data_type<dobot_msgs_v4::srv::Circle_Request>()
{
  return "dobot_msgs_v4::srv::Circle_Request";
}

template<>
inline const char * name<dobot_msgs_v4::srv::Circle_Request>()
{
  return "dobot_msgs_v4/srv/Circle_Request";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::Circle_Request>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::Circle_Request>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<dobot_msgs_v4::srv::Circle_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace dobot_msgs_v4
{

namespace srv
{

inline void to_flow_style_yaml(
  const Circle_Response & msg,
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
  const Circle_Response & msg,
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

inline std::string to_yaml(const Circle_Response & msg, bool use_flow_style = false)
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
  const dobot_msgs_v4::srv::Circle_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  dobot_msgs_v4::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use dobot_msgs_v4::srv::to_yaml() instead")]]
inline std::string to_yaml(const dobot_msgs_v4::srv::Circle_Response & msg)
{
  return dobot_msgs_v4::srv::to_yaml(msg);
}

template<>
inline const char * data_type<dobot_msgs_v4::srv::Circle_Response>()
{
  return "dobot_msgs_v4::srv::Circle_Response";
}

template<>
inline const char * name<dobot_msgs_v4::srv::Circle_Response>()
{
  return "dobot_msgs_v4/srv/Circle_Response";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::Circle_Response>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::Circle_Response>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<dobot_msgs_v4::srv::Circle_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<dobot_msgs_v4::srv::Circle>()
{
  return "dobot_msgs_v4::srv::Circle";
}

template<>
inline const char * name<dobot_msgs_v4::srv::Circle>()
{
  return "dobot_msgs_v4/srv/Circle";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::Circle>
  : std::integral_constant<
    bool,
    has_fixed_size<dobot_msgs_v4::srv::Circle_Request>::value &&
    has_fixed_size<dobot_msgs_v4::srv::Circle_Response>::value
  >
{
};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::Circle>
  : std::integral_constant<
    bool,
    has_bounded_size<dobot_msgs_v4::srv::Circle_Request>::value &&
    has_bounded_size<dobot_msgs_v4::srv::Circle_Response>::value
  >
{
};

template<>
struct is_service<dobot_msgs_v4::srv::Circle>
  : std::true_type
{
};

template<>
struct is_service_request<dobot_msgs_v4::srv::Circle_Request>
  : std::true_type
{
};

template<>
struct is_service_response<dobot_msgs_v4::srv::Circle_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__CIRCLE__TRAITS_HPP_
