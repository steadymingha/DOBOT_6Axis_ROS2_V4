// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ariac_interfaces:msg/InspectionReport.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__INSPECTION_REPORT__STRUCT_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__INSPECTION_REPORT__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'defects'
#include "ariac_interfaces/msg/detail/cell_defect__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__msg__InspectionReport __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__msg__InspectionReport __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct InspectionReport_
{
  using Type = InspectionReport_<ContainerAllocator>;

  explicit InspectionReport_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->passed = false;
    }
  }

  explicit InspectionReport_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->passed = false;
    }
  }

  // field types and members
  using _passed_type =
    bool;
  _passed_type passed;
  using _defects_type =
    std::vector<ariac_interfaces::msg::CellDefect_<ContainerAllocator>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<ariac_interfaces::msg::CellDefect_<ContainerAllocator>>>;
  _defects_type defects;

  // setters for named parameter idiom
  Type & set__passed(
    const bool & _arg)
  {
    this->passed = _arg;
    return *this;
  }
  Type & set__defects(
    const std::vector<ariac_interfaces::msg::CellDefect_<ContainerAllocator>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<ariac_interfaces::msg::CellDefect_<ContainerAllocator>>> & _arg)
  {
    this->defects = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ariac_interfaces::msg::InspectionReport_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::msg::InspectionReport_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::msg::InspectionReport_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::msg::InspectionReport_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::InspectionReport_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::InspectionReport_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::InspectionReport_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::InspectionReport_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::msg::InspectionReport_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::msg::InspectionReport_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__msg__InspectionReport
    std::shared_ptr<ariac_interfaces::msg::InspectionReport_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__msg__InspectionReport
    std::shared_ptr<ariac_interfaces::msg::InspectionReport_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const InspectionReport_ & other) const
  {
    if (this->passed != other.passed) {
      return false;
    }
    if (this->defects != other.defects) {
      return false;
    }
    return true;
  }
  bool operator!=(const InspectionReport_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct InspectionReport_

// alias to use template instance with default allocator
using InspectionReport =
  ariac_interfaces::msg::InspectionReport_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__INSPECTION_REPORT__STRUCT_HPP_
