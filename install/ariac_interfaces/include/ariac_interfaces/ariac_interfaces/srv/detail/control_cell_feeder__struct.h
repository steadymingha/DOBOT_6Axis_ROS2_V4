// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:srv/ControlCellFeeder.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__CONTROL_CELL_FEEDER__STRUCT_H_
#define ARIAC_INTERFACES__SRV__DETAIL__CONTROL_CELL_FEEDER__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/ControlCellFeeder in the package ariac_interfaces.
typedef struct ariac_interfaces__srv__ControlCellFeeder_Request
{
  /// available types are in CellTypes.msg
  uint8_t cell_type;
} ariac_interfaces__srv__ControlCellFeeder_Request;

// Struct for a sequence of ariac_interfaces__srv__ControlCellFeeder_Request.
typedef struct ariac_interfaces__srv__ControlCellFeeder_Request__Sequence
{
  ariac_interfaces__srv__ControlCellFeeder_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__srv__ControlCellFeeder_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/ControlCellFeeder in the package ariac_interfaces.
typedef struct ariac_interfaces__srv__ControlCellFeeder_Response
{
  bool success;
  rosidl_runtime_c__String message;
} ariac_interfaces__srv__ControlCellFeeder_Response;

// Struct for a sequence of ariac_interfaces__srv__ControlCellFeeder_Response.
typedef struct ariac_interfaces__srv__ControlCellFeeder_Response__Sequence
{
  ariac_interfaces__srv__ControlCellFeeder_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__srv__ControlCellFeeder_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__SRV__DETAIL__CONTROL_CELL_FEEDER__STRUCT_H_
