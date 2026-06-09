// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from dobot_msgs_v4:srv/RunTo.idl
// generated code does not contain a copyright notice
#include "dobot_msgs_v4/srv/detail/run_to__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"

bool
dobot_msgs_v4__srv__RunTo_Request__init(dobot_msgs_v4__srv__RunTo_Request * msg)
{
  if (!msg) {
    return false;
  }
  // a1
  // b1
  // c1
  // d1
  // e1
  // f1
  // move_type
  // user
  msg->user = -1l;
  // tool
  msg->tool = -1l;
  // a
  msg->a = -1l;
  // v
  msg->v = -1l;
  return true;
}

void
dobot_msgs_v4__srv__RunTo_Request__fini(dobot_msgs_v4__srv__RunTo_Request * msg)
{
  if (!msg) {
    return;
  }
  // a1
  // b1
  // c1
  // d1
  // e1
  // f1
  // move_type
  // user
  // tool
  // a
  // v
}

bool
dobot_msgs_v4__srv__RunTo_Request__are_equal(const dobot_msgs_v4__srv__RunTo_Request * lhs, const dobot_msgs_v4__srv__RunTo_Request * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // a1
  if (lhs->a1 != rhs->a1) {
    return false;
  }
  // b1
  if (lhs->b1 != rhs->b1) {
    return false;
  }
  // c1
  if (lhs->c1 != rhs->c1) {
    return false;
  }
  // d1
  if (lhs->d1 != rhs->d1) {
    return false;
  }
  // e1
  if (lhs->e1 != rhs->e1) {
    return false;
  }
  // f1
  if (lhs->f1 != rhs->f1) {
    return false;
  }
  // move_type
  if (lhs->move_type != rhs->move_type) {
    return false;
  }
  // user
  if (lhs->user != rhs->user) {
    return false;
  }
  // tool
  if (lhs->tool != rhs->tool) {
    return false;
  }
  // a
  if (lhs->a != rhs->a) {
    return false;
  }
  // v
  if (lhs->v != rhs->v) {
    return false;
  }
  return true;
}

bool
dobot_msgs_v4__srv__RunTo_Request__copy(
  const dobot_msgs_v4__srv__RunTo_Request * input,
  dobot_msgs_v4__srv__RunTo_Request * output)
{
  if (!input || !output) {
    return false;
  }
  // a1
  output->a1 = input->a1;
  // b1
  output->b1 = input->b1;
  // c1
  output->c1 = input->c1;
  // d1
  output->d1 = input->d1;
  // e1
  output->e1 = input->e1;
  // f1
  output->f1 = input->f1;
  // move_type
  output->move_type = input->move_type;
  // user
  output->user = input->user;
  // tool
  output->tool = input->tool;
  // a
  output->a = input->a;
  // v
  output->v = input->v;
  return true;
}

dobot_msgs_v4__srv__RunTo_Request *
dobot_msgs_v4__srv__RunTo_Request__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  dobot_msgs_v4__srv__RunTo_Request * msg = (dobot_msgs_v4__srv__RunTo_Request *)allocator.allocate(sizeof(dobot_msgs_v4__srv__RunTo_Request), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(dobot_msgs_v4__srv__RunTo_Request));
  bool success = dobot_msgs_v4__srv__RunTo_Request__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
