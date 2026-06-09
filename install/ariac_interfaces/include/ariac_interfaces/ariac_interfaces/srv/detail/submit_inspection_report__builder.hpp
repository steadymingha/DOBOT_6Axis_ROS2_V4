// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:srv/SubmitInspectionReport.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_INSPECTION_REPORT__BUILDER_HPP_
#define ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_INSPECTION_REPORT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/srv/detail/submit_inspection_report__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_SubmitInspectionReport_Request_report
{
public:
  explicit Init_SubmitInspectionReport_Request_report(::ariac_interfaces::srv::SubmitInspectionReport_Request & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::srv::SubmitInspectionReport_Request report(::ariac_interfaces::srv::SubmitInspectionReport_Request::_report_type arg)
  {
    msg_.report = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::SubmitInspectionReport_Request msg_;
};

class Init_SubmitInspectionReport_Request_timestamp
{
public:
  Init_SubmitInspectionReport_Request_timestamp()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SubmitInspectionReport_Request_report timestamp(::ariac_interfaces::srv::SubmitInspectionReport_Request::_timestamp_type arg)
  {
    msg_.timestamp = std::move(arg);
    return Init_SubmitInspectionReport_Request_report(msg_);
  }

private:
  ::ariac_interfaces::srv::SubmitInspectionReport_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::SubmitInspectionReport_Request>()
{
  return ariac_interfaces::srv::builder::Init_SubmitInspectionReport_Request_timestamp();
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace srv
{

namespace builder
{

class Init_SubmitInspectionReport_Response_message
{
public:
  explicit Init_SubmitInspectionReport_Response_message(::ariac_interfaces::srv::SubmitInspectionReport_Response & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::srv::SubmitInspectionReport_Response message(::ariac_interfaces::srv::SubmitInspectionReport_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::srv::SubmitInspectionReport_Response msg_;
};

class Init_SubmitInspectionReport_Response_success
{
public:
  Init_SubmitInspectionReport_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_SubmitInspectionReport_Response_message success(::ariac_interfaces::srv::SubmitInspectionReport_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_SubmitInspectionReport_Response_message(msg_);
  }

private:
  ::ariac_interfaces::srv::SubmitInspectionReport_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::srv::SubmitInspectionReport_Response>()
{
  return ariac_interfaces::srv::builder::Init_SubmitInspectionReport_Response_success();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__SRV__DETAIL__SUBMIT_INSPECTION_REPORT__BUILDER_HPP_
