// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ariac_interfaces:msg/AgvStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__AGV_STATUS__STRUCT_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__AGV_STATUS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'pose'
#include "geometry_msgs/msg/detail/pose__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__msg__AgvStatus __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__msg__AgvStatus __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct AgvStatus_
{
  using Type = AgvStatus_<ContainerAllocator>;

  explicit AgvStatus_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : pose(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->station_id = 0;
    }
  }

  explicit AgvStatus_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : pose(_alloc, _init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->station_id = 0;
    }
  }

  // field types and members
  using _station_id_type =
    int8_t;
  _station_id_type station_id;
  using _pose_type =
    geometry_msgs::msg::Pose_<ContainerAllocator>;
  _pose_type pose;

  // setters for named parameter idiom
  Type & set__station_id(
    const int8_t & _arg)
  {
    this->station_id = _arg;
    return *this;
  }
  Type & set__pose(
    const geometry_msgs::msg::Pose_<ContainerAllocator> & _arg)
  {
    this->pose = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ariac_interfaces::msg::AgvStatus_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::msg::AgvStatus_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::msg::AgvStatus_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::msg::AgvStatus_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::AgvStatus_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::AgvStatus_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::AgvStatus_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::AgvStatus_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::msg::AgvStatus_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::msg::AgvStatus_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__msg__AgvStatus
    std::shared_ptr<ariac_interfaces::msg::AgvStatus_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__msg__AgvStatus
    std::shared_ptr<ariac_interfaces::msg::AgvStatus_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const AgvStatus_ & other) const
  {
    if (this->station_id != other.station_id) {
      return false;
    }
    if (this->pose != other.pose) {
      return false;
    }
    return true;
  }
  bool operator!=(const AgvStatus_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct AgvStatus_

// alias to use template instance with default allocator
using AgvStatus =
  ariac_interfaces::msg::AgvStatus_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__AGV_STATUS__STRUCT_HPP_
