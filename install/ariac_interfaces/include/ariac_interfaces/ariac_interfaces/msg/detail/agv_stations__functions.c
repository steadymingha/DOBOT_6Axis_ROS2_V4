// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from ariac_interfaces:msg/AgvStations.idl
// generated code does not contain a copyright notice
#include "ariac_interfaces/msg/detail/agv_stations__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


bool
ariac_interfaces__msg__AgvStations__init(ariac_interfaces__msg__AgvStations * msg)
{
  if (!msg) {
    return false;
  }
  // structure_needs_at_least_one_member
  return true;
}

void
ariac_interfaces__msg__AgvStations__fini(ariac_interfaces__msg__AgvStations * msg)
{
  if (!msg) {
    return;
  }
  // structure_needs_at_least_one_member
}

bool
ariac_interfaces__msg__AgvStations__are_equal(const ariac_interfaces__msg__AgvStations * lhs, const ariac_interfaces__msg__AgvStations * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // structure_needs_at_least_one_member
  if (lhs->structure_needs_at_least_one_member != rhs->structure_needs_at_least_one_member) {
    return false;
  }
  return true;
}

bool
ariac_interfaces__msg__AgvStations__copy(
  const ariac_interfaces__msg__AgvStations * input,
  ariac_interfaces__msg__AgvStations * output)
{
  if (!input || !output) {
    return false;
  }
  // structure_needs_at_least_one_member
  output->structure_needs_at_least_one_member = input->structure_needs_at_least_one_member;
  return true;
}

ariac_interfaces__msg__AgvStations *
ariac_interfaces__msg__AgvStations__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__AgvStations * msg = (ariac_interfaces__msg__AgvStations *)allocator.allocate(sizeof(ariac_interfaces__msg__AgvStations), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(ariac_interfaces__msg__AgvStations));
  bool success = ariac_interfaces__msg__AgvStations__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
ariac_interfaces__msg__AgvStations__destroy(ariac_interfaces__msg__AgvStations * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    ariac_interfaces__msg__AgvStations__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
ariac_interfaces__msg__AgvStations__Sequence__init(ariac_interfaces__msg__AgvStations__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__AgvStations * data = NULL;

  if (size) {
    data = (ariac_interfaces__msg__AgvStations *)allocator.zero_allocate(size, sizeof(ariac_interfaces__msg__AgvStations), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = ariac_interfaces__msg__AgvStations__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        ariac_interfaces__msg__AgvStations__fini(&data[i - 1]);
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
ariac_interfaces__msg__AgvStations__Sequence__fini(ariac_interfaces__msg__AgvStations__Sequence * array)
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
      ariac_interfaces__msg__AgvStations__fini(&array->data[i]);
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

ariac_interfaces__msg__AgvStations__Sequence *
ariac_interfaces__msg__AgvStations__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__AgvStations__Sequence * array = (ariac_interfaces__msg__AgvStations__Sequence *)allocator.allocate(sizeof(ariac_interfaces__msg__AgvStations__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = ariac_interfaces__msg__AgvStations__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
ariac_interfaces__msg__AgvStations__Sequence__destroy(ariac_interfaces__msg__AgvStations__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    ariac_interfaces__msg__AgvStations__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
ariac_interfaces__msg__AgvStations__Sequence__are_equal(const ariac_interfaces__msg__AgvStations__Sequence * lhs, const ariac_interfaces__msg__AgvStations__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!ariac_interfaces__msg__AgvStations__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
ariac_interfaces__msg__AgvStations__Sequence__copy(
  const ariac_interfaces__msg__AgvStations__Sequence * input,
  ariac_interfaces__msg__AgvStations__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(ariac_interfaces__msg__AgvStations);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    ariac_interfaces__msg__AgvStations * data =
      (ariac_interfaces__msg__AgvStations *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!ariac_interfaces__msg__AgvStations__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          ariac_interfaces__msg__AgvStations__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!ariac_interfaces__msg__AgvStations__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
