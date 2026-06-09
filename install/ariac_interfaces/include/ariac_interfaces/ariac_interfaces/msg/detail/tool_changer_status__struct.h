// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/ToolChangerStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__TOOL_CHANGER_STATUS__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__TOOL_CHANGER_STATUS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in msg/ToolChangerStatus in the package ariac_interfaces.
/**
  * ToolChangerStatus.msg
 */
typedef struct ariac_interfaces__msg__ToolChangerStatus
{
  int8_t attached_tool;
} ariac_interfaces__msg__ToolChangerStatus;

// Struct for a sequence of ariac_interfaces__msg__ToolChangerStatus.
typedef struct ariac_interfaces__msg__ToolChangerStatus__Sequence
{
  ariac_interfaces__msg__ToolChangerStatus * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__ToolChangerStatus__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__TOOL_CHANGER_STATUS__STRUCT_H_
