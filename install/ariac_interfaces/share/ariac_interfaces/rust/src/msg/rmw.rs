#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__AgvStations() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__AgvStations__init(msg: *mut AgvStations) -> bool;
    fn ariac_interfaces__msg__AgvStations__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AgvStations>, size: usize) -> bool;
    fn ariac_interfaces__msg__AgvStations__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AgvStations>);
    fn ariac_interfaces__msg__AgvStations__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AgvStations>, out_seq: *mut rosidl_runtime_rs::Sequence<AgvStations>) -> bool;
}

// Corresponds to ariac_interfaces__msg__AgvStations
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// AGVStations.msg

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__AgvStations__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__AgvStations__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AgvStations {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__AgvStations__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__AgvStations__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__AgvStations__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AgvStations {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AgvStations where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/AgvStations";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__AgvStations() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__AgvStatus() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__AgvStatus__init(msg: *mut AgvStatus) -> bool;
    fn ariac_interfaces__msg__AgvStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AgvStatus>, size: usize) -> bool;
    fn ariac_interfaces__msg__AgvStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AgvStatus>);
    fn ariac_interfaces__msg__AgvStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AgvStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<AgvStatus>) -> bool;
}

// Corresponds to ariac_interfaces__msg__AgvStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// AGVStatus.msg

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AgvStatus {
    /// Current location ID.
    /// Should be one of the values defined in AgvStations.msg.
    pub station_id: i8,

    /// Current pose of the AGV
    pub pose: geometry_msgs::msg::rmw::Pose,

}



impl Default for AgvStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__AgvStatus__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__AgvStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AgvStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__AgvStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__AgvStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__AgvStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AgvStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AgvStatus where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/AgvStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__AgvStatus() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__AgvTrayStatus() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__AgvTrayStatus__init(msg: *mut AgvTrayStatus) -> bool;
    fn ariac_interfaces__msg__AgvTrayStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AgvTrayStatus>, size: usize) -> bool;
    fn ariac_interfaces__msg__AgvTrayStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AgvTrayStatus>);
    fn ariac_interfaces__msg__AgvTrayStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AgvTrayStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<AgvTrayStatus>) -> bool;
}

// Corresponds to ariac_interfaces__msg__AgvTrayStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// AgvTrayStatus.msg

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__AgvTrayStatus__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__AgvTrayStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AgvTrayStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__AgvTrayStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__AgvTrayStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__AgvTrayStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AgvTrayStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AgvTrayStatus where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/AgvTrayStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__AgvTrayStatus() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__BreakBeamStatus() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__BreakBeamStatus__init(msg: *mut BreakBeamStatus) -> bool;
    fn ariac_interfaces__msg__BreakBeamStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BreakBeamStatus>, size: usize) -> bool;
    fn ariac_interfaces__msg__BreakBeamStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BreakBeamStatus>);
    fn ariac_interfaces__msg__BreakBeamStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BreakBeamStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<BreakBeamStatus>) -> bool;
}

// Corresponds to ariac_interfaces__msg__BreakBeamStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Single reading from a break beam sensor that detects the presence of
/// objects within its sensing range.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BreakBeamStatus {
    /// timestamp and name of tf frame
    pub header: std_msgs::msg::rmw::Header,

    /// is there something in the proximity of the sensor?
    pub object_detected: bool,

}



impl Default for BreakBeamStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__BreakBeamStatus__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__BreakBeamStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BreakBeamStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__BreakBeamStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__BreakBeamStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__BreakBeamStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BreakBeamStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BreakBeamStatus where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/BreakBeamStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__BreakBeamStatus() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__DistanceSensor() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__DistanceSensor__init(msg: *mut DistanceSensor) -> bool;
    fn ariac_interfaces__msg__DistanceSensor__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DistanceSensor>, size: usize) -> bool;
    fn ariac_interfaces__msg__DistanceSensor__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DistanceSensor>);
    fn ariac_interfaces__msg__DistanceSensor__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DistanceSensor>, out_seq: *mut rosidl_runtime_rs::Sequence<DistanceSensor>) -> bool;
}

// Corresponds to ariac_interfaces__msg__DistanceSensor
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DistanceSensor {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: f64,

}



impl Default for DistanceSensor {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__DistanceSensor__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__DistanceSensor__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DistanceSensor {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__DistanceSensor__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__DistanceSensor__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__DistanceSensor__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DistanceSensor {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DistanceSensor where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/DistanceSensor";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__DistanceSensor() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__CellDefect() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__CellDefect__init(msg: *mut CellDefect) -> bool;
    fn ariac_interfaces__msg__CellDefect__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CellDefect>, size: usize) -> bool;
    fn ariac_interfaces__msg__CellDefect__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CellDefect>);
    fn ariac_interfaces__msg__CellDefect__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CellDefect>, out_seq: *mut rosidl_runtime_rs::Sequence<CellDefect>) -> bool;
}

