#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to ariac_interfaces__srv__AttachTool_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AttachTool_Request {
    /// Available types are in VacuumTools.msg
    pub tool: i8,

}



impl Default for AttachTool_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AttachTool_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AttachTool_Request {
  type RmwMsg = super::srv::rmw::AttachTool_Request;

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


// Corresponds to ariac_interfaces__srv__AttachTool_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AttachTool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for AttachTool_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AttachTool_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AttachTool_Response {
  type RmwMsg = super::srv::rmw::AttachTool_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to ariac_interfaces__srv__BidirectionalConveyorControl_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BidirectionalConveyorControl_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub direction: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speed: f64,

}



impl Default for BidirectionalConveyorControl_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::BidirectionalConveyorControl_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BidirectionalConveyorControl_Request {
  type RmwMsg = super::srv::rmw::BidirectionalConveyorControl_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        direction: msg.direction,
        speed: msg.speed,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      direction: msg.direction,
      speed: msg.speed,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      direction: msg.direction,
      speed: msg.speed,
    }
  }
}


// Corresponds to ariac_interfaces__srv__BidirectionalConveyorControl_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BidirectionalConveyorControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for BidirectionalConveyorControl_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::BidirectionalConveyorControl_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BidirectionalConveyorControl_Response {
  type RmwMsg = super::srv::rmw::BidirectionalConveyorControl_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to ariac_interfaces__srv__CheckKitQuality_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckKitQuality_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cell_type: u8,

}



impl Default for CheckKitQuality_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CheckKitQuality_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CheckKitQuality_Request {
  type RmwMsg = super::srv::rmw::CheckKitQuality_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cell_type: msg.cell_type,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      cell_type: msg.cell_type,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      cell_type: msg.cell_type,
    }
  }
}


// Corresponds to ariac_interfaces__srv__CheckKitQuality_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckKitQuality_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub is_good: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for CheckKitQuality_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CheckKitQuality_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CheckKitQuality_Response {
  type RmwMsg = super::srv::rmw::CheckKitQuality_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        is_good: msg.is_good,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      is_good: msg.is_good,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      is_good: msg.is_good,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to ariac_interfaces__srv__ControlCellFeeder_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ControlCellFeeder_Request {
    /// available types are in CellTypes.msg
    pub cell_type: u8,

}



impl Default for ControlCellFeeder_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ControlCellFeeder_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ControlCellFeeder_Request {
  type RmwMsg = super::srv::rmw::ControlCellFeeder_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cell_type: msg.cell_type,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      cell_type: msg.cell_type,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      cell_type: msg.cell_type,
    }
  }
}


// Corresponds to ariac_interfaces__srv__ControlCellFeeder_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ControlCellFeeder_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for ControlCellFeeder_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ControlCellFeeder_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ControlCellFeeder_Response {
  type RmwMsg = super::srv::rmw::ControlCellFeeder_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to ariac_interfaces__srv__ConveyorControl_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConveyorControl_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub speed: f64,

}



impl Default for ConveyorControl_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ConveyorControl_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ConveyorControl_Request {
  type RmwMsg = super::srv::rmw::ConveyorControl_Request;

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


// Corresponds to ariac_interfaces__srv__ConveyorControl_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConveyorControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for ConveyorControl_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ConveyorControl_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ConveyorControl_Response {
  type RmwMsg = super::srv::rmw::ConveyorControl_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to ariac_interfaces__srv__EndCompetition_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EndCompetition_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub shutdown_gazebo: bool,

}



impl Default for EndCompetition_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EndCompetition_Request::default())
  }
}

impl rosidl_runtime_rs::Message for EndCompetition_Request {
  type RmwMsg = super::srv::rmw::EndCompetition_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        shutdown_gazebo: msg.shutdown_gazebo,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      shutdown_gazebo: msg.shutdown_gazebo,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      shutdown_gazebo: msg.shutdown_gazebo,
    }
  }
}


// Corresponds to ariac_interfaces__srv__EndCompetition_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EndCompetition_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for EndCompetition_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::EndCompetition_Response::default())
  }
}

impl rosidl_runtime_rs::Message for EndCompetition_Response {
  type RmwMsg = super::srv::rmw::EndCompetition_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to ariac_interfaces__srv__SubmitHighPriorityOrder_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmitHighPriorityOrder_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: std::string::String,

}



impl Default for SubmitHighPriorityOrder_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SubmitHighPriorityOrder_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SubmitHighPriorityOrder_Request {
  type RmwMsg = super::srv::rmw::SubmitHighPriorityOrder_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id.to_string(),
    }
  }
}


// Corresponds to ariac_interfaces__srv__SubmitHighPriorityOrder_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmitHighPriorityOrder_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SubmitHighPriorityOrder_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SubmitHighPriorityOrder_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SubmitHighPriorityOrder_Response {
  type RmwMsg = super::srv::rmw::SubmitHighPriorityOrder_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to ariac_interfaces__srv__SubmitInspectionReport_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmitInspectionReport_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub timestamp: builtin_interfaces::msg::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    pub report: super::msg::InspectionReport,

}



