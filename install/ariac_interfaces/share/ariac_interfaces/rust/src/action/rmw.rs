
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_Goal() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__GripperCommand_Goal__init(msg: *mut GripperCommand_Goal) -> bool;
    fn ariac_interfaces__action__GripperCommand_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Goal>, size: usize) -> bool;
    fn ariac_interfaces__action__GripperCommand_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Goal>);
    fn ariac_interfaces__action__GripperCommand_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Goal>) -> bool;
}

// Corresponds to ariac_interfaces__action__GripperCommand_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub width: f64,

}



impl Default for GripperCommand_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__GripperCommand_Goal__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__GripperCommand_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/GripperCommand_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_Goal() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_Result() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__GripperCommand_Result__init(msg: *mut GripperCommand_Result) -> bool;
    fn ariac_interfaces__action__GripperCommand_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Result>, size: usize) -> bool;
    fn ariac_interfaces__action__GripperCommand_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Result>);
    fn ariac_interfaces__action__GripperCommand_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Result>) -> bool;
}

// Corresponds to ariac_interfaces__action__GripperCommand_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stalled: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reached_goal_width: bool,

    /// final width
    pub width: f64,

}



impl Default for GripperCommand_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__GripperCommand_Result__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__GripperCommand_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_Result where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/GripperCommand_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_Result() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__GripperCommand_Feedback__init(msg: *mut GripperCommand_Feedback) -> bool;
    fn ariac_interfaces__action__GripperCommand_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Feedback>, size: usize) -> bool;
    fn ariac_interfaces__action__GripperCommand_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Feedback>);
    fn ariac_interfaces__action__GripperCommand_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_Feedback>) -> bool;
}

// Corresponds to ariac_interfaces__action__GripperCommand_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_Feedback {
    /// current width
    pub width: f64,

}



impl Default for GripperCommand_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__GripperCommand_Feedback__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__GripperCommand_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/GripperCommand_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_Feedback() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__GripperCommand_FeedbackMessage__init(msg: *mut GripperCommand_FeedbackMessage) -> bool;
    fn ariac_interfaces__action__GripperCommand_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_FeedbackMessage>, size: usize) -> bool;
    fn ariac_interfaces__action__GripperCommand_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_FeedbackMessage>);
    fn ariac_interfaces__action__GripperCommand_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_FeedbackMessage>) -> bool;
}

// Corresponds to ariac_interfaces__action__GripperCommand_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::GripperCommand_Feedback,

}



impl Default for GripperCommand_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__GripperCommand_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__GripperCommand_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/GripperCommand_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_FeedbackMessage() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_Goal() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__MoveAgv_Goal__init(msg: *mut MoveAgv_Goal) -> bool;
    fn ariac_interfaces__action__MoveAgv_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_Goal>, size: usize) -> bool;
    fn ariac_interfaces__action__MoveAgv_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_Goal>);
    fn ariac_interfaces__action__MoveAgv_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveAgv_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_Goal>) -> bool;
}

// Corresponds to ariac_interfaces__action__MoveAgv_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveAgv_Goal {
    /// Goal
    pub station_id: i8,

}



impl Default for MoveAgv_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__MoveAgv_Goal__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__MoveAgv_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveAgv_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveAgv_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveAgv_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/MoveAgv_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_Goal() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_Result() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__MoveAgv_Result__init(msg: *mut MoveAgv_Result) -> bool;
    fn ariac_interfaces__action__MoveAgv_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_Result>, size: usize) -> bool;
    fn ariac_interfaces__action__MoveAgv_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_Result>);
    fn ariac_interfaces__action__MoveAgv_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveAgv_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_Result>) -> bool;
}

// Corresponds to ariac_interfaces__action__MoveAgv_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveAgv_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::AgvStatus,

}



