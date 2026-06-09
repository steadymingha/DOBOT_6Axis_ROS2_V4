// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from dobot_msgs_v4:srv/ServoJ.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__SERVO_J__STRUCT_H_
#define DOBOT_MSGS_V4__SRV__DETAIL__SERVO_J__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'param_value'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/ServoJ in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__ServoJ_Request
{
  double a;
  double b;
  double c;
  double d;
  double e;
  double f;
  rosidl_runtime_c__String__Sequence param_value;
} dobot_msgs_v4__srv__ServoJ_Request;

// Struct for a sequence of dobot_msgs_v4__srv__ServoJ_Request.
typedef struct dobot_msgs_v4__srv__ServoJ_Request__Sequence
{
  dobot_msgs_v4__srv__ServoJ_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__ServoJ_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/ServoJ in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__ServoJ_Response
{
  int32_t res;
} dobot_msgs_v4__srv__ServoJ_Response;

// Struct for a sequence of dobot_msgs_v4__srv__ServoJ_Response.
typedef struct dobot_msgs_v4__srv__ServoJ_Response__Sequence
{
  dobot_msgs_v4__srv__ServoJ_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__ServoJ_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__SERVO_J__STRUCT_H_
