// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:action/MoveAgv.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__ACTION__DETAIL__MOVE_AGV__STRUCT_H_
#define ARIAC_INTERFACES__ACTION__DETAIL__MOVE_AGV__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in action/MoveAgv in the package ariac_interfaces.
typedef struct ariac_interfaces__action__MoveAgv_Goal
{
  /// Goal
  int8_t station_id;
} ariac_interfaces__action__MoveAgv_Goal;

// Struct for a sequence of ariac_interfaces__action__MoveAgv_Goal.
typedef struct ariac_interfaces__action__MoveAgv_Goal__Sequence
{
  ariac_interfaces__action__MoveAgv_Goal * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__MoveAgv_Goal__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'status'
#include "ariac_interfaces/msg/detail/agv_status__struct.h"

/// Struct defined in action/MoveAgv in the package ariac_interfaces.
typedef struct ariac_interfaces__action__MoveAgv_Result
{
  ariac_interfaces__msg__AgvStatus status;
} ariac_interfaces__action__MoveAgv_Result;

// Struct for a sequence of ariac_interfaces__action__MoveAgv_Result.
typedef struct ariac_interfaces__action__MoveAgv_Result__Sequence
{
  ariac_interfaces__action__MoveAgv_Result * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__MoveAgv_Result__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'status'
// already included above
// #include "ariac_interfaces/msg/detail/agv_status__struct.h"

/// Struct defined in action/MoveAgv in the package ariac_interfaces.
typedef struct ariac_interfaces__action__MoveAgv_Feedback
{
  ariac_interfaces__msg__AgvStatus status;
} ariac_interfaces__action__MoveAgv_Feedback;

// Struct for a sequence of ariac_interfaces__action__MoveAgv_Feedback.
typedef struct ariac_interfaces__action__MoveAgv_Feedback__Sequence
{
  ariac_interfaces__action__MoveAgv_Feedback * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__MoveAgv_Feedback__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'goal'
#include "ariac_interfaces/action/detail/move_agv__struct.h"

/// Struct defined in action/MoveAgv in the package ariac_interfaces.
typedef struct ariac_interfaces__action__MoveAgv_SendGoal_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
  ariac_interfaces__action__MoveAgv_Goal goal;
} ariac_interfaces__action__MoveAgv_SendGoal_Request;

// Struct for a sequence of ariac_interfaces__action__MoveAgv_SendGoal_Request.
typedef struct ariac_interfaces__action__MoveAgv_SendGoal_Request__Sequence
{
  ariac_interfaces__action__MoveAgv_SendGoal_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__MoveAgv_SendGoal_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.h"

/// Struct defined in action/MoveAgv in the package ariac_interfaces.
typedef struct ariac_interfaces__action__MoveAgv_SendGoal_Response
{
  bool accepted;
  builtin_interfaces__msg__Time stamp;
} ariac_interfaces__action__MoveAgv_SendGoal_Response;

// Struct for a sequence of ariac_interfaces__action__MoveAgv_SendGoal_Response.
typedef struct ariac_interfaces__action__MoveAgv_SendGoal_Response__Sequence
{
  ariac_interfaces__action__MoveAgv_SendGoal_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__MoveAgv_SendGoal_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"

/// Struct defined in action/MoveAgv in the package ariac_interfaces.
typedef struct ariac_interfaces__action__MoveAgv_GetResult_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
} ariac_interfaces__action__MoveAgv_GetResult_Request;

// Struct for a sequence of ariac_interfaces__action__MoveAgv_GetResult_Request.
typedef struct ariac_interfaces__action__MoveAgv_GetResult_Request__Sequence
{
  ariac_interfaces__action__MoveAgv_GetResult_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__MoveAgv_GetResult_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "ariac_interfaces/action/detail/move_agv__struct.h"

/// Struct defined in action/MoveAgv in the package ariac_interfaces.
typedef struct ariac_interfaces__action__MoveAgv_GetResult_Response
{
  int8_t status;
  ariac_interfaces__action__MoveAgv_Result result;
} ariac_interfaces__action__MoveAgv_GetResult_Response;

// Struct for a sequence of ariac_interfaces__action__MoveAgv_GetResult_Response.
typedef struct ariac_interfaces__action__MoveAgv_GetResult_Response__Sequence
{
  ariac_interfaces__action__MoveAgv_GetResult_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__MoveAgv_GetResult_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'feedback'
// already included above
// #include "ariac_interfaces/action/detail/move_agv__struct.h"

/// Struct defined in action/MoveAgv in the package ariac_interfaces.
typedef struct ariac_interfaces__action__MoveAgv_FeedbackMessage
{
  unique_identifier_msgs__msg__UUID goal_id;
  ariac_interfaces__action__MoveAgv_Feedback feedback;
} ariac_interfaces__action__MoveAgv_FeedbackMessage;

// Struct for a sequence of ariac_interfaces__action__MoveAgv_FeedbackMessage.
typedef struct ariac_interfaces__action__MoveAgv_FeedbackMessage__Sequence
{
  ariac_interfaces__action__MoveAgv_FeedbackMessage * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__MoveAgv_FeedbackMessage__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__ACTION__DETAIL__MOVE_AGV__STRUCT_H_
