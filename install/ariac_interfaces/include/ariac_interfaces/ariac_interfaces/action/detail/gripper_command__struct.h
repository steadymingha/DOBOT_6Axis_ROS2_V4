// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from ariac_interfaces:action/GripperCommand.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__ACTION__DETAIL__GRIPPER_COMMAND__STRUCT_H_
#define ARIAC_INTERFACES__ACTION__DETAIL__GRIPPER_COMMAND__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in action/GripperCommand in the package ariac_interfaces.
typedef struct ariac_interfaces__action__GripperCommand_Goal
{
  double width;
} ariac_interfaces__action__GripperCommand_Goal;

// Struct for a sequence of ariac_interfaces__action__GripperCommand_Goal.
typedef struct ariac_interfaces__action__GripperCommand_Goal__Sequence
{
  ariac_interfaces__action__GripperCommand_Goal * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__GripperCommand_Goal__Sequence;


// Constants defined in the message

/// Struct defined in action/GripperCommand in the package ariac_interfaces.
typedef struct ariac_interfaces__action__GripperCommand_Result
{
  bool stalled;
  bool reached_goal_width;
  /// final width
  double width;
} ariac_interfaces__action__GripperCommand_Result;

// Struct for a sequence of ariac_interfaces__action__GripperCommand_Result.
typedef struct ariac_interfaces__action__GripperCommand_Result__Sequence
{
  ariac_interfaces__action__GripperCommand_Result * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__GripperCommand_Result__Sequence;


// Constants defined in the message

/// Struct defined in action/GripperCommand in the package ariac_interfaces.
typedef struct ariac_interfaces__action__GripperCommand_Feedback
{
  /// current width
  double width;
} ariac_interfaces__action__GripperCommand_Feedback;

// Struct for a sequence of ariac_interfaces__action__GripperCommand_Feedback.
typedef struct ariac_interfaces__action__GripperCommand_Feedback__Sequence
{
  ariac_interfaces__action__GripperCommand_Feedback * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__GripperCommand_Feedback__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'goal'
#include "ariac_interfaces/action/detail/gripper_command__struct.h"

/// Struct defined in action/GripperCommand in the package ariac_interfaces.
typedef struct ariac_interfaces__action__GripperCommand_SendGoal_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
  ariac_interfaces__action__GripperCommand_Goal goal;
} ariac_interfaces__action__GripperCommand_SendGoal_Request;

// Struct for a sequence of ariac_interfaces__action__GripperCommand_SendGoal_Request.
typedef struct ariac_interfaces__action__GripperCommand_SendGoal_Request__Sequence
{
  ariac_interfaces__action__GripperCommand_SendGoal_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__GripperCommand_SendGoal_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.h"

/// Struct defined in action/GripperCommand in the package ariac_interfaces.
typedef struct ariac_interfaces__action__GripperCommand_SendGoal_Response
{
  bool accepted;
  builtin_interfaces__msg__Time stamp;
} ariac_interfaces__action__GripperCommand_SendGoal_Response;

// Struct for a sequence of ariac_interfaces__action__GripperCommand_SendGoal_Response.
typedef struct ariac_interfaces__action__GripperCommand_SendGoal_Response__Sequence
{
  ariac_interfaces__action__GripperCommand_SendGoal_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__GripperCommand_SendGoal_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"

/// Struct defined in action/GripperCommand in the package ariac_interfaces.
typedef struct ariac_interfaces__action__GripperCommand_GetResult_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
} ariac_interfaces__action__GripperCommand_GetResult_Request;

// Struct for a sequence of ariac_interfaces__action__GripperCommand_GetResult_Request.
typedef struct ariac_interfaces__action__GripperCommand_GetResult_Request__Sequence
{
  ariac_interfaces__action__GripperCommand_GetResult_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__GripperCommand_GetResult_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "ariac_interfaces/action/detail/gripper_command__struct.h"

/// Struct defined in action/GripperCommand in the package ariac_interfaces.
typedef struct ariac_interfaces__action__GripperCommand_GetResult_Response
{
  int8_t status;
  ariac_interfaces__action__GripperCommand_Result result;
} ariac_interfaces__action__GripperCommand_GetResult_Response;

// Struct for a sequence of ariac_interfaces__action__GripperCommand_GetResult_Response.
typedef struct ariac_interfaces__action__GripperCommand_GetResult_Response__Sequence
{
  ariac_interfaces__action__GripperCommand_GetResult_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__GripperCommand_GetResult_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'feedback'
// already included above
// #include "ariac_interfaces/action/detail/gripper_command__struct.h"

/// Struct defined in action/GripperCommand in the package ariac_interfaces.
typedef struct ariac_interfaces__action__GripperCommand_FeedbackMessage
{
  unique_identifier_msgs__msg__UUID goal_id;
  ariac_interfaces__action__GripperCommand_Feedback feedback;
} ariac_interfaces__action__GripperCommand_FeedbackMessage;

// Struct for a sequence of ariac_interfaces__action__GripperCommand_FeedbackMessage.
typedef struct ariac_interfaces__action__GripperCommand_FeedbackMessage__Sequence
{
  ariac_interfaces__action__GripperCommand_FeedbackMessage * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} ariac_interfaces__action__GripperCommand_FeedbackMessage__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__ACTION__DETAIL__GRIPPER_COMMAND__STRUCT_H_
