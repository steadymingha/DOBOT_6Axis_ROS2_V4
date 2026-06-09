// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from ariac_interfaces:srv/SubmitInspectionReport.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_INSPECTION_REPORT__STRUCT_HPP_
#define ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_INSPECTION_REPORT__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


// Include directives for member types
// Member 'timestamp'
#include "builtin_interfaces/msg/detail/time__struct.hpp"
// Member 'report'
#include "ariac_interfaces/msg/detail/inspection_report__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__srv__SubmitInspectionReport_Request __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__srv__SubmitInspectionReport_Request __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SubmitInspectionReport_Request_
{
  using Type = SubmitInspectionReport_Request_<ContainerAllocator>;

  explicit SubmitInspectionReport_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : timestamp(_init),
    report(_init)
  {
    (void)_init;
  }

  explicit SubmitInspectionReport_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : timestamp(_alloc, _init),
    report(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _timestamp_type =
    builtin_interfaces::msg::Time_<ContainerAllocator>;
  _timestamp_type timestamp;
  using _report_type =
    ariac_interfaces::msg::InspectionReport_<ContainerAllocator>;
  _report_type report;

  // setters for named parameter idiom
  Type & set__timestamp(
    const builtin_interfaces::msg::Time_<ContainerAllocator> & _arg)
  {
    this->timestamp = _arg;
    return *this;
  }
  Type & set__report(
    const ariac_interfaces::msg::InspectionReport_<ContainerAllocator> & _arg)
  {
    this->report = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    ariac_interfaces::srv::SubmitInspectionReport_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::srv::SubmitInspectionReport_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::srv::SubmitInspectionReport_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::srv::SubmitInspectionReport_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::srv::SubmitInspectionReport_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::srv::SubmitInspectionReport_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::srv::SubmitInspectionReport_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::srv::SubmitInspectionReport_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::srv::SubmitInspectionReport_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::srv::SubmitInspectionReport_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__srv__SubmitInspectionReport_Request
    std::shared_ptr<ariac_interfaces::srv::SubmitInspectionReport_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__srv__SubmitInspectionReport_Request
    std::shared_ptr<ariac_interfaces::srv::SubmitInspectionReport_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SubmitInspectionReport_Request_ & other) const
  {
    if (this->timestamp != other.timestamp) {
      return false;
    }
    if (this->report != other.report) {
      return false;
    }
    return true;
  }
  bool operator!=(const SubmitInspectionReport_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SubmitInspectionReport_Request_

// alias to use template instance with default allocator
using SubmitInspectionReport_Request =
  ariac_interfaces::srv::SubmitInspectionReport_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace ariac_interfaces


#ifndef _WIN32
# define DEPRECATED__ariac_interfaces__srv__SubmitInspectionReport_Response __attribute__((deprecated))
#else
# define DEPRECATED__ariac_interfaces__srv__SubmitInspectionReport_Response __declspec(deprecated)
#endif

namespace ariac_interfaces
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SubmitInspectionReport_Response_
{
  using Type = SubmitInspectionReport_Response_<ContainerAllocator>;

  explicit SubmitInspectionReport_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->message = "";
    }
  }

  explicit SubmitInspectionReport_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    ariac_interfaces::srv::SubmitInspectionReport_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const ariac_interfaces::srv::SubmitInspectionReport_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<ariac_interfaces::srv::SubmitInspectionReport_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<ariac_interfaces::srv::SubmitInspectionReport_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::srv::SubmitInspectionReport_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::srv::SubmitInspectionReport_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      ariac_interfaces::srv::SubmitInspectionReport_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<ariac_interfaces::srv::SubmitInspectionReport_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<ariac_interfaces::srv::SubmitInspectionReport_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<ariac_interfaces::srv::SubmitInspectionReport_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__ariac_interfaces__srv__SubmitInspectionReport_Response
    std::shared_ptr<ariac_interfaces::srv::SubmitInspectionReport_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__ariac_interfaces__srv__SubmitInspectionReport_Response
    std::shared_ptr<ariac_interfaces::srv::SubmitInspectionReport_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SubmitInspectionReport_Response_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->message != other.message) {
      return false;
    }
    return true;
  }
  bool operator!=(const SubmitInspectionReport_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SubmitInspectionReport_Response_

// alias to use template instance with default allocator
using SubmitInspectionReport_Response =
  ariac_interfaces::srv::SubmitInspectionReport_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace ariac_interfaces

namespace ariac_interfaces
{

namespace srv
{

struct SubmitInspectionReport
{
  using Request = ariac_interfaces::srv::SubmitInspectionReport_Request;
  using Response = ariac_interfaces::srv::SubmitInspectionReport_Response;
};

}  // namespace srv

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_INSPECTION_REPORT__STRUCT_HPP_
