// generated from rosidl_generator_c/resource/idl__functions.h.em
// with input from ariac_interfaces:msg/CompetitionStates.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATES__FUNCTIONS_H_
#define ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATES__FUNCTIONS_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stdlib.h>

#include "rosidl_runtime_c/visibility_control.h"
#include "ariac_interfaces/msg/rosidl_generator_c__visibility_control.h"

#include "ariac_interfaces/msg/detail/competition_states__struct.h"

/// Initialize msg/CompetitionStates message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * ariac_interfaces__msg__CompetitionStates
 * )) before or use
 * ariac_interfaces__msg__CompetitionStates__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
bool
ariac_interfaces__msg__CompetitionStates__init(ariac_interfaces__msg__CompetitionStates * msg);

/// Finalize msg/CompetitionStates message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
void
ariac_interfaces__msg__CompetitionStates__fini(ariac_interfaces__msg__CompetitionStates * msg);

/// Create msg/CompetitionStates message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * ariac_interfaces__msg__CompetitionStates__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
ariac_interfaces__msg__CompetitionStates *
ariac_interfaces__msg__CompetitionStates__create();

/// Destroy msg/CompetitionStates message.
/**
 * It calls
 * ariac_interfaces__msg__CompetitionStates__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
void
ariac_interfaces__msg__CompetitionStates__destroy(ariac_interfaces__msg__CompetitionStates * msg);

/// Check for msg/CompetitionStates message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
bool
ariac_interfaces__msg__CompetitionStates__are_equal(const ariac_interfaces__msg__CompetitionStates * lhs, const ariac_interfaces__msg__CompetitionStates * rhs);

/// Copy a msg/CompetitionStates message.
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
ariac_interfaces__msg__CompetitionStates__copy(
  const ariac_interfaces__msg__CompetitionStates * input,
  ariac_interfaces__msg__CompetitionStates * output);

/// Initialize array of msg/CompetitionStates messages.
/**
 * It allocates the memory for the number of elements and calls
 * ariac_interfaces__msg__CompetitionStates__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
bool
ariac_interfaces__msg__CompetitionStates__Sequence__init(ariac_interfaces__msg__CompetitionStates__Sequence * array, size_t size);

/// Finalize array of msg/CompetitionStates messages.
/**
 * It calls
 * ariac_interfaces__msg__CompetitionStates__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
void
ariac_interfaces__msg__CompetitionStates__Sequence__fini(ariac_interfaces__msg__CompetitionStates__Sequence * array);

/// Create array of msg/CompetitionStates messages.
/**
 * It allocates the memory for the array and calls
 * ariac_interfaces__msg__CompetitionStates__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
ariac_interfaces__msg__CompetitionStates__Sequence *
ariac_interfaces__msg__CompetitionStates__Sequence__create(size_t size);

/// Destroy array of msg/CompetitionStates messages.
/**
 * It calls
 * ariac_interfaces__msg__CompetitionStates__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
void
ariac_interfaces__msg__CompetitionStates__Sequence__destroy(ariac_interfaces__msg__CompetitionStates__Sequence * array);

/// Check for msg/CompetitionStates message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_ariac_interfaces
bool
ariac_interfaces__msg__CompetitionStates__Sequence__are_equal(const ariac_interfaces__msg__CompetitionStates__Sequence * lhs, const ariac_interfaces__msg__CompetitionStates__Sequence * rhs);

/// Copy an array of msg/CompetitionStates messages.
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
ariac_interfaces__msg__CompetitionStates__Sequence__copy(
  const ariac_interfaces__msg__CompetitionStates__Sequence * input,
  ariac_interfaces__msg__CompetitionStates__Sequence * output);

#ifdef __cplusplus
}
#endif

#endif  // ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATES__FUNCTIONS_H_