// Corresponds to ariac_interfaces__msg__CellDefect
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// CellDefect.msg

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__CellDefect__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__CellDefect__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CellDefect {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CellDefect__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CellDefect__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CellDefect__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CellDefect {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CellDefect where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/CellDefect";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__CellDefect() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__CellFeederStatus() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__CellFeederStatus__init(msg: *mut CellFeederStatus) -> bool;
    fn ariac_interfaces__msg__CellFeederStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CellFeederStatus>, size: usize) -> bool;
    fn ariac_interfaces__msg__CellFeederStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CellFeederStatus>);
    fn ariac_interfaces__msg__CellFeederStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CellFeederStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<CellFeederStatus>) -> bool;
}

// Corresponds to ariac_interfaces__msg__CellFeederStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// CellFeederStatus.msg

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__CellFeederStatus__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__CellFeederStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CellFeederStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CellFeederStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CellFeederStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CellFeederStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CellFeederStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CellFeederStatus where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/CellFeederStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__CellFeederStatus() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__CellTypes() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__CellTypes__init(msg: *mut CellTypes) -> bool;
    fn ariac_interfaces__msg__CellTypes__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CellTypes>, size: usize) -> bool;
    fn ariac_interfaces__msg__CellTypes__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CellTypes>);
    fn ariac_interfaces__msg__CellTypes__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CellTypes>, out_seq: *mut rosidl_runtime_rs::Sequence<CellTypes>) -> bool;
}

// Corresponds to ariac_interfaces__msg__CellTypes
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// CellTypes.msg

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__CellTypes__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__CellTypes__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CellTypes {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CellTypes__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CellTypes__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CellTypes__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CellTypes {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CellTypes where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/CellTypes";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__CellTypes() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__CompetitionStates() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__CompetitionStates__init(msg: *mut CompetitionStates) -> bool;
    fn ariac_interfaces__msg__CompetitionStates__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CompetitionStates>, size: usize) -> bool;
    fn ariac_interfaces__msg__CompetitionStates__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CompetitionStates>);
    fn ariac_interfaces__msg__CompetitionStates__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CompetitionStates>, out_seq: *mut rosidl_runtime_rs::Sequence<CompetitionStates>) -> bool;
}

// Corresponds to ariac_interfaces__msg__CompetitionStates
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__CompetitionStates__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__CompetitionStates__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CompetitionStates {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CompetitionStates__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CompetitionStates__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CompetitionStates__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CompetitionStates {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CompetitionStates where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/CompetitionStates";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__CompetitionStates() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__CompetitionStatus() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__CompetitionStatus__init(msg: *mut CompetitionStatus) -> bool;
    fn ariac_interfaces__msg__CompetitionStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CompetitionStatus>, size: usize) -> bool;
    fn ariac_interfaces__msg__CompetitionStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CompetitionStatus>);
    fn ariac_interfaces__msg__CompetitionStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CompetitionStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<CompetitionStatus>) -> bool;
}

// Corresponds to ariac_interfaces__msg__CompetitionStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// CompetitionStatus.msg

#[repr(C)]
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
    pub time: super::super::msg::rmw::CompetitionTime,


    // This member is not documented.
    #[allow(missing_docs)]
    pub run_id: i32,

}



impl Default for CompetitionStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__CompetitionStatus__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__CompetitionStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CompetitionStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CompetitionStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CompetitionStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CompetitionStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CompetitionStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CompetitionStatus where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/CompetitionStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__CompetitionStatus() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__CompetitionTime() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__CompetitionTime__init(msg: *mut CompetitionTime) -> bool;
    fn ariac_interfaces__msg__CompetitionTime__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CompetitionTime>, size: usize) -> bool;
    fn ariac_interfaces__msg__CompetitionTime__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CompetitionTime>);
    fn ariac_interfaces__msg__CompetitionTime__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CompetitionTime>, out_seq: *mut rosidl_runtime_rs::Sequence<CompetitionTime>) -> bool;
}

// Corresponds to ariac_interfaces__msg__CompetitionTime
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// CompetitionTime.msg

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CompetitionTime {

    // This member is not documented.
    #[allow(missing_docs)]
    pub start: builtin_interfaces::msg::rmw::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    pub elapsed: builtin_interfaces::msg::rmw::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub remaining: builtin_interfaces::msg::rmw::Duration,

}



impl Default for CompetitionTime {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__CompetitionTime__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__CompetitionTime__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CompetitionTime {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CompetitionTime__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CompetitionTime__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__CompetitionTime__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CompetitionTime {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CompetitionTime where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/CompetitionTime";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__CompetitionTime() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__ConveyorStatus() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__ConveyorStatus__init(msg: *mut ConveyorStatus) -> bool;
    fn ariac_interfaces__msg__ConveyorStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConveyorStatus>, size: usize) -> bool;
    fn ariac_interfaces__msg__ConveyorStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConveyorStatus>);
    fn ariac_interfaces__msg__ConveyorStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConveyorStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<ConveyorStatus>) -> bool;
}

