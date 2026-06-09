// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from dobot_msgs_v4:srv/CheckOddMovL.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__CHECK_ODD_MOV_L__STRUCT_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__CHECK_ODD_MOV_L__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__dobot_msgs_v4__srv__CheckOddMovL_Request __attribute__((deprecated))
#else
# define DEPRECATED__dobot_msgs_v4__srv__CheckOddMovL_Request __declspec(deprecated)
#endif

namespace dobot_msgs_v4
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct CheckOddMovL_Request_
{
  using Type = CheckOddMovL_Request_<ContainerAllocator>;

  explicit CheckOddMovL_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->point1_j1 = 0.0;
      this->point1_j2 = 0.0;
      this->point1_j3 = 0.0;
      this->point1_j4 = 0.0;
      this->point1_j5 = 0.0;
      this->point1_j6 = 0.0;
      this->point2_j1 = 0.0;
      this->point2_j2 = 0.0;
      this->point2_j3 = 0.0;
      this->point2_j4 = 0.0;
      this->point2_j5 = 0.0;
      this->point2_j6 = 0.0;
    }
  }

  explicit CheckOddMovL_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->point1_j1 = 0.0;
      this->point1_j2 = 0.0;
      this->point1_j3 = 0.0;
      this->point1_j4 = 0.0;
      this->point1_j5 = 0.0;
      this->point1_j6 = 0.0;
      this->point2_j1 = 0.0;
      this->point2_j2 = 0.0;
      this->point2_j3 = 0.0;
      this->point2_j4 = 0.0;
      this->point2_j5 = 0.0;
      this->point2_j6 = 0.0;
    }
  }

  // field types and members
  using _point1_j1_type =
    double;
  _point1_j1_type point1_j1;
  using _point1_j2_type =
    double;
  _point1_j2_type point1_j2;
  using _point1_j3_type =
    double;
  _point1_j3_type point1_j3;
  using _point1_j4_type =
    double;
  _point1_j4_type point1_j4;
  using _point1_j5_type =
    double;
  _point1_j5_type point1_j5;
  using _point1_j6_type =
    double;
  _point1_j6_type point1_j6;
  using _point2_j1_type =
    double;
  _point2_j1_type point2_j1;
  using _point2_j2_type =
    double;
  _point2_j2_type point2_j2;
  using _point2_j3_type =
    double;
  _point2_j3_type point2_j3;
  using _point2_j4_type =
    double;
  _point2_j4_type point2_j4;
  using _point2_j5_type =
    double;
  _point2_j5_type point2_j5;
  using _point2_j6_type =
    double;
  _point2_j6_type point2_j6;
  using _param_value_type =
    std::vector<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>>>;
  _param_value_type param_value;

  // setters for named parameter idiom
  Type & set__point1_j1(
    const double & _arg)
  {
    this->point1_j1 = _arg;
    return *this;
  }
  Type & set__point1_j2(
    const double & _arg)
  {
    this->point1_j2 = _arg;
    return *this;
  }
  Type & set__point1_j3(
    const double & _arg)
  {
    this->point1_j3 = _arg;
    return *this;
  }
  Type & set__point1_j4(
    const double & _arg)
  {
    this->point1_j4 = _arg;
    return *this;
  }
  Type & set__point1_j5(
    const double & _arg)
  {
    this->point1_j5 = _arg;
    return *this;
  }
  Type & set__point1_j6(
    const double & _arg)
  {
    this->point1_j6 = _arg;
    return *this;
  }
  Type & set__point2_j1(
    const double & _arg)
  {
    this->point2_j1 = _arg;
    return *this;
  }
  Type & set__point2_j2(
    const double & _arg)
  {
    this->point2_j2 = _arg;
    return *this;
  }
  Type & set__point2_j3(
    const double & _arg)
  {
    this->point2_j3 = _arg;
    return *this;
  }
  Type & set__point2_j4(
    const double & _arg)
  {
    this->point2_j4 = _arg;
    return *this;
  }
  Type & set__point2_j5(
    const double & _arg)
  {
    this->point2_j5 = _arg;
    return *this;
  }
  Type & set__point2_j6(
    const double & _arg)
  {
    this->point2_j6 = _arg;
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
    dobot_msgs_v4::srv::CheckOddMovL_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const dobot_msgs_v4::srv::CheckOddMovL_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::CheckOddMovL_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::CheckOddMovL_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::CheckOddMovL_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::CheckOddMovL_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::CheckOddMovL_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::CheckOddMovL_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::CheckOddMovL_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::CheckOddMovL_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__dobot_msgs_v4__srv__CheckOddMovL_Request
    std::shared_ptr<dobot_msgs_v4::srv::CheckOddMovL_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__dobot_msgs_v4__srv__CheckOddMovL_Request
    std::shared_ptr<dobot_msgs_v4::srv::CheckOddMovL_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CheckOddMovL_Request_ & other) const
  {
    if (this->point1_j1 != other.point1_j1) {
      return false;
    }
    if (this->point1_j2 != other.point1_j2) {
      return false;
    }
    if (this->point1_j3 != other.point1_j3) {
      return false;
    }
    if (this->point1_j4 != other.point1_j4) {
      return false;
    }
    if (this->point1_j5 != other.point1_j5) {
      return false;
    }
    if (this->point1_j6 != other.point1_j6) {
      return false;
    }
    if (this->point2_j1 != other.point2_j1) {
      return false;
    }
    if (this->point2_j2 != other.point2_j2) {
      return false;
    }
    if (this->point2_j3 != other.point2_j3) {
      return false;
    }
    if (this->point2_j4 != other.point2_j4) {
      return false;
    }
    if (this->point2_j5 != other.point2_j5) {
      return false;
    }
    if (this->point2_j6 != other.point2_j6) {
      return false;
    }
    if (this->param_value != other.param_value) {
      return false;
    }
    return true;
  }
  bool operator!=(const CheckOddMovL_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CheckOddMovL_Request_

// alias to use template instance with default allocator
using CheckOddMovL_Request =
  dobot_msgs_v4::srv::CheckOddMovL_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace dobot_msgs_v4


#ifndef _WIN32
# define DEPRECATED__dobot_msgs_v4__srv__CheckOddMovL_Response __attribute__((deprecated))
#else
# define DEPRECATED__dobot_msgs_v4__srv__CheckOddMovL_Response __declspec(deprecated)
#endif

namespace dobot_msgs_v4
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct CheckOddMovL_Response_
{
  using Type = CheckOddMovL_Response_<ContainerAllocator>;

  explicit CheckOddMovL_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->robot_return = "";
      this->res = 0l;
    }
  }

  explicit CheckOddMovL_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : robot_return(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->robot_return = "";
      this->res = 0l;
    }
  }

  // field types and members
  using _robot_return_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _robot_return_type robot_return;
  using _res_type =
    int32_t;
  _res_type res;

  // setters for named parameter idiom
  Type & set__robot_return(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->robot_return = _arg;
    return *this;
  }
  Type & set__res(
    const int32_t & _arg)
  {
    this->res = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    dobot_msgs_v4::srv::CheckOddMovL_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const dobot_msgs_v4::srv::CheckOddMovL_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::CheckOddMovL_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::CheckOddMovL_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::CheckOddMovL_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::CheckOddMovL_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::CheckOddMovL_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::CheckOddMovL_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::CheckOddMovL_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::CheckOddMovL_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__dobot_msgs_v4__srv__CheckOddMovL_Response
    std::shared_ptr<dobot_msgs_v4::srv::CheckOddMovL_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__dobot_msgs_v4__srv__CheckOddMovL_Response
    std::shared_ptr<dobot_msgs_v4::srv::CheckOddMovL_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const CheckOddMovL_Response_ & other) const
  {
    if (this->robot_return != other.robot_return) {
      return false;
    }
    if (this->res != other.res) {
      return false;
    }
    return true;
  }
  bool operator!=(const CheckOddMovL_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct CheckOddMovL_Response_

// alias to use template instance with default allocator
using CheckOddMovL_Response =
  dobot_msgs_v4::srv::CheckOddMovL_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace dobot_msgs_v4

namespace dobot_msgs_v4
{

namespace srv
{

struct CheckOddMovL
{
  using Request = dobot_msgs_v4::srv::CheckOddMovL_Request;
  using Response = dobot_msgs_v4::srv::CheckOddMovL_Response;
};

}  // namespace srv

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__CHECK_ODD_MOV_L__STRUCT_HPP_
