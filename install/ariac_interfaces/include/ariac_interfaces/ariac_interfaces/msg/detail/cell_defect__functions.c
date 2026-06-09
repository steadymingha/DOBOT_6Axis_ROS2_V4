// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from ariac_interfaces:msg/CellDefect.idl
// generated code does not contain a copyright notice
#include "ariac_interfaces/msg/detail/cell_defect__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


bool
ariac_interfaces__msg__CellDefect__init(ariac_interfaces__msg__CellDefect * msg)
{
  if (!msg) {
    return false;
  }
  // defect_type
  // theta
  // z
  return true;
}

void
ariac_interfaces__msg__CellDefect__fini(ariac_interfaces__msg__CellDefect * msg)
{
  if (!msg) {
    return;
  }
  // defect_type
  // theta
  // z
}

bool
ariac_interfaces__msg__CellDefect__are_equal(const ariac_interfaces__msg__CellDefect * lhs, const ariac_interfaces__msg__CellDefect * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // defect_type
  if (lhs->defect_type != rhs->defect_type) {
    return false;
  }
  // theta
  if (lhs->theta != rhs->theta) {
    return false;
  }
  // z
  if (lhs->z != rhs->z) {
    return false;
  }
  return true;
}

bool
ariac_interfaces__msg__CellDefect__copy(
  const ariac_interfaces__msg__CellDefect * input,
  ariac_interfaces__msg__CellDefect * output)
{
  if (!input || !output) {
    return false;
  }
  // defect_type
  output->defect_type = input->defect_type;
  // theta
  output->theta = input->theta;
  // z
  output->z = input->z;
  return true;
}

ariac_interfaces__msg__CellDefect *
ariac_interfaces__msg__CellDefect__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__CellDefect * msg = (ariac_interfaces__msg__CellDefect *)allocator.allocate(sizeof(ariac_interfaces__msg__CellDefect), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(ariac_interfaces__msg__CellDefect));
  bool success = ariac_interfaces__msg__CellDefect__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
ariac_interfaces__msg__CellDefect__destroy(ariac_interfaces__msg__CellDefect * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    ariac_interfaces__msg__CellDefect__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
ariac_interfaces__msg__CellDefect__Sequence__init(ariac_interfaces__msg__CellDefect__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__CellDefect * data = NULL;

  if (size) {
    data = (ariac_interfaces__msg__CellDefect *)allocator.zero_allocate(size, sizeof(ariac_interfaces__msg__CellDefect), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = ariac_interfaces__msg__CellDefect__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        ariac_interfaces__msg__CellDefect__fini(&data[i - 1]);
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
ariac_interfaces__msg__CellDefect__Sequence__fini(ariac_interfaces__msg__CellDefect__Sequence * array)
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
      ariac_interfaces__msg__CellDefect__fini(&array->data[i]);
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

ariac_interfaces__msg__CellDefect__Sequence *
ariac_interfaces__msg__CellDefect__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__CellDefect__Sequence * array = (ariac_interfaces__msg__CellDefect__Sequence *)allocator.allocate(sizeof(ariac_interfaces__msg__CellDefect__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = ariac_interfaces__msg__CellDefect__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
ariac_interfaces__msg__CellDefect__Sequence__destroy(ariac_interfaces__msg__CellDefect__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    ariac_interfaces__msg__CellDefect__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
ariac_interfaces__msg__CellDefect__Sequence__are_equal(const ariac_interfaces__msg__CellDefect__Sequence * lhs, const ariac_interfaces__msg__CellDefect__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!ariac_interfaces__msg__CellDefect__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
ariac_interfaces__msg__CellDefect__Sequence__copy(
  const ariac_interfaces__msg__CellDefect__Sequence * input,
  ariac_interfaces__msg__CellDefect__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(ariac_interfaces__msg__CellDefect);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    ariac_interfaces__msg__CellDefect * data =
      (ariac_interfaces__msg__CellDefect *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!ariac_interfaces__msg__CellDefect__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          ariac_interfaces__msg__CellDefect__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!ariac_interfaces__msg__CellDefect__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