// Corresponds to ariac_interfaces__msg__ConveyorStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__ConveyorStatus__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__ConveyorStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConveyorStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__ConveyorStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__ConveyorStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__ConveyorStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConveyorStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConveyorStatus where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/ConveyorStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__ConveyorStatus() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__HighPriorityOrder() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__HighPriorityOrder__init(msg: *mut HighPriorityOrder) -> bool;
    fn ariac_interfaces__msg__HighPriorityOrder__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HighPriorityOrder>, size: usize) -> bool;
    fn ariac_interfaces__msg__HighPriorityOrder__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HighPriorityOrder>);
    fn ariac_interfaces__msg__HighPriorityOrder__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HighPriorityOrder>, out_seq: *mut rosidl_runtime_rs::Sequence<HighPriorityOrder>) -> bool;
}

// Corresponds to ariac_interfaces__msg__HighPriorityOrder
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HighPriorityOrder {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: rosidl_runtime_rs::String,

}



impl Default for HighPriorityOrder {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__HighPriorityOrder__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__HighPriorityOrder__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HighPriorityOrder {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__HighPriorityOrder__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__HighPriorityOrder__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__HighPriorityOrder__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HighPriorityOrder {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HighPriorityOrder where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/HighPriorityOrder";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__HighPriorityOrder() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__InspectionReport() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__InspectionReport__init(msg: *mut InspectionReport) -> bool;
    fn ariac_interfaces__msg__InspectionReport__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InspectionReport>, size: usize) -> bool;
    fn ariac_interfaces__msg__InspectionReport__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InspectionReport>);
    fn ariac_interfaces__msg__InspectionReport__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InspectionReport>, out_seq: *mut rosidl_runtime_rs::Sequence<InspectionReport>) -> bool;
}

// Corresponds to ariac_interfaces__msg__InspectionReport
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// InspectionReport.msg

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InspectionReport {

    // This member is not documented.
    #[allow(missing_docs)]
    pub passed: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub defects: rosidl_runtime_rs::Sequence<super::super::msg::rmw::CellDefect>,

}



impl Default for InspectionReport {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__InspectionReport__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__InspectionReport__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InspectionReport {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__InspectionReport__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__InspectionReport__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__InspectionReport__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InspectionReport {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InspectionReport where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/InspectionReport";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__InspectionReport() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__OperationStates() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__OperationStates__init(msg: *mut OperationStates) -> bool;
    fn ariac_interfaces__msg__OperationStates__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<OperationStates>, size: usize) -> bool;
    fn ariac_interfaces__msg__OperationStates__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<OperationStates>);
    fn ariac_interfaces__msg__OperationStates__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<OperationStates>, out_seq: *mut rosidl_runtime_rs::Sequence<OperationStates>) -> bool;
}

// Corresponds to ariac_interfaces__msg__OperationStates
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__OperationStates__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__OperationStates__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for OperationStates {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__OperationStates__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__OperationStates__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__OperationStates__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for OperationStates {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for OperationStates where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/OperationStates";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__OperationStates() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__ToolChangerStatus() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__ToolChangerStatus__init(msg: *mut ToolChangerStatus) -> bool;
    fn ariac_interfaces__msg__ToolChangerStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolChangerStatus>, size: usize) -> bool;
    fn ariac_interfaces__msg__ToolChangerStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolChangerStatus>);
    fn ariac_interfaces__msg__ToolChangerStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolChangerStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolChangerStatus>) -> bool;
}

// Corresponds to ariac_interfaces__msg__ToolChangerStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// ToolChangerStatus.msg

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolChangerStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub attached_tool: i8,

}



impl Default for ToolChangerStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__ToolChangerStatus__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__ToolChangerStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolChangerStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__ToolChangerStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__ToolChangerStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__ToolChangerStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolChangerStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolChangerStatus where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/ToolChangerStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__ToolChangerStatus() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__VacuumTools() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__VacuumTools__init(msg: *mut VacuumTools) -> bool;
    fn ariac_interfaces__msg__VacuumTools__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<VacuumTools>, size: usize) -> bool;
    fn ariac_interfaces__msg__VacuumTools__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<VacuumTools>);
    fn ariac_interfaces__msg__VacuumTools__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<VacuumTools>, out_seq: *mut rosidl_runtime_rs::Sequence<VacuumTools>) -> bool;
}

// Corresponds to ariac_interfaces__msg__VacuumTools
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// VacuumTools.msg

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__VacuumTools__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__VacuumTools__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for VacuumTools {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__VacuumTools__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__VacuumTools__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__VacuumTools__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for VacuumTools {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for VacuumTools where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/VacuumTools";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__VacuumTools() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__VoltageReading() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__msg__VoltageReading__init(msg: *mut VoltageReading) -> bool;
    fn ariac_interfaces__msg__VoltageReading__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<VoltageReading>, size: usize) -> bool;
    fn ariac_interfaces__msg__VoltageReading__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<VoltageReading>);
    fn ariac_interfaces__msg__VoltageReading__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<VoltageReading>, out_seq: *mut rosidl_runtime_rs::Sequence<VoltageReading>) -> bool;
}

// Corresponds to ariac_interfaces__msg__VoltageReading
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__msg__VoltageReading__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__msg__VoltageReading__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for VoltageReading {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__VoltageReading__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__VoltageReading__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__msg__VoltageReading__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for VoltageReading {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for VoltageReading where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/msg/VoltageReading";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__msg__VoltageReading() }
  }
}


