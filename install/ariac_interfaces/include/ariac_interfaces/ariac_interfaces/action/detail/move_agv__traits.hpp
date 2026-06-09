// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from ariac_interfaces:action/MoveAgv.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__ACTION__DETAIL__MOVE_AGV__TRAITS_HPP_
#define ARIAC_INTERFACES__ACTION__DETAIL__MOVE_AGV__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "ariac_interfaces/action/detail/move_agv__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace ariac_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const MoveAgv_Goal & msg,
  std::ostream & out)
{
  out << "{";
  // member: station_id
  {
    out << "station_id: ";
    rosidl_generator_traits::value_to_yaml(msg.station_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MoveAgv_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: station_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "station_id: ";
    rosidl_generator_traits::value_to_yaml(msg.station_id, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MoveAgv_Goal & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ariac_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use ariac_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ariac_interfaces::action::MoveAgv_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::action::MoveAgv_Goal & msg)
{
  return ariac_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::action::MoveAgv_Goal>()
{
  return "ariac_interfaces::action::MoveAgv_Goal";
}

template<>
inline const char * name<ariac_interfaces::action::MoveAgv_Goal>()
{
  return "ariac_interfaces/action/MoveAgv_Goal";
}

template<>
struct has_fixed_size<ariac_interfaces::action::MoveAgv_Goal>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<ariac_interfaces::action::MoveAgv_Goal>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<ariac_interfaces::action::MoveAgv_Goal>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'status'
#include "ariac_interfaces/msg/detail/agv_status__traits.hpp"

namespace ariac_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const MoveAgv_Result & msg,
  std::ostream & out)
{
  out << "{";
  // member: status
  {
    out << "status: ";
    to_flow_style_yaml(msg.status, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MoveAgv_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "status:\n";
    to_block_style_yaml(msg.status, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MoveAgv_Result & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ariac_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use ariac_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ariac_interfaces::action::MoveAgv_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::action::MoveAgv_Result & msg)
{
  return ariac_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::action::MoveAgv_Result>()
{
  return "ariac_interfaces::action::MoveAgv_Result";
}

template<>
inline const char * name<ariac_interfaces::action::MoveAgv_Result>()
{
  return "ariac_interfaces/action/MoveAgv_Result";
}

template<>
struct has_fixed_size<ariac_interfaces::action::MoveAgv_Result>
  : std::integral_constant<bool, has_fixed_size<ariac_interfaces::msg::AgvStatus>::value> {};

template<>
struct has_bounded_size<ariac_interfaces::action::MoveAgv_Result>
  : std::integral_constant<bool, has_bounded_size<ariac_interfaces::msg::AgvStatus>::value> {};

template<>
struct is_message<ariac_interfaces::action::MoveAgv_Result>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'status'
// already included above
// #include "ariac_interfaces/msg/detail/agv_status__traits.hpp"

namespace ariac_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const MoveAgv_Feedback & msg,
  std::ostream & out)
{
  out << "{";
  // member: status
  {
    out << "status: ";
    to_flow_style_yaml(msg.status, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MoveAgv_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "status:\n";
    to_block_style_yaml(msg.status, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MoveAgv_Feedback & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ariac_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use ariac_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ariac_interfaces::action::MoveAgv_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::action::MoveAgv_Feedback & msg)
{
  return ariac_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::action::MoveAgv_Feedback>()
{
  return "ariac_interfaces::action::MoveAgv_Feedback";
}

template<>
inline const char * name<ariac_interfaces::action::MoveAgv_Feedback>()
{
  return "ariac_interfaces/action/MoveAgv_Feedback";
}

template<>
struct has_fixed_size<ariac_interfaces::action::MoveAgv_Feedback>
  : std::integral_constant<bool, has_fixed_size<ariac_interfaces::msg::AgvStatus>::value> {};

template<>
struct has_bounded_size<ariac_interfaces::action::MoveAgv_Feedback>
  : std::integral_constant<bool, has_bounded_size<ariac_interfaces::msg::AgvStatus>::value> {};

template<>
struct is_message<ariac_interfaces::action::MoveAgv_Feedback>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'goal'
#include "ariac_interfaces/action/detail/move_agv__traits.hpp"

namespace ariac_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const MoveAgv_SendGoal_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
    out << ", ";
  }

  // member: goal
  {
    out << "goal: ";
    to_flow_style_yaml(msg.goal, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MoveAgv_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }

  // member: goal
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal:\n";
    to_block_style_yaml(msg.goal, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MoveAgv_SendGoal_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ariac_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use ariac_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ariac_interfaces::action::MoveAgv_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::action::MoveAgv_SendGoal_Request & msg)
{
  return ariac_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::action::MoveAgv_SendGoal_Request>()
{
  return "ariac_interfaces::action::MoveAgv_SendGoal_Request";
}

template<>
inline const char * name<ariac_interfaces::action::MoveAgv_SendGoal_Request>()
{
  return "ariac_interfaces/action/MoveAgv_SendGoal_Request";
}

template<>
struct has_fixed_size<ariac_interfaces::action::MoveAgv_SendGoal_Request>
  : std::integral_constant<bool, has_fixed_size<ariac_interfaces::action::MoveAgv_Goal>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<ariac_interfaces::action::MoveAgv_SendGoal_Request>
  : std::integral_constant<bool, has_bounded_size<ariac_interfaces::action::MoveAgv_Goal>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<ariac_interfaces::action::MoveAgv_SendGoal_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__traits.hpp"

namespace ariac_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const MoveAgv_SendGoal_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: accepted
  {
    out << "accepted: ";
    rosidl_generator_traits::value_to_yaml(msg.accepted, out);
    out << ", ";
  }

  // member: stamp
  {
    out << "stamp: ";
    to_flow_style_yaml(msg.stamp, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MoveAgv_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: accepted
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "accepted: ";
    rosidl_generator_traits::value_to_yaml(msg.accepted, out);
    out << "\n";
  }

  // member: stamp
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "stamp:\n";
    to_block_style_yaml(msg.stamp, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MoveAgv_SendGoal_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ariac_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use ariac_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ariac_interfaces::action::MoveAgv_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::action::MoveAgv_SendGoal_Response & msg)
{
  return ariac_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::action::MoveAgv_SendGoal_Response>()
{
  return "ariac_interfaces::action::MoveAgv_SendGoal_Response";
}

template<>
inline const char * name<ariac_interfaces::action::MoveAgv_SendGoal_Response>()
{
  return "ariac_interfaces/action/MoveAgv_SendGoal_Response";
}

template<>
struct has_fixed_size<ariac_interfaces::action::MoveAgv_SendGoal_Response>
  : std::integral_constant<bool, has_fixed_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct has_bounded_size<ariac_interfaces::action::MoveAgv_SendGoal_Response>
  : std::integral_constant<bool, has_bounded_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct is_message<ariac_interfaces::action::MoveAgv_SendGoal_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<ariac_interfaces::action::MoveAgv_SendGoal>()
{
  return "ariac_interfaces::action::MoveAgv_SendGoal";
}

template<>
inline const char * name<ariac_interfaces::action::MoveAgv_SendGoal>()
{
  return "ariac_interfaces/action/MoveAgv_SendGoal";
}

template<>
struct has_fixed_size<ariac_interfaces::action::MoveAgv_SendGoal>
  : std::integral_constant<
    bool,
    has_fixed_size<ariac_interfaces::action::MoveAgv_SendGoal_Request>::value &&
    has_fixed_size<ariac_interfaces::action::MoveAgv_SendGoal_Response>::value
  >
{
};

template<>
struct has_bounded_size<ariac_interfaces::action::MoveAgv_SendGoal>
  : std::integral_constant<
    bool,
    has_bounded_size<ariac_interfaces::action::MoveAgv_SendGoal_Request>::value &&
    has_bounded_size<ariac_interfaces::action::MoveAgv_SendGoal_Response>::value
  >
{
};

template<>
struct is_service<ariac_interfaces::action::MoveAgv_SendGoal>
  : std::true_type
{
};

template<>
struct is_service_request<ariac_interfaces::action::MoveAgv_SendGoal_Request>
  : std::true_type
{
};

template<>
struct is_service_response<ariac_interfaces::action::MoveAgv_SendGoal_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"

namespace ariac_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const MoveAgv_GetResult_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MoveAgv_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MoveAgv_GetResult_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ariac_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use ariac_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ariac_interfaces::action::MoveAgv_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::action::MoveAgv_GetResult_Request & msg)
{
  return ariac_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::action::MoveAgv_GetResult_Request>()
{
  return "ariac_interfaces::action::MoveAgv_GetResult_Request";
}

template<>
inline const char * name<ariac_interfaces::action::MoveAgv_GetResult_Request>()
{
  return "ariac_interfaces/action/MoveAgv_GetResult_Request";
}

template<>
struct has_fixed_size<ariac_interfaces::action::MoveAgv_GetResult_Request>
  : std::integral_constant<bool, has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<ariac_interfaces::action::MoveAgv_GetResult_Request>
  : std::integral_constant<bool, has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<ariac_interfaces::action::MoveAgv_GetResult_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'result'
// already included above
// #include "ariac_interfaces/action/detail/move_agv__traits.hpp"

namespace ariac_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const MoveAgv_GetResult_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: status
  {
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << ", ";
  }

  // member: result
  {
    out << "result: ";
    to_flow_style_yaml(msg.result, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MoveAgv_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << "\n";
  }

  // member: result
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "result:\n";
    to_block_style_yaml(msg.result, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MoveAgv_GetResult_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ariac_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use ariac_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ariac_interfaces::action::MoveAgv_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::action::MoveAgv_GetResult_Response & msg)
{
  return ariac_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::action::MoveAgv_GetResult_Response>()
{
  return "ariac_interfaces::action::MoveAgv_GetResult_Response";
}

template<>
inline const char * name<ariac_interfaces::action::MoveAgv_GetResult_Response>()
{
  return "ariac_interfaces/action/MoveAgv_GetResult_Response";
}

template<>
struct has_fixed_size<ariac_interfaces::action::MoveAgv_GetResult_Response>
  : std::integral_constant<bool, has_fixed_size<ariac_interfaces::action::MoveAgv_Result>::value> {};

template<>
struct has_bounded_size<ariac_interfaces::action::MoveAgv_GetResult_Response>
  : std::integral_constant<bool, has_bounded_size<ariac_interfaces::action::MoveAgv_Result>::value> {};

template<>
struct is_message<ariac_interfaces::action::MoveAgv_GetResult_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<ariac_interfaces::action::MoveAgv_GetResult>()
{
  return "ariac_interfaces::action::MoveAgv_GetResult";
}

template<>
inline const char * name<ariac_interfaces::action::MoveAgv_GetResult>()
{
  return "ariac_interfaces/action/MoveAgv_GetResult";
}

template<>
struct has_fixed_size<ariac_interfaces::action::MoveAgv_GetResult>
  : std::integral_constant<
    bool,
    has_fixed_size<ariac_interfaces::action::MoveAgv_GetResult_Request>::value &&
    has_fixed_size<ariac_interfaces::action::MoveAgv_GetResult_Response>::value
  >
{
};

template<>
struct has_bounded_size<ariac_interfaces::action::MoveAgv_GetResult>
  : std::integral_constant<
    bool,
    has_bounded_size<ariac_interfaces::action::MoveAgv_GetResult_Request>::value &&
    has_bounded_size<ariac_interfaces::action::MoveAgv_GetResult_Response>::value
  >
{
};

template<>
struct is_service<ariac_interfaces::action::MoveAgv_GetResult>
  : std::true_type
{
};

template<>
struct is_service_request<ariac_interfaces::action::MoveAgv_GetResult_Request>
  : std::true_type
{
};

template<>
struct is_service_response<ariac_interfaces::action::MoveAgv_GetResult_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'feedback'
// already included above
// #include "ariac_interfaces/action/detail/move_agv__traits.hpp"

namespace ariac_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const MoveAgv_FeedbackMessage & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
    out << ", ";
  }

  // member: feedback
  {
    out << "feedback: ";
    to_flow_style_yaml(msg.feedback, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const MoveAgv_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }

  // member: feedback
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "feedback:\n";
    to_block_style_yaml(msg.feedback, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const MoveAgv_FeedbackMessage & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace ariac_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use ariac_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const ariac_interfaces::action::MoveAgv_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  ariac_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use ariac_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const ariac_interfaces::action::MoveAgv_FeedbackMessage & msg)
{
  return ariac_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<ariac_interfaces::action::MoveAgv_FeedbackMessage>()
{
  return "ariac_interfaces::action::MoveAgv_FeedbackMessage";
}

template<>
inline const char * name<ariac_interfaces::action::MoveAgv_FeedbackMessage>()
{
  return "ariac_interfaces/action/MoveAgv_FeedbackMessage";
}

template<>
struct has_fixed_size<ariac_interfaces::action::MoveAgv_FeedbackMessage>
  : std::integral_constant<bool, has_fixed_size<ariac_interfaces::action::MoveAgv_Feedback>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<ariac_interfaces::action::MoveAgv_FeedbackMessage>
  : std::integral_constant<bool, has_bounded_size<ariac_interfaces::action::MoveAgv_Feedback>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<ariac_interfaces::action::MoveAgv_FeedbackMessage>
  : std::true_type {};

}  // namespace rosidl_generator_traits


namespace rosidl_generator_traits
{

template<>
struct is_action<ariac_interfaces::action::MoveAgv>
  : std::true_type
{
};

template<>
struct is_action_goal<ariac_interfaces::action::MoveAgv_Goal>
  : std::true_type
{
};

template<>
struct is_action_result<ariac_interfaces::action::MoveAgv_Result>
  : std::true_type
{
};

template<>
struct is_action_feedback<ariac_interfaces::action::MoveAgv_Feedback>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits


#endif  // ARIAC_INTERFACES__ACTION__DETAIL__MOVE_AGV__TRAITS_HPP_
