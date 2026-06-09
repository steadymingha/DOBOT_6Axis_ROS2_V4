// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from dobot_msgs_v4:srv/GetStartPose.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__GET_START_POSE__STRUCT_H_
#define DOBOT_MSGS_V4__SRV__DETAIL__GET_START_POSE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'trace_name'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/GetStartPose in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__GetStartPose_Request
{
  rosidl_runtime_c__String trace_name;
} dobot_msgs_v4__srv__GetStartPose_Request;

// Struct for a sequence of dobot_msgs_v4__srv__GetStartPose_Request.
typedef struct dobot_msgs_v4__srv__GetStartPose_Request__Sequence
{
  dobot_msgs_v4__srv__GetStartPose_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__GetStartPose_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'robot_return'
// already included above
// #include "rosidl_runtime_c/string.h"

/// Struct defined in srv/GetStartPose in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__GetStartPose_Response
{
  rosidl_runtime_c__String robot_return;
  int32_t res;
} dobot_msgs_v4__srv__GetStartPose_Response;

// Struct for a sequence of dobot_msgs_v4__srv__GetStartPose_Response.
typedef struct dobot_msgs_v4__srv__GetStartPose_Response__Sequence
{
  dobot_msgs_v4__srv__GetStartPose_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__GetStartPose_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__GET_START_POSE__STRUCT_H_
