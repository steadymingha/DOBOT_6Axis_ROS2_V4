// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from dobot_msgs_v4:srv/ModbusRTUCreate.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__MODBUS_RTU_CREATE__STRUCT_H_
#define DOBOT_MSGS_V4__SRV__DETAIL__MODBUS_RTU_CREATE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'parity'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/ModbusRTUCreate in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__ModbusRTUCreate_Request
{
  int32_t slave_id;
  int32_t baud;
  rosidl_runtime_c__String parity;
  int32_t data_bit;
  int32_t stop_bit;
} dobot_msgs_v4__srv__ModbusRTUCreate_Request;

// Struct for a sequence of dobot_msgs_v4__srv__ModbusRTUCreate_Request.
typedef struct dobot_msgs_v4__srv__ModbusRTUCreate_Request__Sequence
{
  dobot_msgs_v4__srv__ModbusRTUCreate_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__ModbusRTUCreate_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'robot_return'
// already included above
// #include "rosidl_runtime_c/string.h"

/// Struct defined in srv/ModbusRTUCreate in the package dobot_msgs_v4.
typedef struct dobot_msgs_v4__srv__ModbusRTUCreate_Response
{
  rosidl_runtime_c__String robot_return;
  int32_t res;
} dobot_msgs_v4__srv__ModbusRTUCreate_Response;

// Struct for a sequence of dobot_msgs_v4__srv__ModbusRTUCreate_Response.
typedef struct dobot_msgs_v4__srv__ModbusRTUCreate_Response__Sequence
{
  dobot_msgs_v4__srv__ModbusRTUCreate_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} dobot_msgs_v4__srv__ModbusRTUCreate_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__MODBUS_RTU_CREATE__STRUCT_H_
