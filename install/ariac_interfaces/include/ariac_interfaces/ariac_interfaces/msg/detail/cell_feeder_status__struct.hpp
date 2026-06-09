// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ariac_interfaces:msg/CellFeederStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__CELL_FEEDER_STATUS__STRUCT_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__CELL_FEEDER_STATUS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__msg__CellFeederStatus __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__msg__CellFeederStatus __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct CellFeederStatus_
{
  using Type = CellFeederStatus_<ContainerAllocator>;

  explicit CellFeederStatus_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->cell_type = 0;
      this->feed_rate = 0.0;
    }
  }

  explicit CellFeederStatus_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->cell_type = 0;
      this->feed_rate = 0.0;
    }
  }

  // field types and members
  using _cell_type_type =
    uint8_t;
  _cell_type_type cell_type;
  using _feed_rate_type =
    double;
  _feed_rate_type feed_rate;

  // setters for named parameter idiom
  Type & set__cell_type(
    const uint8_t & _arg)
  {
    this->cell_type = _arg;
    return *this;
  }
  Type & set__feed_rate(
    const double & _arg)
  {
    this->feed_rate = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ariac_interfaces::msg::CellFeederStatus_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::msg::CellFeederStatus_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::msg::CellFeederStatus_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::msg::CellFeederStatus_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::CellFeederStatus_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::CellFeederStatus_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::CellFeederStatus_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::CellFeederStatus_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::msg::CellFeederStatus_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::msg::CellFeederStatus_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__msg__CellFeederStatus
    std::shared_ptr<ariac_interfaces::msg::CellFeederStatus_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__msg__CellFeederStatus
    std::shared_ptr<ariac_interfaces::msg::CellFeederStatus_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CellFeederStatus_ & other) const
  {
    if (this->cell_type != other.cell_type) {
      return false;
    }
    if (this->feed_rate != other.feed_rate) {
      return false;
    }
    return true;
  }
  bool operator!=(const CellFeederStatus_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CellFeederStatus_

// alias to use template instance with default allocator
using CellFeederStatus =
  ariac_interfaces::msg::CellFeederStatus_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__CELL_FEEDER_STATUS__STRUCT_HPP_
