// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ariac_interfaces:msg/CompetitionStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATUS__STRUCT_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATUS__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'time'
#include "ariac_interfaces/msg/detail/competition_time__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__msg__CompetitionStatus __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__msg__CompetitionStatus __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct CompetitionStatus_
{
  using Type = CompetitionStatus_<ContainerAllocator>;

  explicit CompetitionStatus_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : time(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->competition_state = 0;
      this->num_kits = 0;
      this->num_modules = 0;
      this->num_kits_remaining = 0;
      this->num_modules_remaining = 0;
      this->run_id = 0l;
    }
  }

  explicit CompetitionStatus_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : time(_alloc, _init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->competition_state = 0;
      this->num_kits = 0;
      this->num_modules = 0;
      this->num_kits_remaining = 0;
      this->num_modules_remaining = 0;
      this->run_id = 0l;
    }
  }

  // field types and members
  using _competition_state_type =
    uint8_t;
  _competition_state_type competition_state;
  using _num_kits_type =
    uint8_t;
  _num_kits_type num_kits;
  using _num_modules_type =
    uint8_t;
  _num_modules_type num_modules;
  using _num_kits_remaining_type =
    uint8_t;
  _num_kits_remaining_type num_kits_remaining;
  using _num_modules_remaining_type =
    uint8_t;
  _num_modules_remaining_type num_modules_remaining;
  using _time_type =
    ariac_interfaces::msg::CompetitionTime_<ContainerAllocator>;
  _time_type time;
  using _run_id_type =
    int32_t;
  _run_id_type run_id;

  // setters for named parameter idiom
  Type & set__competition_state(
    const uint8_t & _arg)
  {
    this->competition_state = _arg;
    return *this;
  }
  Type & set__num_kits(
    const uint8_t & _arg)
  {
    this->num_kits = _arg;
    return *this;
  }
  Type & set__num_modules(
    const uint8_t & _arg)
  {
    this->num_modules = _arg;
    return *this;
  }
  Type & set__num_kits_remaining(
    const uint8_t & _arg)
  {
    this->num_kits_remaining = _arg;
    return *this;
  }
  Type & set__num_modules_remaining(
    const uint8_t & _arg)
  {
    this->num_modules_remaining = _arg;
    return *this;
  }
  Type & set__time(
    const ariac_interfaces::msg::CompetitionTime_<ContainerAllocator> & _arg)
  {
    this->time = _arg;
    return *this;
  }
  Type & set__run_id(
    const int32_t & _arg)
  {
    this->run_id = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ariac_interfaces::msg::CompetitionStatus_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::msg::CompetitionStatus_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::msg::CompetitionStatus_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::msg::CompetitionStatus_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::CompetitionStatus_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::CompetitionStatus_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::msg::CompetitionStatus_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::msg::CompetitionStatus_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::msg::CompetitionStatus_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::msg::CompetitionStatus_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__msg__CompetitionStatus
    std::shared_ptr<ariac_interfaces::msg::CompetitionStatus_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__msg__CompetitionStatus
    std::shared_ptr<ariac_interfaces::msg::CompetitionStatus_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CompetitionStatus_ & other) const
  {
    if (this->competition_state != other.competition_state) {
      return false;
    }
    if (this->num_kits != other.num_kits) {
      return false;
    }
    if (this->num_modules != other.num_modules) {
      return false;
    }
    if (this->num_kits_remaining != other.num_kits_remaining) {
      return false;
    }
    if (this->num_modules_remaining != other.num_modules_remaining) {
      return false;
    }
    if (this->time != other.time) {
      return false;
    }
    if (this->run_id != other.run_id) {
      return false;
    }
    return true;
  }
  bool operator!=(const CompetitionStatus_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CompetitionStatus_

// alias to use template instance with default allocator
using CompetitionStatus =
  ariac_interfaces::msg::CompetitionStatus_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATUS__STRUCT_HPP_
