#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to ariac_interfaces__msg__AgvStations
/// AGVStations.msg

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AgvStations {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}

impl AgvStations {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INSPECTION: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ASSEMBLY: i8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SHIPPING: i8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RECYCLING: i8 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const IN_TRANSIT: i8 = -1;

}


impl Default for AgvStations {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AgvStations::default())
  }
}

impl rosidl_runtime_rs::Message for AgvStations {
  type RmwMsg = super::msg::rmw::AgvStations;

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


// Corresponds to ariac_interfaces__msg__AgvStatus
/// AGVStatus.msg

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AgvStatus {
    /// Current location ID.
    /// Should be one of the values defined in AgvStations.msg.
    pub station_id: i8,

    /// Current pose of the AGV
    pub pose: geometry_msgs::msg::Pose,

}



impl Default for AgvStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AgvStatus::default())
  }
}

impl rosidl_runtime_rs::Message for AgvStatus {
  type RmwMsg = super::msg::rmw::AgvStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        station_id: msg.station_id,
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      station_id: msg.station_id,
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      station_id: msg.station_id,
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
    }
  }
}


// Corresponds to ariac_interfaces__msg__AgvTrayStatus
/// AgvTrayStatus.msg

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AgvTrayStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub slot_1_occupied: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub slot_2_occupied: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub slot_3_occupied: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub slot_4_occupied: bool,

}



impl Default for AgvTrayStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AgvTrayStatus::default())
  }
}

impl rosidl_runtime_rs::Message for AgvTrayStatus {
  type RmwMsg = super::msg::rmw::AgvTrayStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        slot_1_occupied: msg.slot_1_occupied,
        slot_2_occupied: msg.slot_2_occupied,
        slot_3_occupied: msg.slot_3_occupied,
        slot_4_occupied: msg.slot_4_occupied,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      slot_1_occupied: msg.slot_1_occupied,
      slot_2_occupied: msg.slot_2_occupied,
      slot_3_occupied: msg.slot_3_occupied,
      slot_4_occupied: msg.slot_4_occupied,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      slot_1_occupied: msg.slot_1_occupied,
      slot_2_occupied: msg.slot_2_occupied,
      slot_3_occupied: msg.slot_3_occupied,
      slot_4_occupied: msg.slot_4_occupied,
    }
  }
}


// Corresponds to ariac_interfaces__msg__BreakBeamStatus
/// Single reading from a break beam sensor that detects the presence of
/// objects within its sensing range.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BreakBeamStatus {
    /// timestamp and name of tf frame
    pub header: std_msgs::msg::Header,

    /// is there something in the proximity of the sensor?
    pub object_detected: bool,

}



impl Default for BreakBeamStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BreakBeamStatus::default())
  }
}

impl rosidl_runtime_rs::Message for BreakBeamStatus {
  type RmwMsg = super::msg::rmw::BreakBeamStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        object_detected: msg.object_detected,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      object_detected: msg.object_detected,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      object_detected: msg.object_detected,
    }
  }
}


// Corresponds to ariac_interfaces__msg__DistanceSensor

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DistanceSensor {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: f64,

}



impl Default for DistanceSensor {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DistanceSensor::default())
  }
}

impl rosidl_runtime_rs::Message for DistanceSensor {
  type RmwMsg = super::msg::rmw::DistanceSensor;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        distance: msg.distance,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      distance: msg.distance,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      distance: msg.distance,
    }
  }
}


// Corresponds to ariac_interfaces__msg__CellDefect
/// CellDefect.msg

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CellDefect {
    /// one of the defect types above
    pub defect_type: u8,

    /// Location of the defect in cylinderical coordinates
    ///  relative to the cell base (r is implied as the radius of the cell)
    /// azimuthal angle of the defect centroid
    pub theta: f64,

    /// height of the defect`centroid
    pub z: f64,

}

impl CellDefect {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DENT: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const BULGE: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SCRATCH: u8 = 3;

}


impl Default for CellDefect {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CellDefect::default())
  }
}

impl rosidl_runtime_rs::Message for CellDefect {
  type RmwMsg = super::msg::rmw::CellDefect;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        defect_type: msg.defect_type,
        theta: msg.theta,
        z: msg.z,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      defect_type: msg.defect_type,
      theta: msg.theta,
      z: msg.z,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      defect_type: msg.defect_type,
      theta: msg.theta,
      z: msg.z,
    }
  }
}


// Corresponds to ariac_interfaces__msg__CellFeederStatus
/// CellFeederStatus.msg

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CellFeederStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cell_type: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feed_rate: f64,

}



