// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from dobot_msgs_v4:srv/TcpDashboard.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__TCP_DASHBOARD__STRUCT_H_
#define DOBOT_MSGS_V4__SRV__DETAIL__TCP_DASHBOARD__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'command'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/TcpDashboard in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__TcpDashboard_Request
{
  rosidl_runtime_c__String command;
} dobot_msgs_v4__srv__TcpDashboard_Request;

// Struct for a sequence of dobot_msgs_v4__srv__TcpDashboard_Request.
typedef struct dobot_msgs_v4__srv__TcpDashboard_Request__Sequence
{
  dobot_msgs_v4__srv__TcpDashboard_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__TcpDashboard_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "rosidl_runtime_c/string.h"

/// Struct defined in srv/TcpDashboard in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__TcpDashboard_Response
{
  rosidl_runtime_c__String result;
} dobot_msgs_v4__srv__TcpDashboard_Response;

// Struct for a sequence of dobot_msgs_v4__srv__TcpDashboard_Response.
typedef struct dobot_msgs_v4__srv__TcpDashboard_Response__Sequence
{
  dobot_msgs_v4__srv__TcpDashboard_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__TcpDashboard_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__TCP_DASHBOARD__STRUCT_H_
