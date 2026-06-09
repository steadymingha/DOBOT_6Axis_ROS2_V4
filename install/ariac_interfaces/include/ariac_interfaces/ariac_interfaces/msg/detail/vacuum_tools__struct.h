// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/VacuumTools.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__VACUUM_TOOLS__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__VACUUM_TOOLS__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'NONE'.
enum
{
  ariac_interfaces__msg__VacuumTools__NONE = -1
};

/// Constant 'VG_2'.
/**
  * Vacuum Gripper with two suction pads
 */
enum
{
  ariac_interfaces__msg__VacuumTools__VG_2 = 1
};

/// Constant 'VG_4'.
/**
  * Vacuum Gripper with four suction pads
 */
enum
{
  ariac_interfaces__msg__VacuumTools__VG_4 = 2
};

/// Struct defined in msg/VacuumTools in the package ariac_interfaces.
/**
  * VacuumTools.msg
 */
typedef struct ariac_interfaces__msg__VacuumTools
{
  uint8_t structure_needs_at_least_one_member;
} ariac_interfaces__msg__VacuumTools;

// Struct for a sequence of ariac_interfaces__msg__VacuumTools.
typedef struct ariac_interfaces__msg__VacuumTools__Sequence
{
  ariac_interfaces__msg__VacuumTools * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__VacuumTools__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__VACUUM_TOOLS__STRUCT_H_
