// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:msg/InspectionReport.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__INSPECTION_REPORT__STRUCT_H_
#define ARIAC_INTERFACES__MSG__DETAIL__INSPECTION_REPORT__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'defects'
#include "ariac_interfaces/msg/detail/cell_defect__struct.h"

/// Struct defined in msg/InspectionReport in the package ariac_interfaces.
/**
  * InspectionReport.msg
 */
typedef struct ariac_interfaces__msg__InspectionReport
{
  bool passed;
  ariac_interfaces__msg__CellDefect__Sequence defects;
} ariac_interfaces__msg__InspectionReport;

// Struct for a sequence of ariac_interfaces__msg__InspectionReport.
typedef struct ariac_interfaces__msg__InspectionReport__Sequence
{
  ariac_interfaces__msg__InspectionReport * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__msg__InspectionReport__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__INSPECTION_REPORT__STRUCT_H_
