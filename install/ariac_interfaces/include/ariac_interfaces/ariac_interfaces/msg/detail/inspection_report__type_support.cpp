// generated from rosidl_typesupport_introspection_cpp/resource/idl__type_support.cpp.em
// with input from ariac_interfaces:msg/InspectionReport.idl
// generated code does not contain a copyright notice

#include "array"
#include "cstddef"
#include "string"
#include "vector"
#include "rosidl_runtime_c/message_type_support_struct.h"
#include "rosidl_typesupport_cpp/message_type_support.hpp"
#include "rosidl_typesupport_interface/macros.h"
#include "ariac_interfaces/msg/detail/inspection_report__struct.hpp"
#include "rosidl_typesupport_introspection_cpp/field_types.hpp"
#include "rosidl_typesupport_introspection_cpp/identifier.hpp"
#include "rosidl_typesupport_introspection_cpp/message_introspection.hpp"
#include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
#include "rosidl_typesupport_introspection_cpp/visibility_control.h"

namespace ariac_interfaces
{

namespace msg
{

namespace rosidl_typesupport_introspection_cpp
{

void InspectionReport_init_function(
  void * message_memory, rosidl_runtime_cpp::MessageInitialization _init)
{
  new (message_memory) ariac_interfaces::msg::InspectionReport(_init);
}

void InspectionReport_fini_function(void * message_memory)
{
  auto typed_message = static_cast<ariac_interfaces::msg::InspectionReport *>(message_memory);
  typed_message->~InspectionReport();
}

size_t size_function__InspectionReport__defects(const void * untyped_member)
{
  const auto * member = reinterpret_cast<const std::vector<ariac_interfaces::msg::CellDefect> *>(untyped_member);
  return member->size();
}

const void * get_const_function__InspectionReport__defects(const void * untyped_member, size_t index)
{
  const auto & member =
    *reinterpret_cast<const std::vector<ariac_interfaces::msg::CellDefect> *>(untyped_member);
  return &member[index];
}

void * get_function__InspectionReport__defects(void * untyped_member, size_t index)
{
  auto & member =
    *reinterpret_cast<std::vector<ariac_interfaces::msg::CellDefect> *>(untyped_member);
  return &member[index];
}

void fetch_function__InspectionReport__defects(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const auto & item = *reinterpret_cast<const ariac_interfaces::msg::CellDefect *>(
    get_const_function__InspectionReport__defects(untyped_member, index));
  auto & value = *reinterpret_cast<ariac_interfaces::msg::CellDefect *>(untyped_value);
  value = item;
}

void assign_function__InspectionReport__defects(
  void * untyped_member, size_t index, const void * untyped_value)
{
  auto & item = *reinterpret_cast<ariac_interfaces::msg::CellDefect *>(
    get_function__InspectionReport__defects(untyped_member, index));
  const auto & value = *reinterpret_cast<const ariac_interfaces::msg::CellDefect *>(untyped_value);
  item = value;
}

void resize_function__InspectionReport__defects(void * untyped_member, size_t size)
{
  auto * member =
    reinterpret_cast<std::vector<ariac_interfaces::msg::CellDefect> *>(untyped_member);
  member->resize(size);
}

static const ::rosidl_typesupport_introspection_cpp::MessageMember InspectionReport_message_member_array[2] = {
  {
    "passed",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    nullptr,  // members of sub message
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ariac_interfaces::msg::InspectionReport, passed),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr  // resize(index) function pointer
  },
  {
    "defects",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<ariac_interfaces::msg::CellDefect>(),  // members of sub message
    true,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ariac_interfaces::msg::InspectionReport, defects),  // bytes offset in struct
    nullptr,  // default value
    size_function__InspectionReport__defects,  // size() function pointer
    get_const_function__InspectionReport__defects,  // get_const(index) function pointer
    get_function__InspectionReport__defects,  // get(index) function pointer
    fetch_function__InspectionReport__defects,  // fetch(index, &value) function pointer
    assign_function__InspectionReport__defects,  // assign(index, value) function pointer
    resize_function__InspectionReport__defects  // resize(index) function pointer
  }
};

static const ::rosidl_typesupport_introspection_cpp::MessageMembers InspectionReport_message_members = {
  "ariac_interfaces::msg",  // message namespace
  "InspectionReport",  // message name
  2,  // number of fields
  sizeof(ariac_interfaces::msg::InspectionReport),
  InspectionReport_message_member_array,  // message members
  InspectionReport_init_function,  // function to initialize message memory (memory has to be allocated)
  InspectionReport_fini_function  // function to terminate message instance (will not free memory)
};

static const rosidl_message_type_support_t InspectionReport_message_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &InspectionReport_message_members,
  get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace msg

}  // namespace ariac_interfaces


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<ariac_interfaces::msg::InspectionReport>()
{
  return &::ariac_interfaces::msg::rosidl_typesupport_introspection_cpp::InspectionReport_message_type_support_handle;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, ariac_interfaces, msg, InspectionReport)() {
  return &::ariac_interfaces::msg::rosidl_typesupport_introspection_cpp::InspectionReport_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif
