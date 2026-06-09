// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ariac_interfaces:msg/CompetitionTime.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_TIME__STRUCT_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_TIME__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'start'
#include "builtin_interfaces/msg/detail/time__struct.hpp"
// Member 'elapsed'
// Member 'remaining'
#include "builtin_interfaces/msg/detail/duration__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__msg__CompetitionTime __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__msg__CompetitionTime __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct CompetitionTime_
{
  using Type = CompetitionTime_<ContainerAllocator>;

  explicit CompetitionTime_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : start(_init),
    elapsed(_init),
    remaining(_init)
  {
    (void)_init;
  }

  explicit CompetitionTime_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : start(_alloc, _init),
    elapsed(_alloc, _init),
    remaining(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _start_type =
    builtin_interfaces::msg::Time_<ContainerAllocator>;
  _start_type start;
  using _elapsed_type =
    builtin_interfaces::msg::Duration_<ContainerAllocator>;
  _elapsed_type elapsed;
  using _remaining_type =
    builtin_interfaces::msg::Duration_<ContainerAllocator>;
  _remaining_type remaining;

  // setters for named parameter idiom
  Type & set__start(
    const builtin_interfaces::msg::Time_<ContainerAllocator> & _arg)
  {
    this->start = _arg;
    return *this;
  }
  Type & set__elapsed(
    const builtin_interfaces::msg::Duration_<ContainerAllocator> & _arg)
  {
    this->elapsed = _arg;
    return *this;
  }
  Type & set__remaining(
    const builtin_interfaces::msg::Duration_<ContainerAllocator> & _arg)
  {
    this->remaining = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ariac_interfaces::msg::CompetitionTime_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::msg::CompetitionTime_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::msg::CompetitionTime_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::msg::CompetitionTime_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::CompetitionTime_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::CompetitionTime_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::CompetitionTime_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::CompetitionTime_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::msg::CompetitionTime_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::msg::CompetitionTime_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__msg__CompetitionTime
    std::shared_ptr<ariac_interfaces::msg::CompetitionTime_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__msg__CompetitionTime
    std::shared_ptr<ariac_interfaces::msg::CompetitionTime_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CompetitionTime_ & other) const
  {
    if (this->start != other.start) {
      return false;
    }
    if (this->elapsed != other.elapsed) {
      return false;
    }
    if (this->remaining != other.remaining) {
      return false;
    }
    return true;
  }
  bool operator!=(const CompetitionTime_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CompetitionTime_

// alias to use template instance with default allocator
using CompetitionTime =
  ariac_interfaces::msg::CompetitionTime_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_TIME__STRUCT_HPP_
