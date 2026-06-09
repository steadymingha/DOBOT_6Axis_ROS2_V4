// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from dobot_msgs_v4:srv/Circle.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__CIRCLE__STRUCT_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__CIRCLE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__dobot_msgs_v4__srv__Circle_Request __attribute__((deprecated))
#else
# define DEPRECATED__dobot_msgs_v4__srv__Circle_Request __declspec(deprecated)
#endif

namespace dobot_msgs_v4
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct Circle_Request_
{
  using Type = Circle_Request_<ContainerAllocator>;

  explicit Circle_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->mode = false;
      this->a = 0.0;
      this->b = 0.0;
      this->c = 0.0;
      this->d = 0.0;
      this->e = 0.0;
      this->f = 0.0;
      this->a2 = 0.0;
      this->b2 = 0.0;
      this->c2 = 0.0;
      this->d2 = 0.0;
      this->e2 = 0.0;
      this->f2 = 0.0;
      this->count = 0l;
    }
  }

  explicit Circle_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->mode = false;
      this->a = 0.0;
      this->b = 0.0;
      this->c = 0.0;
      this->d = 0.0;
      this->e = 0.0;
      this->f = 0.0;
      this->a2 = 0.0;
      this->b2 = 0.0;
      this->c2 = 0.0;
      this->d2 = 0.0;
      this->e2 = 0.0;
      this->f2 = 0.0;
      this->count = 0l;
    }
  }

  // field types and members
  using _mode_type =
    bool;
  _mode_type mode;
  using _a_type =
    double;
  _a_type a;
  using _b_type =
    double;
  _b_type b;
  using _c_type =
    double;
  _c_type c;
  using _d_type =
    double;
  _d_type d;
  using _e_type =
    double;
  _e_type e;
  using _f_type =
    double;
  _f_type f;
  using _a2_type =
    double;
  _a2_type a2;
  using _b2_type =
    double;
  _b2_type b2;
  using _c2_type =
    double;
  _c2_type c2;
  using _d2_type =
    double;
  _d2_type d2;
  using _e2_type =
    double;
  _e2_type e2;
  using _f2_type =
    double;
  _f2_type f2;
  using _count_type =
    int32_t;
  _count_type count;
  using _param_value_type =
    std::vector<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>>>;
  _param_value_type param_value;

  // setters for named parameter idiom
  Type & set__mode(
    const bool & _arg)
  {
    this->mode = _arg;
    return *this;
  }
  Type & set__a(
    const double & _arg)
  {
    this->a = _arg;
    return *this;
  }
  Type & set__b(
    const double & _arg)
  {
    this->b = _arg;
    return *this;
  }
  Type & set__c(
    const double & _arg)
  {
    this->c = _arg;
    return *this;
  }
  Type & set__d(
    const double & _arg)
  {
    this->d = _arg;
    return *this;
  }
  Type & set__e(
    const double & _arg)
  {
    this->e = _arg;
    return *this;
  }
  Type & set__f(
    const double & _arg)
  {
    this->f = _arg;
    return *this;
  }
  Type & set__a2(
    const double & _arg)
  {
    this->a2 = _arg;
    return *this;
  }
  Type & set__b2(
    const double & _arg)
  {
    this->b2 = _arg;
    return *this;
  }
  Type & set__c2(
    const double & _arg)
  {
    this->c2 = _arg;
    return *this;
  }
  Type & set__d2(
    const double & _arg)
  {
    this->d2 = _arg;
    return *this;
  }
  Type & set__e2(
    const double & _arg)
  {
    this->e2 = _arg;
    return *this;
  }
  Type & set__f2(
    const double & _arg)
  {
    this->f2 = _arg;
    return *this;
  }
  Type & set__count(
    const int32_t & _arg)
  {
    this->count = _arg;
    return *this;
  }
  Type & set__param_value(
    const std::vector<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>>> & _arg)
  {
    this->param_value = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    dobot_msgs_v4::srv::Circle_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const dobot_msgs_v4::srv::Circle_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::Circle_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::Circle_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::Circle_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::Circle_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::Circle_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::Circle_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::Circle_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::Circle_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__dobot_msgs_v4__srv__Circle_Request
    std::shared_ptr<dobot_msgs_v4::srv::Circle_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__dobot_msgs_v4__srv__Circle_Request
    std::shared_ptr<dobot_msgs_v4::srv::Circle_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const Circle_Request_ & other) const
  {
    if (this->mode != other.mode) {
      return false;
    }
    if (this->a != other.a) {
      return false;
    }
    if (this->b != other.b) {
      return false;
    }
    if (this->c != other.c) {
      return false;
    }
    if (this->d != other.d) {
      return false;
    }
    if (this->e != other.e) {
      return false;
    }
    if (this->f != other.f) {
      return false;
    }
    if (this->a2 != other.a2) {
      return false;
    }
    if (this->b2 != other.b2) {
      return false;
    }
    if (this->c2 != other.c2) {
      return false;
    }
    if (this->d2 != other.d2) {
      return false;
    }
    if (this->e2 != other.e2) {
      return false;
    }
    if (this->f2 != other.f2) {
      return false;
    }
    if (this->count != other.count) {
      return false;
    }
    if (this->param_value != other.param_value) {
      return false;
    }
    return true;
  }
  bool operator!=(const Circle_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct Circle_Request_

// alias to use template instance with default allocator
using Circle_Request =
  dobot_msgs_v4::srv::Circle_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace dobot_msgs_v4


#ifndef _WIN32
# define DEPRECATED__dobot_msgs_v4__srv__Circle_Response __attribute__((deprecated))
#else
# define DEPRECATED__dobot_msgs_v4__srv__Circle_Response __declspec(deprecated)
#endif

namespace dobot_msgs_v4
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct Circle_Response_
{
  using Type = Circle_Response_<ContainerAllocator>;

  explicit Circle_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->res = 0l;
    }
  }

  explicit Circle_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    dobot_msgs_v4::srv::Circle_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const dobot_msgs_v4::srv::Circle_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::Circle_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::Circle_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::Circle_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::Circle_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::Circle_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::Circle_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::Circle_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::Circle_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__dobot_msgs_v4__srv__Circle_Response
    std::shared_ptr<dobot_msgs_v4::srv::Circle_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__dobot_msgs_v4__srv__Circle_Response
    std::shared_ptr<dobot_msgs_v4::srv::Circle_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const Circle_Response_ & other) const
  {
    if (this->res != other.res) {
      return false;
    }
    return true;
  }
  bool operator!=(const Circle_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct Circle_Response_

// alias to use template instance with default allocator
using Circle_Response =
  dobot_msgs_v4::srv::Circle_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace dobot_msgs_v4

namespace dobot_msgs_v4
{

namespace srv
{

struct Circle
{
  using Request = dobot_msgs_v4::srv::Circle_Request;
  using Response = dobot_msgs_v4::srv::Circle_Response;
};

}  // namespace srv

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__CIRCLE__STRUCT_HPP_
