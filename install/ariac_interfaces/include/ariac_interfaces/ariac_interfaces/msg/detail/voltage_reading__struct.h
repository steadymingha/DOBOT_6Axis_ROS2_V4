// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/VoltageReading.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__VOLTAGE_READING__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__VOLTAGE_READING__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in msg/VoltageReading in the package ariac_interfaces.
typedef struct ariac_interfaces__msg__VoltageReading
{
  double voltage;
  uint8_t operation_status;
} ariac_interfaces__msg__VoltageReading;

// Struct for a sequence of ariac_interfaces__msg__VoltageReading.
typedef struct ariac_interfaces__msg__VoltageReading__Sequence
{
  ariac_interfaces__msg__VoltageReading * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__VoltageReading__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__VOLTAGE_READING__STRUCT_H_
