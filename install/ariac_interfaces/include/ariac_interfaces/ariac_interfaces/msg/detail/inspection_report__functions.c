// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from ariac_interfaces:msg/InspectionReport.idl
// generated code does not contain a copyright notice
#include "ariac_interfaces/msg/detail/inspection_report__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `defects`
#include "ariac_interfaces/msg/detail/cell_defect__functions.h"

bool
ariac_interfaces__msg__InspectionReport__init(ariac_interfaces__msg__InspectionReport * msg)
{
  if (!msg) {
    return false;
  }
  // passed
  // defects
  if (!ariac_interfaces__msg__CellDefect__Sequence__init(&msg->defects, 0)) {
    ariac_interfaces__msg__InspectionReport__fini(msg);
    return false;
  }
  return true;
}

void
ariac_interfaces__msg__InspectionReport__fini(ariac_interfaces__msg__InspectionReport * msg)
{
  if (!msg) {
    return;
  }
  // passed
  // defects
  ariac_interfaces__msg__CellDefect__Sequence__fini(&msg->defects);
}

bool
ariac_interfaces__msg__InspectionReport__are_equal(const ariac_interfaces__msg__InspectionReport * lhs, const ariac_interfaces__msg__InspectionReport * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // passed
  if (lhs->passed != rhs->passed) {
    return false;
  }
  // defects
  if (!ariac_interfaces__msg__CellDefect__Sequence__are_equal(
      &(lhs->defects), &(rhs->defects)))
  {
    return false;
  }
  return true;
}

bool
ariac_interfaces__msg__InspectionReport__copy(
  const ariac_interfaces__msg__InspectionReport * input,
  ariac_interfaces__msg__InspectionReport * output)
{
  if (!input || !output) {
    return false;
  }
  // passed
  output->passed = input->passed;
  // defects
  if (!ariac_interfaces__msg__CellDefect__Sequence__copy(
      &(input->defects), &(output->defects)))
  {
    return false;
  }
  return true;
}

ariac_interfaces__msg__InspectionReport *
ariac_interfaces__msg__InspectionReport__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__InspectionReport * msg = (ariac_interfaces__msg__InspectionReport *)allocator.allocate(sizeof(ariac_interfaces__msg__InspectionReport), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(ariac_interfaces__msg__InspectionReport));
  bool success = ariac_interfaces__msg__InspectionReport__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
ariac_interfaces__msg__InspectionReport__destroy(ariac_interfaces__msg__InspectionReport * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    ariac_interfaces__msg__InspectionReport__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
ariac_interfaces__msg__InspectionReport__Sequence__init(ariac_interfaces__msg__InspectionReport__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__InspectionReport * data = NULL;

  if (size) {
    data = (ariac_interfaces__msg__InspectionReport *)allocator.zero_allocate(size, sizeof(ariac_interfaces__msg__InspectionReport), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = ariac_interfaces__msg__InspectionReport__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        ariac_interfaces__msg__InspectionReport__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
ariac_interfaces__msg__InspectionReport__Sequence__fini(ariac_interfaces__msg__InspectionReport__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      ariac_interfaces__msg__InspectionReport__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

ariac_interfaces__msg__InspectionReport__Sequence *
ariac_interfaces__msg__InspectionReport__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__InspectionReport__Sequence * array = (ariac_interfaces__msg__InspectionReport__Sequence *)allocator.allocate(sizeof(ariac_interfaces__msg__InspectionReport__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = ariac_interfaces__msg__InspectionReport__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
ariac_interfaces__msg__InspectionReport__Sequence__destroy(ariac_interfaces__msg__InspectionReport__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    ariac_interfaces__msg__InspectionReport__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
ariac_interfaces__msg__InspectionReport__Sequence__are_equal(const ariac_interfaces__msg__InspectionReport__Sequence * lhs, const ariac_interfaces__msg__InspectionReport__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!ariac_interfaces__msg__InspectionReport__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
ariac_interfaces__msg__InspectionReport__Sequence__copy(
  const ariac_interfaces__msg__InspectionReport__Sequence * input,
  ariac_interfaces__msg__InspectionReport__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(ariac_interfaces__msg__InspectionReport);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    ariac_interfaces__msg__InspectionReport * data =
      (ariac_interfaces__msg__InspectionReport *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!ariac_interfaces__msg__InspectionReport__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          ariac_interfaces__msg__InspectionReport__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!ariac_interfaces__msg__InspectionReport__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
