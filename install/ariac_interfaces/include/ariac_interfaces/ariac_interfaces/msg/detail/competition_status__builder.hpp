// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:msg/CompetitionStatus.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATUS__BUILDER_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATUS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/msg/detail/competition_status__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace msg
{

namespace builder
{

class Init_CompetitionStatus_run_id
{
public:
  explicit Init_CompetitionStatus_run_id(::ariac_interfaces::msg::CompetitionStatus & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::msg::CompetitionStatus run_id(::ariac_interfaces::msg::CompetitionStatus::_run_id_type arg)
  {
    msg_.run_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::msg::CompetitionStatus msg_;
};

class Init_CompetitionStatus_time
{
public:
  explicit Init_CompetitionStatus_time(::ariac_interfaces::msg::CompetitionStatus & msg)
  : msg_(msg)
  {}
  Init_CompetitionStatus_run_id time(::ariac_interfaces::msg::CompetitionStatus::_time_type arg)
  {
    msg_.time = std::move(arg);
    return Init_CompetitionStatus_run_id(msg_);
  }

private:
  ::ariac_interfaces::msg::CompetitionStatus msg_;
};

class Init_CompetitionStatus_num_modules_remaining
{
public:
  explicit Init_CompetitionStatus_num_modules_remaining(::ariac_interfaces::msg::CompetitionStatus & msg)
  : msg_(msg)
  {}
  Init_CompetitionStatus_time num_modules_remaining(::ariac_interfaces::msg::CompetitionStatus::_num_modules_remaining_type arg)
  {
    msg_.num_modules_remaining = std::move(arg);
    return Init_CompetitionStatus_time(msg_);
  }

private:
  ::ariac_interfaces::msg::CompetitionStatus msg_;
};

class Init_CompetitionStatus_num_kits_remaining
{
public:
  explicit Init_CompetitionStatus_num_kits_remaining(::ariac_interfaces::msg::CompetitionStatus & msg)
  : msg_(msg)
  {}
  Init_CompetitionStatus_num_modules_remaining num_kits_remaining(::ariac_interfaces::msg::CompetitionStatus::_num_kits_remaining_type arg)
  {
    msg_.num_kits_remaining = std::move(arg);
    return Init_CompetitionStatus_num_modules_remaining(msg_);
  }

private:
  ::ariac_interfaces::msg::CompetitionStatus msg_;
};

class Init_CompetitionStatus_num_modules
{
public:
  explicit Init_CompetitionStatus_num_modules(::ariac_interfaces::msg::CompetitionStatus & msg)
  : msg_(msg)
  {}
  Init_CompetitionStatus_num_kits_remaining num_modules(::ariac_interfaces::msg::CompetitionStatus::_num_modules_type arg)
  {
    msg_.num_modules = std::move(arg);
    return Init_CompetitionStatus_num_kits_remaining(msg_);
  }

private:
  ::ariac_interfaces::msg::CompetitionStatus msg_;
};

class Init_CompetitionStatus_num_kits
{
public:
  explicit Init_CompetitionStatus_num_kits(::ariac_interfaces::msg::CompetitionStatus & msg)
  : msg_(msg)
  {}
  Init_CompetitionStatus_num_modules num_kits(::ariac_interfaces::msg::CompetitionStatus::_num_kits_type arg)
  {
    msg_.num_kits = std::move(arg);
    return Init_CompetitionStatus_num_modules(msg_);
  }

private:
  ::ariac_interfaces::msg::CompetitionStatus msg_;
};

class Init_CompetitionStatus_competition_state
{
public:
  Init_CompetitionStatus_competition_state()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_CompetitionStatus_num_kits competition_state(::ariac_interfaces::msg::CompetitionStatus::_competition_state_type arg)
  {
    msg_.competition_state = std::move(arg);
    return Init_CompetitionStatus_num_kits(msg_);
  }

private:
  ::ariac_interfaces::msg::CompetitionStatus msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::msg::CompetitionStatus>()
{
  return ariac_interfaces::msg::builder::Init_CompetitionStatus_competition_state();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_STATUS__BUILDER_HPP_
