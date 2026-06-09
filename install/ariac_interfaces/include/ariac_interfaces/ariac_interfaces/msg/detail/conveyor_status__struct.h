// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/ConveyorStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'FORWARD'.
enum
{
  ariac_interfaces__msg__ConveyorStatus__FORWARD = 0
};

/// Constant 'BACKWARD'.
enum
{
  ariac_interfaces__msg__ConveyorStatus__BACKWARD = 1
};

/// Struct defined in msg/ConveyorStatus in the package ariac_interfaces.
typedef struct ariac_interfaces__msg__ConveyorStatus
{
  uint8_t direction;
  /// m/s
  double speed;
  uint8_t operating_status;
} ariac_interfaces__msg__ConveyorStatus;

// Struct for a sequence of ariac_interfaces__msg__ConveyorStatus.
typedef struct ariac_interfaces__msg__ConveyorStatus__Sequence
{
  ariac_interfaces__msg__ConveyorStatus * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__ConveyorStatus__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__STRUCT_H_
