// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from dobot_msgs_v4:srv/ModbusRTUCreate.idl
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__SRV__DETAIL__MODBUS_RTU_CREATE__BUILDER_HPP_
#define DOBOT_MSGS_V4__SRV__DETAIL__MODBUS_RTU_CREATE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "dobot_msgs_v4/srv/detail/modbus_rtu_create__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_ModbusRTUCreate_Request_stop_bit
{
public:
  explicit Init_ModbusRTUCreate_Request_stop_bit(::dobot_msgs_v4::srv::ModbusRTUCreate_Request & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::ModbusRTUCreate_Request stop_bit(::dobot_msgs_v4::srv::ModbusRTUCreate_Request::_stop_bit_type arg)
  {
    msg_.stop_bit = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ModbusRTUCreate_Request msg_;
};

class Init_ModbusRTUCreate_Request_data_bit
{
public:
  explicit Init_ModbusRTUCreate_Request_data_bit(::dobot_msgs_v4::srv::ModbusRTUCreate_Request & msg)
  : msg_(msg)
  {}
  Init_ModbusRTUCreate_Request_stop_bit data_bit(::dobot_msgs_v4::srv::ModbusRTUCreate_Request::_data_bit_type arg)
  {
    msg_.data_bit = std::move(arg);
    return Init_ModbusRTUCreate_Request_stop_bit(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ModbusRTUCreate_Request msg_;
};

class Init_ModbusRTUCreate_Request_parity
{
public:
  explicit Init_ModbusRTUCreate_Request_parity(::dobot_msgs_v4::srv::ModbusRTUCreate_Request & msg)
  : msg_(msg)
  {}
  Init_ModbusRTUCreate_Request_data_bit parity(::dobot_msgs_v4::srv::ModbusRTUCreate_Request::_parity_type arg)
  {
    msg_.parity = std::move(arg);
    return Init_ModbusRTUCreate_Request_data_bit(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ModbusRTUCreate_Request msg_;
};

class Init_ModbusRTUCreate_Request_baud
{
public:
  explicit Init_ModbusRTUCreate_Request_baud(::dobot_msgs_v4::srv::ModbusRTUCreate_Request & msg)
  : msg_(msg)
  {}
  Init_ModbusRTUCreate_Request_parity baud(::dobot_msgs_v4::srv::ModbusRTUCreate_Request::_baud_type arg)
  {
    msg_.baud = std::move(arg);
    return Init_ModbusRTUCreate_Request_parity(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ModbusRTUCreate_Request msg_;
};

class Init_ModbusRTUCreate_Request_slave_id
{
public:
  Init_ModbusRTUCreate_Request_slave_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ModbusRTUCreate_Request_baud slave_id(::dobot_msgs_v4::srv::ModbusRTUCreate_Request::_slave_id_type arg)
  {
    msg_.slave_id = std::move(arg);
    return Init_ModbusRTUCreate_Request_baud(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ModbusRTUCreate_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::ModbusRTUCreate_Request>()
{
  return dobot_msgs_v4::srv::builder::Init_ModbusRTUCreate_Request_slave_id();
}

}  // namespace dobot_msgs_v4


namespace dobot_msgs_v4
{

namespace srv
{

namespace builder
{

class Init_ModbusRTUCreate_Response_res
{
public:
  explicit Init_ModbusRTUCreate_Response_res(::dobot_msgs_v4::srv::ModbusRTUCreate_Response & msg)
  : msg_(msg)
  {}
  ::dobot_msgs_v4::srv::ModbusRTUCreate_Response res(::dobot_msgs_v4::srv::ModbusRTUCreate_Response::_res_type arg)
  {
    msg_.res = std::move(arg);
    return std::move(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ModbusRTUCreate_Response msg_;
};

class Init_ModbusRTUCreate_Response_robot_return
{
public:
  Init_ModbusRTUCreate_Response_robot_return()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ModbusRTUCreate_Response_res robot_return(::dobot_msgs_v4::srv::ModbusRTUCreate_Response::_robot_return_type arg)
  {
    msg_.robot_return = std::move(arg);
    return Init_ModbusRTUCreate_Response_res(msg_);
  }

private:
  ::dobot_msgs_v4::srv::ModbusRTUCreate_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::dobot_msgs_v4::srv::ModbusRTUCreate_Response>()
{
  return dobot_msgs_v4::srv::builder::Init_ModbusRTUCreate_Response_robot_return();
}

}  // namespace dobot_msgs_v4

#endif  // DOBOT_MSGS_V4__SRV__DETAIL__MODBUS_RTU_CREATE__BUILDER_HPP_