dobot_msgs_v4__srv__RunTo_Request__destroy(dobot_msgs_v4__srv__RunTo_Request * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    dobot_msgs_v4__srv__RunTo_Request__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
dobot_msgs_v4__srv__RunTo_Request__Sequence__init(dobot_msgs_v4__srv__RunTo_Request__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  dobot_msgs_v4__srv__RunTo_Request * data = NULL;

  if (size) {
    data = (dobot_msgs_v4__srv__RunTo_Request *)allocator.zero_allocate(size, sizeof(dobot_msgs_v4__srv__RunTo_Request), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = dobot_msgs_v4__srv__RunTo_Request__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        dobot_msgs_v4__srv__RunTo_Request__fini(&data[i - 1]);
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
dobot_msgs_v4__srv__RunTo_Request__Sequence__fini(dobot_msgs_v4__srv__RunTo_Request__Sequence * array)
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
      dobot_msgs_v4__srv__RunTo_Request__fini(&array->data[i]);
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

dobot_msgs_v4__srv__RunTo_Request__Sequence *
dobot_msgs_v4__srv__RunTo_Request__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  dobot_msgs_v4__srv__RunTo_Request__Sequence * array = (dobot_msgs_v4__srv__RunTo_Request__Sequence *)allocator.allocate(sizeof(dobot_msgs_v4__srv__RunTo_Request__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = dobot_msgs_v4__srv__RunTo_Request__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
dobot_msgs_v4__srv__RunTo_Request__Sequence__destroy(dobot_msgs_v4__srv__RunTo_Request__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    dobot_msgs_v4__srv__RunTo_Request__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
dobot_msgs_v4__srv__RunTo_Request__Sequence__are_equal(const dobot_msgs_v4__srv__RunTo_Request__Sequence * lhs, const dobot_msgs_v4__srv__RunTo_Request__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!dobot_msgs_v4__srv__RunTo_Request__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
dobot_msgs_v4__srv__RunTo_Request__Sequence__copy(
  const dobot_msgs_v4__srv__RunTo_Request__Sequence * input,
  dobot_msgs_v4__srv__RunTo_Request__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(dobot_msgs_v4__srv__RunTo_Request);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    dobot_msgs_v4__srv__RunTo_Request * data =
      (dobot_msgs_v4__srv__RunTo_Request *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!dobot_msgs_v4__srv__RunTo_Request__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          dobot_msgs_v4__srv__RunTo_Request__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!dobot_msgs_v4__srv__RunTo_Request__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


bool
dobot_msgs_v4__srv__RunTo_Response__init(dobot_msgs_v4__srv__RunTo_Response * msg)
{
  if (!msg) {
    return false;
  }
  // res
  return true;
}

void
dobot_msgs_v4__srv__RunTo_Response__fini(dobot_msgs_v4__srv__RunTo_Response * msg)
{
  if (!msg) {
    return;
  }
  // res
}

bool
dobot_msgs_v4__srv__RunTo_Response__are_equal(const dobot_msgs_v4__srv__RunTo_Response * lhs, const dobot_msgs_v4__srv__RunTo_Response * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // res
  if (lhs->res != rhs->res) {
    return false;
  }
  return true;
}

bool
dobot_msgs_v4__srv__RunTo_Response__copy(
  const dobot_msgs_v4__srv__RunTo_Response * input,
  dobot_msgs_v4__srv__RunTo_Response * output)
{
  if (!input || !output) {
    return false;
  }
  // res
  output->res = input->res;
  return true;
}

dobot_msgs_v4__srv__RunTo_Response *
dobot_msgs_v4__srv__RunTo_Response__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  dobot_msgs_v4__srv__RunTo_Response * msg = (dobot_msgs_v4__srv__RunTo_Response *)allocator.allocate(sizeof(dobot_msgs_v4__srv__RunTo_Response), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(dobot_msgs_v4__srv__RunTo_Response));
  bool success = dobot_msgs_v4__srv__RunTo_Response__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
dobot_msgs_v4__srv__RunTo_Response__destroy(dobot_msgs_v4__srv__RunTo_Response * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    dobot_msgs_v4__srv__RunTo_Response__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
dobot_msgs_v4__srv__RunTo_Response__Sequence__init(dobot_msgs_v4__srv__RunTo_Response__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  dobot_msgs_v4__srv__RunTo_Response * data = NULL;

  if (size) {
    data = (dobot_msgs_v4__srv__RunTo_Response *)allocator.zero_allocate(size, sizeof(dobot_msgs_v4__srv__RunTo_Response), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = dobot_msgs_v4__srv__RunTo_Response__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        dobot_msgs_v4__srv__RunTo_Response__fini(&data[i - 1]);
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
dobot_msgs_v4__srv__RunTo_Response__Sequence__fini(dobot_msgs_v4__srv__RunTo_Response__Sequence * array)
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
      dobot_msgs_v4__srv__RunTo_Response__fini(&array->data[i]);
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

dobot_msgs_v4__srv__RunTo_Response__Sequence *
dobot_msgs_v4__srv__RunTo_Response__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  dobot_msgs_v4__srv__RunTo_Response__Sequence * array = (dobot_msgs_v4__srv__RunTo_Response__Sequence *)allocator.allocate(sizeof(dobot_msgs_v4__srv__RunTo_Response__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = dobot_msgs_v4__srv__RunTo_Response__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
dobot_msgs_v4__srv__RunTo_Response__Sequence__destroy(dobot_msgs_v4__srv__RunTo_Response__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    dobot_msgs_v4__srv__RunTo_Response__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
dobot_msgs_v4__srv__RunTo_Response__Sequence__are_equal(const dobot_msgs_v4__srv__RunTo_Response__Sequence * lhs, const dobot_msgs_v4__srv__RunTo_Response__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!dobot_msgs_v4__srv__RunTo_Response__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
dobot_msgs_v4__srv__RunTo_Response__Sequence__copy(
  const dobot_msgs_v4__srv__RunTo_Response__Sequence * input,
  dobot_msgs_v4__srv__RunTo_Response__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(dobot_msgs_v4__srv__RunTo_Response);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    dobot_msgs_v4__srv__RunTo_Response * data =
      (dobot_msgs_v4__srv__RunTo_Response *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!dobot_msgs_v4__srv__RunTo_Response__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          dobot_msgs_v4__srv__RunTo_Response__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!dobot_msgs_v4__srv__RunTo_Response__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
