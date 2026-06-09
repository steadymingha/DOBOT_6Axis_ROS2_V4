#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to dobot_msgs_v4__srv__EnableRobot_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnableRobot_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for EnableRobot_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EnableRobot_Request::default())
  }
}

impl rosidl_runtime_rs::Message for EnableRobot_Request {
  type RmwMsg = super::srv::rmw::EnableRobot_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__EnableRobot_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnableRobot_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for EnableRobot_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EnableRobot_Response::default())
  }
}

impl rosidl_runtime_rs::Message for EnableRobot_Response {
  type RmwMsg = super::srv::rmw::EnableRobot_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DisableRobot_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DisableRobot_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for DisableRobot_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DisableRobot_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DisableRobot_Request {
  type RmwMsg = super::srv::rmw::DisableRobot_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DisableRobot_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DisableRobot_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DisableRobot_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DisableRobot_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DisableRobot_Response {
  type RmwMsg = super::srv::rmw::DisableRobot_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ClearError_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearError_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ClearError_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ClearError_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ClearError_Request {
  type RmwMsg = super::srv::rmw::ClearError_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ClearError_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearError_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ClearError_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ClearError_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ClearError_Response {
  type RmwMsg = super::srv::rmw::ClearError_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SpeedFactor_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedFactor_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ratio: i32,

}



impl Default for SpeedFactor_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SpeedFactor_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SpeedFactor_Request {
  type RmwMsg = super::srv::rmw::SpeedFactor_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ratio: msg.ratio,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      ratio: msg.ratio,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ratio: msg.ratio,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SpeedFactor_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedFactor_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SpeedFactor_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SpeedFactor_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SpeedFactor_Response {
  type RmwMsg = super::srv::rmw::SpeedFactor_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__User_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct User_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for User_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::User_Request::default())
  }
}

impl rosidl_runtime_rs::Message for User_Request {
  type RmwMsg = super::srv::rmw::User_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__User_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct User_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for User_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::User_Response::default())
  }
}

impl rosidl_runtime_rs::Message for User_Response {
  type RmwMsg = super::srv::rmw::User_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__Tool_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Tool_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for Tool_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Tool_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Tool_Request {
  type RmwMsg = super::srv::rmw::Tool_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__Tool_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Tool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for Tool_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Tool_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Tool_Response {
  type RmwMsg = super::srv::rmw::Tool_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RobotMode_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for RobotMode_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RobotMode_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RobotMode_Request {
  type RmwMsg = super::srv::rmw::RobotMode_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RobotMode_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RobotMode_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RobotMode_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RobotMode_Response {
  type RmwMsg = super::srv::rmw::RobotMode_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetPayload_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPayload_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub load: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: f64,

}



impl Default for SetPayload_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetPayload_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetPayload_Request {
  type RmwMsg = super::srv::rmw::SetPayload_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        load: msg.load,
        x: msg.x,
        y: msg.y,
        z: msg.z,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      load: msg.load,
      x: msg.x,
      y: msg.y,
      z: msg.z,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      load: msg.load,
      x: msg.x,
      y: msg.y,
      z: msg.z,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetPayload_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPayload_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetPayload_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetPayload_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetPayload_Response {
  type RmwMsg = super::srv::rmw::SetPayload_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DO_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DO_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub time: i32,

}



impl Default for DO_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DO_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DO_Request {
  type RmwMsg = super::srv::rmw::DO_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        status: msg.status,
        time: msg.time,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      status: msg.status,
      time: msg.time,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      status: msg.status,
      time: msg.time,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DO_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DO_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DO_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DO_Response {
  type RmwMsg = super::srv::rmw::DO_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DOInstant_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DOInstant_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i32,

}



impl Default for DOInstant_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DOInstant_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DOInstant_Request {
  type RmwMsg = super::srv::rmw::DOInstant_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      status: msg.status,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DOInstant_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DOInstant_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DOInstant_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DOInstant_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DOInstant_Response {
  type RmwMsg = super::srv::rmw::DOInstant_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ToolDO_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDO_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i32,

}



impl Default for ToolDO_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ToolDO_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ToolDO_Request {
  type RmwMsg = super::srv::rmw::ToolDO_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      status: msg.status,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ToolDO_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ToolDO_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ToolDO_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ToolDO_Response {
  type RmwMsg = super::srv::rmw::ToolDO_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ToolDOInstant_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDOInstant_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i32,

}



impl Default for ToolDOInstant_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ToolDOInstant_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ToolDOInstant_Request {
  type RmwMsg = super::srv::rmw::ToolDOInstant_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      status: msg.status,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ToolDOInstant_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDOInstant_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ToolDOInstant_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ToolDOInstant_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ToolDOInstant_Response {
  type RmwMsg = super::srv::rmw::ToolDOInstant_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__AO_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AO_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for AO_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AO_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AO_Request {
  type RmwMsg = super::srv::rmw::AO_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      value: msg.value,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__AO_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for AO_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AO_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AO_Response {
  type RmwMsg = super::srv::rmw::AO_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__AOInstant_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AOInstant_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for AOInstant_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AOInstant_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AOInstant_Request {
  type RmwMsg = super::srv::rmw::AOInstant_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      value: msg.value,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__AOInstant_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AOInstant_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for AOInstant_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AOInstant_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AOInstant_Response {
  type RmwMsg = super::srv::rmw::AOInstant_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__AccJ_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AccJ_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: i32,

}



impl Default for AccJ_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AccJ_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AccJ_Request {
  type RmwMsg = super::srv::rmw::AccJ_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        r: msg.r,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      r: msg.r,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      r: msg.r,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__AccJ_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AccJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for AccJ_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AccJ_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AccJ_Response {
  type RmwMsg = super::srv::rmw::AccJ_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__AccL_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AccL_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: i32,

}



impl Default for AccL_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AccL_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AccL_Request {
  type RmwMsg = super::srv::rmw::AccL_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        r: msg.r,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      r: msg.r,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      r: msg.r,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__AccL_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AccL_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for AccL_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AccL_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AccL_Response {
  type RmwMsg = super::srv::rmw::AccL_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__VelJ_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VelJ_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: i32,

}



impl Default for VelJ_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::VelJ_Request::default())
  }
}

impl rosidl_runtime_rs::Message for VelJ_Request {
  type RmwMsg = super::srv::rmw::VelJ_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        r: msg.r,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      r: msg.r,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      r: msg.r,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__VelJ_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VelJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for VelJ_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::VelJ_Response::default())
  }
}

impl rosidl_runtime_rs::Message for VelJ_Response {
  type RmwMsg = super::srv::rmw::VelJ_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__VelL_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VelL_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: i32,

}



impl Default for VelL_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::VelL_Request::default())
  }
}

impl rosidl_runtime_rs::Message for VelL_Request {
  type RmwMsg = super::srv::rmw::VelL_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        r: msg.r,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      r: msg.r,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      r: msg.r,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__VelL_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VelL_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for VelL_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::VelL_Response::default())
  }
}

impl rosidl_runtime_rs::Message for VelL_Response {
  type RmwMsg = super::srv::rmw::VelL_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__CP_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CP_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: i32,

}



impl Default for CP_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CP_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CP_Request {
  type RmwMsg = super::srv::rmw::CP_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        r: msg.r,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      r: msg.r,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      r: msg.r,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__CP_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CP_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for CP_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CP_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CP_Response {
  type RmwMsg = super::srv::rmw::CP_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__PowerOn_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PowerOn_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for PowerOn_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PowerOn_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PowerOn_Request {
  type RmwMsg = super::srv::rmw::PowerOn_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__PowerOn_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PowerOn_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for PowerOn_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PowerOn_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PowerOn_Response {
  type RmwMsg = super::srv::rmw::PowerOn_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RunScript_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RunScript_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub project_name: std::string::String,

}



impl Default for RunScript_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RunScript_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RunScript_Request {
  type RmwMsg = super::srv::rmw::RunScript_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        project_name: msg.project_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        project_name: msg.project_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      project_name: msg.project_name.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RunScript_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RunScript_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RunScript_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RunScript_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RunScript_Response {
  type RmwMsg = super::srv::rmw::RunScript_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__Stop_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stop_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Stop_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Stop_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Stop_Request {
  type RmwMsg = super::srv::rmw::Stop_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__Stop_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stop_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for Stop_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Stop_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Stop_Response {
  type RmwMsg = super::srv::rmw::Stop_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__Pause_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pause_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Pause_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Pause_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Pause_Request {
  type RmwMsg = super::srv::rmw::Pause_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__Pause_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pause_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for Pause_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Pause_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Pause_Response {
  type RmwMsg = super::srv::rmw::Pause_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__Continue_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Continue_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Continue_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Continue_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Continue_Request {
  type RmwMsg = super::srv::rmw::Continue_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__Continue_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Continue_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for Continue_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Continue_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Continue_Response {
  type RmwMsg = super::srv::rmw::Continue_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__PositiveKin_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PositiveKin_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub j1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j3: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j4: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j5: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub j6: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub user: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tool: std::string::String,

}



impl Default for PositiveKin_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PositiveKin_Request::default())
  }
}

impl rosidl_runtime_rs::Message for PositiveKin_Request {
  type RmwMsg = super::srv::rmw::PositiveKin_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        j1: msg.j1,
        j2: msg.j2,
        j3: msg.j3,
        j4: msg.j4,
        j5: msg.j5,
        j6: msg.j6,
        user: msg.user.as_str().into(),
        tool: msg.tool.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      j1: msg.j1,
      j2: msg.j2,
      j3: msg.j3,
      j4: msg.j4,
      j5: msg.j5,
      j6: msg.j6,
        user: msg.user.as_str().into(),
        tool: msg.tool.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      j1: msg.j1,
      j2: msg.j2,
      j3: msg.j3,
      j4: msg.j4,
      j5: msg.j5,
      j6: msg.j6,
      user: msg.user.to_string(),
      tool: msg.tool.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__PositiveKin_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PositiveKin_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for PositiveKin_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::PositiveKin_Response::default())
  }
}

impl rosidl_runtime_rs::Message for PositiveKin_Response {
  type RmwMsg = super::srv::rmw::PositiveKin_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__InverseKin_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InverseKin_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rx: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ry: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rz: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub use_joint_near: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_near: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub user: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tool: std::string::String,

}



impl Default for InverseKin_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::InverseKin_Request::default())
  }
}

impl rosidl_runtime_rs::Message for InverseKin_Request {
  type RmwMsg = super::srv::rmw::InverseKin_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        rx: msg.rx,
        ry: msg.ry,
        rz: msg.rz,
        use_joint_near: msg.use_joint_near.as_str().into(),
        joint_near: msg.joint_near.as_str().into(),
        user: msg.user.as_str().into(),
        tool: msg.tool.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
        use_joint_near: msg.use_joint_near.as_str().into(),
        joint_near: msg.joint_near.as_str().into(),
        user: msg.user.as_str().into(),
        tool: msg.tool.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      use_joint_near: msg.use_joint_near.to_string(),
      joint_near: msg.joint_near.to_string(),
      user: msg.user.to_string(),
      tool: msg.tool.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__InverseKin_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InverseKin_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for InverseKin_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::InverseKin_Response::default())
  }
}

impl rosidl_runtime_rs::Message for InverseKin_Response {
  type RmwMsg = super::srv::rmw::InverseKin_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetCollisionLevel_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCollisionLevel_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub level: i32,

}



impl Default for SetCollisionLevel_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCollisionLevel_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetCollisionLevel_Request {
  type RmwMsg = super::srv::rmw::SetCollisionLevel_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        level: msg.level,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      level: msg.level,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      level: msg.level,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetCollisionLevel_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCollisionLevel_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetCollisionLevel_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCollisionLevel_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetCollisionLevel_Response {
  type RmwMsg = super::srv::rmw::SetCollisionLevel_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetAngle_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetAngle_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetAngle_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetAngle_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetAngle_Request {
  type RmwMsg = super::srv::rmw::GetAngle_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetAngle_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetAngle_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetAngle_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetAngle_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetAngle_Response {
  type RmwMsg = super::srv::rmw::GetAngle_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetPose_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub user: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tool: i32,

}



impl Default for GetPose_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPose_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPose_Request {
  type RmwMsg = super::srv::rmw::GetPose_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        user: msg.user,
        tool: msg.tool,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      user: msg.user,
      tool: msg.tool,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      user: msg.user,
      tool: msg.tool,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetPose_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetPose_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPose_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPose_Response {
  type RmwMsg = super::srv::rmw::GetPose_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__EmergencyStop_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EmergencyStop_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for EmergencyStop_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EmergencyStop_Request::default())
  }
}

impl rosidl_runtime_rs::Message for EmergencyStop_Request {
  type RmwMsg = super::srv::rmw::EmergencyStop_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      value: msg.value,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__EmergencyStop_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EmergencyStop_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for EmergencyStop_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EmergencyStop_Response::default())
  }
}

impl rosidl_runtime_rs::Message for EmergencyStop_Response {
  type RmwMsg = super::srv::rmw::EmergencyStop_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ModbusRTUCreate_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModbusRTUCreate_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub slave_id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub baud: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub parity: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data_bit: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stop_bit: i32,

}



impl Default for ModbusRTUCreate_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ModbusRTUCreate_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ModbusRTUCreate_Request {
  type RmwMsg = super::srv::rmw::ModbusRTUCreate_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        slave_id: msg.slave_id,
        baud: msg.baud,
        parity: msg.parity.as_str().into(),
        data_bit: msg.data_bit,
        stop_bit: msg.stop_bit,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      slave_id: msg.slave_id,
      baud: msg.baud,
        parity: msg.parity.as_str().into(),
      data_bit: msg.data_bit,
      stop_bit: msg.stop_bit,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      slave_id: msg.slave_id,
      baud: msg.baud,
      parity: msg.parity.to_string(),
      data_bit: msg.data_bit,
      stop_bit: msg.stop_bit,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ModbusRTUCreate_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModbusRTUCreate_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ModbusRTUCreate_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ModbusRTUCreate_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ModbusRTUCreate_Response {
  type RmwMsg = super::srv::rmw::ModbusRTUCreate_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ModbusCreate_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModbusCreate_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ip: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub port: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub slave_id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_rtu: i32,

}



impl Default for ModbusCreate_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ModbusCreate_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ModbusCreate_Request {
  type RmwMsg = super::srv::rmw::ModbusCreate_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ip: msg.ip.as_str().into(),
        port: msg.port,
        slave_id: msg.slave_id,
        is_rtu: msg.is_rtu,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ip: msg.ip.as_str().into(),
      port: msg.port,
      slave_id: msg.slave_id,
      is_rtu: msg.is_rtu,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ip: msg.ip.to_string(),
      port: msg.port,
      slave_id: msg.slave_id,
      is_rtu: msg.is_rtu,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ModbusCreate_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModbusCreate_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ModbusCreate_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ModbusCreate_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ModbusCreate_Response {
  type RmwMsg = super::srv::rmw::ModbusCreate_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ModbusClose_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModbusClose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for ModbusClose_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ModbusClose_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ModbusClose_Request {
  type RmwMsg = super::srv::rmw::ModbusClose_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ModbusClose_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModbusClose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ModbusClose_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ModbusClose_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ModbusClose_Response {
  type RmwMsg = super::srv::rmw::ModbusClose_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetInBits_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInBits_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub addr: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub count: i32,

}



impl Default for GetInBits_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetInBits_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetInBits_Request {
  type RmwMsg = super::srv::rmw::GetInBits_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        addr: msg.addr,
        count: msg.count,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      addr: msg.addr,
      count: msg.count,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      addr: msg.addr,
      count: msg.count,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetInBits_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInBits_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetInBits_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetInBits_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetInBits_Response {
  type RmwMsg = super::srv::rmw::GetInBits_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetInRegs_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInRegs_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub addr: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub count: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub val_type: std::string::String,

}



impl Default for GetInRegs_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetInRegs_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetInRegs_Request {
  type RmwMsg = super::srv::rmw::GetInRegs_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        addr: msg.addr,
        count: msg.count,
        val_type: msg.val_type.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      addr: msg.addr,
      count: msg.count,
        val_type: msg.val_type.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      addr: msg.addr,
      count: msg.count,
      val_type: msg.val_type.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetInRegs_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInRegs_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetInRegs_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetInRegs_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetInRegs_Response {
  type RmwMsg = super::srv::rmw::GetInRegs_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetCoils_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetCoils_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub addr: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub count: i32,

}



impl Default for GetCoils_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetCoils_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetCoils_Request {
  type RmwMsg = super::srv::rmw::GetCoils_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        addr: msg.addr,
        count: msg.count,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      addr: msg.addr,
      count: msg.count,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      addr: msg.addr,
      count: msg.count,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetCoils_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetCoils_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetCoils_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetCoils_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetCoils_Response {
  type RmwMsg = super::srv::rmw::GetCoils_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetCoils_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCoils_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub addr: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub count: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub val_tab: std::string::String,

}



impl Default for SetCoils_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCoils_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetCoils_Request {
  type RmwMsg = super::srv::rmw::SetCoils_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        addr: msg.addr,
        count: msg.count,
        val_tab: msg.val_tab.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      addr: msg.addr,
      count: msg.count,
        val_tab: msg.val_tab.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      addr: msg.addr,
      count: msg.count,
      val_tab: msg.val_tab.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetCoils_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCoils_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetCoils_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetCoils_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetCoils_Response {
  type RmwMsg = super::srv::rmw::SetCoils_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetHoldRegs_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetHoldRegs_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub addr: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub count: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub val_type: std::string::String,

}



impl Default for GetHoldRegs_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetHoldRegs_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetHoldRegs_Request {
  type RmwMsg = super::srv::rmw::GetHoldRegs_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        addr: msg.addr,
        count: msg.count,
        val_type: msg.val_type.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      addr: msg.addr,
      count: msg.count,
        val_type: msg.val_type.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      addr: msg.addr,
      count: msg.count,
      val_type: msg.val_type.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetHoldRegs_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetHoldRegs_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetHoldRegs_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetHoldRegs_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetHoldRegs_Response {
  type RmwMsg = super::srv::rmw::GetHoldRegs_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetHoldRegs_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetHoldRegs_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub addr: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub count: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub val_tab: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub val_type: std::string::String,

}



impl Default for SetHoldRegs_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetHoldRegs_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetHoldRegs_Request {
  type RmwMsg = super::srv::rmw::SetHoldRegs_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        addr: msg.addr,
        count: msg.count,
        val_tab: msg.val_tab.as_str().into(),
        val_type: msg.val_type.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      addr: msg.addr,
      count: msg.count,
        val_tab: msg.val_tab.as_str().into(),
        val_type: msg.val_type.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      addr: msg.addr,
      count: msg.count,
      val_tab: msg.val_tab.to_string(),
      val_type: msg.val_type.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetHoldRegs_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetHoldRegs_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetHoldRegs_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetHoldRegs_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetHoldRegs_Response {
  type RmwMsg = super::srv::rmw::SetHoldRegs_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetSafeSkin_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetSafeSkin_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub part: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i32,

}



impl Default for SetSafeSkin_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetSafeSkin_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetSafeSkin_Request {
  type RmwMsg = super::srv::rmw::SetSafeSkin_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        part: msg.part,
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      part: msg.part,
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      part: msg.part,
      status: msg.status,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetSafeSkin_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetSafeSkin_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetSafeSkin_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetSafeSkin_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetSafeSkin_Response {
  type RmwMsg = super::srv::rmw::SetSafeSkin_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__MovJ_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub a: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for MovJ_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MovJ_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MovJ_Request {
  type RmwMsg = super::srv::rmw::MovJ_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        a: msg.a,
        b: msg.b,
        c: msg.c,
        d: msg.d,
        e: msg.e,
        f: msg.f,
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__MovJ_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for MovJ_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MovJ_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MovJ_Response {
  type RmwMsg = super::srv::rmw::MovJ_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__MovL_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub a: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for MovL_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MovL_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MovL_Request {
  type RmwMsg = super::srv::rmw::MovL_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        a: msg.a,
        b: msg.b,
        c: msg.c,
        d: msg.d,
        e: msg.e,
        f: msg.f,
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__MovL_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for MovL_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MovL_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MovL_Response {
  type RmwMsg = super::srv::rmw::MovL_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RelJointMovJ_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelJointMovJ_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub a: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for RelJointMovJ_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RelJointMovJ_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RelJointMovJ_Request {
  type RmwMsg = super::srv::rmw::RelJointMovJ_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        a: msg.a,
        b: msg.b,
        c: msg.c,
        d: msg.d,
        e: msg.e,
        f: msg.f,
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RelJointMovJ_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelJointMovJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RelJointMovJ_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RelJointMovJ_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RelJointMovJ_Response {
  type RmwMsg = super::srv::rmw::RelJointMovJ_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__MoveJog_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveJog_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub axis_id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for MoveJog_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MoveJog_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MoveJog_Request {
  type RmwMsg = super::srv::rmw::MoveJog_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        axis_id: msg.axis_id.as_str().into(),
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        axis_id: msg.axis_id.as_str().into(),
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      axis_id: msg.axis_id.to_string(),
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__MoveJog_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveJog_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for MoveJog_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MoveJog_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MoveJog_Response {
  type RmwMsg = super::srv::rmw::MoveJog_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__StopMoveJog_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StopMoveJog_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for StopMoveJog_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StopMoveJog_Request::default())
  }
}

impl rosidl_runtime_rs::Message for StopMoveJog_Request {
  type RmwMsg = super::srv::rmw::StopMoveJog_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__StopMoveJog_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StopMoveJog_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for StopMoveJog_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StopMoveJog_Response::default())
  }
}

impl rosidl_runtime_rs::Message for StopMoveJog_Response {
  type RmwMsg = super::srv::rmw::StopMoveJog_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DOGroup_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DOGroup_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub args: Vec<i32>,

}



impl Default for DOGroup_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DOGroup_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DOGroup_Request {
  type RmwMsg = super::srv::rmw::DOGroup_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        args: msg.args.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        args: msg.args.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      args: msg.args
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DOGroup_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DOGroup_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DOGroup_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DOGroup_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DOGroup_Response {
  type RmwMsg = super::srv::rmw::DOGroup_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__BrakeControl_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BrakeControl_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub axis_id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for BrakeControl_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::BrakeControl_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BrakeControl_Request {
  type RmwMsg = super::srv::rmw::BrakeControl_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        axis_id: msg.axis_id,
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      axis_id: msg.axis_id,
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      axis_id: msg.axis_id,
      value: msg.value,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__BrakeControl_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BrakeControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for BrakeControl_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::BrakeControl_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BrakeControl_Response {
  type RmwMsg = super::srv::rmw::BrakeControl_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__StartDrag_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartDrag_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for StartDrag_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StartDrag_Request::default())
  }
}

impl rosidl_runtime_rs::Message for StartDrag_Request {
  type RmwMsg = super::srv::rmw::StartDrag_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__StartDrag_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartDrag_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for StartDrag_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StartDrag_Response::default())
  }
}

impl rosidl_runtime_rs::Message for StartDrag_Response {
  type RmwMsg = super::srv::rmw::StartDrag_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__EnableSafeSkin_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnableSafeSkin_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i32,

}



impl Default for EnableSafeSkin_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EnableSafeSkin_Request::default())
  }
}

impl rosidl_runtime_rs::Message for EnableSafeSkin_Request {
  type RmwMsg = super::srv::rmw::EnableSafeSkin_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__EnableSafeSkin_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnableSafeSkin_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for EnableSafeSkin_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EnableSafeSkin_Response::default())
  }
}

impl rosidl_runtime_rs::Message for EnableSafeSkin_Response {
  type RmwMsg = super::srv::rmw::EnableSafeSkin_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetStartPose_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetStartPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trace_name: std::string::String,

}



impl Default for GetStartPose_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetStartPose_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetStartPose_Request {
  type RmwMsg = super::srv::rmw::GetStartPose_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trace_name: msg.trace_name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trace_name: msg.trace_name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      trace_name: msg.trace_name.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetStartPose_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetStartPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetStartPose_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetStartPose_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetStartPose_Response {
  type RmwMsg = super::srv::rmw::GetStartPose_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__StartPath_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartPath_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trace_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for StartPath_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StartPath_Request::default())
  }
}

impl rosidl_runtime_rs::Message for StartPath_Request {
  type RmwMsg = super::srv::rmw::StartPath_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trace_name: msg.trace_name.as_str().into(),
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trace_name: msg.trace_name.as_str().into(),
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      trace_name: msg.trace_name.to_string(),
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__StartPath_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartPath_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for StartPath_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StartPath_Response::default())
  }
}

impl rosidl_runtime_rs::Message for StartPath_Response {
  type RmwMsg = super::srv::rmw::StartPath_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__InverseSolution_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InverseSolution_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub parameter: std::string::String,

}



impl Default for InverseSolution_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::InverseSolution_Request::default())
  }
}

impl rosidl_runtime_rs::Message for InverseSolution_Request {
  type RmwMsg = super::srv::rmw::InverseSolution_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        parameter: msg.parameter.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        parameter: msg.parameter.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      parameter: msg.parameter.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__InverseSolution_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InverseSolution_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for InverseSolution_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::InverseSolution_Response::default())
  }
}

impl rosidl_runtime_rs::Message for InverseSolution_Response {
  type RmwMsg = super::srv::rmw::InverseSolution_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetErrorID_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetErrorID_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetErrorID_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetErrorID_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetErrorID_Request {
  type RmwMsg = super::srv::rmw::GetErrorID_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetErrorID_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetErrorID_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetErrorID_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetErrorID_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetErrorID_Response {
  type RmwMsg = super::srv::rmw::GetErrorID_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DI_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DI_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for DI_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DI_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DI_Request {
  type RmwMsg = super::srv::rmw::DI_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DI_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DI_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DI_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DI_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DI_Response {
  type RmwMsg = super::srv::rmw::DI_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ToolDI_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDI_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for ToolDI_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ToolDI_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ToolDI_Request {
  type RmwMsg = super::srv::rmw::ToolDI_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ToolDI_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDI_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ToolDI_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ToolDI_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ToolDI_Response {
  type RmwMsg = super::srv::rmw::ToolDI_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__AI_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AI_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for AI_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AI_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AI_Request {
  type RmwMsg = super::srv::rmw::AI_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__AI_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AI_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for AI_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AI_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AI_Response {
  type RmwMsg = super::srv::rmw::AI_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ToolAI_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolAI_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for ToolAI_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ToolAI_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ToolAI_Request {
  type RmwMsg = super::srv::rmw::ToolAI_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ToolAI_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolAI_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ToolAI_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ToolAI_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ToolAI_Response {
  type RmwMsg = super::srv::rmw::ToolAI_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DIGroup_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DIGroup_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub args: Vec<i32>,

}



impl Default for DIGroup_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DIGroup_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DIGroup_Request {
  type RmwMsg = super::srv::rmw::DIGroup_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        args: msg.args.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        args: msg.args.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      args: msg.args
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DIGroup_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DIGroup_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DIGroup_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DIGroup_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DIGroup_Response {
  type RmwMsg = super::srv::rmw::DIGroup_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__StopDrag_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StopDrag_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for StopDrag_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StopDrag_Request::default())
  }
}

impl rosidl_runtime_rs::Message for StopDrag_Request {
  type RmwMsg = super::srv::rmw::StopDrag_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__StopDrag_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StopDrag_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for StopDrag_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StopDrag_Response::default())
  }
}

impl rosidl_runtime_rs::Message for StopDrag_Response {
  type RmwMsg = super::srv::rmw::StopDrag_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DragSensivity_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DragSensivity_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for DragSensivity_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DragSensivity_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DragSensivity_Request {
  type RmwMsg = super::srv::rmw::DragSensivity_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      value: msg.value,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DragSensivity_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DragSensivity_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DragSensivity_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DragSensivity_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DragSensivity_Response {
  type RmwMsg = super::srv::rmw::DragSensivity_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetDO_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetDO_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for GetDO_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetDO_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetDO_Request {
  type RmwMsg = super::srv::rmw::GetDO_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetDO_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetDO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetDO_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetDO_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetDO_Response {
  type RmwMsg = super::srv::rmw::GetDO_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetAO_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetAO_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for GetAO_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetAO_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetAO_Request {
  type RmwMsg = super::srv::rmw::GetAO_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetAO_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetAO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetAO_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetAO_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetAO_Response {
  type RmwMsg = super::srv::rmw::GetAO_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetDOGroup_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetDOGroup_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index_group: Vec<i32>,

}



impl Default for GetDOGroup_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetDOGroup_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetDOGroup_Request {
  type RmwMsg = super::srv::rmw::GetDOGroup_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index_group: msg.index_group.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index_group: msg.index_group.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index_group: msg.index_group
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetDOGroup_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetDOGroup_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetDOGroup_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetDOGroup_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetDOGroup_Response {
  type RmwMsg = super::srv::rmw::GetDOGroup_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetTool485_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetTool485_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub baudrate: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub parity: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stop: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub identify: i32,

}



impl Default for SetTool485_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetTool485_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetTool485_Request {
  type RmwMsg = super::srv::rmw::SetTool485_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        baudrate: msg.baudrate,
        parity: msg.parity.as_str().into(),
        stop: msg.stop,
        identify: msg.identify,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      baudrate: msg.baudrate,
        parity: msg.parity.as_str().into(),
      stop: msg.stop,
      identify: msg.identify,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      baudrate: msg.baudrate,
      parity: msg.parity.to_string(),
      stop: msg.stop,
      identify: msg.identify,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetTool485_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetTool485_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetTool485_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetTool485_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetTool485_Response {
  type RmwMsg = super::srv::rmw::SetTool485_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetSafeWallEnable_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetSafeWallEnable_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for SetSafeWallEnable_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetSafeWallEnable_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetSafeWallEnable_Request {
  type RmwMsg = super::srv::rmw::SetSafeWallEnable_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      value: msg.value,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetSafeWallEnable_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetSafeWallEnable_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetSafeWallEnable_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetSafeWallEnable_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetSafeWallEnable_Response {
  type RmwMsg = super::srv::rmw::SetSafeWallEnable_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetToolPower_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetToolPower_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i32,

}



impl Default for SetToolPower_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetToolPower_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetToolPower_Request {
  type RmwMsg = super::srv::rmw::SetToolPower_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetToolPower_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetToolPower_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetToolPower_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetToolPower_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetToolPower_Response {
  type RmwMsg = super::srv::rmw::SetToolPower_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetToolMode_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetToolMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: i32,

}



impl Default for SetToolMode_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetToolMode_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetToolMode_Request {
  type RmwMsg = super::srv::rmw::SetToolMode_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        type_: msg.type_,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      type_: msg.type_,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      type_: msg.type_,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetToolMode_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetToolMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetToolMode_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetToolMode_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetToolMode_Response {
  type RmwMsg = super::srv::rmw::SetToolMode_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetBackDistance_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetBackDistance_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: f64,

}



impl Default for SetBackDistance_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetBackDistance_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetBackDistance_Request {
  type RmwMsg = super::srv::rmw::SetBackDistance_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        distance: msg.distance,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      distance: msg.distance,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      distance: msg.distance,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetBackDistance_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetBackDistance_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetBackDistance_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetBackDistance_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetBackDistance_Response {
  type RmwMsg = super::srv::rmw::SetBackDistance_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetPostCollisionMode_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPostCollisionMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: i32,

}



impl Default for SetPostCollisionMode_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetPostCollisionMode_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetPostCollisionMode_Request {
  type RmwMsg = super::srv::rmw::SetPostCollisionMode_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetPostCollisionMode_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPostCollisionMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetPostCollisionMode_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetPostCollisionMode_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetPostCollisionMode_Response {
  type RmwMsg = super::srv::rmw::SetPostCollisionMode_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetUser_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetUser_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: std::string::String,

}



impl Default for SetUser_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetUser_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetUser_Request {
  type RmwMsg = super::srv::rmw::SetUser_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        value: msg.value.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
        value: msg.value.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      value: msg.value.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetUser_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetUser_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetUser_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetUser_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetUser_Response {
  type RmwMsg = super::srv::rmw::SetUser_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetTool_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetTool_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: std::string::String,

}



impl Default for SetTool_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetTool_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetTool_Request {
  type RmwMsg = super::srv::rmw::SetTool_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        value: msg.value.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
        value: msg.value.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      value: msg.value.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetTool_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetTool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetTool_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetTool_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetTool_Response {
  type RmwMsg = super::srv::rmw::SetTool_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__CalcUser_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalcUser_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub matrix: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub offset: std::string::String,

}



impl Default for CalcUser_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CalcUser_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CalcUser_Request {
  type RmwMsg = super::srv::rmw::CalcUser_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        matrix: msg.matrix,
        offset: msg.offset.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      matrix: msg.matrix,
        offset: msg.offset.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      matrix: msg.matrix,
      offset: msg.offset.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__CalcUser_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalcUser_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for CalcUser_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CalcUser_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CalcUser_Response {
  type RmwMsg = super::srv::rmw::CalcUser_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__CalcTool_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalcTool_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub matrix: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub offset: std::string::String,

}



impl Default for CalcTool_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CalcTool_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CalcTool_Request {
  type RmwMsg = super::srv::rmw::CalcTool_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        matrix: msg.matrix,
        offset: msg.offset.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      matrix: msg.matrix,
        offset: msg.offset.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      matrix: msg.matrix,
      offset: msg.offset.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__CalcTool_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalcTool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for CalcTool_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CalcTool_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CalcTool_Response {
  type RmwMsg = super::srv::rmw::CalcTool_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetInputBool_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInputBool_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,

}



impl Default for GetInputBool_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetInputBool_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetInputBool_Request {
  type RmwMsg = super::srv::rmw::GetInputBool_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        address: msg.address,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      address: msg.address,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      address: msg.address,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetInputBool_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInputBool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetInputBool_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetInputBool_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetInputBool_Response {
  type RmwMsg = super::srv::rmw::GetInputBool_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetInputInt_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInputInt_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,

}



impl Default for GetInputInt_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetInputInt_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetInputInt_Request {
  type RmwMsg = super::srv::rmw::GetInputInt_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        address: msg.address,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      address: msg.address,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      address: msg.address,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetInputInt_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInputInt_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetInputInt_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetInputInt_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetInputInt_Response {
  type RmwMsg = super::srv::rmw::GetInputInt_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetInputFloat_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInputFloat_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,

}



impl Default for GetInputFloat_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetInputFloat_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetInputFloat_Request {
  type RmwMsg = super::srv::rmw::GetInputFloat_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        address: msg.address,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      address: msg.address,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      address: msg.address,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetInputFloat_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInputFloat_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetInputFloat_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetInputFloat_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetInputFloat_Response {
  type RmwMsg = super::srv::rmw::GetInputFloat_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetOutputBool_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOutputBool_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,

}



impl Default for GetOutputBool_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetOutputBool_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetOutputBool_Request {
  type RmwMsg = super::srv::rmw::GetOutputBool_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        address: msg.address,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      address: msg.address,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      address: msg.address,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetOutputBool_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOutputBool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetOutputBool_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetOutputBool_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetOutputBool_Response {
  type RmwMsg = super::srv::rmw::GetOutputBool_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetOutputInt_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOutputInt_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,

}



impl Default for GetOutputInt_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetOutputInt_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetOutputInt_Request {
  type RmwMsg = super::srv::rmw::GetOutputInt_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        address: msg.address,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      address: msg.address,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      address: msg.address,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetOutputInt_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOutputInt_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetOutputInt_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetOutputInt_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetOutputInt_Response {
  type RmwMsg = super::srv::rmw::GetOutputInt_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetOutputFloat_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOutputFloat_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,

}



impl Default for GetOutputFloat_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetOutputFloat_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetOutputFloat_Request {
  type RmwMsg = super::srv::rmw::GetOutputFloat_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        address: msg.address,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      address: msg.address,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      address: msg.address,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetOutputFloat_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOutputFloat_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetOutputFloat_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetOutputFloat_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetOutputFloat_Response {
  type RmwMsg = super::srv::rmw::GetOutputFloat_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetOutputBool_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOutputBool_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for SetOutputBool_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetOutputBool_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetOutputBool_Request {
  type RmwMsg = super::srv::rmw::SetOutputBool_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        address: msg.address,
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      address: msg.address,
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      address: msg.address,
      value: msg.value,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetOutputBool_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOutputBool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetOutputBool_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetOutputBool_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetOutputBool_Response {
  type RmwMsg = super::srv::rmw::SetOutputBool_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetOutputInt_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOutputInt_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for SetOutputInt_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetOutputInt_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetOutputInt_Request {
  type RmwMsg = super::srv::rmw::SetOutputInt_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        address: msg.address,
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      address: msg.address,
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      address: msg.address,
      value: msg.value,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetOutputInt_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOutputInt_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetOutputInt_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetOutputInt_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetOutputInt_Response {
  type RmwMsg = super::srv::rmw::SetOutputInt_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetOutputFloat_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOutputFloat_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: f64,

}



impl Default for SetOutputFloat_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetOutputFloat_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetOutputFloat_Request {
  type RmwMsg = super::srv::rmw::SetOutputFloat_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        address: msg.address,
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      address: msg.address,
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      address: msg.address,
      value: msg.value,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetOutputFloat_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOutputFloat_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetOutputFloat_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetOutputFloat_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetOutputFloat_Response {
  type RmwMsg = super::srv::rmw::SetOutputFloat_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__MovLIO_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub a: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mdis: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for MovLIO_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MovLIO_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MovLIO_Request {
  type RmwMsg = super::srv::rmw::MovLIO_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        a: msg.a,
        b: msg.b,
        c: msg.c,
        d: msg.d,
        e: msg.e,
        f: msg.f,
        mdis: msg.mdis
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
        mdis: msg.mdis
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      mdis: msg.mdis
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__MovLIO_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for MovLIO_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MovLIO_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MovLIO_Response {
  type RmwMsg = super::srv::rmw::MovLIO_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__MovJIO_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub a: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mdis: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for MovJIO_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MovJIO_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MovJIO_Request {
  type RmwMsg = super::srv::rmw::MovJIO_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        a: msg.a,
        b: msg.b,
        c: msg.c,
        d: msg.d,
        e: msg.e,
        f: msg.f,
        mdis: msg.mdis
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
        mdis: msg.mdis
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      mdis: msg.mdis
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__MovJIO_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for MovJIO_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MovJIO_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MovJIO_Response {
  type RmwMsg = super::srv::rmw::MovJIO_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__Arc_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Arc_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub a: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub a2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for Arc_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Arc_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Arc_Request {
  type RmwMsg = super::srv::rmw::Arc_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        a: msg.a,
        b: msg.b,
        c: msg.c,
        d: msg.d,
        e: msg.e,
        f: msg.f,
        a2: msg.a2,
        b2: msg.b2,
        c2: msg.c2,
        d2: msg.d2,
        e2: msg.e2,
        f2: msg.f2,
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      a2: msg.a2,
      b2: msg.b2,
      c2: msg.c2,
      d2: msg.d2,
      e2: msg.e2,
      f2: msg.f2,
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      a2: msg.a2,
      b2: msg.b2,
      c2: msg.c2,
      d2: msg.d2,
      e2: msg.e2,
      f2: msg.f2,
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__Arc_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Arc_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for Arc_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Arc_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Arc_Response {
  type RmwMsg = super::srv::rmw::Arc_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__Circle_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Circle_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub a: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub a2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub count: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for Circle_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Circle_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Circle_Request {
  type RmwMsg = super::srv::rmw::Circle_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
        a: msg.a,
        b: msg.b,
        c: msg.c,
        d: msg.d,
        e: msg.e,
        f: msg.f,
        a2: msg.a2,
        b2: msg.b2,
        c2: msg.c2,
        d2: msg.d2,
        e2: msg.e2,
        f2: msg.f2,
        count: msg.count,
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      a2: msg.a2,
      b2: msg.b2,
      c2: msg.c2,
      d2: msg.d2,
      e2: msg.e2,
      f2: msg.f2,
      count: msg.count,
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      a2: msg.a2,
      b2: msg.b2,
      c2: msg.c2,
      d2: msg.d2,
      e2: msg.e2,
      f2: msg.f2,
      count: msg.count,
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__Circle_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Circle_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for Circle_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Circle_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Circle_Response {
  type RmwMsg = super::srv::rmw::Circle_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RelMovJTool_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelMovJTool_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub a: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for RelMovJTool_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RelMovJTool_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RelMovJTool_Request {
  type RmwMsg = super::srv::rmw::RelMovJTool_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        a: msg.a,
        b: msg.b,
        c: msg.c,
        d: msg.d,
        e: msg.e,
        f: msg.f,
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RelMovJTool_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelMovJTool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RelMovJTool_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RelMovJTool_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RelMovJTool_Response {
  type RmwMsg = super::srv::rmw::RelMovJTool_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RelMovLTool_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelMovLTool_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub a: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for RelMovLTool_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RelMovLTool_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RelMovLTool_Request {
  type RmwMsg = super::srv::rmw::RelMovLTool_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        a: msg.a,
        b: msg.b,
        c: msg.c,
        d: msg.d,
        e: msg.e,
        f: msg.f,
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RelMovLTool_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelMovLTool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RelMovLTool_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RelMovLTool_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RelMovLTool_Response {
  type RmwMsg = super::srv::rmw::RelMovLTool_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RelMovJUser_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelMovJUser_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub a: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for RelMovJUser_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RelMovJUser_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RelMovJUser_Request {
  type RmwMsg = super::srv::rmw::RelMovJUser_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        a: msg.a,
        b: msg.b,
        c: msg.c,
        d: msg.d,
        e: msg.e,
        f: msg.f,
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RelMovJUser_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelMovJUser_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RelMovJUser_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RelMovJUser_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RelMovJUser_Response {
  type RmwMsg = super::srv::rmw::RelMovJUser_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RelMovLUser_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelMovLUser_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub a: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for RelMovLUser_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RelMovLUser_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RelMovLUser_Request {
  type RmwMsg = super::srv::rmw::RelMovLUser_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        a: msg.a,
        b: msg.b,
        c: msg.c,
        d: msg.d,
        e: msg.e,
        f: msg.f,
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RelMovLUser_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelMovLUser_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RelMovLUser_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RelMovLUser_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RelMovLUser_Response {
  type RmwMsg = super::srv::rmw::RelMovLUser_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetCurrentCommandId_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetCurrentCommandId_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetCurrentCommandId_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetCurrentCommandId_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetCurrentCommandId_Request {
  type RmwMsg = super::srv::rmw::GetCurrentCommandId_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetCurrentCommandId_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetCurrentCommandId_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetCurrentCommandId_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetCurrentCommandId_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetCurrentCommandId_Response {
  type RmwMsg = super::srv::rmw::GetCurrentCommandId_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ServoJ_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ServoJ_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub a: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for ServoJ_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ServoJ_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ServoJ_Request {
  type RmwMsg = super::srv::rmw::ServoJ_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        a: msg.a,
        b: msg.b,
        c: msg.c,
        d: msg.d,
        e: msg.e,
        f: msg.f,
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ServoJ_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ServoJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ServoJ_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ServoJ_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ServoJ_Response {
  type RmwMsg = super::srv::rmw::ServoJ_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ServoP_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ServoP_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub a: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for ServoP_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ServoP_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ServoP_Request {
  type RmwMsg = super::srv::rmw::ServoP_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        a: msg.a,
        b: msg.b,
        c: msg.c,
        d: msg.d,
        e: msg.e,
        f: msg.f,
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      d: msg.d,
      e: msg.e,
      f: msg.f,
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ServoP_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ServoP_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ServoP_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ServoP_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ServoP_Response {
  type RmwMsg = super::srv::rmw::ServoP_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__TcpDashboard_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TcpDashboard_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: std::string::String,

}



impl Default for TcpDashboard_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::TcpDashboard_Request::default())
  }
}

impl rosidl_runtime_rs::Message for TcpDashboard_Request {
  type RmwMsg = super::srv::rmw::TcpDashboard_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        command: msg.command.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        command: msg.command.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      command: msg.command.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__TcpDashboard_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TcpDashboard_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: std::string::String,

}



impl Default for TcpDashboard_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::TcpDashboard_Response::default())
  }
}

impl rosidl_runtime_rs::Message for TcpDashboard_Response {
  type RmwMsg = super::srv::rmw::TcpDashboard_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__EnableFTSensor_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnableFTSensor_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i32,

}



impl Default for EnableFTSensor_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EnableFTSensor_Request::default())
  }
}

impl rosidl_runtime_rs::Message for EnableFTSensor_Request {
  type RmwMsg = super::srv::rmw::EnableFTSensor_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__EnableFTSensor_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnableFTSensor_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for EnableFTSensor_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EnableFTSensor_Response::default())
  }
}

impl rosidl_runtime_rs::Message for EnableFTSensor_Response {
  type RmwMsg = super::srv::rmw::EnableFTSensor_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SixForceHome_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SixForceHome_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SixForceHome_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SixForceHome_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SixForceHome_Request {
  type RmwMsg = super::srv::rmw::SixForceHome_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SixForceHome_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SixForceHome_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SixForceHome_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SixForceHome_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SixForceHome_Response {
  type RmwMsg = super::srv::rmw::SixForceHome_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetForce_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetForce_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub tool: i32,

}



impl Default for GetForce_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetForce_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetForce_Request {
  type RmwMsg = super::srv::rmw::GetForce_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        tool: msg.tool,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      tool: msg.tool,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      tool: msg.tool,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetForce_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetForce_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetForce_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetForce_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetForce_Response {
  type RmwMsg = super::srv::rmw::GetForce_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ForceDriveMode_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceDriveMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rx: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rz: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub user: i32,

}



impl Default for ForceDriveMode_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ForceDriveMode_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ForceDriveMode_Request {
  type RmwMsg = super::srv::rmw::ForceDriveMode_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        rx: msg.rx,
        ry: msg.ry,
        rz: msg.rz,
        user: msg.user,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      user: msg.user,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      user: msg.user,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ForceDriveMode_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceDriveMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ForceDriveMode_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ForceDriveMode_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ForceDriveMode_Response {
  type RmwMsg = super::srv::rmw::ForceDriveMode_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ForceDriveSpeed_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceDriveSpeed_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub speed: i32,

}



impl Default for ForceDriveSpeed_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ForceDriveSpeed_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ForceDriveSpeed_Request {
  type RmwMsg = super::srv::rmw::ForceDriveSpeed_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        speed: msg.speed,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      speed: msg.speed,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      speed: msg.speed,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ForceDriveSpeed_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceDriveSpeed_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ForceDriveSpeed_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ForceDriveSpeed_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ForceDriveSpeed_Response {
  type RmwMsg = super::srv::rmw::ForceDriveSpeed_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCForceMode_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCForceMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rx: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rz: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub fx: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub fy: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub fz: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frx: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub fry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frz: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reference: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub user: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tool: i32,

}



impl Default for FCForceMode_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCForceMode_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FCForceMode_Request {
  type RmwMsg = super::srv::rmw::FCForceMode_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        rx: msg.rx,
        ry: msg.ry,
        rz: msg.rz,
        fx: msg.fx,
        fy: msg.fy,
        fz: msg.fz,
        frx: msg.frx,
        fry: msg.fry,
        frz: msg.frz,
        reference: msg.reference,
        user: msg.user,
        tool: msg.tool,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      fx: msg.fx,
      fy: msg.fy,
      fz: msg.fz,
      frx: msg.frx,
      fry: msg.fry,
      frz: msg.frz,
      reference: msg.reference,
      user: msg.user,
      tool: msg.tool,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      fx: msg.fx,
      fy: msg.fy,
      fz: msg.fz,
      frx: msg.frx,
      fry: msg.fry,
      frz: msg.frz,
      reference: msg.reference,
      user: msg.user,
      tool: msg.tool,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCForceMode_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCForceMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCForceMode_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCForceMode_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FCForceMode_Response {
  type RmwMsg = super::srv::rmw::FCForceMode_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCSetDeviation_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetDeviation_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rx: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rz: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub controltype: i32,

}



impl Default for FCSetDeviation_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCSetDeviation_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FCSetDeviation_Request {
  type RmwMsg = super::srv::rmw::FCSetDeviation_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        rx: msg.rx,
        ry: msg.ry,
        rz: msg.rz,
        controltype: msg.controltype,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      controltype: msg.controltype,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      controltype: msg.controltype,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCSetDeviation_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetDeviation_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCSetDeviation_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCSetDeviation_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FCSetDeviation_Response {
  type RmwMsg = super::srv::rmw::FCSetDeviation_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCSetForceLimit_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetForceLimit_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rx: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rz: i32,

}



impl Default for FCSetForceLimit_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCSetForceLimit_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FCSetForceLimit_Request {
  type RmwMsg = super::srv::rmw::FCSetForceLimit_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        rx: msg.rx,
        ry: msg.ry,
        rz: msg.rz,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCSetForceLimit_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetForceLimit_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCSetForceLimit_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCSetForceLimit_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FCSetForceLimit_Response {
  type RmwMsg = super::srv::rmw::FCSetForceLimit_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCSetMass_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetMass_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rx: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rz: i32,

}



impl Default for FCSetMass_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCSetMass_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FCSetMass_Request {
  type RmwMsg = super::srv::rmw::FCSetMass_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        rx: msg.rx,
        ry: msg.ry,
        rz: msg.rz,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCSetMass_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetMass_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCSetMass_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCSetMass_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FCSetMass_Response {
  type RmwMsg = super::srv::rmw::FCSetMass_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCSetStiffness_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetStiffness_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rx: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rz: i32,

}



impl Default for FCSetStiffness_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCSetStiffness_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FCSetStiffness_Request {
  type RmwMsg = super::srv::rmw::FCSetStiffness_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        rx: msg.rx,
        ry: msg.ry,
        rz: msg.rz,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCSetStiffness_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetStiffness_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCSetStiffness_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCSetStiffness_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FCSetStiffness_Response {
  type RmwMsg = super::srv::rmw::FCSetStiffness_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCSetDamping_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetDamping_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rx: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rz: i32,

}



impl Default for FCSetDamping_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCSetDamping_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FCSetDamping_Request {
  type RmwMsg = super::srv::rmw::FCSetDamping_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        rx: msg.rx,
        ry: msg.ry,
        rz: msg.rz,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCSetDamping_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetDamping_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCSetDamping_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCSetDamping_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FCSetDamping_Response {
  type RmwMsg = super::srv::rmw::FCSetDamping_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCOff_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCOff_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for FCOff_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCOff_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FCOff_Request {
  type RmwMsg = super::srv::rmw::FCOff_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCOff_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCOff_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCOff_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCOff_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FCOff_Response {
  type RmwMsg = super::srv::rmw::FCOff_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCSetForceSpeedLimit_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetForceSpeedLimit_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rx: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rz: i32,

}



impl Default for FCSetForceSpeedLimit_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCSetForceSpeedLimit_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FCSetForceSpeedLimit_Request {
  type RmwMsg = super::srv::rmw::FCSetForceSpeedLimit_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        rx: msg.rx,
        ry: msg.ry,
        rz: msg.rz,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCSetForceSpeedLimit_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetForceSpeedLimit_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCSetForceSpeedLimit_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCSetForceSpeedLimit_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FCSetForceSpeedLimit_Response {
  type RmwMsg = super::srv::rmw::FCSetForceSpeedLimit_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCSetForce_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetForce_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rx: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rz: i32,

}



impl Default for FCSetForce_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCSetForce_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FCSetForce_Request {
  type RmwMsg = super::srv::rmw::FCSetForce_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        rx: msg.rx,
        ry: msg.ry,
        rz: msg.rz,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      rx: msg.rx,
      ry: msg.ry,
      rz: msg.rz,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCSetForce_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetForce_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCSetForce_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCSetForce_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FCSetForce_Response {
  type RmwMsg = super::srv::rmw::FCSetForce_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetFCCollision_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetFCCollision_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub force: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub torque: f64,

}



impl Default for SetFCCollision_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetFCCollision_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetFCCollision_Request {
  type RmwMsg = super::srv::rmw::SetFCCollision_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        force: msg.force,
        torque: msg.torque,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      force: msg.force,
      torque: msg.torque,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      force: msg.force,
      torque: msg.torque,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetFCCollision_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetFCCollision_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetFCCollision_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetFCCollision_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetFCCollision_Response {
  type RmwMsg = super::srv::rmw::SetFCCollision_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCCollisionSwitch_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCCollisionSwitch_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub enable: i32,

}



impl Default for FCCollisionSwitch_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCCollisionSwitch_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FCCollisionSwitch_Request {
  type RmwMsg = super::srv::rmw::FCCollisionSwitch_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        enable: msg.enable,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      enable: msg.enable,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      enable: msg.enable,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__FCCollisionSwitch_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCCollisionSwitch_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCCollisionSwitch_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FCCollisionSwitch_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FCCollisionSwitch_Response {
  type RmwMsg = super::srv::rmw::FCCollisionSwitch_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetWorkZoneEnable_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetWorkZoneEnable_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for SetWorkZoneEnable_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetWorkZoneEnable_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetWorkZoneEnable_Request {
  type RmwMsg = super::srv::rmw::SetWorkZoneEnable_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      value: msg.value,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__SetWorkZoneEnable_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetWorkZoneEnable_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetWorkZoneEnable_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetWorkZoneEnable_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetWorkZoneEnable_Response {
  type RmwMsg = super::srv::rmw::SetWorkZoneEnable_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetToolDO_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetToolDO_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for GetToolDO_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetToolDO_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetToolDO_Request {
  type RmwMsg = super::srv::rmw::GetToolDO_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetToolDO_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetToolDO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetToolDO_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetToolDO_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetToolDO_Response {
  type RmwMsg = super::srv::rmw::GetToolDO_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ResetRobot_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResetRobot_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ResetRobot_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ResetRobot_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ResetRobot_Request {
  type RmwMsg = super::srv::rmw::ResetRobot_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__ResetRobot_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResetRobot_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ResetRobot_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ResetRobot_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ResetRobot_Response {
  type RmwMsg = super::srv::rmw::ResetRobot_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RunTo_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RunTo_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub a1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub d1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub e1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub f1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub move_type: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub user: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tool: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub a: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub v: i32,

}



impl Default for RunTo_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RunTo_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RunTo_Request {
  type RmwMsg = super::srv::rmw::RunTo_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        a1: msg.a1,
        b1: msg.b1,
        c1: msg.c1,
        d1: msg.d1,
        e1: msg.e1,
        f1: msg.f1,
        move_type: msg.move_type,
        user: msg.user,
        tool: msg.tool,
        a: msg.a,
        v: msg.v,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      a1: msg.a1,
      b1: msg.b1,
      c1: msg.c1,
      d1: msg.d1,
      e1: msg.e1,
      f1: msg.f1,
      move_type: msg.move_type,
      user: msg.user,
      tool: msg.tool,
      a: msg.a,
      v: msg.v,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      a1: msg.a1,
      b1: msg.b1,
      c1: msg.c1,
      d1: msg.d1,
      e1: msg.e1,
      f1: msg.f1,
      move_type: msg.move_type,
      user: msg.user,
      tool: msg.tool,
      a: msg.a,
      v: msg.v,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RunTo_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RunTo_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RunTo_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RunTo_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RunTo_Response {
  type RmwMsg = super::srv::rmw::RunTo_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__StartRTOffset_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartRTOffset_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for StartRTOffset_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StartRTOffset_Request::default())
  }
}

impl rosidl_runtime_rs::Message for StartRTOffset_Request {
  type RmwMsg = super::srv::rmw::StartRTOffset_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__StartRTOffset_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartRTOffset_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for StartRTOffset_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StartRTOffset_Response::default())
  }
}

impl rosidl_runtime_rs::Message for StartRTOffset_Response {
  type RmwMsg = super::srv::rmw::StartRTOffset_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__EndRTOffset_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EndRTOffset_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for EndRTOffset_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EndRTOffset_Request::default())
  }
}

impl rosidl_runtime_rs::Message for EndRTOffset_Request {
  type RmwMsg = super::srv::rmw::EndRTOffset_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__EndRTOffset_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EndRTOffset_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for EndRTOffset_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EndRTOffset_Response::default())
  }
}

impl rosidl_runtime_rs::Message for EndRTOffset_Response {
  type RmwMsg = super::srv::rmw::EndRTOffset_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetError_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetError_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub language: std::string::String,

}



impl Default for GetError_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetError_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetError_Request {
  type RmwMsg = super::srv::rmw::GetError_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        language: msg.language.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        language: msg.language.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      language: msg.language.to_string(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetError_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetError_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetError_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetError_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetError_Response {
  type RmwMsg = super::srv::rmw::GetError_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DOGroupDEC_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DOGroupDEC_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub group: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for DOGroupDEC_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DOGroupDEC_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DOGroupDEC_Request {
  type RmwMsg = super::srv::rmw::DOGroupDEC_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group: msg.group,
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      group: msg.group,
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      group: msg.group,
      value: msg.value,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DOGroupDEC_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DOGroupDEC_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DOGroupDEC_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DOGroupDEC_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DOGroupDEC_Response {
  type RmwMsg = super::srv::rmw::DOGroupDEC_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetDOGroupDEC_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetDOGroupDEC_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub group: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for GetDOGroupDEC_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetDOGroupDEC_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetDOGroupDEC_Request {
  type RmwMsg = super::srv::rmw::GetDOGroupDEC_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group: msg.group,
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      group: msg.group,
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      group: msg.group,
      value: msg.value,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__GetDOGroupDEC_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetDOGroupDEC_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetDOGroupDEC_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetDOGroupDEC_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetDOGroupDEC_Response {
  type RmwMsg = super::srv::rmw::GetDOGroupDEC_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DIGroupDEC_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DIGroupDEC_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub group: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for DIGroupDEC_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DIGroupDEC_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DIGroupDEC_Request {
  type RmwMsg = super::srv::rmw::DIGroupDEC_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        group: msg.group,
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      group: msg.group,
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      group: msg.group,
      value: msg.value,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__DIGroupDEC_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DIGroupDEC_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DIGroupDEC_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DIGroupDEC_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DIGroupDEC_Response {
  type RmwMsg = super::srv::rmw::DIGroupDEC_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RequestControl_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RequestControl_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for RequestControl_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RequestControl_Request::default())
  }
}

impl rosidl_runtime_rs::Message for RequestControl_Request {
  type RmwMsg = super::srv::rmw::RequestControl_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__RequestControl_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RequestControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RequestControl_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::RequestControl_Response::default())
  }
}

impl rosidl_runtime_rs::Message for RequestControl_Response {
  type RmwMsg = super::srv::rmw::RequestControl_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__CheckOddMovL_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckOddMovL_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j3: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j4: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j5: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j6: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j3: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j4: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j5: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j6: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for CheckOddMovL_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CheckOddMovL_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CheckOddMovL_Request {
  type RmwMsg = super::srv::rmw::CheckOddMovL_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        point1_j1: msg.point1_j1,
        point1_j2: msg.point1_j2,
        point1_j3: msg.point1_j3,
        point1_j4: msg.point1_j4,
        point1_j5: msg.point1_j5,
        point1_j6: msg.point1_j6,
        point2_j1: msg.point2_j1,
        point2_j2: msg.point2_j2,
        point2_j3: msg.point2_j3,
        point2_j4: msg.point2_j4,
        point2_j5: msg.point2_j5,
        point2_j6: msg.point2_j6,
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      point1_j1: msg.point1_j1,
      point1_j2: msg.point1_j2,
      point1_j3: msg.point1_j3,
      point1_j4: msg.point1_j4,
      point1_j5: msg.point1_j5,
      point1_j6: msg.point1_j6,
      point2_j1: msg.point2_j1,
      point2_j2: msg.point2_j2,
      point2_j3: msg.point2_j3,
      point2_j4: msg.point2_j4,
      point2_j5: msg.point2_j5,
      point2_j6: msg.point2_j6,
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      point1_j1: msg.point1_j1,
      point1_j2: msg.point1_j2,
      point1_j3: msg.point1_j3,
      point1_j4: msg.point1_j4,
      point1_j5: msg.point1_j5,
      point1_j6: msg.point1_j6,
      point2_j1: msg.point2_j1,
      point2_j2: msg.point2_j2,
      point2_j3: msg.point2_j3,
      point2_j4: msg.point2_j4,
      point2_j5: msg.point2_j5,
      point2_j6: msg.point2_j6,
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__CheckOddMovL_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckOddMovL_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for CheckOddMovL_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CheckOddMovL_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CheckOddMovL_Response {
  type RmwMsg = super::srv::rmw::CheckOddMovL_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__CheckOddMovJ_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckOddMovJ_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j3: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j4: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j5: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j6: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j3: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j4: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j5: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j6: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for CheckOddMovJ_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CheckOddMovJ_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CheckOddMovJ_Request {
  type RmwMsg = super::srv::rmw::CheckOddMovJ_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        point1_j1: msg.point1_j1,
        point1_j2: msg.point1_j2,
        point1_j3: msg.point1_j3,
        point1_j4: msg.point1_j4,
        point1_j5: msg.point1_j5,
        point1_j6: msg.point1_j6,
        point2_j1: msg.point2_j1,
        point2_j2: msg.point2_j2,
        point2_j3: msg.point2_j3,
        point2_j4: msg.point2_j4,
        point2_j5: msg.point2_j5,
        point2_j6: msg.point2_j6,
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      point1_j1: msg.point1_j1,
      point1_j2: msg.point1_j2,
      point1_j3: msg.point1_j3,
      point1_j4: msg.point1_j4,
      point1_j5: msg.point1_j5,
      point1_j6: msg.point1_j6,
      point2_j1: msg.point2_j1,
      point2_j2: msg.point2_j2,
      point2_j3: msg.point2_j3,
      point2_j4: msg.point2_j4,
      point2_j5: msg.point2_j5,
      point2_j6: msg.point2_j6,
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      point1_j1: msg.point1_j1,
      point1_j2: msg.point1_j2,
      point1_j3: msg.point1_j3,
      point1_j4: msg.point1_j4,
      point1_j5: msg.point1_j5,
      point1_j6: msg.point1_j6,
      point2_j1: msg.point2_j1,
      point2_j2: msg.point2_j2,
      point2_j3: msg.point2_j3,
      point2_j4: msg.point2_j4,
      point2_j5: msg.point2_j5,
      point2_j6: msg.point2_j6,
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__CheckOddMovJ_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckOddMovJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for CheckOddMovJ_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CheckOddMovJ_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CheckOddMovJ_Response {
  type RmwMsg = super::srv::rmw::CheckOddMovJ_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__CheckOddMovC_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckOddMovC_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j3: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j4: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j5: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point1_j6: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j3: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j4: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j5: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point2_j6: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point3_j1: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point3_j2: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point3_j3: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point3_j4: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point3_j5: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub point3_j6: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: Vec<std::string::String>,

}



impl Default for CheckOddMovC_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CheckOddMovC_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CheckOddMovC_Request {
  type RmwMsg = super::srv::rmw::CheckOddMovC_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        point1_j1: msg.point1_j1,
        point1_j2: msg.point1_j2,
        point1_j3: msg.point1_j3,
        point1_j4: msg.point1_j4,
        point1_j5: msg.point1_j5,
        point1_j6: msg.point1_j6,
        point2_j1: msg.point2_j1,
        point2_j2: msg.point2_j2,
        point2_j3: msg.point2_j3,
        point2_j4: msg.point2_j4,
        point2_j5: msg.point2_j5,
        point2_j6: msg.point2_j6,
        point3_j1: msg.point3_j1,
        point3_j2: msg.point3_j2,
        point3_j3: msg.point3_j3,
        point3_j4: msg.point3_j4,
        point3_j5: msg.point3_j5,
        point3_j6: msg.point3_j6,
        param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      point1_j1: msg.point1_j1,
      point1_j2: msg.point1_j2,
      point1_j3: msg.point1_j3,
      point1_j4: msg.point1_j4,
      point1_j5: msg.point1_j5,
      point1_j6: msg.point1_j6,
      point2_j1: msg.point2_j1,
      point2_j2: msg.point2_j2,
      point2_j3: msg.point2_j3,
      point2_j4: msg.point2_j4,
      point2_j5: msg.point2_j5,
      point2_j6: msg.point2_j6,
      point3_j1: msg.point3_j1,
      point3_j2: msg.point3_j2,
      point3_j3: msg.point3_j3,
      point3_j4: msg.point3_j4,
      point3_j5: msg.point3_j5,
      point3_j6: msg.point3_j6,
        param_value: msg.param_value
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      point1_j1: msg.point1_j1,
      point1_j2: msg.point1_j2,
      point1_j3: msg.point1_j3,
      point1_j4: msg.point1_j4,
      point1_j5: msg.point1_j5,
      point1_j6: msg.point1_j6,
      point2_j1: msg.point2_j1,
      point2_j2: msg.point2_j2,
      point2_j3: msg.point2_j3,
      point2_j4: msg.point2_j4,
      point2_j5: msg.point2_j5,
      point2_j6: msg.point2_j6,
      point3_j1: msg.point3_j1,
      point3_j2: msg.point3_j2,
      point3_j3: msg.point3_j3,
      point3_j4: msg.point3_j4,
      point3_j5: msg.point3_j5,
      point3_j6: msg.point3_j6,
      param_value: msg.param_value
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to dobot_msgs_v4__srv__CheckOddMovC_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckOddMovC_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for CheckOddMovC_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CheckOddMovC_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CheckOddMovC_Response {
  type RmwMsg = super::srv::rmw::CheckOddMovC_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
        res: msg.res,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        robot_return: msg.robot_return.as_str().into(),
      res: msg.res,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      robot_return: msg.robot_return.to_string(),
      res: msg.res,
    }
  }
}






#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__EnableRobot() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__EnableRobot
#[allow(missing_docs, non_camel_case_types)]
pub struct EnableRobot;

impl rosidl_runtime_rs::Service for EnableRobot {
    type Request = EnableRobot_Request;
    type Response = EnableRobot_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__EnableRobot() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DisableRobot() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__DisableRobot
#[allow(missing_docs, non_camel_case_types)]
pub struct DisableRobot;

impl rosidl_runtime_rs::Service for DisableRobot {
    type Request = DisableRobot_Request;
    type Response = DisableRobot_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DisableRobot() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ClearError() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__ClearError
#[allow(missing_docs, non_camel_case_types)]
pub struct ClearError;

impl rosidl_runtime_rs::Service for ClearError {
    type Request = ClearError_Request;
    type Response = ClearError_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ClearError() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SpeedFactor() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SpeedFactor
#[allow(missing_docs, non_camel_case_types)]
pub struct SpeedFactor;

impl rosidl_runtime_rs::Service for SpeedFactor {
    type Request = SpeedFactor_Request;
    type Response = SpeedFactor_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SpeedFactor() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__User() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__User
#[allow(missing_docs, non_camel_case_types)]
pub struct User;

impl rosidl_runtime_rs::Service for User {
    type Request = User_Request;
    type Response = User_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__User() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__Tool() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__Tool
#[allow(missing_docs, non_camel_case_types)]
pub struct Tool;

impl rosidl_runtime_rs::Service for Tool {
    type Request = Tool_Request;
    type Response = Tool_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__Tool() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RobotMode() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__RobotMode
#[allow(missing_docs, non_camel_case_types)]
pub struct RobotMode;

impl rosidl_runtime_rs::Service for RobotMode {
    type Request = RobotMode_Request;
    type Response = RobotMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RobotMode() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetPayload() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetPayload
#[allow(missing_docs, non_camel_case_types)]
pub struct SetPayload;

impl rosidl_runtime_rs::Service for SetPayload {
    type Request = SetPayload_Request;
    type Response = SetPayload_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetPayload() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DO() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__DO
#[allow(missing_docs, non_camel_case_types)]
pub struct DO;

impl rosidl_runtime_rs::Service for DO {
    type Request = DO_Request;
    type Response = DO_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DO() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DOInstant() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__DOInstant
#[allow(missing_docs, non_camel_case_types)]
pub struct DOInstant;

impl rosidl_runtime_rs::Service for DOInstant {
    type Request = DOInstant_Request;
    type Response = DOInstant_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DOInstant() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ToolDO() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__ToolDO
#[allow(missing_docs, non_camel_case_types)]
pub struct ToolDO;

impl rosidl_runtime_rs::Service for ToolDO {
    type Request = ToolDO_Request;
    type Response = ToolDO_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ToolDO() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ToolDOInstant() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__ToolDOInstant
#[allow(missing_docs, non_camel_case_types)]
pub struct ToolDOInstant;

impl rosidl_runtime_rs::Service for ToolDOInstant {
    type Request = ToolDOInstant_Request;
    type Response = ToolDOInstant_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ToolDOInstant() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__AO() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__AO
#[allow(missing_docs, non_camel_case_types)]
pub struct AO;

impl rosidl_runtime_rs::Service for AO {
    type Request = AO_Request;
    type Response = AO_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__AO() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__AOInstant() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__AOInstant
#[allow(missing_docs, non_camel_case_types)]
pub struct AOInstant;

impl rosidl_runtime_rs::Service for AOInstant {
    type Request = AOInstant_Request;
    type Response = AOInstant_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__AOInstant() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__AccJ() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__AccJ
#[allow(missing_docs, non_camel_case_types)]
pub struct AccJ;

impl rosidl_runtime_rs::Service for AccJ {
    type Request = AccJ_Request;
    type Response = AccJ_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__AccJ() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__AccL() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__AccL
#[allow(missing_docs, non_camel_case_types)]
pub struct AccL;

impl rosidl_runtime_rs::Service for AccL {
    type Request = AccL_Request;
    type Response = AccL_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__AccL() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__VelJ() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__VelJ
#[allow(missing_docs, non_camel_case_types)]
pub struct VelJ;

impl rosidl_runtime_rs::Service for VelJ {
    type Request = VelJ_Request;
    type Response = VelJ_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__VelJ() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__VelL() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__VelL
#[allow(missing_docs, non_camel_case_types)]
pub struct VelL;

impl rosidl_runtime_rs::Service for VelL {
    type Request = VelL_Request;
    type Response = VelL_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__VelL() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__CP() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__CP
#[allow(missing_docs, non_camel_case_types)]
pub struct CP;

impl rosidl_runtime_rs::Service for CP {
    type Request = CP_Request;
    type Response = CP_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__CP() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__PowerOn() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__PowerOn
#[allow(missing_docs, non_camel_case_types)]
pub struct PowerOn;

impl rosidl_runtime_rs::Service for PowerOn {
    type Request = PowerOn_Request;
    type Response = PowerOn_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__PowerOn() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RunScript() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__RunScript
#[allow(missing_docs, non_camel_case_types)]
pub struct RunScript;

impl rosidl_runtime_rs::Service for RunScript {
    type Request = RunScript_Request;
    type Response = RunScript_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RunScript() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__Stop() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__Stop
#[allow(missing_docs, non_camel_case_types)]
pub struct Stop;

impl rosidl_runtime_rs::Service for Stop {
    type Request = Stop_Request;
    type Response = Stop_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__Stop() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__Pause() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__Pause
#[allow(missing_docs, non_camel_case_types)]
pub struct Pause;

impl rosidl_runtime_rs::Service for Pause {
    type Request = Pause_Request;
    type Response = Pause_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__Pause() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__Continue() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__Continue
#[allow(missing_docs, non_camel_case_types)]
pub struct Continue;

impl rosidl_runtime_rs::Service for Continue {
    type Request = Continue_Request;
    type Response = Continue_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__Continue() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__PositiveKin() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__PositiveKin
#[allow(missing_docs, non_camel_case_types)]
pub struct PositiveKin;

impl rosidl_runtime_rs::Service for PositiveKin {
    type Request = PositiveKin_Request;
    type Response = PositiveKin_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__PositiveKin() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__InverseKin() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__InverseKin
#[allow(missing_docs, non_camel_case_types)]
pub struct InverseKin;

impl rosidl_runtime_rs::Service for InverseKin {
    type Request = InverseKin_Request;
    type Response = InverseKin_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__InverseKin() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetCollisionLevel() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetCollisionLevel
#[allow(missing_docs, non_camel_case_types)]
pub struct SetCollisionLevel;

impl rosidl_runtime_rs::Service for SetCollisionLevel {
    type Request = SetCollisionLevel_Request;
    type Response = SetCollisionLevel_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetCollisionLevel() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetAngle() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetAngle
#[allow(missing_docs, non_camel_case_types)]
pub struct GetAngle;

impl rosidl_runtime_rs::Service for GetAngle {
    type Request = GetAngle_Request;
    type Response = GetAngle_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetAngle() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetPose() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetPose
#[allow(missing_docs, non_camel_case_types)]
pub struct GetPose;

impl rosidl_runtime_rs::Service for GetPose {
    type Request = GetPose_Request;
    type Response = GetPose_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetPose() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__EmergencyStop() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__EmergencyStop
#[allow(missing_docs, non_camel_case_types)]
pub struct EmergencyStop;

impl rosidl_runtime_rs::Service for EmergencyStop {
    type Request = EmergencyStop_Request;
    type Response = EmergencyStop_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__EmergencyStop() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ModbusRTUCreate() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__ModbusRTUCreate
#[allow(missing_docs, non_camel_case_types)]
pub struct ModbusRTUCreate;

impl rosidl_runtime_rs::Service for ModbusRTUCreate {
    type Request = ModbusRTUCreate_Request;
    type Response = ModbusRTUCreate_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ModbusRTUCreate() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ModbusCreate() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__ModbusCreate
#[allow(missing_docs, non_camel_case_types)]
pub struct ModbusCreate;

impl rosidl_runtime_rs::Service for ModbusCreate {
    type Request = ModbusCreate_Request;
    type Response = ModbusCreate_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ModbusCreate() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ModbusClose() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__ModbusClose
#[allow(missing_docs, non_camel_case_types)]
pub struct ModbusClose;

impl rosidl_runtime_rs::Service for ModbusClose {
    type Request = ModbusClose_Request;
    type Response = ModbusClose_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ModbusClose() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetInBits() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetInBits
#[allow(missing_docs, non_camel_case_types)]
pub struct GetInBits;

impl rosidl_runtime_rs::Service for GetInBits {
    type Request = GetInBits_Request;
    type Response = GetInBits_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetInBits() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetInRegs() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetInRegs
#[allow(missing_docs, non_camel_case_types)]
pub struct GetInRegs;

impl rosidl_runtime_rs::Service for GetInRegs {
    type Request = GetInRegs_Request;
    type Response = GetInRegs_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetInRegs() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetCoils() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetCoils
#[allow(missing_docs, non_camel_case_types)]
pub struct GetCoils;

impl rosidl_runtime_rs::Service for GetCoils {
    type Request = GetCoils_Request;
    type Response = GetCoils_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetCoils() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetCoils() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetCoils
#[allow(missing_docs, non_camel_case_types)]
pub struct SetCoils;

impl rosidl_runtime_rs::Service for SetCoils {
    type Request = SetCoils_Request;
    type Response = SetCoils_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetCoils() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetHoldRegs() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetHoldRegs
#[allow(missing_docs, non_camel_case_types)]
pub struct GetHoldRegs;

impl rosidl_runtime_rs::Service for GetHoldRegs {
    type Request = GetHoldRegs_Request;
    type Response = GetHoldRegs_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetHoldRegs() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetHoldRegs() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetHoldRegs
#[allow(missing_docs, non_camel_case_types)]
pub struct SetHoldRegs;

impl rosidl_runtime_rs::Service for SetHoldRegs {
    type Request = SetHoldRegs_Request;
    type Response = SetHoldRegs_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetHoldRegs() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetSafeSkin() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetSafeSkin
#[allow(missing_docs, non_camel_case_types)]
pub struct SetSafeSkin;

impl rosidl_runtime_rs::Service for SetSafeSkin {
    type Request = SetSafeSkin_Request;
    type Response = SetSafeSkin_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetSafeSkin() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__MovJ() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__MovJ
#[allow(missing_docs, non_camel_case_types)]
pub struct MovJ;

impl rosidl_runtime_rs::Service for MovJ {
    type Request = MovJ_Request;
    type Response = MovJ_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__MovJ() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__MovL() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__MovL
#[allow(missing_docs, non_camel_case_types)]
pub struct MovL;

impl rosidl_runtime_rs::Service for MovL {
    type Request = MovL_Request;
    type Response = MovL_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__MovL() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RelJointMovJ() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__RelJointMovJ
#[allow(missing_docs, non_camel_case_types)]
pub struct RelJointMovJ;

impl rosidl_runtime_rs::Service for RelJointMovJ {
    type Request = RelJointMovJ_Request;
    type Response = RelJointMovJ_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RelJointMovJ() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__MoveJog() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__MoveJog
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveJog;

impl rosidl_runtime_rs::Service for MoveJog {
    type Request = MoveJog_Request;
    type Response = MoveJog_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__MoveJog() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__StopMoveJog() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__StopMoveJog
#[allow(missing_docs, non_camel_case_types)]
pub struct StopMoveJog;

impl rosidl_runtime_rs::Service for StopMoveJog {
    type Request = StopMoveJog_Request;
    type Response = StopMoveJog_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__StopMoveJog() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DOGroup() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__DOGroup
#[allow(missing_docs, non_camel_case_types)]
pub struct DOGroup;

impl rosidl_runtime_rs::Service for DOGroup {
    type Request = DOGroup_Request;
    type Response = DOGroup_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DOGroup() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__BrakeControl() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__BrakeControl
#[allow(missing_docs, non_camel_case_types)]
pub struct BrakeControl;

impl rosidl_runtime_rs::Service for BrakeControl {
    type Request = BrakeControl_Request;
    type Response = BrakeControl_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__BrakeControl() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__StartDrag() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__StartDrag
#[allow(missing_docs, non_camel_case_types)]
pub struct StartDrag;

impl rosidl_runtime_rs::Service for StartDrag {
    type Request = StartDrag_Request;
    type Response = StartDrag_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__StartDrag() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__EnableSafeSkin() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__EnableSafeSkin
#[allow(missing_docs, non_camel_case_types)]
pub struct EnableSafeSkin;

impl rosidl_runtime_rs::Service for EnableSafeSkin {
    type Request = EnableSafeSkin_Request;
    type Response = EnableSafeSkin_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__EnableSafeSkin() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetStartPose() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetStartPose
#[allow(missing_docs, non_camel_case_types)]
pub struct GetStartPose;

impl rosidl_runtime_rs::Service for GetStartPose {
    type Request = GetStartPose_Request;
    type Response = GetStartPose_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetStartPose() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__StartPath() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__StartPath
#[allow(missing_docs, non_camel_case_types)]
pub struct StartPath;

impl rosidl_runtime_rs::Service for StartPath {
    type Request = StartPath_Request;
    type Response = StartPath_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__StartPath() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__InverseSolution() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__InverseSolution
#[allow(missing_docs, non_camel_case_types)]
pub struct InverseSolution;

impl rosidl_runtime_rs::Service for InverseSolution {
    type Request = InverseSolution_Request;
    type Response = InverseSolution_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__InverseSolution() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetErrorID() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetErrorID
#[allow(missing_docs, non_camel_case_types)]
pub struct GetErrorID;

impl rosidl_runtime_rs::Service for GetErrorID {
    type Request = GetErrorID_Request;
    type Response = GetErrorID_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetErrorID() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DI() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__DI
#[allow(missing_docs, non_camel_case_types)]
pub struct DI;

impl rosidl_runtime_rs::Service for DI {
    type Request = DI_Request;
    type Response = DI_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DI() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ToolDI() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__ToolDI
#[allow(missing_docs, non_camel_case_types)]
pub struct ToolDI;

impl rosidl_runtime_rs::Service for ToolDI {
    type Request = ToolDI_Request;
    type Response = ToolDI_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ToolDI() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__AI() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__AI
#[allow(missing_docs, non_camel_case_types)]
pub struct AI;

impl rosidl_runtime_rs::Service for AI {
    type Request = AI_Request;
    type Response = AI_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__AI() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ToolAI() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__ToolAI
#[allow(missing_docs, non_camel_case_types)]
pub struct ToolAI;

impl rosidl_runtime_rs::Service for ToolAI {
    type Request = ToolAI_Request;
    type Response = ToolAI_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ToolAI() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DIGroup() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__DIGroup
#[allow(missing_docs, non_camel_case_types)]
pub struct DIGroup;

impl rosidl_runtime_rs::Service for DIGroup {
    type Request = DIGroup_Request;
    type Response = DIGroup_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DIGroup() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__StopDrag() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__StopDrag
#[allow(missing_docs, non_camel_case_types)]
pub struct StopDrag;

impl rosidl_runtime_rs::Service for StopDrag {
    type Request = StopDrag_Request;
    type Response = StopDrag_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__StopDrag() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DragSensivity() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__DragSensivity
#[allow(missing_docs, non_camel_case_types)]
pub struct DragSensivity;

impl rosidl_runtime_rs::Service for DragSensivity {
    type Request = DragSensivity_Request;
    type Response = DragSensivity_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DragSensivity() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetDO() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetDO
#[allow(missing_docs, non_camel_case_types)]
pub struct GetDO;

impl rosidl_runtime_rs::Service for GetDO {
    type Request = GetDO_Request;
    type Response = GetDO_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetDO() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetAO() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetAO
#[allow(missing_docs, non_camel_case_types)]
pub struct GetAO;

impl rosidl_runtime_rs::Service for GetAO {
    type Request = GetAO_Request;
    type Response = GetAO_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetAO() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetDOGroup() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetDOGroup
#[allow(missing_docs, non_camel_case_types)]
pub struct GetDOGroup;

impl rosidl_runtime_rs::Service for GetDOGroup {
    type Request = GetDOGroup_Request;
    type Response = GetDOGroup_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetDOGroup() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetTool485() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetTool485
#[allow(missing_docs, non_camel_case_types)]
pub struct SetTool485;

impl rosidl_runtime_rs::Service for SetTool485 {
    type Request = SetTool485_Request;
    type Response = SetTool485_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetTool485() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetSafeWallEnable() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetSafeWallEnable
#[allow(missing_docs, non_camel_case_types)]
pub struct SetSafeWallEnable;

impl rosidl_runtime_rs::Service for SetSafeWallEnable {
    type Request = SetSafeWallEnable_Request;
    type Response = SetSafeWallEnable_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetSafeWallEnable() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetToolPower() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetToolPower
#[allow(missing_docs, non_camel_case_types)]
pub struct SetToolPower;

impl rosidl_runtime_rs::Service for SetToolPower {
    type Request = SetToolPower_Request;
    type Response = SetToolPower_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetToolPower() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetToolMode() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetToolMode
#[allow(missing_docs, non_camel_case_types)]
pub struct SetToolMode;

impl rosidl_runtime_rs::Service for SetToolMode {
    type Request = SetToolMode_Request;
    type Response = SetToolMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetToolMode() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetBackDistance() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetBackDistance
#[allow(missing_docs, non_camel_case_types)]
pub struct SetBackDistance;

impl rosidl_runtime_rs::Service for SetBackDistance {
    type Request = SetBackDistance_Request;
    type Response = SetBackDistance_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetBackDistance() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetPostCollisionMode() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetPostCollisionMode
#[allow(missing_docs, non_camel_case_types)]
pub struct SetPostCollisionMode;

impl rosidl_runtime_rs::Service for SetPostCollisionMode {
    type Request = SetPostCollisionMode_Request;
    type Response = SetPostCollisionMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetPostCollisionMode() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetUser() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetUser
#[allow(missing_docs, non_camel_case_types)]
pub struct SetUser;

impl rosidl_runtime_rs::Service for SetUser {
    type Request = SetUser_Request;
    type Response = SetUser_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetUser() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetTool() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetTool
#[allow(missing_docs, non_camel_case_types)]
pub struct SetTool;

impl rosidl_runtime_rs::Service for SetTool {
    type Request = SetTool_Request;
    type Response = SetTool_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetTool() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__CalcUser() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__CalcUser
#[allow(missing_docs, non_camel_case_types)]
pub struct CalcUser;

impl rosidl_runtime_rs::Service for CalcUser {
    type Request = CalcUser_Request;
    type Response = CalcUser_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__CalcUser() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__CalcTool() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__CalcTool
#[allow(missing_docs, non_camel_case_types)]
pub struct CalcTool;

impl rosidl_runtime_rs::Service for CalcTool {
    type Request = CalcTool_Request;
    type Response = CalcTool_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__CalcTool() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetInputBool() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetInputBool
#[allow(missing_docs, non_camel_case_types)]
pub struct GetInputBool;

impl rosidl_runtime_rs::Service for GetInputBool {
    type Request = GetInputBool_Request;
    type Response = GetInputBool_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetInputBool() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetInputInt() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetInputInt
#[allow(missing_docs, non_camel_case_types)]
pub struct GetInputInt;

impl rosidl_runtime_rs::Service for GetInputInt {
    type Request = GetInputInt_Request;
    type Response = GetInputInt_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetInputInt() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetInputFloat() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetInputFloat
#[allow(missing_docs, non_camel_case_types)]
pub struct GetInputFloat;

impl rosidl_runtime_rs::Service for GetInputFloat {
    type Request = GetInputFloat_Request;
    type Response = GetInputFloat_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetInputFloat() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetOutputBool() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetOutputBool
#[allow(missing_docs, non_camel_case_types)]
pub struct GetOutputBool;

impl rosidl_runtime_rs::Service for GetOutputBool {
    type Request = GetOutputBool_Request;
    type Response = GetOutputBool_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetOutputBool() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetOutputInt() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetOutputInt
#[allow(missing_docs, non_camel_case_types)]
pub struct GetOutputInt;

impl rosidl_runtime_rs::Service for GetOutputInt {
    type Request = GetOutputInt_Request;
    type Response = GetOutputInt_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetOutputInt() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetOutputFloat() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetOutputFloat
#[allow(missing_docs, non_camel_case_types)]
pub struct GetOutputFloat;

impl rosidl_runtime_rs::Service for GetOutputFloat {
    type Request = GetOutputFloat_Request;
    type Response = GetOutputFloat_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetOutputFloat() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetOutputBool() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetOutputBool
#[allow(missing_docs, non_camel_case_types)]
pub struct SetOutputBool;

impl rosidl_runtime_rs::Service for SetOutputBool {
    type Request = SetOutputBool_Request;
    type Response = SetOutputBool_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetOutputBool() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetOutputInt() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetOutputInt
#[allow(missing_docs, non_camel_case_types)]
pub struct SetOutputInt;

impl rosidl_runtime_rs::Service for SetOutputInt {
    type Request = SetOutputInt_Request;
    type Response = SetOutputInt_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetOutputInt() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetOutputFloat() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetOutputFloat
#[allow(missing_docs, non_camel_case_types)]
pub struct SetOutputFloat;

impl rosidl_runtime_rs::Service for SetOutputFloat {
    type Request = SetOutputFloat_Request;
    type Response = SetOutputFloat_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetOutputFloat() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__MovLIO() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__MovLIO
#[allow(missing_docs, non_camel_case_types)]
pub struct MovLIO;

impl rosidl_runtime_rs::Service for MovLIO {
    type Request = MovLIO_Request;
    type Response = MovLIO_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__MovLIO() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__MovJIO() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__MovJIO
#[allow(missing_docs, non_camel_case_types)]
pub struct MovJIO;

impl rosidl_runtime_rs::Service for MovJIO {
    type Request = MovJIO_Request;
    type Response = MovJIO_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__MovJIO() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__Arc() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__Arc
#[allow(missing_docs, non_camel_case_types)]
pub struct Arc;

impl rosidl_runtime_rs::Service for Arc {
    type Request = Arc_Request;
    type Response = Arc_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__Arc() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__Circle() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__Circle
#[allow(missing_docs, non_camel_case_types)]
pub struct Circle;

impl rosidl_runtime_rs::Service for Circle {
    type Request = Circle_Request;
    type Response = Circle_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__Circle() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RelMovJTool() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__RelMovJTool
#[allow(missing_docs, non_camel_case_types)]
pub struct RelMovJTool;

impl rosidl_runtime_rs::Service for RelMovJTool {
    type Request = RelMovJTool_Request;
    type Response = RelMovJTool_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RelMovJTool() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RelMovLTool() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__RelMovLTool
#[allow(missing_docs, non_camel_case_types)]
pub struct RelMovLTool;

impl rosidl_runtime_rs::Service for RelMovLTool {
    type Request = RelMovLTool_Request;
    type Response = RelMovLTool_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RelMovLTool() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RelMovJUser() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__RelMovJUser
#[allow(missing_docs, non_camel_case_types)]
pub struct RelMovJUser;

impl rosidl_runtime_rs::Service for RelMovJUser {
    type Request = RelMovJUser_Request;
    type Response = RelMovJUser_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RelMovJUser() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RelMovLUser() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__RelMovLUser
#[allow(missing_docs, non_camel_case_types)]
pub struct RelMovLUser;

impl rosidl_runtime_rs::Service for RelMovLUser {
    type Request = RelMovLUser_Request;
    type Response = RelMovLUser_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RelMovLUser() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetCurrentCommandId() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetCurrentCommandId
#[allow(missing_docs, non_camel_case_types)]
pub struct GetCurrentCommandId;

impl rosidl_runtime_rs::Service for GetCurrentCommandId {
    type Request = GetCurrentCommandId_Request;
    type Response = GetCurrentCommandId_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetCurrentCommandId() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ServoJ() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__ServoJ
#[allow(missing_docs, non_camel_case_types)]
pub struct ServoJ;

impl rosidl_runtime_rs::Service for ServoJ {
    type Request = ServoJ_Request;
    type Response = ServoJ_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ServoJ() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ServoP() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__ServoP
#[allow(missing_docs, non_camel_case_types)]
pub struct ServoP;

impl rosidl_runtime_rs::Service for ServoP {
    type Request = ServoP_Request;
    type Response = ServoP_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ServoP() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__TcpDashboard() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__TcpDashboard
#[allow(missing_docs, non_camel_case_types)]
pub struct TcpDashboard;

impl rosidl_runtime_rs::Service for TcpDashboard {
    type Request = TcpDashboard_Request;
    type Response = TcpDashboard_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__TcpDashboard() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__EnableFTSensor() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__EnableFTSensor
#[allow(missing_docs, non_camel_case_types)]
pub struct EnableFTSensor;

impl rosidl_runtime_rs::Service for EnableFTSensor {
    type Request = EnableFTSensor_Request;
    type Response = EnableFTSensor_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__EnableFTSensor() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SixForceHome() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SixForceHome
#[allow(missing_docs, non_camel_case_types)]
pub struct SixForceHome;

impl rosidl_runtime_rs::Service for SixForceHome {
    type Request = SixForceHome_Request;
    type Response = SixForceHome_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SixForceHome() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetForce() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetForce
#[allow(missing_docs, non_camel_case_types)]
pub struct GetForce;

impl rosidl_runtime_rs::Service for GetForce {
    type Request = GetForce_Request;
    type Response = GetForce_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetForce() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ForceDriveMode() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__ForceDriveMode
#[allow(missing_docs, non_camel_case_types)]
pub struct ForceDriveMode;

impl rosidl_runtime_rs::Service for ForceDriveMode {
    type Request = ForceDriveMode_Request;
    type Response = ForceDriveMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ForceDriveMode() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ForceDriveSpeed() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__ForceDriveSpeed
#[allow(missing_docs, non_camel_case_types)]
pub struct ForceDriveSpeed;

impl rosidl_runtime_rs::Service for ForceDriveSpeed {
    type Request = ForceDriveSpeed_Request;
    type Response = ForceDriveSpeed_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ForceDriveSpeed() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCForceMode() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__FCForceMode
#[allow(missing_docs, non_camel_case_types)]
pub struct FCForceMode;

impl rosidl_runtime_rs::Service for FCForceMode {
    type Request = FCForceMode_Request;
    type Response = FCForceMode_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCForceMode() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCSetDeviation() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__FCSetDeviation
#[allow(missing_docs, non_camel_case_types)]
pub struct FCSetDeviation;

impl rosidl_runtime_rs::Service for FCSetDeviation {
    type Request = FCSetDeviation_Request;
    type Response = FCSetDeviation_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCSetDeviation() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCSetForceLimit() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__FCSetForceLimit
#[allow(missing_docs, non_camel_case_types)]
pub struct FCSetForceLimit;

impl rosidl_runtime_rs::Service for FCSetForceLimit {
    type Request = FCSetForceLimit_Request;
    type Response = FCSetForceLimit_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCSetForceLimit() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCSetMass() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__FCSetMass
#[allow(missing_docs, non_camel_case_types)]
pub struct FCSetMass;

impl rosidl_runtime_rs::Service for FCSetMass {
    type Request = FCSetMass_Request;
    type Response = FCSetMass_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCSetMass() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCSetStiffness() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__FCSetStiffness
#[allow(missing_docs, non_camel_case_types)]
pub struct FCSetStiffness;

impl rosidl_runtime_rs::Service for FCSetStiffness {
    type Request = FCSetStiffness_Request;
    type Response = FCSetStiffness_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCSetStiffness() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCSetDamping() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__FCSetDamping
#[allow(missing_docs, non_camel_case_types)]
pub struct FCSetDamping;

impl rosidl_runtime_rs::Service for FCSetDamping {
    type Request = FCSetDamping_Request;
    type Response = FCSetDamping_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCSetDamping() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCOff() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__FCOff
#[allow(missing_docs, non_camel_case_types)]
pub struct FCOff;

impl rosidl_runtime_rs::Service for FCOff {
    type Request = FCOff_Request;
    type Response = FCOff_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCOff() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCSetForceSpeedLimit() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__FCSetForceSpeedLimit
#[allow(missing_docs, non_camel_case_types)]
pub struct FCSetForceSpeedLimit;

impl rosidl_runtime_rs::Service for FCSetForceSpeedLimit {
    type Request = FCSetForceSpeedLimit_Request;
    type Response = FCSetForceSpeedLimit_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCSetForceSpeedLimit() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCSetForce() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__FCSetForce
#[allow(missing_docs, non_camel_case_types)]
pub struct FCSetForce;

impl rosidl_runtime_rs::Service for FCSetForce {
    type Request = FCSetForce_Request;
    type Response = FCSetForce_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCSetForce() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetFCCollision() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetFCCollision
#[allow(missing_docs, non_camel_case_types)]
pub struct SetFCCollision;

impl rosidl_runtime_rs::Service for SetFCCollision {
    type Request = SetFCCollision_Request;
    type Response = SetFCCollision_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetFCCollision() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCCollisionSwitch() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__FCCollisionSwitch
#[allow(missing_docs, non_camel_case_types)]
pub struct FCCollisionSwitch;

impl rosidl_runtime_rs::Service for FCCollisionSwitch {
    type Request = FCCollisionSwitch_Request;
    type Response = FCCollisionSwitch_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__FCCollisionSwitch() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetWorkZoneEnable() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__SetWorkZoneEnable
#[allow(missing_docs, non_camel_case_types)]
pub struct SetWorkZoneEnable;

impl rosidl_runtime_rs::Service for SetWorkZoneEnable {
    type Request = SetWorkZoneEnable_Request;
    type Response = SetWorkZoneEnable_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__SetWorkZoneEnable() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetToolDO() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetToolDO
#[allow(missing_docs, non_camel_case_types)]
pub struct GetToolDO;

impl rosidl_runtime_rs::Service for GetToolDO {
    type Request = GetToolDO_Request;
    type Response = GetToolDO_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetToolDO() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ResetRobot() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__ResetRobot
#[allow(missing_docs, non_camel_case_types)]
pub struct ResetRobot;

impl rosidl_runtime_rs::Service for ResetRobot {
    type Request = ResetRobot_Request;
    type Response = ResetRobot_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__ResetRobot() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RunTo() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__RunTo
#[allow(missing_docs, non_camel_case_types)]
pub struct RunTo;

impl rosidl_runtime_rs::Service for RunTo {
    type Request = RunTo_Request;
    type Response = RunTo_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RunTo() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__StartRTOffset() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__StartRTOffset
#[allow(missing_docs, non_camel_case_types)]
pub struct StartRTOffset;

impl rosidl_runtime_rs::Service for StartRTOffset {
    type Request = StartRTOffset_Request;
    type Response = StartRTOffset_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__StartRTOffset() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__EndRTOffset() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__EndRTOffset
#[allow(missing_docs, non_camel_case_types)]
pub struct EndRTOffset;

impl rosidl_runtime_rs::Service for EndRTOffset {
    type Request = EndRTOffset_Request;
    type Response = EndRTOffset_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__EndRTOffset() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetError() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetError
#[allow(missing_docs, non_camel_case_types)]
pub struct GetError;

impl rosidl_runtime_rs::Service for GetError {
    type Request = GetError_Request;
    type Response = GetError_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetError() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DOGroupDEC() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__DOGroupDEC
#[allow(missing_docs, non_camel_case_types)]
pub struct DOGroupDEC;

impl rosidl_runtime_rs::Service for DOGroupDEC {
    type Request = DOGroupDEC_Request;
    type Response = DOGroupDEC_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DOGroupDEC() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetDOGroupDEC() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__GetDOGroupDEC
#[allow(missing_docs, non_camel_case_types)]
pub struct GetDOGroupDEC;

impl rosidl_runtime_rs::Service for GetDOGroupDEC {
    type Request = GetDOGroupDEC_Request;
    type Response = GetDOGroupDEC_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__GetDOGroupDEC() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DIGroupDEC() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__DIGroupDEC
#[allow(missing_docs, non_camel_case_types)]
pub struct DIGroupDEC;

impl rosidl_runtime_rs::Service for DIGroupDEC {
    type Request = DIGroupDEC_Request;
    type Response = DIGroupDEC_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__DIGroupDEC() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RequestControl() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__RequestControl
#[allow(missing_docs, non_camel_case_types)]
pub struct RequestControl;

impl rosidl_runtime_rs::Service for RequestControl {
    type Request = RequestControl_Request;
    type Response = RequestControl_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__RequestControl() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__CheckOddMovL() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__CheckOddMovL
#[allow(missing_docs, non_camel_case_types)]
pub struct CheckOddMovL;

impl rosidl_runtime_rs::Service for CheckOddMovL {
    type Request = CheckOddMovL_Request;
    type Response = CheckOddMovL_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__CheckOddMovL() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__CheckOddMovJ() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__CheckOddMovJ
#[allow(missing_docs, non_camel_case_types)]
pub struct CheckOddMovJ;

impl rosidl_runtime_rs::Service for CheckOddMovJ {
    type Request = CheckOddMovJ_Request;
    type Response = CheckOddMovJ_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__CheckOddMovJ() }
    }
}




#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__CheckOddMovC() -> *const std::ffi::c_void;
}

// Corresponds to dobot_msgs_v4__srv__CheckOddMovC
#[allow(missing_docs, non_camel_case_types)]
pub struct CheckOddMovC;

impl rosidl_runtime_rs::Service for CheckOddMovC {
    type Request = CheckOddMovC_Request;
    type Response = CheckOddMovC_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__dobot_msgs_v4__srv__CheckOddMovC() }
    }
}


