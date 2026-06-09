// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ariac_interfaces:msg/CellDefect.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__CELL_DEFECT__STRUCT_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__CELL_DEFECT__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__msg__CellDefect __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__msg__CellDefect __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct CellDefect_
{
  using Type = CellDefect_<ContainerAllocator>;

  explicit CellDefect_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->defect_type = 0;
      this->theta = 0.0;
      this->z = 0.0;
    }
  }

  explicit CellDefect_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->defect_type = 0;
      this->theta = 0.0;
      this->z = 0.0;
    }
  }

  // field types and members
  using _defect_type_type =
    uint8_t;
  _defect_type_type defect_type;
  using _theta_type =
    double;
  _theta_type theta;
  using _z_type =
    double;
  _z_type z;

  // setters for named parameter idiom
  Type & set__defect_type(
    const uint8_t & _arg)
  {
    this->defect_type = _arg;
    return *this;
  }
  Type & set__theta(
    const double & _arg)
  {
    this->theta = _arg;
    return *this;
  }
  Type & set__z(
    const double & _arg)
  {
    this->z = _arg;
    return *this;
  }

  // constant declarations
  static constexpr uint8_t DENT =
    1u;
  static constexpr uint8_t BULGE =
    2u;
  static constexpr uint8_t SCRATCH =
    3u;

  // pointer types
  using RawPtr =
    ariac_interfaces::msg::CellDefect_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::msg::CellDefect_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::msg::CellDefect_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::msg::CellDefect_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::CellDefect_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::CellDefect_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::CellDefect_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::CellDefect_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::msg::CellDefect_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::msg::CellDefect_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__msg__CellDefect
    std::shared_ptr<ariac_interfaces::msg::CellDefect_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__msg__CellDefect
    std::shared_ptr<ariac_interfaces::msg::CellDefect_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CellDefect_ & other) const
  {
    if (this->defect_type != other.defect_type) {
      return false;
    }
    if (this->theta != other.theta) {
      return false;
    }
    if (this->z != other.z) {
      return false;
    }
    return true;
  }
  bool operator!=(const CellDefect_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CellDefect_

// alias to use template instance with default allocator
using CellDefect =
  ariac_interfaces::msg::CellDefect_<std::allocator<void>>;

// constant definitions
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t CellDefect_<ContainerAllocator>::DENT;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t CellDefect_<ContainerAllocator>::BULGE;
#endif  // __cplusplus < 201703L
#if __cplusplus < 201703L
// static constexpr member variable definitions are only needed in C++14 and below, deprecated in C++17
template<typename ContainerAllocator>
constexpr uint8_t CellDefect_<ContainerAllocator>::SCRATCH;
#endif  // __cplusplus < 201703L

}  // namespace msg

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__CELL_DEFECT__STRUCT_HPP_
