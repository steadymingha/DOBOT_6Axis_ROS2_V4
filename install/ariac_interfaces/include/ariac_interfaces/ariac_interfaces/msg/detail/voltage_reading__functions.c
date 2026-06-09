// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from ariac_interfaces:msg/VoltageReading.idl
// generated code does not contain a copyright notice
#include "ariac_interfaces/msg/detail/voltage_reading__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


bool
ariac_interfaces__msg__VoltageReading__init(ariac_interfaces__msg__VoltageReading * msg)
{
  if (!msg) {
    return false;
  }
  // voltage
  // operation_status
  return true;
}

void
ariac_interfaces__msg__VoltageReading__fini(ariac_interfaces__msg__VoltageReading * msg)
{
  if (!msg) {
    return;
  }
  // voltage
  // operation_status
}

bool
ariac_interfaces__msg__VoltageReading__are_equal(const ariac_interfaces__msg__VoltageReading * lhs, const ariac_interfaces__msg__VoltageReading * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // voltage
  if (lhs->voltage != rhs->voltage) {
    return false;
  }
  // operation_status
  if (lhs->operation_status != rhs->operation_status) {
    return false;
  }
  return true;
}

bool
ariac_interfaces__msg__VoltageReading__copy(
  const ariac_interfaces__msg__VoltageReading * input,
  ariac_interfaces__msg__VoltageReading * output)
{
  if (!input || !output) {
    return false;
  }
  // voltage
  output->voltage = input->voltage;
  // operation_status
  output->operation_status = input->operation_status;
  return true;
}

ariac_interfaces__msg__VoltageReading *
ariac_interfaces__msg__VoltageReading__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__VoltageReading * msg = (ariac_interfaces__msg__VoltageReading *)allocator.allocate(sizeof(ariac_interfaces__msg__VoltageReading), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(ariac_interfaces__msg__VoltageReading));
  bool success = ariac_interfaces__msg__VoltageReading__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
ariac_interfaces__msg__VoltageReading__destroy(ariac_interfaces__msg__VoltageReading * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    ariac_interfaces__msg__VoltageReading__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
ariac_interfaces__msg__VoltageReading__Sequence__init(ariac_interfaces__msg__VoltageReading__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__VoltageReading * data = NULL;

  if (size) {
    data = (ariac_interfaces__msg__VoltageReading *)allocator.zero_allocate(size, sizeof(ariac_interfaces__msg__VoltageReading), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = ariac_interfaces__msg__VoltageReading__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        ariac_interfaces__msg__VoltageReading__fini(&data[i - 1]);
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
ariac_interfaces__msg__VoltageReading__Sequence__fini(ariac_interfaces__msg__VoltageReading__Sequence * array)
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
      ariac_interfaces__msg__VoltageReading__fini(&array->data[i]);
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

ariac_interfaces__msg__VoltageReading__Sequence *
ariac_interfaces__msg__VoltageReading__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  ariac_interfaces__msg__VoltageReading__Sequence * array = (ariac_interfaces__msg__VoltageReading__Sequence *)allocator.allocate(sizeof(ariac_interfaces__msg__VoltageReading__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = ariac_interfaces__msg__VoltageReading__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
ariac_interfaces__msg__VoltageReading__Sequence__destroy(ariac_interfaces__msg__VoltageReading__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    ariac_interfaces__msg__VoltageReading__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
ariac_interfaces__msg__VoltageReading__Sequence__are_equal(const ariac_interfaces__msg__VoltageReading__Sequence * lhs, const ariac_interfaces__msg__VoltageReading__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!ariac_interfaces__msg__VoltageReading__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
ariac_interfaces__msg__VoltageReading__Sequence__copy(
  const ariac_interfaces__msg__VoltageReading__Sequence * input,
  ariac_interfaces__msg__VoltageReading__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(ariac_interfaces__msg__VoltageReading);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    ariac_interfaces__msg__VoltageReading * data =
      (ariac_interfaces__msg__VoltageReading *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!ariac_interfaces__msg__VoltageReading__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          ariac_interfaces__msg__VoltageReading__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!ariac_interfaces__msg__VoltageReading__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