impl Default for CellFeederStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CellFeederStatus::default())
  }
}

impl rosidl_runtime_rs::Message for CellFeederStatus {
  type RmwMsg = super::msg::rmw::CellFeederStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cell_type: msg.cell_type,
        feed_rate: msg.feed_rate,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      cell_type: msg.cell_type,
      feed_rate: msg.feed_rate,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      cell_type: msg.cell_type,
      feed_rate: msg.feed_rate,
    }
  }
}


// Corresponds to ariac_interfaces__msg__CellTypes
/// CellTypes.msg

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CellTypes {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}

impl CellTypes {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const NONE: u8 = 0;

    /// Lithium Ion cells
    pub const LI_ION: u8 = 1;

    /// Nickel-Metal Hydride
    pub const NIMH: u8 = 2;

    /// Volts
    pub const LI_ION_NOMINAL_VOLTAGE: f64 = 3.6;

    /// Volts
    pub const NIMH_NOMINAL_VOLTAGE: f64 = 1.2;

    /// ± Volts
    pub const CELL_VOLTAGE_TOLERANCE: f64 = 0.2;

    /// ± Volts
    pub const KIT_VOLTAGE_TOLERANCE: f64 = 0.15;

}


impl Default for CellTypes {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CellTypes::default())
  }
}

impl rosidl_runtime_rs::Message for CellTypes {
  type RmwMsg = super::msg::rmw::CellTypes;

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


// Corresponds to ariac_interfaces__msg__CompetitionStates

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CompetitionStates {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}

impl CompetitionStates {
    /// Competition cannot be started yet by the competitor
    pub const PREPARING: u8 = 0;

    /// Competition can be started by the competitor
    pub const READY: u8 = 1;

    /// Competition has been started
    pub const STARTED: u8 = 2;

    /// All orders announced and complete
    pub const ORDERS_COMPLETE: u8 = 3;

    /// Competition has ended
    pub const ENDED: u8 = 4;

}


impl Default for CompetitionStates {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CompetitionStates::default())
  }
}

impl rosidl_runtime_rs::Message for CompetitionStates {
  type RmwMsg = super::msg::rmw::CompetitionStates;

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


// Corresponds to ariac_interfaces__msg__CompetitionStatus
/// CompetitionStatus.msg

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CompetitionStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub competition_state: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub num_kits: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub num_modules: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub num_kits_remaining: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub num_modules_remaining: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub time: super::msg::CompetitionTime,


    // This member is not documented.
    #[allow(missing_docs)]
    pub run_id: i32,

}



impl Default for CompetitionStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CompetitionStatus::default())
  }
}

impl rosidl_runtime_rs::Message for CompetitionStatus {
  type RmwMsg = super::msg::rmw::CompetitionStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        competition_state: msg.competition_state,
        num_kits: msg.num_kits,
        num_modules: msg.num_modules,
        num_kits_remaining: msg.num_kits_remaining,
        num_modules_remaining: msg.num_modules_remaining,
        time: super::msg::CompetitionTime::into_rmw_message(std::borrow::Cow::Owned(msg.time)).into_owned(),
        run_id: msg.run_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      competition_state: msg.competition_state,
      num_kits: msg.num_kits,
      num_modules: msg.num_modules,
      num_kits_remaining: msg.num_kits_remaining,
      num_modules_remaining: msg.num_modules_remaining,
        time: super::msg::CompetitionTime::into_rmw_message(std::borrow::Cow::Borrowed(&msg.time)).into_owned(),
      run_id: msg.run_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      competition_state: msg.competition_state,
      num_kits: msg.num_kits,
      num_modules: msg.num_modules,
      num_kits_remaining: msg.num_kits_remaining,
      num_modules_remaining: msg.num_modules_remaining,
      time: super::msg::CompetitionTime::from_rmw_message(msg.time),
      run_id: msg.run_id,
    }
  }
}


// Corresponds to ariac_interfaces__msg__CompetitionTime
/// CompetitionTime.msg

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CompetitionTime {

    // This member is not documented.
    #[allow(missing_docs)]
    pub start: builtin_interfaces::msg::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    pub elapsed: builtin_interfaces::msg::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub remaining: builtin_interfaces::msg::Duration,

}



impl Default for CompetitionTime {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CompetitionTime::default())
  }
}

