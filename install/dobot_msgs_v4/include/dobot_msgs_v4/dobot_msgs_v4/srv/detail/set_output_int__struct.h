// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from dobot_msgs_v4:srv/SetOutputInt.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__SET_OUTPUT_INT__STRUCT_H_
#define DOBOT_MSGS_V4__SRV__DETAIL__SET_OUTPUT_INT__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/SetOutputInt in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__SetOutputInt_Request
{
  int32_t address;
  int32_t value;
} dobot_msgs_v4__srv__SetOutputInt_Request;

// Struct for a sequence of dobot_msgs_v4__srv__SetOutputInt_Request.
typedef struct dobot_msgs_v4__srv__SetOutputInt_Request__Sequence
{
  dobot_msgs_v4__srv__SetOutputInt_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__SetOutputInt_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/SetOutputInt in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__SetOutputInt_Response
{
  int32_t res;
} dobot_msgs_v4__srv__SetOutputInt_Response;

// Struct for a sequence of dobot_msgs_v4__srv__SetOutputInt_Response.
typedef struct dobot_msgs_v4__srv__SetOutputInt_Response__Sequence
{
  dobot_msgs_v4__srv__SetOutputInt_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__SetOutputInt_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__SET_OUTPUT_INT__STRUCT_H_
