// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from ariac_interfaces:msg/CompetitionTime.idl
// generated code does not contain a copyright notice
#include "ariac_interfaces/msg/detail/competition_time__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `start`
#include "builtin_interfaces/msg/detail/time__functions.h"
// Member `elapsed`
// Member `remaining`
#include "builtin_interfaces/msg/detail/duration__functions.h"

bool
ariac_interfaces__msg__CompetitionTime__init(ariac_interfaces__msg__CompetitionTime * msg)
{
  if (!msg) {
    return false;
  }
  // start
  if (!builtin_interfaces__msg__Time__init(&msg->start)) {
    ariac_interfaces__msg__CompetitionTime__fini(msg);
    return false;
  }
  // elapsed
  if (!builtin_interfaces__msg__Duration__init(&msg->elapsed)) {
    ariac_interfaces__msg__CompetitionTime__fini(msg);
    return false;
  }
  // remaining
  if (!builtin_interfaces__msg__Duration__init(&msg->remaining)) {
    ariac_interfaces__msg__CompetitionTime__fini(msg);
    return false;
  }
  return true;
}

void
ariac_interfaces__msg__CompetitionTime__fini(ariac_interfaces__msg__CompetitionTime * msg)
{
  if (!msg) {
    return;
  }
  // start
  builtin_interfaces__msg__Time__fini(&msg->start);
  // elapsed
  builtin_interfaces__msg__Duration__fini(&msg->elapsed);
  // remaining
  builtin_interfaces__msg__Duration__fini(&msg->remaining);
}

bool
ariac_interfaces__msg__CompetitionTime__are_equal(const ariac_interfaces__msg__CompetitionTime * lhs, const ariac_interfaces__msg__CompetitionTime * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // start
  if (!builtin_interfaces__msg__Time__are_equal(
      &(lhs->start), &(rhs->start)))
  {
    return false;
  }
  // elapsed
  if (!builtin_interfaces__msg__Duration__are_equal(
      &(lhs->elapsed), &(rhs->elapsed)))
  {
    return false;
  }
  // remaining
  if (!builtin_interfaces__msg__Duration__are_equal(
      &(lhs->remaining), &(rhs->remaining)))
  {
    return false;
  }
  return true;
}

bool
ariac_interfaces__msg__CompetitionTime__copy(
  const ariac_interfaces__msg__CompetitionTime * input,
  ariac_interfaces__msg__CompetitionTime * output)
{
  if (!input || !output) {
    return false;
  }
  // start
  if (!builtin_interfaces__msg__Time__copy(
      &(input->start), &(output->start)))
  {
    return false;
  }
  // elapsed
  if (!builtin_interfaces__msg__Duration__copy(
      &(input->elapsed), &(output->elapsed)))
  {
    return false;
  }
  // remaining
  if (!builtin_interfaces__msg__Duration__copy(
      &(input->remaining), &(output->remaining)))
  {
    return false;
  }
  return true;
}

ariac_interfaces__msg__CompetitionTime *
ariac_interfaces__msg__CompetitionTime__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__CompetitionTime * msg = (ariac_interfaces__msg__CompetitionTime *)allocator.allocate(sizeof(ariac_interfaces__msg__CompetitionTime), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(ariac_interfaces__msg__CompetitionTime));
  bool success = ariac_interfaces__msg__CompetitionTime__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
ariac_interfaces__msg__CompetitionTime__destroy(ariac_interfaces__msg__CompetitionTime * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    ariac_interfaces__msg__CompetitionTime__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
ariac_interfaces__msg__CompetitionTime__Sequence__init(ariac_interfaces__msg__CompetitionTime__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__CompetitionTime * data = NULL;

  if (size) {
    data = (ariac_interfaces__msg__CompetitionTime *)allocator.zero_allocate(size, sizeof(ariac_interfaces__msg__CompetitionTime), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = ariac_interfaces__msg__CompetitionTime__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        ariac_interfaces__msg__CompetitionTime__fini(&data[i - 1]);
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
ariac_interfaces__msg__CompetitionTime__Sequence__fini(ariac_interfaces__msg__CompetitionTime__Sequence * array)
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
      ariac_interfaces__msg__CompetitionTime__fini(&array->data[i]);
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

ariac_interfaces__msg__CompetitionTime__Sequence *
ariac_interfaces__msg__CompetitionTime__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__CompetitionTime__Sequence * array = (ariac_interfaces__msg__CompetitionTime__Sequence *)allocator.allocate(sizeof(ariac_interfaces__msg__CompetitionTime__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = ariac_interfaces__msg__CompetitionTime__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
ariac_interfaces__msg__CompetitionTime__Sequence__destroy(ariac_interfaces__msg__CompetitionTime__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    ariac_interfaces__msg__CompetitionTime__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
ariac_interfaces__msg__CompetitionTime__Sequence__are_equal(const ariac_interfaces__msg__CompetitionTime__Sequence * lhs, const ariac_interfaces__msg__CompetitionTime__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!ariac_interfaces__msg__CompetitionTime__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
ariac_interfaces__msg__CompetitionTime__Sequence__copy(
  const ariac_interfaces__msg__CompetitionTime__Sequence * input,
  ariac_interfaces__msg__CompetitionTime__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(ariac_interfaces__msg__CompetitionTime);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    ariac_interfaces__msg__CompetitionTime * data =
      (ariac_interfaces__msg__CompetitionTime *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!ariac_interfaces__msg__CompetitionTime__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          ariac_interfaces__msg__CompetitionTime__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!ariac_interfaces__msg__CompetitionTime__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
