// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ariac_interfaces:msg/ConveyorStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__STRUCT_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__msg__ConveyorStatus __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__msg__ConveyorStatus __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct ConveyorStatus_
{
  using Type = ConveyorStatus_<ContainerAllocator>;

  explicit ConveyorStatus_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->direction = 0;
      this->speed = 0.0;
      this->operating_status = 0;
    }
  }

  explicit ConveyorStatus_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->direction = 0;
      this->speed = 0.0;
      this->operating_status = 0;
    }
  }

  // field types and members
  using _direction_type =
    uint8_t;
  _direction_type direction;
  using _speed_type =
    double;
  _speed_type speed;
  using _operating_status_type =
    uint8_t;
  _operating_status_type operating_status;

  // setters for named parameter idiom
  Type & set__direction(
    const uint8_t & _arg)
  {
    this->direction = _arg;
    return *this;
  }
  Type & set__speed(
    const double & _arg)
  {
    this->speed = _arg;
    return *this;
  }
  Type & set__operating_status(
    const uint8_t & _arg)
  {
    this->operating_status = _arg;
    return *this;
  }

  // constant declarations
  static constexpr uint8_t FORWARD =
    0u;
  static constexpr uint8_t BACKWARD =
    1u;

  // pointer types
  using RawPtr =
    ariac_interfaces::msg::ConveyorStatus_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::msg::ConveyorStatus_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::msg::ConveyorStatus_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::msg::ConveyorStatus_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::ConveyorStatus_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::ConveyorStatus_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::ConveyorStatus_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::ConveyorStatus_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::msg::ConveyorStatus_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::msg::ConveyorStatus_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__msg__ConveyorStatus
    std::shared_ptr<ariac_interfaces::msg::ConveyorStatus_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__msg__ConveyorStatus
    std::shared_ptr<ariac_interfaces::msg::ConveyorStatus_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ConveyorStatus_ & other) const
  {
    if (this->direction != other.direction) {
      return false;
    }
    if (this->speed != other.speed) {
      return false;
    }
    if (this->operating_status != other.operating_status) {
      return false;
    }
    return true;
  }
  bool operator!=(const ConveyorStatus_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ConveyorStatus_

// alias to use template instance with default allocator
using ConveyorStatus =
  ariac_interfaces::msg::ConveyorStatus_<std::allocator<void>>;

// constant definitions
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t ConveyorStatus_<ContainerAllocator>::FORWARD;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t ConveyorStatus_<ContainerAllocator>::BACKWARD;
#endif  // __cplusplus < 201703L

}  // namespace msg

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__CONVEYOR_STATUS__STRUCT_HPP_
