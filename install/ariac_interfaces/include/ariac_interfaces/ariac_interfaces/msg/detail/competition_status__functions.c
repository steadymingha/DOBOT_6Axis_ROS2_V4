// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from ariac_interfaces:msg/CompetitionStatus.idl
// generated code does not contain a copyright notice
#include "ariac_interfaces/msg/detail/competition_status__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


// Include directives for member types
// Member `time`
#include "ariac_interfaces/msg/detail/competition_time__functions.h"

bool
ariac_interfaces__msg__CompetitionStatus__init(ariac_interfaces__msg__CompetitionStatus * msg)
{
  if (!msg) {
    return false;
  }
  // competition_state
  // num_kits
  // num_modules
  // num_kits_remaining
  // num_modules_remaining
  // time
  if (!ariac_interfaces__msg__CompetitionTime__init(&msg->time)) {
    ariac_interfaces__msg__CompetitionStatus__fini(msg);
    return false;
  }
  // run_id
  return true;
}

void
ariac_interfaces__msg__CompetitionStatus__fini(ariac_interfaces__msg__CompetitionStatus * msg)
{
  if (!msg) {
    return;
  }
  // competition_state
  // num_kits
  // num_modules
  // num_kits_remaining
  // num_modules_remaining
  // time
  ariac_interfaces__msg__CompetitionTime__fini(&msg->time);
  // run_id
}

bool
ariac_interfaces__msg__CompetitionStatus__are_equal(const ariac_interfaces__msg__CompetitionStatus * lhs, const ariac_interfaces__msg__CompetitionStatus * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // competition_state
  if (lhs->competition_state != rhs->competition_state) {
    return false;
  }
  // num_kits
  if (lhs->num_kits != rhs->num_kits) {
    return false;
  }
  // num_modules
  if (lhs->num_modules != rhs->num_modules) {
    return false;
  }
  // num_kits_remaining
  if (lhs->num_kits_remaining != rhs->num_kits_remaining) {
    return false;
  }
  // num_modules_remaining
  if (lhs->num_modules_remaining != rhs->num_modules_remaining) {
    return false;
  }
  // time
  if (!ariac_interfaces__msg__CompetitionTime__are_equal(
      &(lhs->time), &(rhs->time)))
  {
    return false;
  }
  // run_id
  if (lhs->run_id != rhs->run_id) {
    return false;
  }
  return true;
}

bool
ariac_interfaces__msg__CompetitionStatus__copy(
  const ariac_interfaces__msg__CompetitionStatus * input,
  ariac_interfaces__msg__CompetitionStatus * output)
{
  if (!input || !output) {
    return false;
  }
  // competition_state
  output->competition_state = input->competition_state;
  // num_kits
  output->num_kits = input->num_kits;
  // num_modules
  output->num_modules = input->num_modules;
  // num_kits_remaining
  output->num_kits_remaining = input->num_kits_remaining;
  // num_modules_remaining
  output->num_modules_remaining = input->num_modules_remaining;
  // time
  if (!ariac_interfaces__msg__CompetitionTime__copy(
      &(input->time), &(output->time)))
  {
    return false;
  }
  // run_id
  output->run_id = input->run_id;
  return true;
}

ariac_interfaces__msg__CompetitionStatus *
ariac_interfaces__msg__CompetitionStatus__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__CompetitionStatus * msg = (ariac_interfaces__msg__CompetitionStatus *)allocator.allocate(sizeof(ariac_interfaces__msg__CompetitionStatus), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(ariac_interfaces__msg__CompetitionStatus));
  bool success = ariac_interfaces__msg__CompetitionStatus__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
ariac_interfaces__msg__CompetitionStatus__destroy(ariac_interfaces__msg__CompetitionStatus * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    ariac_interfaces__msg__CompetitionStatus__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
ariac_interfaces__msg__CompetitionStatus__Sequence__init(ariac_interfaces__msg__CompetitionStatus__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__CompetitionStatus * data = NULL;

  if (size) {
    data = (ariac_interfaces__msg__CompetitionStatus *)allocator.zero_allocate(size, sizeof(ariac_interfaces__msg__CompetitionStatus), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = ariac_interfaces__msg__CompetitionStatus__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        ariac_interfaces__msg__CompetitionStatus__fini(&data[i - 1]);
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
ariac_interfaces__msg__CompetitionStatus__Sequence__fini(ariac_interfaces__msg__CompetitionStatus__Sequence * array)
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
      ariac_interfaces__msg__CompetitionStatus__fini(&array->data[i]);
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

ariac_interfaces__msg__CompetitionStatus__Sequence *
ariac_interfaces__msg__CompetitionStatus__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__CompetitionStatus__Sequence * array = (ariac_interfaces__msg__CompetitionStatus__Sequence *)allocator.allocate(sizeof(ariac_interfaces__msg__CompetitionStatus__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = ariac_interfaces__msg__CompetitionStatus__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
ariac_interfaces__msg__CompetitionStatus__Sequence__destroy(ariac_interfaces__msg__CompetitionStatus__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    ariac_interfaces__msg__CompetitionStatus__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
ariac_interfaces__msg__CompetitionStatus__Sequence__are_equal(const ariac_interfaces__msg__CompetitionStatus__Sequence * lhs, const ariac_interfaces__msg__CompetitionStatus__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!ariac_interfaces__msg__CompetitionStatus__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
ariac_interfaces__msg__CompetitionStatus__Sequence__copy(
  const ariac_interfaces__msg__CompetitionStatus__Sequence * input,
  ariac_interfaces__msg__CompetitionStatus__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(ariac_interfaces__msg__CompetitionStatus);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    ariac_interfaces__msg__CompetitionStatus * data =
      (ariac_interfaces__msg__CompetitionStatus *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!ariac_interfaces__msg__CompetitionStatus__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          ariac_interfaces__msg__CompetitionStatus__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!ariac_interfaces__msg__CompetitionStatus__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
