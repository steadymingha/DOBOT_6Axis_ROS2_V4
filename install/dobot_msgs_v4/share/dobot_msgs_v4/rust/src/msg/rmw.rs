#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__msg__RobotStatus() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__msg__RobotStatus__init(msg: *mut RobotStatus) -> bool;
    fn dobot_msgs_v4__msg__RobotStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotStatus>, size: usize) -> bool;
    fn dobot_msgs_v4__msg__RobotStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotStatus>);
    fn dobot_msgs_v4__msg__RobotStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotStatus>) -> bool;
}

// Corresponds to dobot_msgs_v4__msg__RobotStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub is_enable: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_connected: bool,

}



impl Default for RobotStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__msg__RobotStatus__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__msg__RobotStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__msg__RobotStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__msg__RobotStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__msg__RobotStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotStatus where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/msg/RobotStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__msg__RobotStatus() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__msg__ToolVectorActual() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__msg__ToolVectorActual__init(msg: *mut ToolVectorActual) -> bool;
    fn dobot_msgs_v4__msg__ToolVectorActual__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolVectorActual>, size: usize) -> bool;
    fn dobot_msgs_v4__msg__ToolVectorActual__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolVectorActual>);
    fn dobot_msgs_v4__msg__ToolVectorActual__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolVectorActual>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolVectorActual>) -> bool;
}

// Corresponds to dobot_msgs_v4__msg__ToolVectorActual
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolVectorActual {

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

}



impl Default for ToolVectorActual {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__msg__ToolVectorActual__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__msg__ToolVectorActual__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolVectorActual {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__msg__ToolVectorActual__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__msg__ToolVectorActual__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__msg__ToolVectorActual__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolVectorActual {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolVectorActual where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/msg/ToolVectorActual";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__msg__ToolVectorActual() }
  }
}


