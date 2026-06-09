// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from dobot_msgs_v4:srv/SetToolPower.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__SET_TOOL_POWER__STRUCT_H_
#define DOBOT_MSGS_V4__SRV__DETAIL__SET_TOOL_POWER__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/SetToolPower in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__SetToolPower_Request
{
  int32_t status;
} dobot_msgs_v4__srv__SetToolPower_Request;

// Struct for a sequence of dobot_msgs_v4__srv__SetToolPower_Request.
typedef struct dobot_msgs_v4__srv__SetToolPower_Request__Sequence
{
  dobot_msgs_v4__srv__SetToolPower_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__SetToolPower_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/SetToolPower in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__SetToolPower_Response
{
  int32_t res;
} dobot_msgs_v4__srv__SetToolPower_Response;

// Struct for a sequence of dobot_msgs_v4__srv__SetToolPower_Response.
typedef struct dobot_msgs_v4__srv__SetToolPower_Response__Sequence
{
  dobot_msgs_v4__srv__SetToolPower_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__SetToolPower_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__SET_TOOL_POWER__STRUCT_H_
