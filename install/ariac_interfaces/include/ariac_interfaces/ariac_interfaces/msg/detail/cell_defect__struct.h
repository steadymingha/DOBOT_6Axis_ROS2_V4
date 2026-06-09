// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/CellDefect.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__CELL_DEFECT__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__CELL_DEFECT__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'DENT'.
enum
{
  ariac_interfaces__msg__CellDefect__DENT = 1
};

/// Constant 'BULGE'.
enum
{
  ariac_interfaces__msg__CellDefect__BULGE = 2
};

/// Constant 'SCRATCH'.
enum
{
  ariac_interfaces__msg__CellDefect__SCRATCH = 3
};

/// Struct defined in msg/CellDefect in the package ariac_interfaces.
/**
  * CellDefect.msg
 */
typedef struct ariac_interfaces__msg__CellDefect
{
  /// one of the defect types above
  uint8_t defect_type;
  /// Location of the defect in cylinderical coordinates
  ///  relative to the cell base (r is implied as the radius of the cell)
  /// azimuthal angle of the defect centroid
  double theta;
  /// height of the defect`centroid
  double z;
} ariac_interfaces__msg__CellDefect;

// Struct for a sequence of ariac_interfaces__msg__CellDefect.
typedef struct ariac_interfaces__msg__CellDefect__Sequence
{
  ariac_interfaces__msg__CellDefect * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__CellDefect__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__CELL_DEFECT__STRUCT_H_
