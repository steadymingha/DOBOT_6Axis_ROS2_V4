// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from ariac_interfaces:msg/InspectionReport.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "ariac_interfaces/msg/detail/inspection_report__rosidl_typesupport_introspection_c.h"
#include "ariac_interfaces/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "ariac_interfaces/msg/detail/inspection_report__functions.h"
#include "ariac_interfaces/msg/detail/inspection_report__struct.h"


// Include directives for member types
// Member `defects`
#include "ariac_interfaces/msg/cell_defect.h"
// Member `defects`
#include "ariac_interfaces/msg/detail/cell_defect__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__InspectionReport_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ariac_interfaces__msg__InspectionReport__init(message_memory);
}

void ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__InspectionReport_fini_function(void * message_memory)
{
  ariac_interfaces__msg__InspectionReport__fini(message_memory);
}

size_t ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__size_function__InspectionReport__defects(
  const void * untyped_member)
{
  const ariac_interfaces__msg__CellDefect__Sequence * member =
    (const ariac_interfaces__msg__CellDefect__Sequence *)(untyped_member);
  return member->size;
}

const void * ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__get_const_function__InspectionReport__defects(
  const void * untyped_member, size_t index)
{
  const ariac_interfaces__msg__CellDefect__Sequence * member =
    (const ariac_interfaces__msg__CellDefect__Sequence *)(untyped_member);
  return &member->data[index];
}

void * ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__get_function__InspectionReport__defects(
  void * untyped_member, size_t index)
{
  ariac_interfaces__msg__CellDefect__Sequence * member =
    (ariac_interfaces__msg__CellDefect__Sequence *)(untyped_member);
  return &member->data[index];
}

void ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__fetch_function__InspectionReport__defects(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const ariac_interfaces__msg__CellDefect * item =
    ((const ariac_interfaces__msg__CellDefect *)
    ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__get_const_function__InspectionReport__defects(untyped_member, index));
  ariac_interfaces__msg__CellDefect * value =
    (ariac_interfaces__msg__CellDefect *)(untyped_value);
  *value = *item;
}

void ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__assign_function__InspectionReport__defects(
  void * untyped_member, size_t index, const void * untyped_value)
{
  ariac_interfaces__msg__CellDefect * item =
    ((ariac_interfaces__msg__CellDefect *)
    ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__get_function__InspectionReport__defects(untyped_member, index));
  const ariac_interfaces__msg__CellDefect * value =
    (const ariac_interfaces__msg__CellDefect *)(untyped_value);
  *item = *value;
}

bool ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__resize_function__InspectionReport__defects(
  void * untyped_member, size_t size)
{
  ariac_interfaces__msg__CellDefect__Sequence * member =
    (ariac_interfaces__msg__CellDefect__Sequence *)(untyped_member);
  ariac_interfaces__msg__CellDefect__Sequence__fini(member);
  return ariac_interfaces__msg__CellDefect__Sequence__init(member, size);
}

static rosidl_typesupport_introspection_c__MessageMember ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__InspectionReport_message_member_array[2] = {
  {
    "passed",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ariac_interfaces__msg__InspectionReport, passed),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "defects",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ariac_interfaces__msg__InspectionReport, defects),  // bytes offset in struct
    NULL,  // default value
    ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__size_function__InspectionReport__defects,  // size() function pointer
    ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__get_const_function__InspectionReport__defects,  // get_const(index) function pointer
    ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__get_function__InspectionReport__defects,  // get(index) function pointer
    ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__fetch_function__InspectionReport__defects,  // fetch(index, &value) function pointer
    ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__assign_function__InspectionReport__defects,  // assign(index, value) function pointer
    ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__resize_function__InspectionReport__defects  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__InspectionReport_message_members = {
  "ariac_interfaces__msg",  // message namespace
  "InspectionReport",  // message name
  2,  // number of fields
  sizeof(ariac_interfaces__msg__InspectionReport),
  ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__InspectionReport_message_member_array,  // message members
  ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__InspectionReport_init_function,  // function to initialize message memory (memory has to be allocated)
  ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__InspectionReport_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__InspectionReport_message_type_support_handle = {
  0,
  &ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__InspectionReport_message_members,
  get_message_typesupport_handle_function,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ariac_interfaces
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ariac_interfaces, msg, InspectionReport)() {
  ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__InspectionReport_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ariac_interfaces, msg, CellDefect)();
  if (!ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__InspectionReport_message_type_support_handle.typesupport_identifier) {
    ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__InspectionReport_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ariac_interfaces__msg__InspectionReport__rosidl_typesupport_introspection_c__InspectionReport_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif
