// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from dobot_msgs_v4:srv/CheckOddMovJ.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__CHECK_ODD_MOV_J__STRUCT_H_
#define DOBOT_MSGS_V4__SRV__DETAIL__CHECK_ODD_MOV_J__STRUCT_H_

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

/// Struct defined in srv/CheckOddMovJ in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__CheckOddMovJ_Request
{
  double point1_j1;
  double point1_j2;
  double point1_j3;
  double point1_j4;
  double point1_j5;
  double point1_j6;
  double point2_j1;
  double point2_j2;
  double point2_j3;
  double point2_j4;
  double point2_j5;
  double point2_j6;
  rosidl_runtime_c__String__Sequence param_value;
} dobot_msgs_v4__srv__CheckOddMovJ_Request;

// Struct for a sequence of dobot_msgs_v4__srv__CheckOddMovJ_Request.
typedef struct dobot_msgs_v4__srv__CheckOddMovJ_Request__Sequence
{
  dobot_msgs_v4__srv__CheckOddMovJ_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__CheckOddMovJ_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'robot_return'
// already included above
// #include "rosidl_runtime_c/string.h"

/// Struct defined in srv/CheckOddMovJ in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__CheckOddMovJ_Response
{
  rosidl_runtime_c__String robot_return;
  int32_t res;
} dobot_msgs_v4__srv__CheckOddMovJ_Response;

// Struct for a sequence of dobot_msgs_v4__srv__CheckOddMovJ_Response.
typedef struct dobot_msgs_v4__srv__CheckOddMovJ_Response__Sequence
{
  dobot_msgs_v4__srv__CheckOddMovJ_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__CheckOddMovJ_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__CHECK_ODD_MOV_J__STRUCT_H_
