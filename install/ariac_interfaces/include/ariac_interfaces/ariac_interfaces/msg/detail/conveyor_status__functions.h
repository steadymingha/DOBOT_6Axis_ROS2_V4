// generated from rosidl_generator_c/resource/idl__functions.h.em
// with input from ariac_interfaces:msg/ConveyorStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__FUNCTIONS_H_
#define ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__FUNCTIONS_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stdlib.h>

#include "rosidl_runtime_c/visibility_control.h"
#include "ariac_interfaces/msg/rosidl_generator_c__visibility_control.h"

#include "ariac_interfaces/msg/detail/conveyor_status__struct.h"

/// Initialize msg/ConveyorStatus message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * ariac_interfaces__msg__ConveyorStatus
 * )) before or use
 * ariac_interfaces__msg__ConveyorStatus__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
bool
ariac_interfaces__msg__ConveyorStatus__init(ariac_interfaces__msg__ConveyorStatus * msg);

/// Finalize msg/ConveyorStatus message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
void
ariac_interfaces__msg__ConveyorStatus__fini(ariac_interfaces__msg__ConveyorStatus * msg);

/// Create msg/ConveyorStatus message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * ariac_interfaces__msg__ConveyorStatus__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
ariac_interfaces__msg__ConveyorStatus *
ariac_interfaces__msg__ConveyorStatus__create();

/// Destroy msg/ConveyorStatus message.
/**
 * It calls
 * ariac_interfaces__msg__ConveyorStatus__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
void
ariac_interfaces__msg__ConveyorStatus__destroy(ariac_interfaces__msg__ConveyorStatus * msg);

/// Check for msg/ConveyorStatus message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
bool
ariac_interfaces__msg__ConveyorStatus__are_equal(const ariac_interfaces__msg__ConveyorStatus * lhs, const ariac_interfaces__msg__ConveyorStatus * rhs);

/// Copy a msg/ConveyorStatus message.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source message pointer.
 * \param[out] output The target message pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer is null
 *   or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
bool
ariac_interfaces__msg__ConveyorStatus__copy(
  const ariac_interfaces__msg__ConveyorStatus * input,
  ariac_interfaces__msg__ConveyorStatus * output);

/// Initialize array of msg/ConveyorStatus messages.
/**
 * It allocates the memory for the number of elements and calls
 * ariac_interfaces__msg__ConveyorStatus__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
bool
ariac_interfaces__msg__ConveyorStatus__Sequence__init(ariac_interfaces__msg__ConveyorStatus__Sequence * array, size_t size);

/// Finalize array of msg/ConveyorStatus messages.
/**
 * It calls
 * ariac_interfaces__msg__ConveyorStatus__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
void
ariac_interfaces__msg__ConveyorStatus__Sequence__fini(ariac_interfaces__msg__ConveyorStatus__Sequence * array);

/// Create array of msg/ConveyorStatus messages.
/**
 * It allocates the memory for the array and calls
 * ariac_interfaces__msg__ConveyorStatus__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
ariac_interfaces__msg__ConveyorStatus__Sequence *
ariac_interfaces__msg__ConveyorStatus__Sequence__create(size_t size);

/// Destroy array of msg/ConveyorStatus messages.
/**
 * It calls
 * ariac_interfaces__msg__ConveyorStatus__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
void
ariac_interfaces__msg__ConveyorStatus__Sequence__destroy(ariac_interfaces__msg__ConveyorStatus__Sequence * array);

/// Check for msg/ConveyorStatus message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
bool
ariac_interfaces__msg__ConveyorStatus__Sequence__are_equal(const ariac_interfaces__msg__ConveyorStatus__Sequence * lhs, const ariac_interfaces__msg__ConveyorStatus__Sequence * rhs);

/// Copy an array of msg/ConveyorStatus messages.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source array pointer.
 * \param[out] output The target array pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer
 *   is null or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
bool
ariac_interfaces__msg__ConveyorStatus__Sequence__copy(
  const ariac_interfaces__msg__ConveyorStatus__Sequence * input,
  ariac_interfaces__msg__ConveyorStatus__Sequence * output);

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__FUNCTIONS_H_
