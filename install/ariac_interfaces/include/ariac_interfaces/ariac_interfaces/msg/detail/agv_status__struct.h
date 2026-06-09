// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/AgvStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__AGV_STATUS__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__AGV_STATUS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'pose'
#include "geometry_msgs/msg/detail/pose__struct.h"

/// Struct defined in msg/AgvStatus in the package ariac_interfaces.
/**
  * AGVStatus.msg
 */
typedef struct ariac_interfaces__msg__AgvStatus
{
  /// Current location ID.
  /// Should be one of the values defined in AgvStations.msg.
  int8_t station_id;
  /// Current pose of the AGV
  geometry_msgs__msg__Pose pose;
} ariac_interfaces__msg__AgvStatus;

// Struct for a sequence of ariac_interfaces__msg__AgvStatus.
typedef struct ariac_interfaces__msg__AgvStatus__Sequence
{
  ariac_interfaces__msg__AgvStatus * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__AgvStatus__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__AGV_STATUS__STRUCT_H_