impl Default for SubmitInspectionReport_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SubmitInspectionReport_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SubmitInspectionReport_Request {
  type RmwMsg = super::srv::rmw::SubmitInspectionReport_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        timestamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.timestamp)).into_owned(),
        report: super::msg::InspectionReport::into_rmw_message(std::borrow::Cow::Owned(msg.report)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        timestamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.timestamp)).into_owned(),
        report: super::msg::InspectionReport::into_rmw_message(std::borrow::Cow::Borrowed(&msg.report)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      timestamp: builtin_interfaces::msg::Time::from_rmw_message(msg.timestamp),
      report: super::msg::InspectionReport::from_rmw_message(msg.report),
    }
  }
}


// Corresponds to ariac_interfaces__srv__SubmitInspectionReport_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmitInspectionReport_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for SubmitInspectionReport_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SubmitInspectionReport_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SubmitInspectionReport_Response {
  type RmwMsg = super::srv::rmw::SubmitInspectionReport_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to ariac_interfaces__srv__Trigger_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Trigger_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Trigger_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Trigger_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Trigger_Request {
  type RmwMsg = super::srv::rmw::Trigger_Request;

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


// Corresponds to ariac_interfaces__srv__Trigger_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Trigger_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for Trigger_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Trigger_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Trigger_Response {
  type RmwMsg = super::srv::rmw::Trigger_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}






#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__AttachTool() -> *const std::ffi::c_void;
}

// Corresponds to ariac_interfaces__srv__AttachTool
#[allow(missing_docs, non_camel_case_types)]
pub struct AttachTool;

impl rosidl_runtime_rs::Service for AttachTool {
    type Request = AttachTool_Request;
    type Response = AttachTool_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__AttachTool() }
    }
}




#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__BidirectionalConveyorControl() -> *const std::ffi::c_void;
}

// Corresponds to ariac_interfaces__srv__BidirectionalConveyorControl
#[allow(missing_docs, non_camel_case_types)]
pub struct BidirectionalConveyorControl;

impl rosidl_runtime_rs::Service for BidirectionalConveyorControl {
    type Request = BidirectionalConveyorControl_Request;
    type Response = BidirectionalConveyorControl_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__BidirectionalConveyorControl() }
    }
}




#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__CheckKitQuality() -> *const std::ffi::c_void;
}

// Corresponds to ariac_interfaces__srv__CheckKitQuality
#[allow(missing_docs, non_camel_case_types)]
pub struct CheckKitQuality;

impl rosidl_runtime_rs::Service for CheckKitQuality {
    type Request = CheckKitQuality_Request;
    type Response = CheckKitQuality_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__CheckKitQuality() }
    }
}




#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__ControlCellFeeder() -> *const std::ffi::c_void;
}

// Corresponds to ariac_interfaces__srv__ControlCellFeeder
#[allow(missing_docs, non_camel_case_types)]
pub struct ControlCellFeeder;

impl rosidl_runtime_rs::Service for ControlCellFeeder {
    type Request = ControlCellFeeder_Request;
    type Response = ControlCellFeeder_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__ControlCellFeeder() }
    }
}




#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__ConveyorControl() -> *const std::ffi::c_void;
}

// Corresponds to ariac_interfaces__srv__ConveyorControl
#[allow(missing_docs, non_camel_case_types)]
pub struct ConveyorControl;

impl rosidl_runtime_rs::Service for ConveyorControl {
    type Request = ConveyorControl_Request;
    type Response = ConveyorControl_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__ConveyorControl() }
    }
}




#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__EndCompetition() -> *const std::ffi::c_void;
}

// Corresponds to ariac_interfaces__srv__EndCompetition
#[allow(missing_docs, non_camel_case_types)]
pub struct EndCompetition;

impl rosidl_runtime_rs::Service for EndCompetition {
    type Request = EndCompetition_Request;
    type Response = EndCompetition_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__EndCompetition() }
    }
}




#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__SubmitHighPriorityOrder() -> *const std::ffi::c_void;
}

// Corresponds to ariac_interfaces__srv__SubmitHighPriorityOrder
#[allow(missing_docs, non_camel_case_types)]
pub struct SubmitHighPriorityOrder;

impl rosidl_runtime_rs::Service for SubmitHighPriorityOrder {
    type Request = SubmitHighPriorityOrder_Request;
    type Response = SubmitHighPriorityOrder_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__SubmitHighPriorityOrder() }
    }
}




#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__SubmitInspectionReport() -> *const std::ffi::c_void;
}

// Corresponds to ariac_interfaces__srv__SubmitInspectionReport
#[allow(missing_docs, non_camel_case_types)]
pub struct SubmitInspectionReport;

impl rosidl_runtime_rs::Service for SubmitInspectionReport {
    type Request = SubmitInspectionReport_Request;
    type Response = SubmitInspectionReport_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__SubmitInspectionReport() }
    }
}




#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__Trigger() -> *const std::ffi::c_void;
}

// Corresponds to ariac_interfaces__srv__Trigger
#[allow(missing_docs, non_camel_case_types)]
pub struct Trigger;

impl rosidl_runtime_rs::Service for Trigger {
    type Request = Trigger_Request;
    type Response = Trigger_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__srv__Trigger() }
    }
}


