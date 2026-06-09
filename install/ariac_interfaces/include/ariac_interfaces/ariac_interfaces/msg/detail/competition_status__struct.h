// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/CompetitionStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATUS__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATUS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'time'
#include "ariac_interfaces/msg/detail/competition_time__struct.h"

/// Struct defined in msg/CompetitionStatus in the package ariac_interfaces.
/**
  * CompetitionStatus.msg
 */
typedef struct ariac_interfaces__msg__CompetitionStatus
{
  uint8_t competition_state;
  uint8_t num_kits;
  uint8_t num_modules;
  uint8_t num_kits_remaining;
  uint8_t num_modules_remaining;
  ariac_interfaces__msg__CompetitionTime time;
  int32_t run_id;
} ariac_interfaces__msg__CompetitionStatus;

// Struct for a sequence of ariac_interfaces__msg__CompetitionStatus.
typedef struct ariac_interfaces__msg__CompetitionStatus__Sequence
{
  ariac_interfaces__msg__CompetitionStatus * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__CompetitionStatus__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATUS__STRUCT_H_