impl Default for MoveAgv_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__MoveAgv_Result__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__MoveAgv_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveAgv_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveAgv_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveAgv_Result where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/MoveAgv_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_Result() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__MoveAgv_Feedback__init(msg: *mut MoveAgv_Feedback) -> bool;
    fn ariac_interfaces__action__MoveAgv_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_Feedback>, size: usize) -> bool;
    fn ariac_interfaces__action__MoveAgv_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_Feedback>);
    fn ariac_interfaces__action__MoveAgv_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveAgv_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_Feedback>) -> bool;
}

// Corresponds to ariac_interfaces__action__MoveAgv_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveAgv_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::AgvStatus,

}



impl Default for MoveAgv_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__MoveAgv_Feedback__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__MoveAgv_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveAgv_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveAgv_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveAgv_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/MoveAgv_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_Feedback() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__MoveAgv_FeedbackMessage__init(msg: *mut MoveAgv_FeedbackMessage) -> bool;
    fn ariac_interfaces__action__MoveAgv_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_FeedbackMessage>, size: usize) -> bool;
    fn ariac_interfaces__action__MoveAgv_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_FeedbackMessage>);
    fn ariac_interfaces__action__MoveAgv_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveAgv_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_FeedbackMessage>) -> bool;
}

// Corresponds to ariac_interfaces__action__MoveAgv_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveAgv_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MoveAgv_Feedback,

}



impl Default for MoveAgv_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__MoveAgv_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__MoveAgv_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveAgv_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveAgv_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveAgv_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/MoveAgv_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_FeedbackMessage() }
  }
}




#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__GripperCommand_SendGoal_Request__init(msg: *mut GripperCommand_SendGoal_Request) -> bool;
    fn ariac_interfaces__action__GripperCommand_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Request>, size: usize) -> bool;
    fn ariac_interfaces__action__GripperCommand_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Request>);
    fn ariac_interfaces__action__GripperCommand_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Request>) -> bool;
}

// Corresponds to ariac_interfaces__action__GripperCommand_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::GripperCommand_Goal,

}



impl Default for GripperCommand_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__GripperCommand_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__GripperCommand_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/GripperCommand_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_SendGoal_Request() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__GripperCommand_SendGoal_Response__init(msg: *mut GripperCommand_SendGoal_Response) -> bool;
    fn ariac_interfaces__action__GripperCommand_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Response>, size: usize) -> bool;
    fn ariac_interfaces__action__GripperCommand_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Response>);
    fn ariac_interfaces__action__GripperCommand_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_SendGoal_Response>) -> bool;
}

// Corresponds to ariac_interfaces__action__GripperCommand_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for GripperCommand_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__GripperCommand_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__GripperCommand_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/GripperCommand_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_SendGoal_Response() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__GripperCommand_GetResult_Request__init(msg: *mut GripperCommand_GetResult_Request) -> bool;
    fn ariac_interfaces__action__GripperCommand_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Request>, size: usize) -> bool;
    fn ariac_interfaces__action__GripperCommand_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Request>);
    fn ariac_interfaces__action__GripperCommand_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Request>) -> bool;
}

// Corresponds to ariac_interfaces__action__GripperCommand_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for GripperCommand_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__GripperCommand_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__GripperCommand_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/GripperCommand_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_GetResult_Request() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__GripperCommand_GetResult_Response__init(msg: *mut GripperCommand_GetResult_Response) -> bool;
    fn ariac_interfaces__action__GripperCommand_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Response>, size: usize) -> bool;
    fn ariac_interfaces__action__GripperCommand_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Response>);
    fn ariac_interfaces__action__GripperCommand_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperCommand_GetResult_Response>) -> bool;
}

// Corresponds to ariac_interfaces__action__GripperCommand_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperCommand_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::GripperCommand_Result,

}



impl Default for GripperCommand_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__GripperCommand_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__GripperCommand_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperCommand_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__GripperCommand_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperCommand_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperCommand_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/GripperCommand_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__GripperCommand_GetResult_Response() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__MoveAgv_SendGoal_Request__init(msg: *mut MoveAgv_SendGoal_Request) -> bool;
    fn ariac_interfaces__action__MoveAgv_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_SendGoal_Request>, size: usize) -> bool;
    fn ariac_interfaces__action__MoveAgv_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_SendGoal_Request>);
    fn ariac_interfaces__action__MoveAgv_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveAgv_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_SendGoal_Request>) -> bool;
}

