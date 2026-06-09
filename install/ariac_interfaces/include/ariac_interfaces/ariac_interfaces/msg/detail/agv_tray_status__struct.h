// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/AgvTrayStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__AGV_TRAY_STATUS__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__AGV_TRAY_STATUS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in msg/AgvTrayStatus in the package ariac_interfaces.
/**
  * AgvTrayStatus.msg
 */
typedef struct ariac_interfaces__msg__AgvTrayStatus
{
  bool slot_1_occupied;
  bool slot_2_occupied;
  bool slot_3_occupied;
  bool slot_4_occupied;
} ariac_interfaces__msg__AgvTrayStatus;

// Struct for a sequence of ariac_interfaces__msg__AgvTrayStatus.
typedef struct ariac_interfaces__msg__AgvTrayStatus__Sequence
{
  ariac_interfaces__msg__AgvTrayStatus * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__AgvTrayStatus__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__AGV_TRAY_STATUS__STRUCT_H_