impl rosidl_runtime_rs::Message for CompetitionTime {
  type RmwMsg = super::msg::rmw::CompetitionTime;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        start: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.start)).into_owned(),
        elapsed: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.elapsed)).into_owned(),
        remaining: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.remaining)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        start: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.start)).into_owned(),
        elapsed: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.elapsed)).into_owned(),
        remaining: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.remaining)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      start: builtin_interfaces::msg::Time::from_rmw_message(msg.start),
      elapsed: builtin_interfaces::msg::Duration::from_rmw_message(msg.elapsed),
      remaining: builtin_interfaces::msg::Duration::from_rmw_message(msg.remaining),
    }
  }
}


// Corresponds to ariac_interfaces__msg__ConveyorStatus

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConveyorStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub direction: u8,

    /// m/s
    pub speed: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub operating_status: u8,

}

impl ConveyorStatus {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FORWARD: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const BACKWARD: u8 = 1;

}


impl Default for ConveyorStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ConveyorStatus::default())
  }
}

impl rosidl_runtime_rs::Message for ConveyorStatus {
  type RmwMsg = super::msg::rmw::ConveyorStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        direction: msg.direction,
        speed: msg.speed,
        operating_status: msg.operating_status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      direction: msg.direction,
      speed: msg.speed,
      operating_status: msg.operating_status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      direction: msg.direction,
      speed: msg.speed,
      operating_status: msg.operating_status,
    }
  }
}


// Corresponds to ariac_interfaces__msg__HighPriorityOrder

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HighPriorityOrder {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: std::string::String,

}



impl Default for HighPriorityOrder {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HighPriorityOrder::default())
  }
}

impl rosidl_runtime_rs::Message for HighPriorityOrder {
  type RmwMsg = super::msg::rmw::HighPriorityOrder;

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


// Corresponds to ariac_interfaces__msg__InspectionReport
/// InspectionReport.msg

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InspectionReport {

    // This member is not documented.
    #[allow(missing_docs)]
    pub passed: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub defects: Vec<super::msg::CellDefect>,

}



impl Default for InspectionReport {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::InspectionReport::default())
  }
}

impl rosidl_runtime_rs::Message for InspectionReport {
  type RmwMsg = super::msg::rmw::InspectionReport;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        passed: msg.passed,
        defects: msg.defects
          .into_iter()
          .map(|elem| super::msg::CellDefect::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      passed: msg.passed,
        defects: msg.defects
          .iter()
          .map(|elem| super::msg::CellDefect::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      passed: msg.passed,
      defects: msg.defects
          .into_iter()
          .map(super::msg::CellDefect::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to ariac_interfaces__msg__OperationStates

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OperationStates {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}

impl OperationStates {
    /// Device is running normally
    pub const OPERATIONAL: u8 = 1;

    /// Device is malfunctioning
    pub const MALFUNCTIONING: u8 = 2;

}


impl Default for OperationStates {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::OperationStates::default())
  }
}

impl rosidl_runtime_rs::Message for OperationStates {
  type RmwMsg = super::msg::rmw::OperationStates;

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


// Corresponds to ariac_interfaces__msg__ToolChangerStatus
/// ToolChangerStatus.msg

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolChangerStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub attached_tool: i8,

}



impl Default for ToolChangerStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ToolChangerStatus::default())
  }
}

impl rosidl_runtime_rs::Message for ToolChangerStatus {
  type RmwMsg = super::msg::rmw::ToolChangerStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        attached_tool: msg.attached_tool,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      attached_tool: msg.attached_tool,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      attached_tool: msg.attached_tool,
    }
  }
}


// Corresponds to ariac_interfaces__msg__VacuumTools
/// VacuumTools.msg

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VacuumTools {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}

impl VacuumTools {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const NONE: i8 = -1;

    /// Vacuum Gripper with two suction pads
    pub const VG_2: i8 = 1;

    /// Vacuum Gripper with four suction pads
    pub const VG_4: i8 = 2;

}


impl Default for VacuumTools {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::VacuumTools::default())
  }
}

impl rosidl_runtime_rs::Message for VacuumTools {
  type RmwMsg = super::msg::rmw::VacuumTools;

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


// Corresponds to ariac_interfaces__msg__VoltageReading

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VoltageReading {

    // This member is not documented.
    #[allow(missing_docs)]
    pub voltage: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub operation_status: u8,

}



impl Default for VoltageReading {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::VoltageReading::default())
  }
}

impl rosidl_runtime_rs::Message for VoltageReading {
  type RmwMsg = super::msg::rmw::VoltageReading;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        voltage: msg.voltage,
        operation_status: msg.operation_status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      voltage: msg.voltage,
      operation_status: msg.operation_status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      voltage: msg.voltage,
      operation_status: msg.operation_status,
    }
  }
}


