// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/OperationStates.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__OPERATION_STATES__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__OPERATION_STATES__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'OPERATIONAL'.
/**
  * Device is running normally
 */
enum
{
  ariac_interfaces__msg__OperationStates__OPERATIONAL = 1
};

/// Constant 'MALFUNCTIONING'.
/**
  * Device is malfunctioning
 */
enum
{
  ariac_interfaces__msg__OperationStates__MALFUNCTIONING = 2
};

/// Struct defined in msg/OperationStates in the package ariac_interfaces.
typedef struct ariac_interfaces__msg__OperationStates
{
  uint8_t structure_needs_at_least_one_member;
} ariac_interfaces__msg__OperationStates;

// Struct for a sequence of ariac_interfaces__msg__OperationStates.
typedef struct ariac_interfaces__msg__OperationStates__Sequence
{
  ariac_interfaces__msg__OperationStates * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__OperationStates__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__OPERATION_STATES__STRUCT_H_
