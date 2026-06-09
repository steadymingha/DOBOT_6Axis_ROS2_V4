// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/AgvStations.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__AGV_STATIONS__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__AGV_STATIONS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'INSPECTION'.
enum
{
  ariac_interfaces__msg__AgvStations__INSPECTION = 1
};

/// Constant 'ASSEMBLY'.
enum
{
  ariac_interfaces__msg__AgvStations__ASSEMBLY = 2
};

/// Constant 'SHIPPING'.
enum
{
  ariac_interfaces__msg__AgvStations__SHIPPING = 3
};

/// Constant 'RECYCLING'.
enum
{
  ariac_interfaces__msg__AgvStations__RECYCLING = 4
};

/// Constant 'IN_TRANSIT'.
enum
{
  ariac_interfaces__msg__AgvStations__IN_TRANSIT = -1
};

/// Struct defined in msg/AgvStations in the package ariac_interfaces.
/**
  * AGVStations.msg
 */
typedef struct ariac_interfaces__msg__AgvStations
{
  uint8_t structure_needs_at_least_one_member;
} ariac_interfaces__msg__AgvStations;

// Struct for a sequence of ariac_interfaces__msg__AgvStations.
typedef struct ariac_interfaces__msg__AgvStations__Sequence
{
  ariac_interfaces__msg__AgvStations * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__AgvStations__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__AGV_STATIONS__STRUCT_H_
