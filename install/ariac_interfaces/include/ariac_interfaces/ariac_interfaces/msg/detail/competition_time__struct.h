// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/CompetitionTime.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_TIME__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_TIME__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'start'
#include "builtin_interfaces/msg/detail/time__struct.h"
// Member 'elapsed'
// Member 'remaining'
#include "builtin_interfaces/msg/detail/duration__struct.h"

/// Struct defined in msg/CompetitionTime in the package ariac_interfaces.
/**
  * CompetitionTime.msg
 */
typedef struct ariac_interfaces__msg__CompetitionTime
{
  builtin_interfaces__msg__Time start;
  builtin_interfaces__msg__Duration elapsed;
  builtin_interfaces__msg__Duration remaining;
} ariac_interfaces__msg__CompetitionTime;

// Struct for a sequence of ariac_interfaces__msg__CompetitionTime.
typedef struct ariac_interfaces__msg__CompetitionTime__Sequence
{
  ariac_interfaces__msg__CompetitionTime * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__CompetitionTime__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_TIME__STRUCT_H_
