// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from ariac_interfaces:msg/CompetitionStatus.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "ariac_interfaces/msg/detail/competition_status__rosidl_typesupport_introspection_c.h"
#include "ariac_interfaces/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "ariac_interfaces/msg/detail/competition_status__functions.h"
#include "ariac_interfaces/msg/detail/competition_status__struct.h"


// Include directives for member types
// Member `time`
#include "ariac_interfaces/msg/competition_time.h"
// Member `time`
#include "ariac_interfaces/msg/detail/competition_time__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ariac_interfaces__msg__CompetitionStatus__rosidl_typesupport_introspection_c__CompetitionStatus_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ariac_interfaces__msg__CompetitionStatus__init(message_memory);
}

void ariac_interfaces__msg__CompetitionStatus__rosidl_typesupport_introspection_c__CompetitionStatus_fini_function(void * message_memory)
{
  ariac_interfaces__msg__CompetitionStatus__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember ariac_interfaces__msg__CompetitionStatus__rosidl_typesupport_introspection_c__CompetitionStatus_message_member_array[7] = {
  {
    "competition_state",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ariac_interfaces__msg__CompetitionStatus, competition_state),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "num_kits",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ariac_interfaces__msg__CompetitionStatus, num_kits),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "num_modules",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ariac_interfaces__msg__CompetitionStatus, num_modules),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "num_kits_remaining",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ariac_interfaces__msg__CompetitionStatus, num_kits_remaining),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "num_modules_remaining",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ariac_interfaces__msg__CompetitionStatus, num_modules_remaining),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "time",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ariac_interfaces__msg__CompetitionStatus, time),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "run_id",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_INT32,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ariac_interfaces__msg__CompetitionStatus, run_id),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ariac_interfaces__msg__CompetitionStatus__rosidl_typesupport_introspection_c__CompetitionStatus_message_members = {
  "ariac_interfaces__msg",  // message namespace
  "CompetitionStatus",  // message name
  7,  // number of fields
  sizeof(ariac_interfaces__msg__CompetitionStatus),
  ariac_interfaces__msg__CompetitionStatus__rosidl_typesupport_introspection_c__CompetitionStatus_message_member_array,  // message members
  ariac_interfaces__msg__CompetitionStatus__rosidl_typesupport_introspection_c__CompetitionStatus_init_function,  // function to initialize message memory (memory has to be allocated)
  ariac_interfaces__msg__CompetitionStatus__rosidl_typesupport_introspection_c__CompetitionStatus_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ariac_interfaces__msg__CompetitionStatus__rosidl_typesupport_introspection_c__CompetitionStatus_message_type_support_handle = {
  0,
  &ariac_interfaces__msg__CompetitionStatus__rosidl_typesupport_introspection_c__CompetitionStatus_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ariac_interfaces
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ariac_interfaces, msg, CompetitionStatus)() {
  ariac_interfaces__msg__CompetitionStatus__rosidl_typesupport_introspection_c__CompetitionStatus_message_member_array[5].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ariac_interfaces, msg, CompetitionTime)();
  if (!ariac_interfaces__msg__CompetitionStatus__rosidl_typesupport_introspection_c__CompetitionStatus_message_type_support_handle.typesupport_identifier) {
    ariac_interfaces__msg__CompetitionStatus__rosidl_typesupport_introspection_c__CompetitionStatus_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ariac_interfaces__msg__CompetitionStatus__rosidl_typesupport_introspection_c__CompetitionStatus_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
