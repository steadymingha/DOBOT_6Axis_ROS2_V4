// NOLINT: This file starts with a BOM since it contain non-ASCII characters
// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/CellTypes.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__CELL_TYPES__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__CELL_TYPES__STRUCT_H_

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
  ariac_interfaces__msg__CellTypes__NONE = 0
};

/// Constant 'LI_ION'.
/**
  * Lithium Ion cells
 */
enum
{
  ariac_interfaces__msg__CellTypes__LI_ION = 1
};

/// Constant 'NIMH'.
/**
  * Nickel-Metal Hydride
 */
enum
{
  ariac_interfaces__msg__CellTypes__NIMH = 2
};

/// Constant 'LI_ION_NOMINAL_VOLTAGE'.
/**
  * Volts
 */
static const double ariac_interfaces__msg__CellTypes__LI_ION_NOMINAL_VOLTAGE = 3.6l;

/// Constant 'NIMH_NOMINAL_VOLTAGE'.
/**
  * Volts
 */
static const double ariac_interfaces__msg__CellTypes__NIMH_NOMINAL_VOLTAGE = 1.2l;

/// Constant 'CELL_VOLTAGE_TOLERANCE'.
/**
  * ± Volts
 */
static const double ariac_interfaces__msg__CellTypes__CELL_VOLTAGE_TOLERANCE = 0.2l;

/// Constant 'KIT_VOLTAGE_TOLERANCE'.
/**
  * ± Volts
 */
static const double ariac_interfaces__msg__CellTypes__KIT_VOLTAGE_TOLERANCE = 0.15l;

/// Struct defined in msg/CellTypes in the package ariac_interfaces.
/**
  * CellTypes.msg
 */
typedef struct ariac_interfaces__msg__CellTypes
{
  uint8_t structure_needs_at_least_one_member;
} ariac_interfaces__msg__CellTypes;

// Struct for a sequence of ariac_interfaces__msg__CellTypes.
typedef struct ariac_interfaces__msg__CellTypes__Sequence
{
  ariac_interfaces__msg__CellTypes * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__CellTypes__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__CELL_TYPES__STRUCT_H_
