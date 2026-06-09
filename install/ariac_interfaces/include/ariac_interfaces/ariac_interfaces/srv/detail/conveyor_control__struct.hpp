// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ariac_interfaces:srv/ConveyorControl.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__CONVEYOR_CONTROL__STRUCT_HPP_
#define ARIAC_INTERFACES__SRV__DETAIL__CONVEYOR_CONTROL__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__srv__ConveyorControl_Request __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__srv__ConveyorControl_Request __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct ConveyorControl_Request_
{
  using Type = ConveyorControl_Request_<ContainerAllocator>;

  explicit ConveyorControl_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->speed = 0.0;
    }
  }

  explicit ConveyorControl_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->speed = 0.0;
    }
  }

  // field types and members
  using _speed_type =
    double;
  _speed_type speed;

  // setters for named parameter idiom
  Type & set__speed(
    const double & _arg)
  {
    this->speed = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ariac_interfaces::srv::ConveyorControl_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::srv::ConveyorControl_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::srv::ConveyorControl_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::srv::ConveyorControl_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::srv::ConveyorControl_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::srv::ConveyorControl_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::srv::ConveyorControl_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::srv::ConveyorControl_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::srv::ConveyorControl_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::srv::ConveyorControl_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__srv__ConveyorControl_Request
    std::shared_ptr<ariac_interfaces::srv::ConveyorControl_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__srv__ConveyorControl_Request
    std::shared_ptr<ariac_interfaces::srv::ConveyorControl_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ConveyorControl_Request_ & other) const
  {
    if (this->speed != other.speed) {
      return false;
    }
    return true;
  }
  bool operator!=(const ConveyorControl_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ConveyorControl_Request_

// alias to use template instance with default allocator
using ConveyorControl_Request =
  ariac_interfaces::srv::ConveyorControl_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace ariac_interfaces


#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__srv__ConveyorControl_Response __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__srv__ConveyorControl_Response __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct ConveyorControl_Response_
{
  using Type = ConveyorControl_Response_<ContainerAllocator>;

  explicit ConveyorControl_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
    }
  }

  explicit ConveyorControl_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
    }
  }

  // field types and members
  using _success_type =
    bool;
  _success_type success;

  // setters for named parameter idiom
  Type & set__success(
    const bool & _arg)
  {
    this->success = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ariac_interfaces::srv::ConveyorControl_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::srv::ConveyorControl_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::srv::ConveyorControl_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::srv::ConveyorControl_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::srv::ConveyorControl_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::srv::ConveyorControl_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::srv::ConveyorControl_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::srv::ConveyorControl_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::srv::ConveyorControl_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::srv::ConveyorControl_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__srv__ConveyorControl_Response
    std::shared_ptr<ariac_interfaces::srv::ConveyorControl_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__srv__ConveyorControl_Response
    std::shared_ptr<ariac_interfaces::srv::ConveyorControl_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ConveyorControl_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    return true;
  }
  bool operator!=(const ConveyorControl_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ConveyorControl_Response_

// alias to use template instance with default allocator
using ConveyorControl_Response =
  ariac_interfaces::srv::ConveyorControl_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace ariac_interfaces

namespace ariac_interfaces
{

namespace srv
{

struct ConveyorControl
{
  using Request = ariac_interfaces::srv::ConveyorControl_Request;
  using Response = ariac_interfaces::srv::ConveyorControl_Response;
};

}  // namespace srv

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__SRV__DETAIL__CONVEYOR_CONTROL__STRUCT_HPP_
