// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from dobot_msgs_v4:srv/SetOutputBool.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__SET_OUTPUT_BOOL__STRUCT_H_
#define DOBOT_MSGS_V4__SRV__DETAIL__SET_OUTPUT_BOOL__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/SetOutputBool in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__SetOutputBool_Request
{
  int32_t address;
  int32_t value;
} dobot_msgs_v4__srv__SetOutputBool_Request;

// Struct for a sequence of dobot_msgs_v4__srv__SetOutputBool_Request.
typedef struct dobot_msgs_v4__srv__SetOutputBool_Request__Sequence
{
  dobot_msgs_v4__srv__SetOutputBool_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__SetOutputBool_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/SetOutputBool in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__SetOutputBool_Response
{
  int32_t res;
} dobot_msgs_v4__srv__SetOutputBool_Response;

// Struct for a sequence of dobot_msgs_v4__srv__SetOutputBool_Response.
typedef struct dobot_msgs_v4__srv__SetOutputBool_Response__Sequence
{
  dobot_msgs_v4__srv__SetOutputBool_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__SetOutputBool_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__SET_OUTPUT_BOOL__STRUCT_H_
