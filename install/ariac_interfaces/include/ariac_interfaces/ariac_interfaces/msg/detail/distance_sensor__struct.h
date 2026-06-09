// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/DistanceSensor.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__DISTANCE_SENSOR__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__DISTANCE_SENSOR__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__struct.h"

/// Struct defined in msg/DistanceSensor in the package ariac_interfaces.
typedef struct ariac_interfaces__msg__DistanceSensor
{
  std_msgs__msg__Header header;
  double distance;
} ariac_interfaces__msg__DistanceSensor;

// Struct for a sequence of ariac_interfaces__msg__DistanceSensor.
typedef struct ariac_interfaces__msg__DistanceSensor__Sequence
{
  ariac_interfaces__msg__DistanceSensor * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__DistanceSensor__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__DISTANCE_SENSOR__STRUCT_H_
