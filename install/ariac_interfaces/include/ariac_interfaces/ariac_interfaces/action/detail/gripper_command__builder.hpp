// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ariac_interfaces:action/GripperCommand.idl
// generated code does not contain a copyright notice

#ifndef ARIAC_INTERFACES__ACTION__DETAIL__GRIPPER_COMMAND__BUILDER_HPP_
#define ARIAC_INTERFACES__ACTION__DETAIL__GRIPPER_COMMAND__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ariac_interfaces/action/detail/gripper_command__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ariac_interfaces
{

namespace action
{

namespace builder
{

class Init_GripperCommand_Goal_width
{
public:
  Init_GripperCommand_Goal_width()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ariac_interfaces::action::GripperCommand_Goal width(::ariac_interfaces::action::GripperCommand_Goal::_width_type arg)
  {
    msg_.width = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::action::GripperCommand_Goal msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::action::GripperCommand_Goal>()
{
  return ariac_interfaces::action::builder::Init_GripperCommand_Goal_width();
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace action
{

namespace builder
{

class Init_GripperCommand_Result_width
{
public:
  explicit Init_GripperCommand_Result_width(::ariac_interfaces::action::GripperCommand_Result & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::action::GripperCommand_Result width(::ariac_interfaces::action::GripperCommand_Result::_width_type arg)
  {
    msg_.width = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::action::GripperCommand_Result msg_;
};

class Init_GripperCommand_Result_reached_goal_width
{
public:
  explicit Init_GripperCommand_Result_reached_goal_width(::ariac_interfaces::action::GripperCommand_Result & msg)
  : msg_(msg)
  {}
  Init_GripperCommand_Result_width reached_goal_width(::ariac_interfaces::action::GripperCommand_Result::_reached_goal_width_type arg)
  {
    msg_.reached_goal_width = std::move(arg);
    return Init_GripperCommand_Result_width(msg_);
  }

private:
  ::ariac_interfaces::action::GripperCommand_Result msg_;
};

class Init_GripperCommand_Result_stalled
{
public:
  Init_GripperCommand_Result_stalled()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GripperCommand_Result_reached_goal_width stalled(::ariac_interfaces::action::GripperCommand_Result::_stalled_type arg)
  {
    msg_.stalled = std::move(arg);
    return Init_GripperCommand_Result_reached_goal_width(msg_);
  }

private:
  ::ariac_interfaces::action::GripperCommand_Result msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::action::GripperCommand_Result>()
{
  return ariac_interfaces::action::builder::Init_GripperCommand_Result_stalled();
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace action
{

namespace builder
{

class Init_GripperCommand_Feedback_width
{
public:
  Init_GripperCommand_Feedback_width()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ariac_interfaces::action::GripperCommand_Feedback width(::ariac_interfaces::action::GripperCommand_Feedback::_width_type arg)
  {
    msg_.width = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::action::GripperCommand_Feedback msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::action::GripperCommand_Feedback>()
{
  return ariac_interfaces::action::builder::Init_GripperCommand_Feedback_width();
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace action
{

namespace builder
{

class Init_GripperCommand_SendGoal_Request_goal
{
public:
  explicit Init_GripperCommand_SendGoal_Request_goal(::ariac_interfaces::action::GripperCommand_SendGoal_Request & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::action::GripperCommand_SendGoal_Request goal(::ariac_interfaces::action::GripperCommand_SendGoal_Request::_goal_type arg)
  {
    msg_.goal = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::action::GripperCommand_SendGoal_Request msg_;
};

class Init_GripperCommand_SendGoal_Request_goal_id
{
public:
  Init_GripperCommand_SendGoal_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GripperCommand_SendGoal_Request_goal goal_id(::ariac_interfaces::action::GripperCommand_SendGoal_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_GripperCommand_SendGoal_Request_goal(msg_);
  }

private:
  ::ariac_interfaces::action::GripperCommand_SendGoal_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::action::GripperCommand_SendGoal_Request>()
{
  return ariac_interfaces::action::builder::Init_GripperCommand_SendGoal_Request_goal_id();
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace action
{

namespace builder
{

class Init_GripperCommand_SendGoal_Response_stamp
{
public:
  explicit Init_GripperCommand_SendGoal_Response_stamp(::ariac_interfaces::action::GripperCommand_SendGoal_Response & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::action::GripperCommand_SendGoal_Response stamp(::ariac_interfaces::action::GripperCommand_SendGoal_Response::_stamp_type arg)
  {
    msg_.stamp = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::action::GripperCommand_SendGoal_Response msg_;
};

class Init_GripperCommand_SendGoal_Response_accepted
{
public:
  Init_GripperCommand_SendGoal_Response_accepted()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GripperCommand_SendGoal_Response_stamp accepted(::ariac_interfaces::action::GripperCommand_SendGoal_Response::_accepted_type arg)
  {
    msg_.accepted = std::move(arg);
    return Init_GripperCommand_SendGoal_Response_stamp(msg_);
  }

private:
  ::ariac_interfaces::action::GripperCommand_SendGoal_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::action::GripperCommand_SendGoal_Response>()
{
  return ariac_interfaces::action::builder::Init_GripperCommand_SendGoal_Response_accepted();
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace action
{

namespace builder
{

class Init_GripperCommand_GetResult_Request_goal_id
{
public:
  Init_GripperCommand_GetResult_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ariac_interfaces::action::GripperCommand_GetResult_Request goal_id(::ariac_interfaces::action::GripperCommand_GetResult_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::action::GripperCommand_GetResult_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::action::GripperCommand_GetResult_Request>()
{
  return ariac_interfaces::action::builder::Init_GripperCommand_GetResult_Request_goal_id();
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace action
{

namespace builder
{

class Init_GripperCommand_GetResult_Response_result
{
public:
  explicit Init_GripperCommand_GetResult_Response_result(::ariac_interfaces::action::GripperCommand_GetResult_Response & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::action::GripperCommand_GetResult_Response result(::ariac_interfaces::action::GripperCommand_GetResult_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::action::GripperCommand_GetResult_Response msg_;
};

class Init_GripperCommand_GetResult_Response_status
{
public:
  Init_GripperCommand_GetResult_Response_status()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GripperCommand_GetResult_Response_result status(::ariac_interfaces::action::GripperCommand_GetResult_Response::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_GripperCommand_GetResult_Response_result(msg_);
  }

private:
  ::ariac_interfaces::action::GripperCommand_GetResult_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::action::GripperCommand_GetResult_Response>()
{
  return ariac_interfaces::action::builder::Init_GripperCommand_GetResult_Response_status();
}

}  // namespace ariac_interfaces


namespace ariac_interfaces
{

namespace action
{

namespace builder
{

class Init_GripperCommand_FeedbackMessage_feedback
{
public:
  explicit Init_GripperCommand_FeedbackMessage_feedback(::ariac_interfaces::action::GripperCommand_FeedbackMessage & msg)
  : msg_(msg)
  {}
  ::ariac_interfaces::action::GripperCommand_FeedbackMessage feedback(::ariac_interfaces::action::GripperCommand_FeedbackMessage::_feedback_type arg)
  {
    msg_.feedback = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ariac_interfaces::action::GripperCommand_FeedbackMessage msg_;
};

class Init_GripperCommand_FeedbackMessage_goal_id
{
public:
  Init_GripperCommand_FeedbackMessage_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_GripperCommand_FeedbackMessage_feedback goal_id(::ariac_interfaces::action::GripperCommand_FeedbackMessage::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_GripperCommand_FeedbackMessage_feedback(msg_);
  }

private:
  ::ariac_interfaces::action::GripperCommand_FeedbackMessage msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::ariac_interfaces::action::GripperCommand_FeedbackMessage>()
{
  return ariac_interfaces::action::builder::Init_GripperCommand_FeedbackMessage_goal_id();
}

}  // namespace ariac_interfaces

#endif  // ARIAC_INTERFACES__ACTION__DETAIL__GRIPPER_COMMAND__BUILDER_HPP_
