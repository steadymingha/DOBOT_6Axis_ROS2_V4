// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from dobot_msgs_v4:srv/RunTo.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__RUN_TO__STRUCT_H_
#define DOBOT_MSGS_V4__SRV__DETAIL__RUN_TO__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/RunTo in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__RunTo_Request
{
  double a1;
  double b1;
  double c1;
  double d1;
  double e1;
  double f1;
  int32_t move_type;
  int32_t user;
  int32_t tool;
  int32_t a;
  int32_t v;
} dobot_msgs_v4__srv__RunTo_Request;

// Struct for a sequence of dobot_msgs_v4__srv__RunTo_Request.
typedef struct dobot_msgs_v4__srv__RunTo_Request__Sequence
{
  dobot_msgs_v4__srv__RunTo_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__RunTo_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/RunTo in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__RunTo_Response
{
  int32_t res;
} dobot_msgs_v4__srv__RunTo_Response;

// Struct for a sequence of dobot_msgs_v4__srv__RunTo_Response.
typedef struct dobot_msgs_v4__srv__RunTo_Response__Sequence
{
  dobot_msgs_v4__srv__RunTo_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__RunTo_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__RUN_TO__STRUCT_H_