// Corresponds to ariac_interfaces__action__MoveAgv_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveAgv_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MoveAgv_Goal,

}



impl Default for MoveAgv_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__MoveAgv_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__MoveAgv_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveAgv_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveAgv_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveAgv_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/MoveAgv_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_SendGoal_Request() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__MoveAgv_SendGoal_Response__init(msg: *mut MoveAgv_SendGoal_Response) -> bool;
    fn ariac_interfaces__action__MoveAgv_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_SendGoal_Response>, size: usize) -> bool;
    fn ariac_interfaces__action__MoveAgv_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_SendGoal_Response>);
    fn ariac_interfaces__action__MoveAgv_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveAgv_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_SendGoal_Response>) -> bool;
}

// Corresponds to ariac_interfaces__action__MoveAgv_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveAgv_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MoveAgv_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__MoveAgv_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__MoveAgv_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveAgv_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveAgv_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveAgv_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/MoveAgv_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_SendGoal_Response() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__MoveAgv_GetResult_Request__init(msg: *mut MoveAgv_GetResult_Request) -> bool;
    fn ariac_interfaces__action__MoveAgv_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_GetResult_Request>, size: usize) -> bool;
    fn ariac_interfaces__action__MoveAgv_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_GetResult_Request>);
    fn ariac_interfaces__action__MoveAgv_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveAgv_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_GetResult_Request>) -> bool;
}

// Corresponds to ariac_interfaces__action__MoveAgv_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveAgv_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MoveAgv_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__MoveAgv_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__MoveAgv_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveAgv_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveAgv_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveAgv_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/MoveAgv_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_GetResult_Request() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__action__MoveAgv_GetResult_Response__init(msg: *mut MoveAgv_GetResult_Response) -> bool;
    fn ariac_interfaces__action__MoveAgv_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_GetResult_Response>, size: usize) -> bool;
    fn ariac_interfaces__action__MoveAgv_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_GetResult_Response>);
    fn ariac_interfaces__action__MoveAgv_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveAgv_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveAgv_GetResult_Response>) -> bool;
}

// Corresponds to ariac_interfaces__action__MoveAgv_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveAgv_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MoveAgv_Result,

}



impl Default for MoveAgv_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__action__MoveAgv_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__action__MoveAgv_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveAgv_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__action__MoveAgv_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveAgv_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveAgv_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/action/MoveAgv_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__action__MoveAgv_GetResult_Response() }
  }
}






#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__action__GripperCommand_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to ariac_interfaces__action__GripperCommand_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct GripperCommand_SendGoal;

impl rosidl_runtime_rs::Service for GripperCommand_SendGoal {
    type Request = GripperCommand_SendGoal_Request;
    type Response = GripperCommand_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__action__GripperCommand_SendGoal() }
    }
}




#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__action__GripperCommand_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to ariac_interfaces__action__GripperCommand_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct GripperCommand_GetResult;

impl rosidl_runtime_rs::Service for GripperCommand_GetResult {
    type Request = GripperCommand_GetResult_Request;
    type Response = GripperCommand_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__action__GripperCommand_GetResult() }
    }
}




#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__action__MoveAgv_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to ariac_interfaces__action__MoveAgv_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveAgv_SendGoal;

impl rosidl_runtime_rs::Service for MoveAgv_SendGoal {
    type Request = MoveAgv_SendGoal_Request;
    type Response = MoveAgv_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__action__MoveAgv_SendGoal() }
    }
}




#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__action__MoveAgv_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to ariac_interfaces__action__MoveAgv_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveAgv_GetResult;

impl rosidl_runtime_rs::Service for MoveAgv_GetResult {
    type Request = MoveAgv_GetResult_Request;
    type Response = MoveAgv_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ariac_interfaces__action__MoveAgv_GetResult() }
    }
}


