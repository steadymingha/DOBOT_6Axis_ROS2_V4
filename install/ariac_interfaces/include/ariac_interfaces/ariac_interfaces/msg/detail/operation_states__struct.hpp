// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ariac_interfaces:msg/OperationStates.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__OPERATION_STATES__STRUCT_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__OPERATION_STATES__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__msg__OperationStates __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__msg__OperationStates __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct OperationStates_
{
  using Type = OperationStates_<ContainerAllocator>;

  explicit OperationStates_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->structure_needs_at_least_one_member = 0;
    }
  }

  explicit OperationStates_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->structure_needs_at_least_one_member = 0;
    }
  }

  // field types and members
  using _structure_needs_at_least_one_member_type =
    uint8_t;
  _structure_needs_at_least_one_member_type structure_needs_at_least_one_member;


  // constant declarations
  static constexpr uint8_t OPERATIONAL =
    1u;
  static constexpr uint8_t MALFUNCTIONING =
    2u;

  // pointer types
  using RawPtr =
    ariac_interfaces::msg::OperationStates_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::msg::OperationStates_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::msg::OperationStates_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::msg::OperationStates_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::OperationStates_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::OperationStates_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::OperationStates_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::OperationStates_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::msg::OperationStates_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::msg::OperationStates_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__msg__OperationStates
    std::shared_ptr<ariac_interfaces::msg::OperationStates_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__msg__OperationStates
    std::shared_ptr<ariac_interfaces::msg::OperationStates_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const OperationStates_ & other) const
  {
    if (this->structure_needs_at_least_one_member != other.structure_needs_at_least_one_member) {
      return false;
    }
    return true;
  }
  bool operator!=(const OperationStates_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct OperationStates_

// alias to use template instance with default allocator
using OperationStates =
  ariac_interfaces::msg::OperationStates_<std::allocator<void>>;

// constant definitions
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t OperationStates_<ContainerAllocator>::OPERATIONAL;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t OperationStates_<ContainerAllocator>::MALFUNCTIONING;
#endif  // __cplusplus < 201703L

}  // namespace msg

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__OPERATION_STATES__STRUCT_HPP_
