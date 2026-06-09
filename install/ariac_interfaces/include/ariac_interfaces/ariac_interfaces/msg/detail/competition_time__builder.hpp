// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:msg/CompetitionTime.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_TIME__BUILDER_HPP_
#define ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_TIME__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/msg/detail/competition_time__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace msg
{

namespace builder
{

class Init_CompetitionTime_remaining
{
public:
  explicit Init_CompetitionTime_remaining(::ariac_interfaces::msg::CompetitionTime & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::msg::CompetitionTime remaining(::ariac_interfaces::msg::CompetitionTime::_remaining_type arg)
  {
    msg_.remaining = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::msg::CompetitionTime msg_;
};

class Init_CompetitionTime_elapsed
{
public:
  explicit Init_CompetitionTime_elapsed(::ariac_interfaces::msg::CompetitionTime & msg)
  : msg_(msg)
  {}
  Init_CompetitionTime_remaining elapsed(::ariac_interfaces::msg::CompetitionTime::_elapsed_type arg)
  {
    msg_.elapsed = std::move(arg);
    return Init_CompetitionTime_remaining(msg_);
  }

private:
  ::ariac_interfaces::msg::CompetitionTime msg_;
};

class Init_CompetitionTime_start
{
public:
  Init_CompetitionTime_start()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_CompetitionTime_elapsed start(::ariac_interfaces::msg::CompetitionTime::_start_type arg)
  {
    msg_.start = std::move(arg);
    return Init_CompetitionTime_elapsed(msg_);
  }

private:
  ::ariac_interfaces::msg::CompetitionTime msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::msg::CompetitionTime>()
{
  return ariac_interfaces::msg::builder::Init_CompetitionTime_start();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__MSG__DETAIL__COMPETITION_TIME__BUILDER_HPP_
