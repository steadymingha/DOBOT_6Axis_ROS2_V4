// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from dobot_msgs_v4:srv/ModbusRTUCreate.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__MODBUS_RTU_CREATE__TRAITS_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__MODBUS_RTU_CREATE__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "dobot_msgs_v4/srv/detail/modbus_rtu_create__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace dobot_msgs_v4
{

namespace srv
{

inline void to_flow_style_yaml(
  const ModbusRTUCreate_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: slave_id
  {
    out << "slave_id: ";
    rosidl_generator_traits::value_to_yaml(msg.slave_id, out);
    out << ", ";
  }

  // member: baud
  {
    out << "baud: ";
    rosidl_generator_traits::value_to_yaml(msg.baud, out);
    out << ", ";
  }

  // member: parity
  {
    out << "parity: ";
    rosidl_generator_traits::value_to_yaml(msg.parity, out);
    out << ", ";
  }

  // member: data_bit
  {
    out << "data_bit: ";
    rosidl_generator_traits::value_to_yaml(msg.data_bit, out);
    out << ", ";
  }

  // member: stop_bit
  {
    out << "stop_bit: ";
    rosidl_generator_traits::value_to_yaml(msg.stop_bit, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ModbusRTUCreate_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: slave_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "slave_id: ";
    rosidl_generator_traits::value_to_yaml(msg.slave_id, out);
    out << "\n";
  }

  // member: baud
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "baud: ";
    rosidl_generator_traits::value_to_yaml(msg.baud, out);
    out << "\n";
  }

  // member: parity
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "parity: ";
    rosidl_generator_traits::value_to_yaml(msg.parity, out);
    out << "\n";
  }

  // member: data_bit
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "data_bit: ";
    rosidl_generator_traits::value_to_yaml(msg.data_bit, out);
    out << "\n";
  }

  // member: stop_bit
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "stop_bit: ";
    rosidl_generator_traits::value_to_yaml(msg.stop_bit, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ModbusRTUCreate_Request & msg, bool use_flow_style = false)
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
  const dobot_msgs_v4::srv::ModbusRTUCreate_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  dobot_msgs_v4::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use dobot_msgs_v4::srv::to_yaml() instead")]]
inline std::string to_yaml(const dobot_msgs_v4::srv::ModbusRTUCreate_Request & msg)
{
  return dobot_msgs_v4::srv::to_yaml(msg);
}

template<>
inline const char * data_type<dobot_msgs_v4::srv::ModbusRTUCreate_Request>()
{
  return "dobot_msgs_v4::srv::ModbusRTUCreate_Request";
}

template<>
inline const char * name<dobot_msgs_v4::srv::ModbusRTUCreate_Request>()
{
  return "dobot_msgs_v4/srv/ModbusRTUCreate_Request";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::ModbusRTUCreate_Request>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::ModbusRTUCreate_Request>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<dobot_msgs_v4::srv::ModbusRTUCreate_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace dobot_msgs_v4
{

namespace srv
{

inline void to_flow_style_yaml(
  const ModbusRTUCreate_Response & msg,
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
  const ModbusRTUCreate_Response & msg,
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

inline std::string to_yaml(const ModbusRTUCreate_Response & msg, bool use_flow_style = false)
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
  const dobot_msgs_v4::srv::ModbusRTUCreate_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  dobot_msgs_v4::srv::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use dobot_msgs_v4::srv::to_yaml() instead")]]
inline std::string to_yaml(const dobot_msgs_v4::srv::ModbusRTUCreate_Response & msg)
{
  return dobot_msgs_v4::srv::to_yaml(msg);
}

template<>
inline const char * data_type<dobot_msgs_v4::srv::ModbusRTUCreate_Response>()
{
  return "dobot_msgs_v4::srv::ModbusRTUCreate_Response";
}

template<>
inline const char * name<dobot_msgs_v4::srv::ModbusRTUCreate_Response>()
{
  return "dobot_msgs_v4/srv/ModbusRTUCreate_Response";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::ModbusRTUCreate_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::ModbusRTUCreate_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<dobot_msgs_v4::srv::ModbusRTUCreate_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<dobot_msgs_v4::srv::ModbusRTUCreate>()
{
  return "dobot_msgs_v4::srv::ModbusRTUCreate";
}

template<>
inline const char * name<dobot_msgs_v4::srv::ModbusRTUCreate>()
{
  return "dobot_msgs_v4/srv/ModbusRTUCreate";
}

template<>
struct has_fixed_size<dobot_msgs_v4::srv::ModbusRTUCreate>
  : std::integral_constant<
    bool,
    has_fixed_size<dobot_msgs_v4::srv::ModbusRTUCreate_Request>::value &&
    has_fixed_size<dobot_msgs_v4::srv::ModbusRTUCreate_Response>::value
  >
{
};

template<>
struct has_bounded_size<dobot_msgs_v4::srv::ModbusRTUCreate>
  : std::integral_constant<
    bool,
    has_bounded_size<dobot_msgs_v4::srv::ModbusRTUCreate_Request>::value &&
    has_bounded_size<dobot_msgs_v4::srv::ModbusRTUCreate_Response>::value
  >
{
};

template<>
struct is_service<dobot_msgs_v4::srv::ModbusRTUCreate>
  : std::true_type
{
};

template<>
struct is_service_request<dobot_msgs_v4::srv::ModbusRTUCreate_Request>
  : std::true_type
{
};

template<>
struct is_service_response<dobot_msgs_v4::srv::ModbusRTUCreate_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__MODBUS_RTU_CREATE__TRAITS_HPP_
