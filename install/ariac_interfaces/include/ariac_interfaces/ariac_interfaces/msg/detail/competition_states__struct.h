// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/CompetitionStates.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATES__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATES__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'PREPARING'.
/**
  * Competition cannot be started yet by the competitor
 */
enum
{
  ariac_interfaces__msg__CompetitionStates__PREPARING = 0
};

/// Constant 'READY'.
/**
  * Competition can be started by the competitor
 */
enum
{
  ariac_interfaces__msg__CompetitionStates__READY = 1
};

/// Constant 'STARTED'.
/**
  * Competition has been started
 */
enum
{
  ariac_interfaces__msg__CompetitionStates__STARTED = 2
};

/// Constant 'ORDERS_COMPLETE'.
/**
  * All orders announced and complete
 */
enum
{
  ariac_interfaces__msg__CompetitionStates__ORDERS_COMPLETE = 3
};

/// Constant 'ENDED'.
/**
  * Competition has ended
 */
enum
{
  ariac_interfaces__msg__CompetitionStates__ENDED = 4
};

/// Struct defined in msg/CompetitionStates in the package ariac_interfaces.
typedef struct ariac_interfaces__msg__CompetitionStates
{
  uint8_t structure_needs_at_least_one_member;
} ariac_interfaces__msg__CompetitionStates;

// Struct for a sequence of ariac_interfaces__msg__CompetitionStates.
typedef struct ariac_interfaces__msg__CompetitionStates__Sequence
{
  ariac_interfaces__msg__CompetitionStates * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__CompetitionStates__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATES__STRUCT_H_
