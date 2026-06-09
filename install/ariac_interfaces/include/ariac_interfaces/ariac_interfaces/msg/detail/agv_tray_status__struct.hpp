// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ariac_interfaces:msg/AgvTrayStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__AGV_TRAY_STATUS__STRUCT_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__AGV_TRAY_STATUS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__msg__AgvTrayStatus __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__msg__AgvTrayStatus __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct AgvTrayStatus_
{
  using Type = AgvTrayStatus_<ContainerAllocator>;

  explicit AgvTrayStatus_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->slot_1_occupied = false;
      this->slot_2_occupied = false;
      this->slot_3_occupied = false;
      this->slot_4_occupied = false;
    }
  }

  explicit AgvTrayStatus_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->slot_1_occupied = false;
      this->slot_2_occupied = false;
      this->slot_3_occupied = false;
      this->slot_4_occupied = false;
    }
  }

  // field types and members
  using _slot_1_occupied_type =
    bool;
  _slot_1_occupied_type slot_1_occupied;
  using _slot_2_occupied_type =
    bool;
  _slot_2_occupied_type slot_2_occupied;
  using _slot_3_occupied_type =
    bool;
  _slot_3_occupied_type slot_3_occupied;
  using _slot_4_occupied_type =
    bool;
  _slot_4_occupied_type slot_4_occupied;

  // setters for named parameter idiom
  Type & set__slot_1_occupied(
    const bool & _arg)
  {
    this->slot_1_occupied = _arg;
    return *this;
  }
  Type & set__slot_2_occupied(
    const bool & _arg)
  {
    this->slot_2_occupied = _arg;
    return *this;
  }
  Type & set__slot_3_occupied(
    const bool & _arg)
  {
    this->slot_3_occupied = _arg;
    return *this;
  }
  Type & set__slot_4_occupied(
    const bool & _arg)
  {
    this->slot_4_occupied = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ariac_interfaces::msg::AgvTrayStatus_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::msg::AgvTrayStatus_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::msg::AgvTrayStatus_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::msg::AgvTrayStatus_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::AgvTrayStatus_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::AgvTrayStatus_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::AgvTrayStatus_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::AgvTrayStatus_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::msg::AgvTrayStatus_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::msg::AgvTrayStatus_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__msg__AgvTrayStatus
    std::shared_ptr<ariac_interfaces::msg::AgvTrayStatus_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__msg__AgvTrayStatus
    std::shared_ptr<ariac_interfaces::msg::AgvTrayStatus_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const AgvTrayStatus_ & other) const
  {
    if (this->slot_1_occupied != other.slot_1_occupied) {
      return false;
    }
    if (this->slot_2_occupied != other.slot_2_occupied) {
      return false;
    }
    if (this->slot_3_occupied != other.slot_3_occupied) {
      return false;
    }
    if (this->slot_4_occupied != other.slot_4_occupied) {
      return false;
    }
    return true;
  }
  bool operator!=(const AgvTrayStatus_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct AgvTrayStatus_

// alias to use template instance with default allocator
using AgvTrayStatus =
  ariac_interfaces::msg::AgvTrayStatus_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__AGV_TRAY_STATUS__STRUCT_HPP_
