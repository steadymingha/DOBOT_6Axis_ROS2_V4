// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from dobot_msgs_v4:srv/FCForceMode.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__FC_FORCE_MODE__STRUCT_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__FC_FORCE_MODE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__dobot_msgs_v4__srv__FCForceMode_Request __attribute__((deprecated))
#else
# define DEPRECATED__dobot_msgs_v4__srv__FCForceMode_Request __declspec(deprecated)
#endif

namespace dobot_msgs_v4
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct FCForceMode_Request_
{
  using Type = FCForceMode_Request_<ContainerAllocator>;

  explicit FCForceMode_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::DEFAULTS_ONLY == _init)
    {
      this->reference = -1l;
      this->user = -1l;
      this->tool = -1l;
    } else if (rosidl_runtime_cpp::MessageInitialization::ZERO == _init) {
      this->x = 0l;
      this->y = 0l;
      this->z = 0l;
      this->rx = 0l;
      this->ry = 0l;
      this->rz = 0l;
      this->fx = 0l;
      this->fy = 0l;
      this->fz = 0l;
      this->frx = 0l;
      this->fry = 0l;
      this->frz = 0l;
      this->reference = 0l;
      this->user = 0l;
      this->tool = 0l;
    }
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->x = 0l;
      this->y = 0l;
      this->z = 0l;
      this->rx = 0l;
      this->ry = 0l;
      this->rz = 0l;
      this->fx = 0l;
      this->fy = 0l;
      this->fz = 0l;
      this->frx = 0l;
      this->fry = 0l;
      this->frz = 0l;
    }
  }

  explicit FCForceMode_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::DEFAULTS_ONLY == _init)
    {
      this->reference = -1l;
      this->user = -1l;
      this->tool = -1l;
    } else if (rosidl_runtime_cpp::MessageInitialization::ZERO == _init) {
      this->x = 0l;
      this->y = 0l;
      this->z = 0l;
      this->rx = 0l;
      this->ry = 0l;
      this->rz = 0l;
      this->fx = 0l;
      this->fy = 0l;
      this->fz = 0l;
      this->frx = 0l;
      this->fry = 0l;
      this->frz = 0l;
      this->reference = 0l;
      this->user = 0l;
      this->tool = 0l;
    }
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->x = 0l;
      this->y = 0l;
      this->z = 0l;
      this->rx = 0l;
      this->ry = 0l;
      this->rz = 0l;
      this->fx = 0l;
      this->fy = 0l;
      this->fz = 0l;
      this->frx = 0l;
      this->fry = 0l;
      this->frz = 0l;
    }
  }

  // field types and members
  using _x_type =
    int32_t;
  _x_type x;
  using _y_type =
    int32_t;
  _y_type y;
  using _z_type =
    int32_t;
  _z_type z;
  using _rx_type =
    int32_t;
  _rx_type rx;
  using _ry_type =
    int32_t;
  _ry_type ry;
  using _rz_type =
    int32_t;
  _rz_type rz;
  using _fx_type =
    int32_t;
  _fx_type fx;
  using _fy_type =
    int32_t;
  _fy_type fy;
  using _fz_type =
    int32_t;
  _fz_type fz;
  using _frx_type =
    int32_t;
  _frx_type frx;
  using _fry_type =
    int32_t;
  _fry_type fry;
  using _frz_type =
    int32_t;
  _frz_type frz;
  using _reference_type =
    int32_t;
  _reference_type reference;
  using _user_type =
    int32_t;
  _user_type user;
  using _tool_type =
    int32_t;
  _tool_type tool;

  // setters for named parameter idiom
  Type & set__x(
    const int32_t & _arg)
  {
    this->x = _arg;
    return *this;
  }
  Type & set__y(
    const int32_t & _arg)
  {
    this->y = _arg;
    return *this;
  }
  Type & set__z(
    const int32_t & _arg)
  {
    this->z = _arg;
    return *this;
  }
  Type & set__rx(
    const int32_t & _arg)
  {
    this->rx = _arg;
    return *this;
  }
  Type & set__ry(
    const int32_t & _arg)
  {
    this->ry = _arg;
    return *this;
  }
  Type & set__rz(
    const int32_t & _arg)
  {
    this->rz = _arg;
    return *this;
  }
  Type & set__fx(
    const int32_t & _arg)
  {
    this->fx = _arg;
    return *this;
  }
  Type & set__fy(
    const int32_t & _arg)
  {
    this->fy = _arg;
    return *this;
  }
  Type & set__fz(
    const int32_t & _arg)
  {
    this->fz = _arg;
    return *this;
  }
  Type & set__frx(
    const int32_t & _arg)
  {
    this->frx = _arg;
    return *this;
  }
  Type & set__fry(
    const int32_t & _arg)
  {
    this->fry = _arg;
    return *this;
  }
  Type & set__frz(
    const int32_t & _arg)
  {
    this->frz = _arg;
    return *this;
  }
  Type & set__reference(
    const int32_t & _arg)
  {
    this->reference = _arg;
    return *this;
  }
  Type & set__user(
    const int32_t & _arg)
  {
    this->user = _arg;
    return *this;
  }
  Type & set__tool(
    const int32_t & _arg)
  {
    this->tool = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    dobot_msgs_v4::srv::FCForceMode_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const dobot_msgs_v4::srv::FCForceMode_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::FCForceMode_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::FCForceMode_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::FCForceMode_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::FCForceMode_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::FCForceMode_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::FCForceMode_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::FCForceMode_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::FCForceMode_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__dobot_msgs_v4__srv__FCForceMode_Request
    std::shared_ptr<dobot_msgs_v4::srv::FCForceMode_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__dobot_msgs_v4__srv__FCForceMode_Request
    std::shared_ptr<dobot_msgs_v4::srv::FCForceMode_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const FCForceMode_Request_ & other) const
  {
    if (this->x != other.x) {
      return false;
    }
    if (this->y != other.y) {
      return false;
    }
    if (this->z != other.z) {
      return false;
    }
    if (this->rx != other.rx) {
      return false;
    }
    if (this->ry != other.ry) {
      return false;
    }
    if (this->rz != other.rz) {
      return false;
    }
    if (this->fx != other.fx) {
      return false;
    }
    if (this->fy != other.fy) {
      return false;
    }
    if (this->fz != other.fz) {
      return false;
    }
    if (this->frx != other.frx) {
      return false;
    }
    if (this->fry != other.fry) {
      return false;
    }
    if (this->frz != other.frz) {
      return false;
    }
    if (this->reference != other.reference) {
      return false;
    }
    if (this->user != other.user) {
      return false;
    }
    if (this->tool != other.tool) {
      return false;
    }
    return true;
  }
  bool operator!=(const FCForceMode_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct FCForceMode_Request_

// alias to use template instance with default allocator
using FCForceMode_Request =
  dobot_msgs_v4::srv::FCForceMode_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace dobot_msgs_v4


#ifndef _WIN32
# define DEPRECATED__dobot_msgs_v4__srv__FCForceMode_Response __attribute__((deprecated))
#else
# define DEPRECATED__dobot_msgs_v4__srv__FCForceMode_Response __declspec(deprecated)
#endif

namespace dobot_msgs_v4
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct FCForceMode_Response_
{
  using Type = FCForceMode_Response_<ContainerAllocator>;

  explicit FCForceMode_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->res = 0l;
    }
  }

  explicit FCForceMode_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    dobot_msgs_v4::srv::FCForceMode_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const dobot_msgs_v4::srv::FCForceMode_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::FCForceMode_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::FCForceMode_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::FCForceMode_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::FCForceMode_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::FCForceMode_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::FCForceMode_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::FCForceMode_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::FCForceMode_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__dobot_msgs_v4__srv__FCForceMode_Response
    std::shared_ptr<dobot_msgs_v4::srv::FCForceMode_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__dobot_msgs_v4__srv__FCForceMode_Response
    std::shared_ptr<dobot_msgs_v4::srv::FCForceMode_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const FCForceMode_Response_ & other) const
  {
    if (this->res != other.res) {
      return false;
    }
    return true;
  }
  bool operator!=(const FCForceMode_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct FCForceMode_Response_

// alias to use template instance with default allocator
using FCForceMode_Response =
  dobot_msgs_v4::srv::FCForceMode_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace dobot_msgs_v4

namespace dobot_msgs_v4
{

namespace srv
{

struct FCForceMode
{
  using Request = dobot_msgs_v4::srv::FCForceMode_Request;
  using Response = dobot_msgs_v4::srv::FCForceMode_Response;
};

}  // namespace srv

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__FC_FORCE_MODE__STRUCT_HPP_
