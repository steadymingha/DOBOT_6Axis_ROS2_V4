// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:srv/ConveyorControl.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__CONVEYOR_CONTROL__STRUCT_H_
#define ARIAC_INTERFACES__SRV__DETAIL__CONVEYOR_CONTROL__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/ConveyorControl in the package ariac_interfaces.
typedef struct ariac_interfaces__srv__ConveyorControl_Request
{
  double speed;
} ariac_interfaces__srv__ConveyorControl_Request;

// Struct for a sequence of ariac_interfaces__srv__ConveyorControl_Request.
typedef struct ariac_interfaces__srv__ConveyorControl_Request__Sequence
{
  ariac_interfaces__srv__ConveyorControl_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__srv__ConveyorControl_Request__Sequence;


// Constants defined in the message

/// Struct defined in srv/ConveyorControl in the package ariac_interfaces.
typedef struct ariac_interfaces__srv__ConveyorControl_Response
{
  bool success;
} ariac_interfaces__srv__ConveyorControl_Response;

// Struct for a sequence of ariac_interfaces__srv__ConveyorControl_Response.
typedef struct ariac_interfaces__srv__ConveyorControl_Response__Sequence
{
  ariac_interfaces__srv__ConveyorControl_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__srv__ConveyorControl_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__SRV__DETAIL__CONVEYOR_CONTROL__STRUCT_H_
