// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from dobot_msgs_v4:srv/ModbusRTUCreate.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__MODBUS_RTU_CREATE__STRUCT_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__MODBUS_RTU_CREATE__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__dobot_msgs_v4__srv__ModbusRTUCreate_Request __attribute__((deprecated))
#else
# define DEPRECATED__dobot_msgs_v4__srv__ModbusRTUCreate_Request __declspec(deprecated)
#endif

namespace dobot_msgs_v4
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct ModbusRTUCreate_Request_
{
  using Type = ModbusRTUCreate_Request_<ContainerAllocator>;

  explicit ModbusRTUCreate_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->slave_id = 0l;
      this->baud = 0l;
      this->parity = "";
      this->data_bit = 0l;
      this->stop_bit = 0l;
    }
  }

  explicit ModbusRTUCreate_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : parity(_alloc)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->slave_id = 0l;
      this->baud = 0l;
      this->parity = "";
      this->data_bit = 0l;
      this->stop_bit = 0l;
    }
  }

  // field types and members
  using _slave_id_type =
    int32_t;
  _slave_id_type slave_id;
  using _baud_type =
    int32_t;
  _baud_type baud;
  using _parity_type =
    std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>>;
  _parity_type parity;
  using _data_bit_type =
    int32_t;
  _data_bit_type data_bit;
  using _stop_bit_type =
    int32_t;
  _stop_bit_type stop_bit;

  // setters for named parameter idiom
  Type & set__slave_id(
    const int32_t & _arg)
  {
    this->slave_id = _arg;
    return *this;
  }
  Type & set__baud(
    const int32_t & _arg)
  {
    this->baud = _arg;
    return *this;
  }
  Type & set__parity(
    const std::basic_string<char, std::char_traits<char>, typename std::allocator_traits<ContainerAllocator>::template rebind_alloc<char>> & _arg)
  {
    this->parity = _arg;
    return *this;
  }
  Type & set__data_bit(
    const int32_t & _arg)
  {
    this->data_bit = _arg;
    return *this;
  }
  Type & set__stop_bit(
    const int32_t & _arg)
  {
    this->stop_bit = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    dobot_msgs_v4::srv::ModbusRTUCreate_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const dobot_msgs_v4::srv::ModbusRTUCreate_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::ModbusRTUCreate_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::ModbusRTUCreate_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__dobot_msgs_v4__srv__ModbusRTUCreate_Request
    std::shared_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__dobot_msgs_v4__srv__ModbusRTUCreate_Request
    std::shared_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ModbusRTUCreate_Request_ & other) const
  {
    if (this->slave_id != other.slave_id) {
      return false;
    }
    if (this->baud != other.baud) {
      return false;
    }
    if (this->parity != other.parity) {
      return false;
    }
    if (this->data_bit != other.data_bit) {
      return false;
    }
    if (this->stop_bit != other.stop_bit) {
      return false;
    }
    return true;
  }
  bool operator!=(const ModbusRTUCreate_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ModbusRTUCreate_Request_

// alias to use template instance with default allocator
using ModbusRTUCreate_Request =
  dobot_msgs_v4::srv::ModbusRTUCreate_Request_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace dobot_msgs_v4


#ifndef _WIN32
# define DEPRECATED__dobot_msgs_v4__srv__ModbusRTUCreate_Response __attribute__((deprecated))
#else
# define DEPRECATED__dobot_msgs_v4__srv__ModbusRTUCreate_Response __declspec(deprecated)
#endif

namespace dobot_msgs_v4
{

namespace srv
{

// message struct
template<class ContainerAllocator>
struct ModbusRTUCreate_Response_
{
  using Type = ModbusRTUCreate_Response_<ContainerAllocator>;

  explicit ModbusRTUCreate_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->robot_return = "";
      this->res = 0l;
    }
  }

  explicit ModbusRTUCreate_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
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
    dobot_msgs_v4::srv::ModbusRTUCreate_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const dobot_msgs_v4::srv::ModbusRTUCreate_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::ModbusRTUCreate_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      dobot_msgs_v4::srv::ModbusRTUCreate_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__dobot_msgs_v4__srv__ModbusRTUCreate_Response
    std::shared_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__dobot_msgs_v4__srv__ModbusRTUCreate_Response
    std::shared_ptr<dobot_msgs_v4::srv::ModbusRTUCreate_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const ModbusRTUCreate_Response_ & other) const
  {
    if (this->robot_return != other.robot_return) {
      return false;
    }
    if (this->res != other.res) {
      return false;
    }
    return true;
  }
  bool operator!=(const ModbusRTUCreate_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct ModbusRTUCreate_Response_

// alias to use template instance with default allocator
using ModbusRTUCreate_Response =
  dobot_msgs_v4::srv::ModbusRTUCreate_Response_<std::allocator<void>>;

// constant definitions

}  // namespace srv

}  // namespace dobot_msgs_v4

namespace dobot_msgs_v4
{

namespace srv
{

struct ModbusRTUCreate
{
  using Request = dobot_msgs_v4::srv::ModbusRTUCreate_Request;
  using Response = dobot_msgs_v4::srv::ModbusRTUCreate_Response;
};

}  // namespace srv

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__MODBUS_RTU_CREATE__STRUCT_HPP_
