// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from dobot_msgs_v4:srv/RunTo.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__RUN_TO__STRUCT_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__RUN_TO__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__dobot_msgs_v4__srv__RunTo_Request __attribute__((deprecated))
#else
# define DEPRECATED__dobot_msgs_v4__srv__RunTo_Request __declspec(deprecated)
#endif

namespace dobot_msgs_v4
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct RunTo_Request_
{
  using Type = RunTo_Request_<ContainerAllocator>;

  explicit RunTo_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::DEFAULTS_ONLY == _init)
    {
      this->user = -1l;
      this->tool = -1l;
      this->a = -1l;
      this->v = -1l;
    } else if (rosidl_runtime_cpp::MessageInitialization::ZERO == _init) {
      this->a1 = 0.0;
      this->b1 = 0.0;
      this->c1 = 0.0;
      this->d1 = 0.0;
      this->e1 = 0.0;
      this->f1 = 0.0;
      this->move_type = 0l;
      this->user = 0l;
      this->tool = 0l;
      this->a = 0l;
      this->v = 0l;
    }
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->a1 = 0.0;
      this->b1 = 0.0;
      this->c1 = 0.0;
      this->d1 = 0.0;
      this->e1 = 0.0;
      this->f1 = 0.0;
      this->move_type = 0l;
    }
  }

  explicit RunTo_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::DEFAULTS_ONLY == _init)
    {
      this->user = -1l;
      this->tool = -1l;
      this->a = -1l;
      this->v = -1l;
    } else if (rosidl_runtime_cpp::MessageInitialization::ZERO == _init) {
      this->a1 = 0.0;
      this->b1 = 0.0;
      this->c1 = 0.0;
      this->d1 = 0.0;
      this->e1 = 0.0;
      this->f1 = 0.0;
      this->move_type = 0l;
      this->user = 0l;
      this->tool = 0l;
      this->a = 0l;
      this->v = 0l;
    }
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->a1 = 0.0;
      this->b1 = 0.0;
      this->c1 = 0.0;
      this->d1 = 0.0;
      this->e1 = 0.0;
      this->f1 = 0.0;
      this->move_type = 0l;
    }
  }

  // field types and members
  using _a1_type =
    double;
  _a1_type a1;
  using _b1_type =
    double;
  _b1_type b1;
  using _c1_type =
    double;
  _c1_type c1;
  using _d1_type =
    double;
  _d1_type d1;
  using _e1_type =
    double;
  _e1_type e1;
  using _f1_type =
    double;
  _f1_type f1;
  using _move_type_type =
    int32_t;
  _move_type_type move_type;
  using _user_type =
    int32_t;
  _user_type user;
  using _tool_type =
    int32_t;
  _tool_type tool;
  using _a_type =
    int32_t;
  _a_type a;
  using _v_type =
    int32_t;
  _v_type v;

  // setters for named parameter idiom
  Type & set__a1(
    const double & _arg)
  {
    this->a1 = _arg;
    return *this;
  }
  Type & set__b1(
    const double & _arg)
  {
    this->b1 = _arg;
    return *this;
  }
  Type & set__c1(
    const double & _arg)
  {
    this->c1 = _arg;
    return *this;
  }
  Type & set__d1(
    const double & _arg)
  {
    this->d1 = _arg;
    return *this;
  }
  Type & set__e1(
    const double & _arg)
  {
    this->e1 = _arg;
    return *this;
  }
  Type & set__f1(
    const double & _arg)
  {
    this->f1 = _arg;
    return *this;
  }
  Type & set__move_type(
    const int32_t & _arg)
  {
    this->move_type = _arg;
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
  Type & set__a(
    const int32_t & _arg)
  {
    this->a = _arg;
    return *this;
  }
  Type & set__v(
    const int32_t & _arg)
  {
    this->v = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    dobot_msgs_v4::srv::RunTo_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const dobot_msgs_v4::srv::RunTo_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::RunTo_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::RunTo_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::RunTo_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::RunTo_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::RunTo_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::RunTo_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::RunTo_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::RunTo_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__dobot_msgs_v4__srv__RunTo_Request
    std::shared_ptr<dobot_msgs_v4::srv::RunTo_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__dobot_msgs_v4__srv__RunTo_Request
    std::shared_ptr<dobot_msgs_v4::srv::RunTo_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const RunTo_Request_ & other) const
  {
    if (this->a1 != other.a1) {
      return false;
    }
    if (this->b1 != other.b1) {
      return false;
    }
    if (this->c1 != other.c1) {
      return false;
    }
    if (this->d1 != other.d1) {
      return false;
    }
    if (this->e1 != other.e1) {
      return false;
    }
    if (this->f1 != other.f1) {
      return false;
    }
    if (this->move_type != other.move_type) {
      return false;
    }
    if (this->user != other.user) {
      return false;
    }
    if (this->tool != other.tool) {
      return false;
    }
    if (this->a != other.a) {
      return false;
    }
    if (this->v != other.v) {
      return false;
    }
    return true;
  }
  bool operator!=(const RunTo_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct RunTo_Request_

// alias to use template instance with default allocator
using RunTo_Request =
  dobot_msgs_v4::srv::RunTo_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace dobot_msgs_v4


#ifndef _WIN32
# define DEPRECATED__dobot_msgs_v4__srv__RunTo_Response __attribute__((deprecated))
#else
# define DEPRECATED__dobot_msgs_v4__srv__RunTo_Response __declspec(deprecated)
#endif

namespace dobot_msgs_v4
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct RunTo_Response_
{
  using Type = RunTo_Response_<ContainerAllocator>;

  explicit RunTo_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->res = 0l;
    }
  }

  explicit RunTo_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    dobot_msgs_v4::srv::RunTo_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const dobot_msgs_v4::srv::RunTo_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::RunTo_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::RunTo_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::RunTo_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::RunTo_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::RunTo_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::RunTo_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::RunTo_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::RunTo_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__dobot_msgs_v4__srv__RunTo_Response
    std::shared_ptr<dobot_msgs_v4::srv::RunTo_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__dobot_msgs_v4__srv__RunTo_Response
    std::shared_ptr<dobot_msgs_v4::srv::RunTo_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const RunTo_Response_ & other) const
  {
    if (this->res != other.res) {
      return false;
    }
    return true;
  }
  bool operator!=(const RunTo_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct RunTo_Response_

// alias to use template instance with default allocator
using RunTo_Response =
  dobot_msgs_v4::srv::RunTo_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace dobot_msgs_v4

namespace dobot_msgs_v4
{

namespace srv
{

struct RunTo
{
  using Request = dobot_msgs_v4::srv::RunTo_Request;
  using Response = dobot_msgs_v4::srv::RunTo_Response;
};

}  // namespace srv

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__RUN_TO__STRUCT_HPP_
