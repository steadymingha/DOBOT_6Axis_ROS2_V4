// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ariac_interfaces:msg/VoltageReading.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__VOLTAGE_READING__STRUCT_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__VOLTAGE_READING__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__msg__VoltageReading __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__msg__VoltageReading __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct VoltageReading_
{
  using Type = VoltageReading_<ContainerAllocator>;

  explicit VoltageReading_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->voltage = 0.0;
      this->operation_status = 0;
    }
  }

  explicit VoltageReading_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->voltage = 0.0;
      this->operation_status = 0;
    }
  }

  // field types and members
  using _voltage_type =
    double;
  _voltage_type voltage;
  using _operation_status_type =
    uint8_t;
  _operation_status_type operation_status;

  // setters for named parameter idiom
  Type & set__voltage(
    const double & _arg)
  {
    this->voltage = _arg;
    return *this;
  }
  Type & set__operation_status(
    const uint8_t & _arg)
  {
    this->operation_status = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ariac_interfaces::msg::VoltageReading_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::msg::VoltageReading_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::msg::VoltageReading_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::msg::VoltageReading_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::VoltageReading_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::VoltageReading_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::VoltageReading_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::VoltageReading_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::msg::VoltageReading_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::msg::VoltageReading_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__msg__VoltageReading
    std::shared_ptr<ariac_interfaces::msg::VoltageReading_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__msg__VoltageReading
    std::shared_ptr<ariac_interfaces::msg::VoltageReading_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const VoltageReading_ & other) const
  {
    if (this->voltage != other.voltage) {
      return false;
    }
    if (this->operation_status != other.operation_status) {
      return false;
    }
    return true;
  }
  bool operator!=(const VoltageReading_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct VoltageReading_

// alias to use template instance with default allocator
using VoltageReading =
  ariac_interfaces::msg::VoltageReading_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__VOLTAGE_READING__STRUCT_HPP_
