// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ariac_interfaces:msg/ToolChangerStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__TOOL_CHANGER_STATUS__STRUCT_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__TOOL_CHANGER_STATUS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__msg__ToolChangerStatus __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__msg__ToolChangerStatus __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct ToolChangerStatus_
{
  using Type = ToolChangerStatus_<ContainerAllocator>;

  explicit ToolChangerStatus_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->attached_tool = 0;
    }
  }

  explicit ToolChangerStatus_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->attached_tool = 0;
    }
  }

  // field types and members
  using _attached_tool_type =
    int8_t;
  _attached_tool_type attached_tool;

  // setters for named parameter idiom
  Type & set__attached_tool(
    const int8_t & _arg)
  {
    this->attached_tool = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ariac_interfaces::msg::ToolChangerStatus_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::msg::ToolChangerStatus_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::msg::ToolChangerStatus_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::msg::ToolChangerStatus_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::ToolChangerStatus_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::ToolChangerStatus_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::ToolChangerStatus_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::ToolChangerStatus_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::msg::ToolChangerStatus_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::msg::ToolChangerStatus_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__msg__ToolChangerStatus
    std::shared_ptr<ariac_interfaces::msg::ToolChangerStatus_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__msg__ToolChangerStatus
    std::shared_ptr<ariac_interfaces::msg::ToolChangerStatus_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ToolChangerStatus_ & other) const
  {
    if (this->attached_tool != other.attached_tool) {
      return false;
    }
    return true;
  }
  bool operator!=(const ToolChangerStatus_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ToolChangerStatus_

// alias to use template instance with default allocator
using ToolChangerStatus =
  ariac_interfaces::msg::ToolChangerStatus_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__TOOL_CHANGER_STATUS__STRUCT_HPP_
