// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:srv/SubmitInspectionReport.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_INSPECTION_REPORT__STRUCT_H_
#define ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_INSPECTION_REPORT__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'timestamp'
#include "builtin_interfaces/msg/detail/time__struct.h"
// Member 'report'
#include "ariac_interfaces/msg/detail/inspection_report__struct.h"

/// Struct defined in srv/SubmitInspectionReport in the package ariac_interfaces.
typedef struct ariac_interfaces__srv__SubmitInspectionReport_Request
{
  builtin_interfaces__msg__Time timestamp;
  ariac_interfaces__msg__InspectionReport report;
} ariac_interfaces__srv__SubmitInspectionReport_Request;

// Struct for a sequence of ariac_interfaces__srv__SubmitInspectionReport_Request.
typedef struct ariac_interfaces__srv__SubmitInspectionReport_Request__Sequence
{
  ariac_interfaces__srv__SubmitInspectionReport_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__srv__SubmitInspectionReport_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'message'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/SubmitInspectionReport in the package ariac_interfaces.
typedef struct ariac_interfaces__srv__SubmitInspectionReport_Response
{
  bool success;
  rosidl_runtime_c__String message;
} ariac_interfaces__srv__SubmitInspectionReport_Response;

// Struct for a sequence of ariac_interfaces__srv__SubmitInspectionReport_Response.
typedef struct ariac_interfaces__srv__SubmitInspectionReport_Response__Sequence
{
  ariac_interfaces__srv__SubmitInspectionReport_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__srv__SubmitInspectionReport_Response__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_INSPECTION_REPORT__STRUCT_H_
