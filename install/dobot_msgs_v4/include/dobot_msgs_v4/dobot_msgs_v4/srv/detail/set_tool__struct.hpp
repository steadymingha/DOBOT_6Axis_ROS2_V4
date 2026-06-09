// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from dobot_msgs_v4:srv/SetTool.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__SET_TOOL__STRUCT_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__SET_TOOL__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__dobot_msgs_v4__srv__SetTool_Request __attribute__((deprecated))
#else
# define DEPRECATED__dobot_msgs_v4__srv__SetTool_Request __declspec(deprecated)
#endif

namespace dobot_msgs_v4
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SetTool_Request_
{
  using Type = SetTool_Request_<ContainerAllocator>;

  explicit SetTool_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->index = 0l;
      this->value = "";
    }
  }

  explicit SetTool_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : value(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->index = 0l;
      this->value = "";
    }
  }

  // field types and members
  using _index_type =
    int32_t;
  _index_type index;
  using _value_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _value_type value;

  // setters for named parameter idiom
  Type & set__index(
    const int32_t & _arg)
  {
    this->index = _arg;
    return *this;
  }
  Type & set__value(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->value = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    dobot_msgs_v4::srv::SetTool_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const dobot_msgs_v4::srv::SetTool_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::SetTool_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::SetTool_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::SetTool_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::SetTool_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::SetTool_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::SetTool_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::SetTool_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::SetTool_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__dobot_msgs_v4__srv__SetTool_Request
    std::shared_ptr<dobot_msgs_v4::srv::SetTool_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__dobot_msgs_v4__srv__SetTool_Request
    std::shared_ptr<dobot_msgs_v4::srv::SetTool_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SetTool_Request_ & other) const
  {
    if (this->index != other.index) {
      return false;
    }
    if (this->value != other.value) {
      return false;
    }
    return true;
  }
  bool operator!=(const SetTool_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SetTool_Request_

// alias to use template instance with default allocator
using SetTool_Request =
  dobot_msgs_v4::srv::SetTool_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace dobot_msgs_v4


#ifndef _WIN32
# define DEPRECATED__dobot_msgs_v4__srv__SetTool_Response __attribute__((deprecated))
#else
# define DEPRECATED__dobot_msgs_v4__srv__SetTool_Response __declspec(deprecated)
#endif

namespace dobot_msgs_v4
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct SetTool_Response_
{
  using Type = SetTool_Response_<ContainerAllocator>;

  explicit SetTool_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->res = 0l;
    }
  }

  explicit SetTool_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->res = 0l;
    }
  }

  // field types and members
  using _res_type =
    int32_t;
  _res_type res;

  // setters for named parameter idiom
  Type & set__res(
    const int32_t & _arg)
  {
    this->res = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    dobot_msgs_v4::srv::SetTool_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const dobot_msgs_v4::srv::SetTool_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::SetTool_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::SetTool_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::SetTool_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::SetTool_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::SetTool_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::SetTool_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::SetTool_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::SetTool_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__dobot_msgs_v4__srv__SetTool_Response
    std::shared_ptr<dobot_msgs_v4::srv::SetTool_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__dobot_msgs_v4__srv__SetTool_Response
    std::shared_ptr<dobot_msgs_v4::srv::SetTool_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const SetTool_Response_ & other) const
  {
    if (this->res != other.res) {
      return false;
    }
    return true;
  }
  bool operator!=(const SetTool_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct SetTool_Response_

// alias to use template instance with default allocator
using SetTool_Response =
  dobot_msgs_v4::srv::SetTool_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace dobot_msgs_v4

namespace dobot_msgs_v4
{

namespace srv
{

struct SetTool
{
  using Request = dobot_msgs_v4::srv::SetTool_Request;
  using Response = dobot_msgs_v4::srv::SetTool_Response;
};

}  // namespace srv

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__SET_TOOL__STRUCT_HPP_
