// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/HighPriorityOrder.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__HIGH_PRIORITY_ORDER__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__HIGH_PRIORITY_ORDER__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'id'
#include "rosidl_runtime_c/string.h"

/// Struct defined in msg/HighPriorityOrder in the package ariac_interfaces.
typedef struct ariac_interfaces__msg__HighPriorityOrder
{
  rosidl_runtime_c__String id;
} ariac_interfaces__msg__HighPriorityOrder;

// Struct for a sequence of ariac_interfaces__msg__HighPriorityOrder.
typedef struct ariac_interfaces__msg__HighPriorityOrder__Sequence
{
  ariac_interfaces__msg__HighPriorityOrder * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__HighPriorityOrder__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__HIGH_PRIORITY_ORDER__STRUCT_H_
