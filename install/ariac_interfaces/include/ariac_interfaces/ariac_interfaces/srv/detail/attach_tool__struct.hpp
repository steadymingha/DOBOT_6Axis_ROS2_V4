// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ariac_interfaces:srv/AttachTool.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__ATTACH_TOOL__STRUCT_HPP_
#define ARIAC_INTERFACES__SRV__DETAIL__ATTACH_TOOL__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__srv__AttachTool_Request __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__srv__AttachTool_Request __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct AttachTool_Request_
{
  using Type = AttachTool_Request_<ContainerAllocator>;

  explicit AttachTool_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->tool = 0;
    }
  }

  explicit AttachTool_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->tool = 0;
    }
  }

  // field types and members
  using _tool_type =
    int8_t;
  _tool_type tool;

  // setters for named parameter idiom
  Type & set__tool(
    const int8_t & _arg)
  {
    this->tool = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ariac_interfaces::srv::AttachTool_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::srv::AttachTool_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::srv::AttachTool_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::srv::AttachTool_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::srv::AttachTool_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::srv::AttachTool_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::srv::AttachTool_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::srv::AttachTool_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::srv::AttachTool_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::srv::AttachTool_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__srv__AttachTool_Request
    std::shared_ptr<ariac_interfaces::srv::AttachTool_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__srv__AttachTool_Request
    std::shared_ptr<ariac_interfaces::srv::AttachTool_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const AttachTool_Request_ & other) const
  {
    if (this->tool != other.tool) {
      return false;
    }
    return true;
  }
  bool operator!=(const AttachTool_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct AttachTool_Request_

// alias to use template instance with default allocator
using AttachTool_Request =
  ariac_interfaces::srv::AttachTool_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace ariac_interfaces


#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__srv__AttachTool_Response __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__srv__AttachTool_Response __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct AttachTool_Response_
{
  using Type = AttachTool_Response_<ContainerAllocator>;

  explicit AttachTool_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
    }
  }

  explicit AttachTool_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : message(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
    }
  }

  // field types and members
  using _success_type =
    bool;
  _success_type success;
  using _message_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _message_type message;

  // setters for named parameter idiom
  Type & set__success(
    const bool & _arg)
  {
    this->success = _arg;
    return *this;
  }
  Type & set__message(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->message = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ariac_interfaces::srv::AttachTool_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::srv::AttachTool_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::srv::AttachTool_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::srv::AttachTool_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::srv::AttachTool_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::srv::AttachTool_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::srv::AttachTool_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::srv::AttachTool_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::srv::AttachTool_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::srv::AttachTool_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__srv__AttachTool_Response
    std::shared_ptr<ariac_interfaces::srv::AttachTool_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__srv__AttachTool_Response
    std::shared_ptr<ariac_interfaces::srv::AttachTool_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const AttachTool_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->message != other.message) {
      return false;
    }
    return true;
  }
  bool operator!=(const AttachTool_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct AttachTool_Response_

// alias to use template instance with default allocator
using AttachTool_Response =
  ariac_interfaces::srv::AttachTool_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace ariac_interfaces

namespace ariac_interfaces
{

namespace srv
{

struct AttachTool
{
  using Request = ariac_interfaces::srv::AttachTool_Request;
  using Response = ariac_interfaces::srv::AttachTool_Response;
};

}  // namespace srv

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__SRV__DETAIL__ATTACH_TOOL__STRUCT_HPP_
