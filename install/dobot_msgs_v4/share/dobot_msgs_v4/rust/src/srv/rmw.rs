#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EnableRobot_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__EnableRobot_Request__init(msg: *mut EnableRobot_Request) -> bool;
    fn dobot_msgs_v4__srv__EnableRobot_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EnableRobot_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__EnableRobot_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EnableRobot_Request>);
    fn dobot_msgs_v4__srv__EnableRobot_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EnableRobot_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<EnableRobot_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__EnableRobot_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnableRobot_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for EnableRobot_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__EnableRobot_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__EnableRobot_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EnableRobot_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableRobot_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableRobot_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableRobot_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EnableRobot_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EnableRobot_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/EnableRobot_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EnableRobot_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EnableRobot_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__EnableRobot_Response__init(msg: *mut EnableRobot_Response) -> bool;
    fn dobot_msgs_v4__srv__EnableRobot_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EnableRobot_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__EnableRobot_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EnableRobot_Response>);
    fn dobot_msgs_v4__srv__EnableRobot_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EnableRobot_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<EnableRobot_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__EnableRobot_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnableRobot_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for EnableRobot_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__EnableRobot_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__EnableRobot_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EnableRobot_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableRobot_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableRobot_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableRobot_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EnableRobot_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EnableRobot_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/EnableRobot_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EnableRobot_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DisableRobot_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DisableRobot_Request__init(msg: *mut DisableRobot_Request) -> bool;
    fn dobot_msgs_v4__srv__DisableRobot_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DisableRobot_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DisableRobot_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DisableRobot_Request>);
    fn dobot_msgs_v4__srv__DisableRobot_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DisableRobot_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DisableRobot_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DisableRobot_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DisableRobot_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for DisableRobot_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DisableRobot_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DisableRobot_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DisableRobot_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DisableRobot_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DisableRobot_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DisableRobot_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DisableRobot_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DisableRobot_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DisableRobot_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DisableRobot_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DisableRobot_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DisableRobot_Response__init(msg: *mut DisableRobot_Response) -> bool;
    fn dobot_msgs_v4__srv__DisableRobot_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DisableRobot_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DisableRobot_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DisableRobot_Response>);
    fn dobot_msgs_v4__srv__DisableRobot_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DisableRobot_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DisableRobot_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DisableRobot_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DisableRobot_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DisableRobot_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DisableRobot_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DisableRobot_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DisableRobot_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DisableRobot_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DisableRobot_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DisableRobot_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DisableRobot_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DisableRobot_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DisableRobot_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DisableRobot_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ClearError_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ClearError_Request__init(msg: *mut ClearError_Request) -> bool;
    fn dobot_msgs_v4__srv__ClearError_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ClearError_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ClearError_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ClearError_Request>);
    fn dobot_msgs_v4__srv__ClearError_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ClearError_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ClearError_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ClearError_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearError_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ClearError_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ClearError_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ClearError_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ClearError_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ClearError_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ClearError_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ClearError_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ClearError_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ClearError_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ClearError_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ClearError_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ClearError_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ClearError_Response__init(msg: *mut ClearError_Response) -> bool;
    fn dobot_msgs_v4__srv__ClearError_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ClearError_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ClearError_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ClearError_Response>);
    fn dobot_msgs_v4__srv__ClearError_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ClearError_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ClearError_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ClearError_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearError_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ClearError_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ClearError_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ClearError_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ClearError_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ClearError_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ClearError_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ClearError_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ClearError_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ClearError_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ClearError_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ClearError_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SpeedFactor_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SpeedFactor_Request__init(msg: *mut SpeedFactor_Request) -> bool;
    fn dobot_msgs_v4__srv__SpeedFactor_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpeedFactor_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SpeedFactor_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpeedFactor_Request>);
    fn dobot_msgs_v4__srv__SpeedFactor_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpeedFactor_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SpeedFactor_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SpeedFactor_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedFactor_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ratio: i32,

}



impl Default for SpeedFactor_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SpeedFactor_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SpeedFactor_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpeedFactor_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SpeedFactor_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SpeedFactor_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SpeedFactor_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpeedFactor_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpeedFactor_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SpeedFactor_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SpeedFactor_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SpeedFactor_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SpeedFactor_Response__init(msg: *mut SpeedFactor_Response) -> bool;
    fn dobot_msgs_v4__srv__SpeedFactor_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpeedFactor_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SpeedFactor_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpeedFactor_Response>);
    fn dobot_msgs_v4__srv__SpeedFactor_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpeedFactor_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SpeedFactor_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SpeedFactor_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeedFactor_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SpeedFactor_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SpeedFactor_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SpeedFactor_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpeedFactor_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SpeedFactor_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SpeedFactor_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SpeedFactor_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpeedFactor_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpeedFactor_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SpeedFactor_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SpeedFactor_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__User_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__User_Request__init(msg: *mut User_Request) -> bool;
    fn dobot_msgs_v4__srv__User_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<User_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__User_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<User_Request>);
    fn dobot_msgs_v4__srv__User_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<User_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<User_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__User_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct User_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for User_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__User_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__User_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for User_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__User_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__User_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__User_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for User_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for User_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/User_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__User_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__User_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__User_Response__init(msg: *mut User_Response) -> bool;
    fn dobot_msgs_v4__srv__User_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<User_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__User_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<User_Response>);
    fn dobot_msgs_v4__srv__User_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<User_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<User_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__User_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct User_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for User_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__User_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__User_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for User_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__User_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__User_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__User_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for User_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for User_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/User_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__User_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Tool_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__Tool_Request__init(msg: *mut Tool_Request) -> bool;
    fn dobot_msgs_v4__srv__Tool_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Tool_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__Tool_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Tool_Request>);
    fn dobot_msgs_v4__srv__Tool_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Tool_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Tool_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__Tool_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Tool_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for Tool_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__Tool_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__Tool_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Tool_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Tool_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Tool_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Tool_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Tool_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Tool_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/Tool_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Tool_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Tool_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__Tool_Response__init(msg: *mut Tool_Response) -> bool;
    fn dobot_msgs_v4__srv__Tool_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Tool_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__Tool_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Tool_Response>);
    fn dobot_msgs_v4__srv__Tool_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Tool_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Tool_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__Tool_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Tool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for Tool_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__Tool_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__Tool_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Tool_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Tool_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Tool_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Tool_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Tool_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Tool_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/Tool_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Tool_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RobotMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RobotMode_Request__init(msg: *mut RobotMode_Request) -> bool;
    fn dobot_msgs_v4__srv__RobotMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotMode_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RobotMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotMode_Request>);
    fn dobot_msgs_v4__srv__RobotMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotMode_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RobotMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for RobotMode_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RobotMode_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RobotMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RobotMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RobotMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RobotMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RobotMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RobotMode_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RobotMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RobotMode_Response__init(msg: *mut RobotMode_Response) -> bool;
    fn dobot_msgs_v4__srv__RobotMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotMode_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RobotMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotMode_Response>);
    fn dobot_msgs_v4__srv__RobotMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotMode_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RobotMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RobotMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RobotMode_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RobotMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RobotMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RobotMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RobotMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RobotMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RobotMode_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetPayload_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetPayload_Request__init(msg: *mut SetPayload_Request) -> bool;
    fn dobot_msgs_v4__srv__SetPayload_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetPayload_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetPayload_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetPayload_Request>);
    fn dobot_msgs_v4__srv__SetPayload_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetPayload_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetPayload_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetPayload_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetPayload_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetPayload_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetPayload_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetPayload_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetPayload_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetPayload_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetPayload_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetPayload_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetPayload_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetPayload_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetPayload_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetPayload_Response__init(msg: *mut SetPayload_Response) -> bool;
    fn dobot_msgs_v4__srv__SetPayload_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetPayload_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetPayload_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetPayload_Response>);
    fn dobot_msgs_v4__srv__SetPayload_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetPayload_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetPayload_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetPayload_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPayload_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetPayload_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetPayload_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetPayload_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetPayload_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetPayload_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetPayload_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetPayload_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetPayload_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetPayload_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetPayload_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetPayload_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DO_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DO_Request__init(msg: *mut DO_Request) -> bool;
    fn dobot_msgs_v4__srv__DO_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DO_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DO_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DO_Request>);
    fn dobot_msgs_v4__srv__DO_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DO_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DO_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DO_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DO_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DO_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DO_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DO_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DO_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DO_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DO_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DO_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DO_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DO_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DO_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DO_Response__init(msg: *mut DO_Response) -> bool;
    fn dobot_msgs_v4__srv__DO_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DO_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DO_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DO_Response>);
    fn dobot_msgs_v4__srv__DO_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DO_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DO_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DO_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DO_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DO_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DO_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DO_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DO_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DO_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DO_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DO_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DO_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DO_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DO_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DOInstant_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DOInstant_Request__init(msg: *mut DOInstant_Request) -> bool;
    fn dobot_msgs_v4__srv__DOInstant_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DOInstant_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DOInstant_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DOInstant_Request>);
    fn dobot_msgs_v4__srv__DOInstant_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DOInstant_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DOInstant_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DOInstant_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DOInstant_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DOInstant_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DOInstant_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOInstant_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOInstant_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOInstant_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DOInstant_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DOInstant_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DOInstant_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DOInstant_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DOInstant_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DOInstant_Response__init(msg: *mut DOInstant_Response) -> bool;
    fn dobot_msgs_v4__srv__DOInstant_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DOInstant_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DOInstant_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DOInstant_Response>);
    fn dobot_msgs_v4__srv__DOInstant_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DOInstant_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DOInstant_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DOInstant_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DOInstant_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DOInstant_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DOInstant_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DOInstant_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DOInstant_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOInstant_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOInstant_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOInstant_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DOInstant_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DOInstant_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DOInstant_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DOInstant_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolDO_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ToolDO_Request__init(msg: *mut ToolDO_Request) -> bool;
    fn dobot_msgs_v4__srv__ToolDO_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolDO_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ToolDO_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolDO_Request>);
    fn dobot_msgs_v4__srv__ToolDO_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolDO_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolDO_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ToolDO_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ToolDO_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ToolDO_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolDO_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDO_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDO_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDO_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolDO_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolDO_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ToolDO_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolDO_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolDO_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ToolDO_Response__init(msg: *mut ToolDO_Response) -> bool;
    fn dobot_msgs_v4__srv__ToolDO_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolDO_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ToolDO_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolDO_Response>);
    fn dobot_msgs_v4__srv__ToolDO_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolDO_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolDO_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ToolDO_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ToolDO_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ToolDO_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ToolDO_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolDO_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDO_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDO_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDO_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolDO_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolDO_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ToolDO_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolDO_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolDOInstant_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ToolDOInstant_Request__init(msg: *mut ToolDOInstant_Request) -> bool;
    fn dobot_msgs_v4__srv__ToolDOInstant_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolDOInstant_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ToolDOInstant_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolDOInstant_Request>);
    fn dobot_msgs_v4__srv__ToolDOInstant_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolDOInstant_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolDOInstant_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ToolDOInstant_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ToolDOInstant_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ToolDOInstant_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolDOInstant_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDOInstant_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDOInstant_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDOInstant_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolDOInstant_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolDOInstant_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ToolDOInstant_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolDOInstant_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolDOInstant_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ToolDOInstant_Response__init(msg: *mut ToolDOInstant_Response) -> bool;
    fn dobot_msgs_v4__srv__ToolDOInstant_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolDOInstant_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ToolDOInstant_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolDOInstant_Response>);
    fn dobot_msgs_v4__srv__ToolDOInstant_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolDOInstant_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolDOInstant_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ToolDOInstant_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDOInstant_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ToolDOInstant_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ToolDOInstant_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ToolDOInstant_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolDOInstant_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDOInstant_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDOInstant_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDOInstant_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolDOInstant_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolDOInstant_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ToolDOInstant_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolDOInstant_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AO_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__AO_Request__init(msg: *mut AO_Request) -> bool;
    fn dobot_msgs_v4__srv__AO_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AO_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__AO_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AO_Request>);
    fn dobot_msgs_v4__srv__AO_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AO_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AO_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__AO_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__AO_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__AO_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AO_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AO_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AO_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AO_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AO_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AO_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/AO_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AO_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AO_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__AO_Response__init(msg: *mut AO_Response) -> bool;
    fn dobot_msgs_v4__srv__AO_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AO_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__AO_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AO_Response>);
    fn dobot_msgs_v4__srv__AO_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AO_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AO_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__AO_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for AO_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__AO_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__AO_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AO_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AO_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AO_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AO_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AO_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AO_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/AO_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AO_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AOInstant_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__AOInstant_Request__init(msg: *mut AOInstant_Request) -> bool;
    fn dobot_msgs_v4__srv__AOInstant_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AOInstant_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__AOInstant_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AOInstant_Request>);
    fn dobot_msgs_v4__srv__AOInstant_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AOInstant_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AOInstant_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__AOInstant_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__AOInstant_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__AOInstant_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AOInstant_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AOInstant_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AOInstant_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AOInstant_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AOInstant_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AOInstant_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/AOInstant_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AOInstant_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AOInstant_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__AOInstant_Response__init(msg: *mut AOInstant_Response) -> bool;
    fn dobot_msgs_v4__srv__AOInstant_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AOInstant_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__AOInstant_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AOInstant_Response>);
    fn dobot_msgs_v4__srv__AOInstant_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AOInstant_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AOInstant_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__AOInstant_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AOInstant_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for AOInstant_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__AOInstant_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__AOInstant_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AOInstant_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AOInstant_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AOInstant_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AOInstant_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AOInstant_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AOInstant_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/AOInstant_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AOInstant_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AccJ_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__AccJ_Request__init(msg: *mut AccJ_Request) -> bool;
    fn dobot_msgs_v4__srv__AccJ_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AccJ_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__AccJ_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AccJ_Request>);
    fn dobot_msgs_v4__srv__AccJ_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AccJ_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AccJ_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__AccJ_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AccJ_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: i32,

}



impl Default for AccJ_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__AccJ_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__AccJ_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AccJ_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AccJ_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AccJ_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AccJ_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AccJ_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AccJ_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/AccJ_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AccJ_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AccJ_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__AccJ_Response__init(msg: *mut AccJ_Response) -> bool;
    fn dobot_msgs_v4__srv__AccJ_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AccJ_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__AccJ_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AccJ_Response>);
    fn dobot_msgs_v4__srv__AccJ_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AccJ_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AccJ_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__AccJ_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AccJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for AccJ_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__AccJ_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__AccJ_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AccJ_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AccJ_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AccJ_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AccJ_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AccJ_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AccJ_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/AccJ_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AccJ_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AccL_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__AccL_Request__init(msg: *mut AccL_Request) -> bool;
    fn dobot_msgs_v4__srv__AccL_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AccL_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__AccL_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AccL_Request>);
    fn dobot_msgs_v4__srv__AccL_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AccL_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AccL_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__AccL_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AccL_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: i32,

}



impl Default for AccL_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__AccL_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__AccL_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AccL_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AccL_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AccL_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AccL_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AccL_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AccL_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/AccL_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AccL_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AccL_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__AccL_Response__init(msg: *mut AccL_Response) -> bool;
    fn dobot_msgs_v4__srv__AccL_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AccL_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__AccL_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AccL_Response>);
    fn dobot_msgs_v4__srv__AccL_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AccL_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AccL_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__AccL_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AccL_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for AccL_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__AccL_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__AccL_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AccL_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AccL_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AccL_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AccL_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AccL_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AccL_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/AccL_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AccL_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__VelJ_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__VelJ_Request__init(msg: *mut VelJ_Request) -> bool;
    fn dobot_msgs_v4__srv__VelJ_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<VelJ_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__VelJ_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<VelJ_Request>);
    fn dobot_msgs_v4__srv__VelJ_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<VelJ_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<VelJ_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__VelJ_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VelJ_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: i32,

}



impl Default for VelJ_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__VelJ_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__VelJ_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for VelJ_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__VelJ_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__VelJ_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__VelJ_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for VelJ_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for VelJ_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/VelJ_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__VelJ_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__VelJ_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__VelJ_Response__init(msg: *mut VelJ_Response) -> bool;
    fn dobot_msgs_v4__srv__VelJ_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<VelJ_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__VelJ_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<VelJ_Response>);
    fn dobot_msgs_v4__srv__VelJ_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<VelJ_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<VelJ_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__VelJ_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VelJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for VelJ_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__VelJ_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__VelJ_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for VelJ_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__VelJ_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__VelJ_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__VelJ_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for VelJ_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for VelJ_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/VelJ_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__VelJ_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__VelL_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__VelL_Request__init(msg: *mut VelL_Request) -> bool;
    fn dobot_msgs_v4__srv__VelL_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<VelL_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__VelL_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<VelL_Request>);
    fn dobot_msgs_v4__srv__VelL_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<VelL_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<VelL_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__VelL_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VelL_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: i32,

}



impl Default for VelL_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__VelL_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__VelL_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for VelL_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__VelL_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__VelL_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__VelL_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for VelL_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for VelL_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/VelL_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__VelL_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__VelL_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__VelL_Response__init(msg: *mut VelL_Response) -> bool;
    fn dobot_msgs_v4__srv__VelL_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<VelL_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__VelL_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<VelL_Response>);
    fn dobot_msgs_v4__srv__VelL_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<VelL_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<VelL_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__VelL_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VelL_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for VelL_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__VelL_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__VelL_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for VelL_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__VelL_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__VelL_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__VelL_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for VelL_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for VelL_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/VelL_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__VelL_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CP_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__CP_Request__init(msg: *mut CP_Request) -> bool;
    fn dobot_msgs_v4__srv__CP_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CP_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__CP_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CP_Request>);
    fn dobot_msgs_v4__srv__CP_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CP_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CP_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__CP_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CP_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub r: i32,

}



impl Default for CP_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__CP_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__CP_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CP_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CP_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CP_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CP_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CP_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CP_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/CP_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CP_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CP_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__CP_Response__init(msg: *mut CP_Response) -> bool;
    fn dobot_msgs_v4__srv__CP_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CP_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__CP_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CP_Response>);
    fn dobot_msgs_v4__srv__CP_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CP_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CP_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__CP_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CP_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for CP_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__CP_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__CP_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CP_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CP_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CP_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CP_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CP_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CP_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/CP_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CP_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__PowerOn_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__PowerOn_Request__init(msg: *mut PowerOn_Request) -> bool;
    fn dobot_msgs_v4__srv__PowerOn_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PowerOn_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__PowerOn_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PowerOn_Request>);
    fn dobot_msgs_v4__srv__PowerOn_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PowerOn_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PowerOn_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__PowerOn_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PowerOn_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for PowerOn_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__PowerOn_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__PowerOn_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PowerOn_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__PowerOn_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__PowerOn_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__PowerOn_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PowerOn_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PowerOn_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/PowerOn_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__PowerOn_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__PowerOn_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__PowerOn_Response__init(msg: *mut PowerOn_Response) -> bool;
    fn dobot_msgs_v4__srv__PowerOn_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PowerOn_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__PowerOn_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PowerOn_Response>);
    fn dobot_msgs_v4__srv__PowerOn_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PowerOn_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PowerOn_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__PowerOn_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PowerOn_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for PowerOn_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__PowerOn_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__PowerOn_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PowerOn_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__PowerOn_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__PowerOn_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__PowerOn_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PowerOn_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PowerOn_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/PowerOn_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__PowerOn_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RunScript_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RunScript_Request__init(msg: *mut RunScript_Request) -> bool;
    fn dobot_msgs_v4__srv__RunScript_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RunScript_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RunScript_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RunScript_Request>);
    fn dobot_msgs_v4__srv__RunScript_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RunScript_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RunScript_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RunScript_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RunScript_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub project_name: rosidl_runtime_rs::String,

}



impl Default for RunScript_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RunScript_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RunScript_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RunScript_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RunScript_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RunScript_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RunScript_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RunScript_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RunScript_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RunScript_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RunScript_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RunScript_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RunScript_Response__init(msg: *mut RunScript_Response) -> bool;
    fn dobot_msgs_v4__srv__RunScript_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RunScript_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RunScript_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RunScript_Response>);
    fn dobot_msgs_v4__srv__RunScript_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RunScript_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RunScript_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RunScript_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RunScript_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RunScript_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RunScript_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RunScript_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RunScript_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RunScript_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RunScript_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RunScript_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RunScript_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RunScript_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RunScript_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RunScript_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Stop_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__Stop_Request__init(msg: *mut Stop_Request) -> bool;
    fn dobot_msgs_v4__srv__Stop_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Stop_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__Stop_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Stop_Request>);
    fn dobot_msgs_v4__srv__Stop_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Stop_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Stop_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__Stop_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stop_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Stop_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__Stop_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__Stop_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Stop_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Stop_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Stop_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Stop_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Stop_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Stop_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/Stop_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Stop_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Stop_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__Stop_Response__init(msg: *mut Stop_Response) -> bool;
    fn dobot_msgs_v4__srv__Stop_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Stop_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__Stop_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Stop_Response>);
    fn dobot_msgs_v4__srv__Stop_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Stop_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Stop_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__Stop_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Stop_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for Stop_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__Stop_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__Stop_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Stop_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Stop_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Stop_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Stop_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Stop_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Stop_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/Stop_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Stop_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Pause_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__Pause_Request__init(msg: *mut Pause_Request) -> bool;
    fn dobot_msgs_v4__srv__Pause_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Pause_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__Pause_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Pause_Request>);
    fn dobot_msgs_v4__srv__Pause_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Pause_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Pause_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__Pause_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pause_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Pause_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__Pause_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__Pause_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Pause_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Pause_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Pause_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Pause_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Pause_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Pause_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/Pause_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Pause_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Pause_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__Pause_Response__init(msg: *mut Pause_Response) -> bool;
    fn dobot_msgs_v4__srv__Pause_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Pause_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__Pause_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Pause_Response>);
    fn dobot_msgs_v4__srv__Pause_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Pause_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Pause_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__Pause_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pause_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for Pause_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__Pause_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__Pause_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Pause_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Pause_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Pause_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Pause_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Pause_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Pause_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/Pause_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Pause_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Continue_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__Continue_Request__init(msg: *mut Continue_Request) -> bool;
    fn dobot_msgs_v4__srv__Continue_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Continue_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__Continue_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Continue_Request>);
    fn dobot_msgs_v4__srv__Continue_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Continue_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Continue_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__Continue_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Continue_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Continue_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__Continue_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__Continue_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Continue_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Continue_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Continue_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Continue_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Continue_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Continue_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/Continue_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Continue_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Continue_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__Continue_Response__init(msg: *mut Continue_Response) -> bool;
    fn dobot_msgs_v4__srv__Continue_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Continue_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__Continue_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Continue_Response>);
    fn dobot_msgs_v4__srv__Continue_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Continue_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Continue_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__Continue_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Continue_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for Continue_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__Continue_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__Continue_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Continue_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Continue_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Continue_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Continue_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Continue_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Continue_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/Continue_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Continue_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__PositiveKin_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__PositiveKin_Request__init(msg: *mut PositiveKin_Request) -> bool;
    fn dobot_msgs_v4__srv__PositiveKin_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PositiveKin_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__PositiveKin_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PositiveKin_Request>);
    fn dobot_msgs_v4__srv__PositiveKin_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PositiveKin_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PositiveKin_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__PositiveKin_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub user: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tool: rosidl_runtime_rs::String,

}



impl Default for PositiveKin_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__PositiveKin_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__PositiveKin_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PositiveKin_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__PositiveKin_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__PositiveKin_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__PositiveKin_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PositiveKin_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PositiveKin_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/PositiveKin_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__PositiveKin_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__PositiveKin_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__PositiveKin_Response__init(msg: *mut PositiveKin_Response) -> bool;
    fn dobot_msgs_v4__srv__PositiveKin_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PositiveKin_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__PositiveKin_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PositiveKin_Response>);
    fn dobot_msgs_v4__srv__PositiveKin_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PositiveKin_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PositiveKin_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__PositiveKin_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PositiveKin_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for PositiveKin_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__PositiveKin_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__PositiveKin_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PositiveKin_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__PositiveKin_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__PositiveKin_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__PositiveKin_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PositiveKin_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PositiveKin_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/PositiveKin_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__PositiveKin_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__InverseKin_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__InverseKin_Request__init(msg: *mut InverseKin_Request) -> bool;
    fn dobot_msgs_v4__srv__InverseKin_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InverseKin_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__InverseKin_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InverseKin_Request>);
    fn dobot_msgs_v4__srv__InverseKin_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InverseKin_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<InverseKin_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__InverseKin_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub use_joint_near: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_near: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub user: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tool: rosidl_runtime_rs::String,

}



impl Default for InverseKin_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__InverseKin_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__InverseKin_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InverseKin_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__InverseKin_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__InverseKin_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__InverseKin_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InverseKin_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InverseKin_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/InverseKin_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__InverseKin_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__InverseKin_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__InverseKin_Response__init(msg: *mut InverseKin_Response) -> bool;
    fn dobot_msgs_v4__srv__InverseKin_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InverseKin_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__InverseKin_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InverseKin_Response>);
    fn dobot_msgs_v4__srv__InverseKin_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InverseKin_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<InverseKin_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__InverseKin_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InverseKin_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for InverseKin_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__InverseKin_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__InverseKin_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InverseKin_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__InverseKin_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__InverseKin_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__InverseKin_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InverseKin_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InverseKin_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/InverseKin_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__InverseKin_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetCollisionLevel_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetCollisionLevel_Request__init(msg: *mut SetCollisionLevel_Request) -> bool;
    fn dobot_msgs_v4__srv__SetCollisionLevel_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCollisionLevel_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetCollisionLevel_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCollisionLevel_Request>);
    fn dobot_msgs_v4__srv__SetCollisionLevel_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCollisionLevel_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCollisionLevel_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetCollisionLevel_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCollisionLevel_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub level: i32,

}



impl Default for SetCollisionLevel_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetCollisionLevel_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetCollisionLevel_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCollisionLevel_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetCollisionLevel_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetCollisionLevel_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetCollisionLevel_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCollisionLevel_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCollisionLevel_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetCollisionLevel_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetCollisionLevel_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetCollisionLevel_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetCollisionLevel_Response__init(msg: *mut SetCollisionLevel_Response) -> bool;
    fn dobot_msgs_v4__srv__SetCollisionLevel_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCollisionLevel_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetCollisionLevel_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCollisionLevel_Response>);
    fn dobot_msgs_v4__srv__SetCollisionLevel_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCollisionLevel_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCollisionLevel_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetCollisionLevel_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCollisionLevel_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetCollisionLevel_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetCollisionLevel_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetCollisionLevel_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCollisionLevel_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetCollisionLevel_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetCollisionLevel_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetCollisionLevel_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCollisionLevel_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCollisionLevel_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetCollisionLevel_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetCollisionLevel_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetAngle_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetAngle_Request__init(msg: *mut GetAngle_Request) -> bool;
    fn dobot_msgs_v4__srv__GetAngle_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetAngle_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetAngle_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetAngle_Request>);
    fn dobot_msgs_v4__srv__GetAngle_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetAngle_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetAngle_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetAngle_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetAngle_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetAngle_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetAngle_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetAngle_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetAngle_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetAngle_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetAngle_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetAngle_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetAngle_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetAngle_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetAngle_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetAngle_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetAngle_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetAngle_Response__init(msg: *mut GetAngle_Response) -> bool;
    fn dobot_msgs_v4__srv__GetAngle_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetAngle_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetAngle_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetAngle_Response>);
    fn dobot_msgs_v4__srv__GetAngle_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetAngle_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetAngle_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetAngle_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetAngle_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetAngle_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetAngle_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetAngle_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetAngle_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetAngle_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetAngle_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetAngle_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetAngle_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetAngle_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetAngle_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetAngle_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetPose_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetPose_Request__init(msg: *mut GetPose_Request) -> bool;
    fn dobot_msgs_v4__srv__GetPose_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPose_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetPose_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPose_Request>);
    fn dobot_msgs_v4__srv__GetPose_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPose_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPose_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetPose_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetPose_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetPose_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPose_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetPose_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetPose_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetPose_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPose_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPose_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetPose_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetPose_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetPose_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetPose_Response__init(msg: *mut GetPose_Response) -> bool;
    fn dobot_msgs_v4__srv__GetPose_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPose_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetPose_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPose_Response>);
    fn dobot_msgs_v4__srv__GetPose_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPose_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPose_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetPose_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetPose_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetPose_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetPose_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPose_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetPose_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetPose_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetPose_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPose_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPose_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetPose_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetPose_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EmergencyStop_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__EmergencyStop_Request__init(msg: *mut EmergencyStop_Request) -> bool;
    fn dobot_msgs_v4__srv__EmergencyStop_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EmergencyStop_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__EmergencyStop_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EmergencyStop_Request>);
    fn dobot_msgs_v4__srv__EmergencyStop_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EmergencyStop_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<EmergencyStop_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__EmergencyStop_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EmergencyStop_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i32,

}



impl Default for EmergencyStop_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__EmergencyStop_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__EmergencyStop_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EmergencyStop_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EmergencyStop_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EmergencyStop_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EmergencyStop_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EmergencyStop_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EmergencyStop_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/EmergencyStop_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EmergencyStop_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EmergencyStop_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__EmergencyStop_Response__init(msg: *mut EmergencyStop_Response) -> bool;
    fn dobot_msgs_v4__srv__EmergencyStop_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EmergencyStop_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__EmergencyStop_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EmergencyStop_Response>);
    fn dobot_msgs_v4__srv__EmergencyStop_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EmergencyStop_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<EmergencyStop_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__EmergencyStop_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EmergencyStop_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for EmergencyStop_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__EmergencyStop_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__EmergencyStop_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EmergencyStop_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EmergencyStop_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EmergencyStop_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EmergencyStop_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EmergencyStop_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EmergencyStop_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/EmergencyStop_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EmergencyStop_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ModbusRTUCreate_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ModbusRTUCreate_Request__init(msg: *mut ModbusRTUCreate_Request) -> bool;
    fn dobot_msgs_v4__srv__ModbusRTUCreate_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ModbusRTUCreate_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ModbusRTUCreate_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ModbusRTUCreate_Request>);
    fn dobot_msgs_v4__srv__ModbusRTUCreate_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ModbusRTUCreate_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ModbusRTUCreate_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ModbusRTUCreate_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub parity: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data_bit: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stop_bit: i32,

}



impl Default for ModbusRTUCreate_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ModbusRTUCreate_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ModbusRTUCreate_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ModbusRTUCreate_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusRTUCreate_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusRTUCreate_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusRTUCreate_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ModbusRTUCreate_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ModbusRTUCreate_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ModbusRTUCreate_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ModbusRTUCreate_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ModbusRTUCreate_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ModbusRTUCreate_Response__init(msg: *mut ModbusRTUCreate_Response) -> bool;
    fn dobot_msgs_v4__srv__ModbusRTUCreate_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ModbusRTUCreate_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ModbusRTUCreate_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ModbusRTUCreate_Response>);
    fn dobot_msgs_v4__srv__ModbusRTUCreate_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ModbusRTUCreate_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ModbusRTUCreate_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ModbusRTUCreate_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModbusRTUCreate_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ModbusRTUCreate_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ModbusRTUCreate_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ModbusRTUCreate_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ModbusRTUCreate_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusRTUCreate_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusRTUCreate_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusRTUCreate_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ModbusRTUCreate_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ModbusRTUCreate_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ModbusRTUCreate_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ModbusRTUCreate_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ModbusCreate_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ModbusCreate_Request__init(msg: *mut ModbusCreate_Request) -> bool;
    fn dobot_msgs_v4__srv__ModbusCreate_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ModbusCreate_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ModbusCreate_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ModbusCreate_Request>);
    fn dobot_msgs_v4__srv__ModbusCreate_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ModbusCreate_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ModbusCreate_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ModbusCreate_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModbusCreate_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ip: rosidl_runtime_rs::String,


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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ModbusCreate_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ModbusCreate_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ModbusCreate_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusCreate_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusCreate_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusCreate_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ModbusCreate_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ModbusCreate_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ModbusCreate_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ModbusCreate_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ModbusCreate_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ModbusCreate_Response__init(msg: *mut ModbusCreate_Response) -> bool;
    fn dobot_msgs_v4__srv__ModbusCreate_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ModbusCreate_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ModbusCreate_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ModbusCreate_Response>);
    fn dobot_msgs_v4__srv__ModbusCreate_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ModbusCreate_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ModbusCreate_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ModbusCreate_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModbusCreate_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ModbusCreate_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ModbusCreate_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ModbusCreate_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ModbusCreate_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusCreate_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusCreate_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusCreate_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ModbusCreate_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ModbusCreate_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ModbusCreate_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ModbusCreate_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ModbusClose_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ModbusClose_Request__init(msg: *mut ModbusClose_Request) -> bool;
    fn dobot_msgs_v4__srv__ModbusClose_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ModbusClose_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ModbusClose_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ModbusClose_Request>);
    fn dobot_msgs_v4__srv__ModbusClose_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ModbusClose_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ModbusClose_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ModbusClose_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModbusClose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for ModbusClose_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ModbusClose_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ModbusClose_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ModbusClose_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusClose_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusClose_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusClose_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ModbusClose_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ModbusClose_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ModbusClose_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ModbusClose_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ModbusClose_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ModbusClose_Response__init(msg: *mut ModbusClose_Response) -> bool;
    fn dobot_msgs_v4__srv__ModbusClose_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ModbusClose_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ModbusClose_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ModbusClose_Response>);
    fn dobot_msgs_v4__srv__ModbusClose_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ModbusClose_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ModbusClose_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ModbusClose_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ModbusClose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ModbusClose_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ModbusClose_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ModbusClose_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ModbusClose_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusClose_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusClose_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ModbusClose_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ModbusClose_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ModbusClose_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ModbusClose_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ModbusClose_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInBits_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetInBits_Request__init(msg: *mut GetInBits_Request) -> bool;
    fn dobot_msgs_v4__srv__GetInBits_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetInBits_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetInBits_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetInBits_Request>);
    fn dobot_msgs_v4__srv__GetInBits_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetInBits_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetInBits_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetInBits_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetInBits_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetInBits_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetInBits_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInBits_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInBits_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInBits_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetInBits_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetInBits_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetInBits_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInBits_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInBits_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetInBits_Response__init(msg: *mut GetInBits_Response) -> bool;
    fn dobot_msgs_v4__srv__GetInBits_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetInBits_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetInBits_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetInBits_Response>);
    fn dobot_msgs_v4__srv__GetInBits_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetInBits_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetInBits_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetInBits_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInBits_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetInBits_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetInBits_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetInBits_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetInBits_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInBits_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInBits_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInBits_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetInBits_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetInBits_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetInBits_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInBits_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInRegs_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetInRegs_Request__init(msg: *mut GetInRegs_Request) -> bool;
    fn dobot_msgs_v4__srv__GetInRegs_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetInRegs_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetInRegs_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetInRegs_Request>);
    fn dobot_msgs_v4__srv__GetInRegs_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetInRegs_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetInRegs_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetInRegs_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub val_type: rosidl_runtime_rs::String,

}



impl Default for GetInRegs_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetInRegs_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetInRegs_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetInRegs_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInRegs_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInRegs_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInRegs_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetInRegs_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetInRegs_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetInRegs_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInRegs_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInRegs_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetInRegs_Response__init(msg: *mut GetInRegs_Response) -> bool;
    fn dobot_msgs_v4__srv__GetInRegs_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetInRegs_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetInRegs_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetInRegs_Response>);
    fn dobot_msgs_v4__srv__GetInRegs_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetInRegs_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetInRegs_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetInRegs_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInRegs_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetInRegs_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetInRegs_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetInRegs_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetInRegs_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInRegs_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInRegs_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInRegs_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetInRegs_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetInRegs_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetInRegs_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInRegs_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetCoils_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetCoils_Request__init(msg: *mut GetCoils_Request) -> bool;
    fn dobot_msgs_v4__srv__GetCoils_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetCoils_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetCoils_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetCoils_Request>);
    fn dobot_msgs_v4__srv__GetCoils_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetCoils_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetCoils_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetCoils_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetCoils_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetCoils_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetCoils_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetCoils_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetCoils_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetCoils_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetCoils_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetCoils_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetCoils_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetCoils_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetCoils_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetCoils_Response__init(msg: *mut GetCoils_Response) -> bool;
    fn dobot_msgs_v4__srv__GetCoils_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetCoils_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetCoils_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetCoils_Response>);
    fn dobot_msgs_v4__srv__GetCoils_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetCoils_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetCoils_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetCoils_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetCoils_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetCoils_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetCoils_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetCoils_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetCoils_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetCoils_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetCoils_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetCoils_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetCoils_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetCoils_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetCoils_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetCoils_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetCoils_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetCoils_Request__init(msg: *mut SetCoils_Request) -> bool;
    fn dobot_msgs_v4__srv__SetCoils_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCoils_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetCoils_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCoils_Request>);
    fn dobot_msgs_v4__srv__SetCoils_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCoils_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCoils_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetCoils_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub val_tab: rosidl_runtime_rs::String,

}



impl Default for SetCoils_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetCoils_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetCoils_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCoils_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetCoils_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetCoils_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetCoils_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCoils_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCoils_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetCoils_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetCoils_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetCoils_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetCoils_Response__init(msg: *mut SetCoils_Response) -> bool;
    fn dobot_msgs_v4__srv__SetCoils_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetCoils_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetCoils_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetCoils_Response>);
    fn dobot_msgs_v4__srv__SetCoils_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetCoils_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetCoils_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetCoils_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetCoils_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetCoils_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetCoils_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetCoils_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetCoils_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetCoils_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetCoils_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetCoils_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetCoils_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetCoils_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetCoils_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetCoils_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetHoldRegs_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetHoldRegs_Request__init(msg: *mut GetHoldRegs_Request) -> bool;
    fn dobot_msgs_v4__srv__GetHoldRegs_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetHoldRegs_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetHoldRegs_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetHoldRegs_Request>);
    fn dobot_msgs_v4__srv__GetHoldRegs_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetHoldRegs_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetHoldRegs_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetHoldRegs_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub val_type: rosidl_runtime_rs::String,

}



impl Default for GetHoldRegs_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetHoldRegs_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetHoldRegs_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetHoldRegs_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetHoldRegs_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetHoldRegs_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetHoldRegs_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetHoldRegs_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetHoldRegs_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetHoldRegs_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetHoldRegs_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetHoldRegs_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetHoldRegs_Response__init(msg: *mut GetHoldRegs_Response) -> bool;
    fn dobot_msgs_v4__srv__GetHoldRegs_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetHoldRegs_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetHoldRegs_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetHoldRegs_Response>);
    fn dobot_msgs_v4__srv__GetHoldRegs_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetHoldRegs_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetHoldRegs_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetHoldRegs_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetHoldRegs_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetHoldRegs_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetHoldRegs_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetHoldRegs_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetHoldRegs_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetHoldRegs_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetHoldRegs_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetHoldRegs_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetHoldRegs_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetHoldRegs_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetHoldRegs_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetHoldRegs_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetHoldRegs_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetHoldRegs_Request__init(msg: *mut SetHoldRegs_Request) -> bool;
    fn dobot_msgs_v4__srv__SetHoldRegs_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetHoldRegs_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetHoldRegs_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetHoldRegs_Request>);
    fn dobot_msgs_v4__srv__SetHoldRegs_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetHoldRegs_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetHoldRegs_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetHoldRegs_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub val_tab: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub val_type: rosidl_runtime_rs::String,

}



impl Default for SetHoldRegs_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetHoldRegs_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetHoldRegs_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetHoldRegs_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetHoldRegs_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetHoldRegs_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetHoldRegs_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetHoldRegs_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetHoldRegs_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetHoldRegs_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetHoldRegs_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetHoldRegs_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetHoldRegs_Response__init(msg: *mut SetHoldRegs_Response) -> bool;
    fn dobot_msgs_v4__srv__SetHoldRegs_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetHoldRegs_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetHoldRegs_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetHoldRegs_Response>);
    fn dobot_msgs_v4__srv__SetHoldRegs_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetHoldRegs_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetHoldRegs_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetHoldRegs_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetHoldRegs_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetHoldRegs_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetHoldRegs_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetHoldRegs_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetHoldRegs_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetHoldRegs_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetHoldRegs_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetHoldRegs_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetHoldRegs_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetHoldRegs_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetHoldRegs_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetHoldRegs_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetSafeSkin_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetSafeSkin_Request__init(msg: *mut SetSafeSkin_Request) -> bool;
    fn dobot_msgs_v4__srv__SetSafeSkin_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetSafeSkin_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetSafeSkin_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetSafeSkin_Request>);
    fn dobot_msgs_v4__srv__SetSafeSkin_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetSafeSkin_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetSafeSkin_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetSafeSkin_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetSafeSkin_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetSafeSkin_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetSafeSkin_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetSafeSkin_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetSafeSkin_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetSafeSkin_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetSafeSkin_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetSafeSkin_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetSafeSkin_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetSafeSkin_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetSafeSkin_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetSafeSkin_Response__init(msg: *mut SetSafeSkin_Response) -> bool;
    fn dobot_msgs_v4__srv__SetSafeSkin_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetSafeSkin_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetSafeSkin_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetSafeSkin_Response>);
    fn dobot_msgs_v4__srv__SetSafeSkin_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetSafeSkin_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetSafeSkin_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetSafeSkin_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetSafeSkin_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetSafeSkin_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetSafeSkin_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetSafeSkin_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetSafeSkin_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetSafeSkin_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetSafeSkin_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetSafeSkin_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetSafeSkin_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetSafeSkin_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetSafeSkin_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetSafeSkin_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovJ_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__MovJ_Request__init(msg: *mut MovJ_Request) -> bool;
    fn dobot_msgs_v4__srv__MovJ_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJ_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__MovJ_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJ_Request>);
    fn dobot_msgs_v4__srv__MovJ_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJ_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJ_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__MovJ_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for MovJ_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__MovJ_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__MovJ_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJ_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovJ_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovJ_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovJ_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJ_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJ_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/MovJ_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovJ_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovJ_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__MovJ_Response__init(msg: *mut MovJ_Response) -> bool;
    fn dobot_msgs_v4__srv__MovJ_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJ_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__MovJ_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJ_Response>);
    fn dobot_msgs_v4__srv__MovJ_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJ_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJ_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__MovJ_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for MovJ_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__MovJ_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__MovJ_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJ_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovJ_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovJ_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovJ_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJ_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJ_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/MovJ_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovJ_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovL_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__MovL_Request__init(msg: *mut MovL_Request) -> bool;
    fn dobot_msgs_v4__srv__MovL_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovL_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__MovL_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovL_Request>);
    fn dobot_msgs_v4__srv__MovL_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovL_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovL_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__MovL_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for MovL_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__MovL_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__MovL_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovL_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovL_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovL_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovL_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovL_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovL_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/MovL_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovL_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovL_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__MovL_Response__init(msg: *mut MovL_Response) -> bool;
    fn dobot_msgs_v4__srv__MovL_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovL_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__MovL_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovL_Response>);
    fn dobot_msgs_v4__srv__MovL_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovL_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovL_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__MovL_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovL_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for MovL_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__MovL_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__MovL_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovL_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovL_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovL_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovL_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovL_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovL_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/MovL_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovL_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelJointMovJ_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RelJointMovJ_Request__init(msg: *mut RelJointMovJ_Request) -> bool;
    fn dobot_msgs_v4__srv__RelJointMovJ_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RelJointMovJ_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RelJointMovJ_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RelJointMovJ_Request>);
    fn dobot_msgs_v4__srv__RelJointMovJ_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RelJointMovJ_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RelJointMovJ_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RelJointMovJ_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for RelJointMovJ_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RelJointMovJ_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RelJointMovJ_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RelJointMovJ_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelJointMovJ_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelJointMovJ_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelJointMovJ_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RelJointMovJ_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RelJointMovJ_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RelJointMovJ_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelJointMovJ_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelJointMovJ_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RelJointMovJ_Response__init(msg: *mut RelJointMovJ_Response) -> bool;
    fn dobot_msgs_v4__srv__RelJointMovJ_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RelJointMovJ_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RelJointMovJ_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RelJointMovJ_Response>);
    fn dobot_msgs_v4__srv__RelJointMovJ_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RelJointMovJ_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RelJointMovJ_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RelJointMovJ_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelJointMovJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RelJointMovJ_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RelJointMovJ_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RelJointMovJ_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RelJointMovJ_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelJointMovJ_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelJointMovJ_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelJointMovJ_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RelJointMovJ_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RelJointMovJ_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RelJointMovJ_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelJointMovJ_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MoveJog_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__MoveJog_Request__init(msg: *mut MoveJog_Request) -> bool;
    fn dobot_msgs_v4__srv__MoveJog_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveJog_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__MoveJog_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveJog_Request>);
    fn dobot_msgs_v4__srv__MoveJog_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveJog_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveJog_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__MoveJog_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveJog_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub axis_id: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for MoveJog_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__MoveJog_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__MoveJog_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveJog_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MoveJog_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MoveJog_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MoveJog_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveJog_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveJog_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/MoveJog_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MoveJog_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MoveJog_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__MoveJog_Response__init(msg: *mut MoveJog_Response) -> bool;
    fn dobot_msgs_v4__srv__MoveJog_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveJog_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__MoveJog_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveJog_Response>);
    fn dobot_msgs_v4__srv__MoveJog_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveJog_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveJog_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__MoveJog_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveJog_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for MoveJog_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__MoveJog_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__MoveJog_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveJog_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MoveJog_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MoveJog_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MoveJog_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveJog_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveJog_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/MoveJog_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MoveJog_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StopMoveJog_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__StopMoveJog_Request__init(msg: *mut StopMoveJog_Request) -> bool;
    fn dobot_msgs_v4__srv__StopMoveJog_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StopMoveJog_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__StopMoveJog_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StopMoveJog_Request>);
    fn dobot_msgs_v4__srv__StopMoveJog_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StopMoveJog_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<StopMoveJog_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__StopMoveJog_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StopMoveJog_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for StopMoveJog_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__StopMoveJog_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__StopMoveJog_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StopMoveJog_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StopMoveJog_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StopMoveJog_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StopMoveJog_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StopMoveJog_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StopMoveJog_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/StopMoveJog_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StopMoveJog_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StopMoveJog_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__StopMoveJog_Response__init(msg: *mut StopMoveJog_Response) -> bool;
    fn dobot_msgs_v4__srv__StopMoveJog_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StopMoveJog_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__StopMoveJog_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StopMoveJog_Response>);
    fn dobot_msgs_v4__srv__StopMoveJog_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StopMoveJog_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<StopMoveJog_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__StopMoveJog_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StopMoveJog_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for StopMoveJog_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__StopMoveJog_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__StopMoveJog_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StopMoveJog_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StopMoveJog_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StopMoveJog_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StopMoveJog_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StopMoveJog_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StopMoveJog_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/StopMoveJog_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StopMoveJog_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DOGroup_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DOGroup_Request__init(msg: *mut DOGroup_Request) -> bool;
    fn dobot_msgs_v4__srv__DOGroup_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DOGroup_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DOGroup_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DOGroup_Request>);
    fn dobot_msgs_v4__srv__DOGroup_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DOGroup_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DOGroup_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DOGroup_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DOGroup_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub args: rosidl_runtime_rs::Sequence<i32>,

}



impl Default for DOGroup_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DOGroup_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DOGroup_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DOGroup_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOGroup_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOGroup_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOGroup_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DOGroup_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DOGroup_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DOGroup_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DOGroup_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DOGroup_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DOGroup_Response__init(msg: *mut DOGroup_Response) -> bool;
    fn dobot_msgs_v4__srv__DOGroup_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DOGroup_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DOGroup_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DOGroup_Response>);
    fn dobot_msgs_v4__srv__DOGroup_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DOGroup_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DOGroup_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DOGroup_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DOGroup_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DOGroup_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DOGroup_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DOGroup_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DOGroup_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOGroup_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOGroup_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOGroup_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DOGroup_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DOGroup_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DOGroup_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DOGroup_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__BrakeControl_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__BrakeControl_Request__init(msg: *mut BrakeControl_Request) -> bool;
    fn dobot_msgs_v4__srv__BrakeControl_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BrakeControl_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__BrakeControl_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BrakeControl_Request>);
    fn dobot_msgs_v4__srv__BrakeControl_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BrakeControl_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BrakeControl_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__BrakeControl_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__BrakeControl_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__BrakeControl_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BrakeControl_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__BrakeControl_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__BrakeControl_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__BrakeControl_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BrakeControl_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BrakeControl_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/BrakeControl_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__BrakeControl_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__BrakeControl_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__BrakeControl_Response__init(msg: *mut BrakeControl_Response) -> bool;
    fn dobot_msgs_v4__srv__BrakeControl_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BrakeControl_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__BrakeControl_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BrakeControl_Response>);
    fn dobot_msgs_v4__srv__BrakeControl_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BrakeControl_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BrakeControl_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__BrakeControl_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BrakeControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for BrakeControl_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__BrakeControl_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__BrakeControl_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BrakeControl_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__BrakeControl_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__BrakeControl_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__BrakeControl_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BrakeControl_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BrakeControl_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/BrakeControl_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__BrakeControl_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StartDrag_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__StartDrag_Request__init(msg: *mut StartDrag_Request) -> bool;
    fn dobot_msgs_v4__srv__StartDrag_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StartDrag_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__StartDrag_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StartDrag_Request>);
    fn dobot_msgs_v4__srv__StartDrag_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StartDrag_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<StartDrag_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__StartDrag_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartDrag_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for StartDrag_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__StartDrag_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__StartDrag_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StartDrag_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartDrag_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartDrag_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartDrag_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StartDrag_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StartDrag_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/StartDrag_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StartDrag_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StartDrag_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__StartDrag_Response__init(msg: *mut StartDrag_Response) -> bool;
    fn dobot_msgs_v4__srv__StartDrag_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StartDrag_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__StartDrag_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StartDrag_Response>);
    fn dobot_msgs_v4__srv__StartDrag_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StartDrag_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<StartDrag_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__StartDrag_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartDrag_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for StartDrag_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__StartDrag_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__StartDrag_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StartDrag_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartDrag_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartDrag_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartDrag_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StartDrag_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StartDrag_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/StartDrag_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StartDrag_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EnableSafeSkin_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__EnableSafeSkin_Request__init(msg: *mut EnableSafeSkin_Request) -> bool;
    fn dobot_msgs_v4__srv__EnableSafeSkin_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EnableSafeSkin_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__EnableSafeSkin_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EnableSafeSkin_Request>);
    fn dobot_msgs_v4__srv__EnableSafeSkin_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EnableSafeSkin_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<EnableSafeSkin_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__EnableSafeSkin_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnableSafeSkin_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i32,

}



impl Default for EnableSafeSkin_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__EnableSafeSkin_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__EnableSafeSkin_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EnableSafeSkin_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableSafeSkin_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableSafeSkin_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableSafeSkin_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EnableSafeSkin_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EnableSafeSkin_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/EnableSafeSkin_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EnableSafeSkin_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EnableSafeSkin_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__EnableSafeSkin_Response__init(msg: *mut EnableSafeSkin_Response) -> bool;
    fn dobot_msgs_v4__srv__EnableSafeSkin_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EnableSafeSkin_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__EnableSafeSkin_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EnableSafeSkin_Response>);
    fn dobot_msgs_v4__srv__EnableSafeSkin_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EnableSafeSkin_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<EnableSafeSkin_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__EnableSafeSkin_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnableSafeSkin_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for EnableSafeSkin_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__EnableSafeSkin_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__EnableSafeSkin_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EnableSafeSkin_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableSafeSkin_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableSafeSkin_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableSafeSkin_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EnableSafeSkin_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EnableSafeSkin_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/EnableSafeSkin_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EnableSafeSkin_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetStartPose_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetStartPose_Request__init(msg: *mut GetStartPose_Request) -> bool;
    fn dobot_msgs_v4__srv__GetStartPose_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetStartPose_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetStartPose_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetStartPose_Request>);
    fn dobot_msgs_v4__srv__GetStartPose_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetStartPose_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetStartPose_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetStartPose_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetStartPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trace_name: rosidl_runtime_rs::String,

}



impl Default for GetStartPose_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetStartPose_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetStartPose_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetStartPose_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetStartPose_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetStartPose_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetStartPose_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetStartPose_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetStartPose_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetStartPose_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetStartPose_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetStartPose_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetStartPose_Response__init(msg: *mut GetStartPose_Response) -> bool;
    fn dobot_msgs_v4__srv__GetStartPose_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetStartPose_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetStartPose_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetStartPose_Response>);
    fn dobot_msgs_v4__srv__GetStartPose_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetStartPose_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetStartPose_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetStartPose_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetStartPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetStartPose_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetStartPose_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetStartPose_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetStartPose_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetStartPose_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetStartPose_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetStartPose_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetStartPose_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetStartPose_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetStartPose_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetStartPose_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StartPath_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__StartPath_Request__init(msg: *mut StartPath_Request) -> bool;
    fn dobot_msgs_v4__srv__StartPath_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StartPath_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__StartPath_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StartPath_Request>);
    fn dobot_msgs_v4__srv__StartPath_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StartPath_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<StartPath_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__StartPath_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartPath_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trace_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for StartPath_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__StartPath_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__StartPath_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StartPath_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartPath_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartPath_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartPath_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StartPath_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StartPath_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/StartPath_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StartPath_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StartPath_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__StartPath_Response__init(msg: *mut StartPath_Response) -> bool;
    fn dobot_msgs_v4__srv__StartPath_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StartPath_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__StartPath_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StartPath_Response>);
    fn dobot_msgs_v4__srv__StartPath_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StartPath_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<StartPath_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__StartPath_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartPath_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for StartPath_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__StartPath_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__StartPath_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StartPath_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartPath_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartPath_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartPath_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StartPath_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StartPath_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/StartPath_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StartPath_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__InverseSolution_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__InverseSolution_Request__init(msg: *mut InverseSolution_Request) -> bool;
    fn dobot_msgs_v4__srv__InverseSolution_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InverseSolution_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__InverseSolution_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InverseSolution_Request>);
    fn dobot_msgs_v4__srv__InverseSolution_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InverseSolution_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<InverseSolution_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__InverseSolution_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InverseSolution_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub parameter: rosidl_runtime_rs::String,

}



impl Default for InverseSolution_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__InverseSolution_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__InverseSolution_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InverseSolution_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__InverseSolution_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__InverseSolution_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__InverseSolution_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InverseSolution_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InverseSolution_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/InverseSolution_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__InverseSolution_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__InverseSolution_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__InverseSolution_Response__init(msg: *mut InverseSolution_Response) -> bool;
    fn dobot_msgs_v4__srv__InverseSolution_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InverseSolution_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__InverseSolution_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InverseSolution_Response>);
    fn dobot_msgs_v4__srv__InverseSolution_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InverseSolution_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<InverseSolution_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__InverseSolution_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InverseSolution_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for InverseSolution_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__InverseSolution_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__InverseSolution_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InverseSolution_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__InverseSolution_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__InverseSolution_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__InverseSolution_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InverseSolution_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InverseSolution_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/InverseSolution_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__InverseSolution_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetErrorID_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetErrorID_Request__init(msg: *mut GetErrorID_Request) -> bool;
    fn dobot_msgs_v4__srv__GetErrorID_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetErrorID_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetErrorID_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetErrorID_Request>);
    fn dobot_msgs_v4__srv__GetErrorID_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetErrorID_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetErrorID_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetErrorID_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetErrorID_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetErrorID_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetErrorID_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetErrorID_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetErrorID_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetErrorID_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetErrorID_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetErrorID_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetErrorID_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetErrorID_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetErrorID_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetErrorID_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetErrorID_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetErrorID_Response__init(msg: *mut GetErrorID_Response) -> bool;
    fn dobot_msgs_v4__srv__GetErrorID_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetErrorID_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetErrorID_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetErrorID_Response>);
    fn dobot_msgs_v4__srv__GetErrorID_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetErrorID_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetErrorID_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetErrorID_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetErrorID_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetErrorID_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetErrorID_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetErrorID_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetErrorID_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetErrorID_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetErrorID_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetErrorID_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetErrorID_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetErrorID_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetErrorID_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetErrorID_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DI_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DI_Request__init(msg: *mut DI_Request) -> bool;
    fn dobot_msgs_v4__srv__DI_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DI_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DI_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DI_Request>);
    fn dobot_msgs_v4__srv__DI_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DI_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DI_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DI_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DI_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for DI_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DI_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DI_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DI_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DI_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DI_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DI_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DI_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DI_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DI_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DI_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DI_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DI_Response__init(msg: *mut DI_Response) -> bool;
    fn dobot_msgs_v4__srv__DI_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DI_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DI_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DI_Response>);
    fn dobot_msgs_v4__srv__DI_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DI_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DI_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DI_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DI_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DI_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DI_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DI_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DI_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DI_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DI_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DI_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DI_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DI_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DI_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DI_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolDI_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ToolDI_Request__init(msg: *mut ToolDI_Request) -> bool;
    fn dobot_msgs_v4__srv__ToolDI_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolDI_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ToolDI_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolDI_Request>);
    fn dobot_msgs_v4__srv__ToolDI_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolDI_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolDI_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ToolDI_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDI_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for ToolDI_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ToolDI_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ToolDI_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolDI_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDI_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDI_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDI_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolDI_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolDI_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ToolDI_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolDI_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolDI_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ToolDI_Response__init(msg: *mut ToolDI_Response) -> bool;
    fn dobot_msgs_v4__srv__ToolDI_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolDI_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ToolDI_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolDI_Response>);
    fn dobot_msgs_v4__srv__ToolDI_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolDI_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolDI_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ToolDI_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolDI_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ToolDI_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ToolDI_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ToolDI_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolDI_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDI_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDI_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolDI_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolDI_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolDI_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ToolDI_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolDI_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AI_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__AI_Request__init(msg: *mut AI_Request) -> bool;
    fn dobot_msgs_v4__srv__AI_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AI_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__AI_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AI_Request>);
    fn dobot_msgs_v4__srv__AI_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AI_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AI_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__AI_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AI_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for AI_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__AI_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__AI_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AI_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AI_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AI_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AI_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AI_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AI_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/AI_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AI_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AI_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__AI_Response__init(msg: *mut AI_Response) -> bool;
    fn dobot_msgs_v4__srv__AI_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AI_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__AI_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AI_Response>);
    fn dobot_msgs_v4__srv__AI_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AI_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AI_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__AI_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AI_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for AI_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__AI_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__AI_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AI_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AI_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AI_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__AI_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AI_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AI_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/AI_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__AI_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolAI_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ToolAI_Request__init(msg: *mut ToolAI_Request) -> bool;
    fn dobot_msgs_v4__srv__ToolAI_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolAI_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ToolAI_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolAI_Request>);
    fn dobot_msgs_v4__srv__ToolAI_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolAI_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolAI_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ToolAI_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolAI_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for ToolAI_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ToolAI_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ToolAI_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolAI_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolAI_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolAI_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolAI_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolAI_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolAI_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ToolAI_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolAI_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolAI_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ToolAI_Response__init(msg: *mut ToolAI_Response) -> bool;
    fn dobot_msgs_v4__srv__ToolAI_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToolAI_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ToolAI_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToolAI_Response>);
    fn dobot_msgs_v4__srv__ToolAI_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToolAI_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ToolAI_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ToolAI_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToolAI_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ToolAI_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ToolAI_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ToolAI_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToolAI_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolAI_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolAI_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ToolAI_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToolAI_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToolAI_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ToolAI_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ToolAI_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DIGroup_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DIGroup_Request__init(msg: *mut DIGroup_Request) -> bool;
    fn dobot_msgs_v4__srv__DIGroup_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DIGroup_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DIGroup_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DIGroup_Request>);
    fn dobot_msgs_v4__srv__DIGroup_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DIGroup_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DIGroup_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DIGroup_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DIGroup_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub args: rosidl_runtime_rs::Sequence<i32>,

}



impl Default for DIGroup_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DIGroup_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DIGroup_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DIGroup_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DIGroup_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DIGroup_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DIGroup_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DIGroup_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DIGroup_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DIGroup_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DIGroup_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DIGroup_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DIGroup_Response__init(msg: *mut DIGroup_Response) -> bool;
    fn dobot_msgs_v4__srv__DIGroup_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DIGroup_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DIGroup_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DIGroup_Response>);
    fn dobot_msgs_v4__srv__DIGroup_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DIGroup_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DIGroup_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DIGroup_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DIGroup_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DIGroup_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DIGroup_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DIGroup_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DIGroup_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DIGroup_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DIGroup_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DIGroup_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DIGroup_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DIGroup_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DIGroup_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DIGroup_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StopDrag_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__StopDrag_Request__init(msg: *mut StopDrag_Request) -> bool;
    fn dobot_msgs_v4__srv__StopDrag_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StopDrag_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__StopDrag_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StopDrag_Request>);
    fn dobot_msgs_v4__srv__StopDrag_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StopDrag_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<StopDrag_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__StopDrag_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StopDrag_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for StopDrag_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__StopDrag_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__StopDrag_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StopDrag_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StopDrag_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StopDrag_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StopDrag_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StopDrag_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StopDrag_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/StopDrag_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StopDrag_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StopDrag_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__StopDrag_Response__init(msg: *mut StopDrag_Response) -> bool;
    fn dobot_msgs_v4__srv__StopDrag_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StopDrag_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__StopDrag_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StopDrag_Response>);
    fn dobot_msgs_v4__srv__StopDrag_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StopDrag_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<StopDrag_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__StopDrag_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StopDrag_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for StopDrag_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__StopDrag_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__StopDrag_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StopDrag_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StopDrag_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StopDrag_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StopDrag_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StopDrag_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StopDrag_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/StopDrag_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StopDrag_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DragSensivity_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DragSensivity_Request__init(msg: *mut DragSensivity_Request) -> bool;
    fn dobot_msgs_v4__srv__DragSensivity_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DragSensivity_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DragSensivity_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DragSensivity_Request>);
    fn dobot_msgs_v4__srv__DragSensivity_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DragSensivity_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DragSensivity_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DragSensivity_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DragSensivity_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DragSensivity_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DragSensivity_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DragSensivity_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DragSensivity_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DragSensivity_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DragSensivity_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DragSensivity_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DragSensivity_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DragSensivity_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DragSensivity_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DragSensivity_Response__init(msg: *mut DragSensivity_Response) -> bool;
    fn dobot_msgs_v4__srv__DragSensivity_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DragSensivity_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DragSensivity_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DragSensivity_Response>);
    fn dobot_msgs_v4__srv__DragSensivity_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DragSensivity_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DragSensivity_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DragSensivity_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DragSensivity_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DragSensivity_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DragSensivity_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DragSensivity_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DragSensivity_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DragSensivity_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DragSensivity_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DragSensivity_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DragSensivity_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DragSensivity_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DragSensivity_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DragSensivity_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetDO_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetDO_Request__init(msg: *mut GetDO_Request) -> bool;
    fn dobot_msgs_v4__srv__GetDO_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetDO_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetDO_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetDO_Request>);
    fn dobot_msgs_v4__srv__GetDO_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetDO_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetDO_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetDO_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetDO_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for GetDO_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetDO_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetDO_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetDO_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDO_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDO_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDO_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetDO_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetDO_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetDO_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetDO_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetDO_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetDO_Response__init(msg: *mut GetDO_Response) -> bool;
    fn dobot_msgs_v4__srv__GetDO_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetDO_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetDO_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetDO_Response>);
    fn dobot_msgs_v4__srv__GetDO_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetDO_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetDO_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetDO_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetDO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetDO_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetDO_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetDO_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetDO_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDO_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDO_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDO_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetDO_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetDO_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetDO_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetDO_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetAO_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetAO_Request__init(msg: *mut GetAO_Request) -> bool;
    fn dobot_msgs_v4__srv__GetAO_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetAO_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetAO_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetAO_Request>);
    fn dobot_msgs_v4__srv__GetAO_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetAO_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetAO_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetAO_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetAO_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for GetAO_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetAO_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetAO_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetAO_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetAO_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetAO_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetAO_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetAO_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetAO_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetAO_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetAO_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetAO_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetAO_Response__init(msg: *mut GetAO_Response) -> bool;
    fn dobot_msgs_v4__srv__GetAO_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetAO_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetAO_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetAO_Response>);
    fn dobot_msgs_v4__srv__GetAO_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetAO_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetAO_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetAO_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetAO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetAO_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetAO_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetAO_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetAO_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetAO_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetAO_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetAO_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetAO_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetAO_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetAO_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetAO_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetDOGroup_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetDOGroup_Request__init(msg: *mut GetDOGroup_Request) -> bool;
    fn dobot_msgs_v4__srv__GetDOGroup_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetDOGroup_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetDOGroup_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetDOGroup_Request>);
    fn dobot_msgs_v4__srv__GetDOGroup_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetDOGroup_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetDOGroup_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetDOGroup_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetDOGroup_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index_group: rosidl_runtime_rs::Sequence<i32>,

}



impl Default for GetDOGroup_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetDOGroup_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetDOGroup_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetDOGroup_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDOGroup_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDOGroup_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDOGroup_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetDOGroup_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetDOGroup_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetDOGroup_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetDOGroup_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetDOGroup_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetDOGroup_Response__init(msg: *mut GetDOGroup_Response) -> bool;
    fn dobot_msgs_v4__srv__GetDOGroup_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetDOGroup_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetDOGroup_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetDOGroup_Response>);
    fn dobot_msgs_v4__srv__GetDOGroup_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetDOGroup_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetDOGroup_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetDOGroup_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetDOGroup_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetDOGroup_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetDOGroup_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetDOGroup_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetDOGroup_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDOGroup_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDOGroup_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDOGroup_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetDOGroup_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetDOGroup_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetDOGroup_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetDOGroup_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetTool485_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetTool485_Request__init(msg: *mut SetTool485_Request) -> bool;
    fn dobot_msgs_v4__srv__SetTool485_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetTool485_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetTool485_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetTool485_Request>);
    fn dobot_msgs_v4__srv__SetTool485_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetTool485_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetTool485_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetTool485_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetTool485_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub baudrate: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub parity: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stop: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub identify: i32,

}



impl Default for SetTool485_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetTool485_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetTool485_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetTool485_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetTool485_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetTool485_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetTool485_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetTool485_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetTool485_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetTool485_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetTool485_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetTool485_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetTool485_Response__init(msg: *mut SetTool485_Response) -> bool;
    fn dobot_msgs_v4__srv__SetTool485_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetTool485_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetTool485_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetTool485_Response>);
    fn dobot_msgs_v4__srv__SetTool485_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetTool485_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetTool485_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetTool485_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetTool485_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetTool485_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetTool485_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetTool485_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetTool485_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetTool485_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetTool485_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetTool485_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetTool485_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetTool485_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetTool485_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetTool485_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetSafeWallEnable_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetSafeWallEnable_Request__init(msg: *mut SetSafeWallEnable_Request) -> bool;
    fn dobot_msgs_v4__srv__SetSafeWallEnable_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetSafeWallEnable_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetSafeWallEnable_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetSafeWallEnable_Request>);
    fn dobot_msgs_v4__srv__SetSafeWallEnable_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetSafeWallEnable_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetSafeWallEnable_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetSafeWallEnable_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetSafeWallEnable_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetSafeWallEnable_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetSafeWallEnable_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetSafeWallEnable_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetSafeWallEnable_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetSafeWallEnable_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetSafeWallEnable_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetSafeWallEnable_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetSafeWallEnable_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetSafeWallEnable_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetSafeWallEnable_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetSafeWallEnable_Response__init(msg: *mut SetSafeWallEnable_Response) -> bool;
    fn dobot_msgs_v4__srv__SetSafeWallEnable_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetSafeWallEnable_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetSafeWallEnable_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetSafeWallEnable_Response>);
    fn dobot_msgs_v4__srv__SetSafeWallEnable_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetSafeWallEnable_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetSafeWallEnable_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetSafeWallEnable_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetSafeWallEnable_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetSafeWallEnable_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetSafeWallEnable_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetSafeWallEnable_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetSafeWallEnable_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetSafeWallEnable_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetSafeWallEnable_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetSafeWallEnable_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetSafeWallEnable_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetSafeWallEnable_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetSafeWallEnable_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetSafeWallEnable_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetToolPower_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetToolPower_Request__init(msg: *mut SetToolPower_Request) -> bool;
    fn dobot_msgs_v4__srv__SetToolPower_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetToolPower_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetToolPower_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetToolPower_Request>);
    fn dobot_msgs_v4__srv__SetToolPower_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetToolPower_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetToolPower_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetToolPower_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetToolPower_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i32,

}



impl Default for SetToolPower_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetToolPower_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetToolPower_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetToolPower_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetToolPower_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetToolPower_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetToolPower_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetToolPower_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetToolPower_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetToolPower_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetToolPower_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetToolPower_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetToolPower_Response__init(msg: *mut SetToolPower_Response) -> bool;
    fn dobot_msgs_v4__srv__SetToolPower_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetToolPower_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetToolPower_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetToolPower_Response>);
    fn dobot_msgs_v4__srv__SetToolPower_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetToolPower_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetToolPower_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetToolPower_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetToolPower_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetToolPower_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetToolPower_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetToolPower_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetToolPower_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetToolPower_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetToolPower_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetToolPower_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetToolPower_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetToolPower_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetToolPower_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetToolPower_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetToolMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetToolMode_Request__init(msg: *mut SetToolMode_Request) -> bool;
    fn dobot_msgs_v4__srv__SetToolMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetToolMode_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetToolMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetToolMode_Request>);
    fn dobot_msgs_v4__srv__SetToolMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetToolMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetToolMode_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetToolMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetToolMode_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetToolMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetToolMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetToolMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetToolMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetToolMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetToolMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetToolMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetToolMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetToolMode_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetToolMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetToolMode_Response__init(msg: *mut SetToolMode_Response) -> bool;
    fn dobot_msgs_v4__srv__SetToolMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetToolMode_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetToolMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetToolMode_Response>);
    fn dobot_msgs_v4__srv__SetToolMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetToolMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetToolMode_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetToolMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetToolMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetToolMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetToolMode_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetToolMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetToolMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetToolMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetToolMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetToolMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetToolMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetToolMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetToolMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetToolMode_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetBackDistance_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetBackDistance_Request__init(msg: *mut SetBackDistance_Request) -> bool;
    fn dobot_msgs_v4__srv__SetBackDistance_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetBackDistance_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetBackDistance_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetBackDistance_Request>);
    fn dobot_msgs_v4__srv__SetBackDistance_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetBackDistance_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetBackDistance_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetBackDistance_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetBackDistance_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub distance: f64,

}



impl Default for SetBackDistance_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetBackDistance_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetBackDistance_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetBackDistance_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetBackDistance_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetBackDistance_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetBackDistance_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetBackDistance_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetBackDistance_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetBackDistance_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetBackDistance_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetBackDistance_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetBackDistance_Response__init(msg: *mut SetBackDistance_Response) -> bool;
    fn dobot_msgs_v4__srv__SetBackDistance_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetBackDistance_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetBackDistance_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetBackDistance_Response>);
    fn dobot_msgs_v4__srv__SetBackDistance_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetBackDistance_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetBackDistance_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetBackDistance_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetBackDistance_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetBackDistance_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetBackDistance_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetBackDistance_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetBackDistance_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetBackDistance_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetBackDistance_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetBackDistance_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetBackDistance_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetBackDistance_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetBackDistance_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetBackDistance_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetPostCollisionMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetPostCollisionMode_Request__init(msg: *mut SetPostCollisionMode_Request) -> bool;
    fn dobot_msgs_v4__srv__SetPostCollisionMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetPostCollisionMode_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetPostCollisionMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetPostCollisionMode_Request>);
    fn dobot_msgs_v4__srv__SetPostCollisionMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetPostCollisionMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetPostCollisionMode_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetPostCollisionMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPostCollisionMode_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mode: i32,

}



impl Default for SetPostCollisionMode_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetPostCollisionMode_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetPostCollisionMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetPostCollisionMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetPostCollisionMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetPostCollisionMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetPostCollisionMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetPostCollisionMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetPostCollisionMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetPostCollisionMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetPostCollisionMode_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetPostCollisionMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetPostCollisionMode_Response__init(msg: *mut SetPostCollisionMode_Response) -> bool;
    fn dobot_msgs_v4__srv__SetPostCollisionMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetPostCollisionMode_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetPostCollisionMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetPostCollisionMode_Response>);
    fn dobot_msgs_v4__srv__SetPostCollisionMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetPostCollisionMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetPostCollisionMode_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetPostCollisionMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetPostCollisionMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetPostCollisionMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetPostCollisionMode_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetPostCollisionMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetPostCollisionMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetPostCollisionMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetPostCollisionMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetPostCollisionMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetPostCollisionMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetPostCollisionMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetPostCollisionMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetPostCollisionMode_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetUser_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetUser_Request__init(msg: *mut SetUser_Request) -> bool;
    fn dobot_msgs_v4__srv__SetUser_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetUser_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetUser_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetUser_Request>);
    fn dobot_msgs_v4__srv__SetUser_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetUser_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetUser_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetUser_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetUser_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: rosidl_runtime_rs::String,

}



impl Default for SetUser_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetUser_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetUser_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetUser_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetUser_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetUser_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetUser_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetUser_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetUser_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetUser_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetUser_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetUser_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetUser_Response__init(msg: *mut SetUser_Response) -> bool;
    fn dobot_msgs_v4__srv__SetUser_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetUser_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetUser_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetUser_Response>);
    fn dobot_msgs_v4__srv__SetUser_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetUser_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetUser_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetUser_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetUser_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetUser_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetUser_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetUser_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetUser_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetUser_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetUser_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetUser_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetUser_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetUser_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetUser_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetUser_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetTool_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetTool_Request__init(msg: *mut SetTool_Request) -> bool;
    fn dobot_msgs_v4__srv__SetTool_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetTool_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetTool_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetTool_Request>);
    fn dobot_msgs_v4__srv__SetTool_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetTool_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetTool_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetTool_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetTool_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: rosidl_runtime_rs::String,

}



impl Default for SetTool_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetTool_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetTool_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetTool_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetTool_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetTool_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetTool_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetTool_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetTool_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetTool_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetTool_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetTool_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetTool_Response__init(msg: *mut SetTool_Response) -> bool;
    fn dobot_msgs_v4__srv__SetTool_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetTool_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetTool_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetTool_Response>);
    fn dobot_msgs_v4__srv__SetTool_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetTool_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetTool_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetTool_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetTool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetTool_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetTool_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetTool_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetTool_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetTool_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetTool_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetTool_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetTool_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetTool_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetTool_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetTool_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CalcUser_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__CalcUser_Request__init(msg: *mut CalcUser_Request) -> bool;
    fn dobot_msgs_v4__srv__CalcUser_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CalcUser_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__CalcUser_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CalcUser_Request>);
    fn dobot_msgs_v4__srv__CalcUser_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CalcUser_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CalcUser_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__CalcUser_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub offset: rosidl_runtime_rs::String,

}



impl Default for CalcUser_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__CalcUser_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__CalcUser_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CalcUser_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CalcUser_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CalcUser_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CalcUser_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CalcUser_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CalcUser_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/CalcUser_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CalcUser_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CalcUser_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__CalcUser_Response__init(msg: *mut CalcUser_Response) -> bool;
    fn dobot_msgs_v4__srv__CalcUser_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CalcUser_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__CalcUser_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CalcUser_Response>);
    fn dobot_msgs_v4__srv__CalcUser_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CalcUser_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CalcUser_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__CalcUser_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalcUser_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for CalcUser_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__CalcUser_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__CalcUser_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CalcUser_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CalcUser_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CalcUser_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CalcUser_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CalcUser_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CalcUser_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/CalcUser_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CalcUser_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CalcTool_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__CalcTool_Request__init(msg: *mut CalcTool_Request) -> bool;
    fn dobot_msgs_v4__srv__CalcTool_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CalcTool_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__CalcTool_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CalcTool_Request>);
    fn dobot_msgs_v4__srv__CalcTool_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CalcTool_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CalcTool_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__CalcTool_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub offset: rosidl_runtime_rs::String,

}



impl Default for CalcTool_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__CalcTool_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__CalcTool_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CalcTool_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CalcTool_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CalcTool_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CalcTool_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CalcTool_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CalcTool_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/CalcTool_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CalcTool_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CalcTool_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__CalcTool_Response__init(msg: *mut CalcTool_Response) -> bool;
    fn dobot_msgs_v4__srv__CalcTool_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CalcTool_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__CalcTool_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CalcTool_Response>);
    fn dobot_msgs_v4__srv__CalcTool_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CalcTool_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CalcTool_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__CalcTool_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CalcTool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for CalcTool_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__CalcTool_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__CalcTool_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CalcTool_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CalcTool_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CalcTool_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CalcTool_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CalcTool_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CalcTool_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/CalcTool_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CalcTool_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInputBool_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetInputBool_Request__init(msg: *mut GetInputBool_Request) -> bool;
    fn dobot_msgs_v4__srv__GetInputBool_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetInputBool_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetInputBool_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetInputBool_Request>);
    fn dobot_msgs_v4__srv__GetInputBool_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetInputBool_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetInputBool_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetInputBool_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInputBool_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,

}



impl Default for GetInputBool_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetInputBool_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetInputBool_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetInputBool_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputBool_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputBool_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputBool_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetInputBool_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetInputBool_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetInputBool_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInputBool_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInputBool_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetInputBool_Response__init(msg: *mut GetInputBool_Response) -> bool;
    fn dobot_msgs_v4__srv__GetInputBool_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetInputBool_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetInputBool_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetInputBool_Response>);
    fn dobot_msgs_v4__srv__GetInputBool_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetInputBool_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetInputBool_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetInputBool_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInputBool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetInputBool_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetInputBool_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetInputBool_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetInputBool_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputBool_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputBool_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputBool_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetInputBool_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetInputBool_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetInputBool_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInputBool_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInputInt_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetInputInt_Request__init(msg: *mut GetInputInt_Request) -> bool;
    fn dobot_msgs_v4__srv__GetInputInt_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetInputInt_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetInputInt_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetInputInt_Request>);
    fn dobot_msgs_v4__srv__GetInputInt_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetInputInt_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetInputInt_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetInputInt_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInputInt_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,

}



impl Default for GetInputInt_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetInputInt_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetInputInt_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetInputInt_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputInt_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputInt_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputInt_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetInputInt_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetInputInt_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetInputInt_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInputInt_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInputInt_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetInputInt_Response__init(msg: *mut GetInputInt_Response) -> bool;
    fn dobot_msgs_v4__srv__GetInputInt_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetInputInt_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetInputInt_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetInputInt_Response>);
    fn dobot_msgs_v4__srv__GetInputInt_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetInputInt_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetInputInt_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetInputInt_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInputInt_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetInputInt_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetInputInt_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetInputInt_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetInputInt_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputInt_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputInt_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputInt_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetInputInt_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetInputInt_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetInputInt_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInputInt_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInputFloat_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetInputFloat_Request__init(msg: *mut GetInputFloat_Request) -> bool;
    fn dobot_msgs_v4__srv__GetInputFloat_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetInputFloat_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetInputFloat_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetInputFloat_Request>);
    fn dobot_msgs_v4__srv__GetInputFloat_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetInputFloat_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetInputFloat_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetInputFloat_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInputFloat_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,

}



impl Default for GetInputFloat_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetInputFloat_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetInputFloat_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetInputFloat_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputFloat_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputFloat_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputFloat_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetInputFloat_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetInputFloat_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetInputFloat_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInputFloat_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInputFloat_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetInputFloat_Response__init(msg: *mut GetInputFloat_Response) -> bool;
    fn dobot_msgs_v4__srv__GetInputFloat_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetInputFloat_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetInputFloat_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetInputFloat_Response>);
    fn dobot_msgs_v4__srv__GetInputFloat_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetInputFloat_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetInputFloat_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetInputFloat_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetInputFloat_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetInputFloat_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetInputFloat_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetInputFloat_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetInputFloat_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputFloat_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputFloat_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetInputFloat_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetInputFloat_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetInputFloat_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetInputFloat_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetInputFloat_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetOutputBool_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetOutputBool_Request__init(msg: *mut GetOutputBool_Request) -> bool;
    fn dobot_msgs_v4__srv__GetOutputBool_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetOutputBool_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetOutputBool_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetOutputBool_Request>);
    fn dobot_msgs_v4__srv__GetOutputBool_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetOutputBool_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetOutputBool_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetOutputBool_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOutputBool_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,

}



impl Default for GetOutputBool_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetOutputBool_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetOutputBool_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetOutputBool_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputBool_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputBool_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputBool_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetOutputBool_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetOutputBool_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetOutputBool_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetOutputBool_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetOutputBool_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetOutputBool_Response__init(msg: *mut GetOutputBool_Response) -> bool;
    fn dobot_msgs_v4__srv__GetOutputBool_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetOutputBool_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetOutputBool_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetOutputBool_Response>);
    fn dobot_msgs_v4__srv__GetOutputBool_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetOutputBool_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetOutputBool_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetOutputBool_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOutputBool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetOutputBool_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetOutputBool_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetOutputBool_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetOutputBool_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputBool_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputBool_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputBool_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetOutputBool_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetOutputBool_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetOutputBool_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetOutputBool_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetOutputInt_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetOutputInt_Request__init(msg: *mut GetOutputInt_Request) -> bool;
    fn dobot_msgs_v4__srv__GetOutputInt_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetOutputInt_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetOutputInt_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetOutputInt_Request>);
    fn dobot_msgs_v4__srv__GetOutputInt_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetOutputInt_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetOutputInt_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetOutputInt_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOutputInt_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,

}



impl Default for GetOutputInt_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetOutputInt_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetOutputInt_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetOutputInt_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputInt_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputInt_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputInt_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetOutputInt_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetOutputInt_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetOutputInt_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetOutputInt_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetOutputInt_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetOutputInt_Response__init(msg: *mut GetOutputInt_Response) -> bool;
    fn dobot_msgs_v4__srv__GetOutputInt_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetOutputInt_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetOutputInt_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetOutputInt_Response>);
    fn dobot_msgs_v4__srv__GetOutputInt_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetOutputInt_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetOutputInt_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetOutputInt_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOutputInt_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetOutputInt_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetOutputInt_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetOutputInt_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetOutputInt_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputInt_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputInt_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputInt_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetOutputInt_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetOutputInt_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetOutputInt_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetOutputInt_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetOutputFloat_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetOutputFloat_Request__init(msg: *mut GetOutputFloat_Request) -> bool;
    fn dobot_msgs_v4__srv__GetOutputFloat_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetOutputFloat_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetOutputFloat_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetOutputFloat_Request>);
    fn dobot_msgs_v4__srv__GetOutputFloat_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetOutputFloat_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetOutputFloat_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetOutputFloat_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOutputFloat_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub address: i32,

}



impl Default for GetOutputFloat_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetOutputFloat_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetOutputFloat_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetOutputFloat_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputFloat_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputFloat_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputFloat_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetOutputFloat_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetOutputFloat_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetOutputFloat_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetOutputFloat_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetOutputFloat_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetOutputFloat_Response__init(msg: *mut GetOutputFloat_Response) -> bool;
    fn dobot_msgs_v4__srv__GetOutputFloat_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetOutputFloat_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetOutputFloat_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetOutputFloat_Response>);
    fn dobot_msgs_v4__srv__GetOutputFloat_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetOutputFloat_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetOutputFloat_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetOutputFloat_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetOutputFloat_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetOutputFloat_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetOutputFloat_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetOutputFloat_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetOutputFloat_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputFloat_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputFloat_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetOutputFloat_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetOutputFloat_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetOutputFloat_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetOutputFloat_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetOutputFloat_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetOutputBool_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetOutputBool_Request__init(msg: *mut SetOutputBool_Request) -> bool;
    fn dobot_msgs_v4__srv__SetOutputBool_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOutputBool_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetOutputBool_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOutputBool_Request>);
    fn dobot_msgs_v4__srv__SetOutputBool_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOutputBool_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOutputBool_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetOutputBool_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetOutputBool_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetOutputBool_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOutputBool_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputBool_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputBool_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputBool_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOutputBool_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOutputBool_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetOutputBool_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetOutputBool_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetOutputBool_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetOutputBool_Response__init(msg: *mut SetOutputBool_Response) -> bool;
    fn dobot_msgs_v4__srv__SetOutputBool_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOutputBool_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetOutputBool_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOutputBool_Response>);
    fn dobot_msgs_v4__srv__SetOutputBool_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOutputBool_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOutputBool_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetOutputBool_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOutputBool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetOutputBool_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetOutputBool_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetOutputBool_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOutputBool_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputBool_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputBool_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputBool_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOutputBool_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOutputBool_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetOutputBool_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetOutputBool_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetOutputInt_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetOutputInt_Request__init(msg: *mut SetOutputInt_Request) -> bool;
    fn dobot_msgs_v4__srv__SetOutputInt_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOutputInt_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetOutputInt_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOutputInt_Request>);
    fn dobot_msgs_v4__srv__SetOutputInt_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOutputInt_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOutputInt_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetOutputInt_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetOutputInt_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetOutputInt_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOutputInt_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputInt_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputInt_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputInt_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOutputInt_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOutputInt_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetOutputInt_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetOutputInt_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetOutputInt_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetOutputInt_Response__init(msg: *mut SetOutputInt_Response) -> bool;
    fn dobot_msgs_v4__srv__SetOutputInt_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOutputInt_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetOutputInt_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOutputInt_Response>);
    fn dobot_msgs_v4__srv__SetOutputInt_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOutputInt_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOutputInt_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetOutputInt_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOutputInt_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetOutputInt_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetOutputInt_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetOutputInt_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOutputInt_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputInt_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputInt_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputInt_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOutputInt_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOutputInt_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetOutputInt_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetOutputInt_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetOutputFloat_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetOutputFloat_Request__init(msg: *mut SetOutputFloat_Request) -> bool;
    fn dobot_msgs_v4__srv__SetOutputFloat_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOutputFloat_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetOutputFloat_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOutputFloat_Request>);
    fn dobot_msgs_v4__srv__SetOutputFloat_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOutputFloat_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOutputFloat_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetOutputFloat_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetOutputFloat_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetOutputFloat_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOutputFloat_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputFloat_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputFloat_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputFloat_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOutputFloat_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOutputFloat_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetOutputFloat_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetOutputFloat_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetOutputFloat_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetOutputFloat_Response__init(msg: *mut SetOutputFloat_Response) -> bool;
    fn dobot_msgs_v4__srv__SetOutputFloat_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetOutputFloat_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetOutputFloat_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetOutputFloat_Response>);
    fn dobot_msgs_v4__srv__SetOutputFloat_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetOutputFloat_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetOutputFloat_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetOutputFloat_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetOutputFloat_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetOutputFloat_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetOutputFloat_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetOutputFloat_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetOutputFloat_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputFloat_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputFloat_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetOutputFloat_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetOutputFloat_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetOutputFloat_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetOutputFloat_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetOutputFloat_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovLIO_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__MovLIO_Request__init(msg: *mut MovLIO_Request) -> bool;
    fn dobot_msgs_v4__srv__MovLIO_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__MovLIO_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Request>);
    fn dobot_msgs_v4__srv__MovLIO_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovLIO_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__MovLIO_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub mdis: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for MovLIO_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__MovLIO_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__MovLIO_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovLIO_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovLIO_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovLIO_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovLIO_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovLIO_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovLIO_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/MovLIO_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovLIO_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovLIO_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__MovLIO_Response__init(msg: *mut MovLIO_Response) -> bool;
    fn dobot_msgs_v4__srv__MovLIO_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__MovLIO_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Response>);
    fn dobot_msgs_v4__srv__MovLIO_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovLIO_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovLIO_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__MovLIO_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovLIO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for MovLIO_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__MovLIO_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__MovLIO_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovLIO_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovLIO_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovLIO_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovLIO_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovLIO_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovLIO_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/MovLIO_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovLIO_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovJIO_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__MovJIO_Request__init(msg: *mut MovJIO_Request) -> bool;
    fn dobot_msgs_v4__srv__MovJIO_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__MovJIO_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Request>);
    fn dobot_msgs_v4__srv__MovJIO_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJIO_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__MovJIO_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub mdis: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for MovJIO_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__MovJIO_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__MovJIO_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJIO_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovJIO_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovJIO_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovJIO_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJIO_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJIO_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/MovJIO_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovJIO_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovJIO_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__MovJIO_Response__init(msg: *mut MovJIO_Response) -> bool;
    fn dobot_msgs_v4__srv__MovJIO_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__MovJIO_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Response>);
    fn dobot_msgs_v4__srv__MovJIO_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovJIO_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovJIO_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__MovJIO_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovJIO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for MovJIO_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__MovJIO_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__MovJIO_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovJIO_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovJIO_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovJIO_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__MovJIO_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovJIO_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovJIO_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/MovJIO_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__MovJIO_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Arc_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__Arc_Request__init(msg: *mut Arc_Request) -> bool;
    fn dobot_msgs_v4__srv__Arc_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Arc_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__Arc_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Arc_Request>);
    fn dobot_msgs_v4__srv__Arc_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Arc_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Arc_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__Arc_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for Arc_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__Arc_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__Arc_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Arc_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Arc_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Arc_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Arc_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Arc_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Arc_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/Arc_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Arc_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Arc_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__Arc_Response__init(msg: *mut Arc_Response) -> bool;
    fn dobot_msgs_v4__srv__Arc_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Arc_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__Arc_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Arc_Response>);
    fn dobot_msgs_v4__srv__Arc_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Arc_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Arc_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__Arc_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Arc_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for Arc_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__Arc_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__Arc_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Arc_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Arc_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Arc_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Arc_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Arc_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Arc_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/Arc_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Arc_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Circle_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__Circle_Request__init(msg: *mut Circle_Request) -> bool;
    fn dobot_msgs_v4__srv__Circle_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Circle_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__Circle_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Circle_Request>);
    fn dobot_msgs_v4__srv__Circle_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Circle_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Circle_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__Circle_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for Circle_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__Circle_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__Circle_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Circle_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Circle_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Circle_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Circle_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Circle_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Circle_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/Circle_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Circle_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Circle_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__Circle_Response__init(msg: *mut Circle_Response) -> bool;
    fn dobot_msgs_v4__srv__Circle_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Circle_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__Circle_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Circle_Response>);
    fn dobot_msgs_v4__srv__Circle_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Circle_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Circle_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__Circle_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Circle_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for Circle_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__Circle_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__Circle_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Circle_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Circle_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Circle_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__Circle_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Circle_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Circle_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/Circle_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__Circle_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovJTool_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RelMovJTool_Request__init(msg: *mut RelMovJTool_Request) -> bool;
    fn dobot_msgs_v4__srv__RelMovJTool_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RelMovJTool_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RelMovJTool_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RelMovJTool_Request>);
    fn dobot_msgs_v4__srv__RelMovJTool_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RelMovJTool_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RelMovJTool_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RelMovJTool_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for RelMovJTool_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RelMovJTool_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RelMovJTool_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RelMovJTool_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovJTool_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovJTool_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovJTool_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RelMovJTool_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RelMovJTool_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RelMovJTool_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovJTool_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovJTool_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RelMovJTool_Response__init(msg: *mut RelMovJTool_Response) -> bool;
    fn dobot_msgs_v4__srv__RelMovJTool_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RelMovJTool_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RelMovJTool_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RelMovJTool_Response>);
    fn dobot_msgs_v4__srv__RelMovJTool_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RelMovJTool_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RelMovJTool_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RelMovJTool_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelMovJTool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RelMovJTool_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RelMovJTool_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RelMovJTool_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RelMovJTool_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovJTool_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovJTool_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovJTool_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RelMovJTool_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RelMovJTool_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RelMovJTool_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovJTool_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovLTool_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RelMovLTool_Request__init(msg: *mut RelMovLTool_Request) -> bool;
    fn dobot_msgs_v4__srv__RelMovLTool_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RelMovLTool_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RelMovLTool_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RelMovLTool_Request>);
    fn dobot_msgs_v4__srv__RelMovLTool_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RelMovLTool_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RelMovLTool_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RelMovLTool_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for RelMovLTool_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RelMovLTool_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RelMovLTool_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RelMovLTool_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovLTool_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovLTool_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovLTool_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RelMovLTool_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RelMovLTool_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RelMovLTool_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovLTool_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovLTool_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RelMovLTool_Response__init(msg: *mut RelMovLTool_Response) -> bool;
    fn dobot_msgs_v4__srv__RelMovLTool_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RelMovLTool_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RelMovLTool_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RelMovLTool_Response>);
    fn dobot_msgs_v4__srv__RelMovLTool_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RelMovLTool_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RelMovLTool_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RelMovLTool_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelMovLTool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RelMovLTool_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RelMovLTool_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RelMovLTool_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RelMovLTool_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovLTool_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovLTool_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovLTool_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RelMovLTool_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RelMovLTool_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RelMovLTool_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovLTool_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovJUser_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RelMovJUser_Request__init(msg: *mut RelMovJUser_Request) -> bool;
    fn dobot_msgs_v4__srv__RelMovJUser_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RelMovJUser_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RelMovJUser_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RelMovJUser_Request>);
    fn dobot_msgs_v4__srv__RelMovJUser_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RelMovJUser_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RelMovJUser_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RelMovJUser_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for RelMovJUser_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RelMovJUser_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RelMovJUser_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RelMovJUser_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovJUser_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovJUser_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovJUser_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RelMovJUser_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RelMovJUser_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RelMovJUser_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovJUser_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovJUser_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RelMovJUser_Response__init(msg: *mut RelMovJUser_Response) -> bool;
    fn dobot_msgs_v4__srv__RelMovJUser_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RelMovJUser_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RelMovJUser_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RelMovJUser_Response>);
    fn dobot_msgs_v4__srv__RelMovJUser_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RelMovJUser_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RelMovJUser_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RelMovJUser_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelMovJUser_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RelMovJUser_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RelMovJUser_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RelMovJUser_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RelMovJUser_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovJUser_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovJUser_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovJUser_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RelMovJUser_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RelMovJUser_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RelMovJUser_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovJUser_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovLUser_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RelMovLUser_Request__init(msg: *mut RelMovLUser_Request) -> bool;
    fn dobot_msgs_v4__srv__RelMovLUser_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RelMovLUser_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RelMovLUser_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RelMovLUser_Request>);
    fn dobot_msgs_v4__srv__RelMovLUser_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RelMovLUser_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RelMovLUser_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RelMovLUser_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for RelMovLUser_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RelMovLUser_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RelMovLUser_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RelMovLUser_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovLUser_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovLUser_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovLUser_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RelMovLUser_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RelMovLUser_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RelMovLUser_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovLUser_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovLUser_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RelMovLUser_Response__init(msg: *mut RelMovLUser_Response) -> bool;
    fn dobot_msgs_v4__srv__RelMovLUser_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RelMovLUser_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RelMovLUser_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RelMovLUser_Response>);
    fn dobot_msgs_v4__srv__RelMovLUser_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RelMovLUser_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RelMovLUser_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RelMovLUser_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RelMovLUser_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RelMovLUser_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RelMovLUser_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RelMovLUser_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RelMovLUser_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovLUser_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovLUser_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RelMovLUser_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RelMovLUser_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RelMovLUser_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RelMovLUser_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RelMovLUser_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetCurrentCommandId_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetCurrentCommandId_Request__init(msg: *mut GetCurrentCommandId_Request) -> bool;
    fn dobot_msgs_v4__srv__GetCurrentCommandId_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetCurrentCommandId_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetCurrentCommandId_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetCurrentCommandId_Request>);
    fn dobot_msgs_v4__srv__GetCurrentCommandId_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetCurrentCommandId_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetCurrentCommandId_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetCurrentCommandId_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetCurrentCommandId_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetCurrentCommandId_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetCurrentCommandId_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetCurrentCommandId_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetCurrentCommandId_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetCurrentCommandId_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetCurrentCommandId_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetCurrentCommandId_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetCurrentCommandId_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetCurrentCommandId_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetCurrentCommandId_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetCurrentCommandId_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetCurrentCommandId_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetCurrentCommandId_Response__init(msg: *mut GetCurrentCommandId_Response) -> bool;
    fn dobot_msgs_v4__srv__GetCurrentCommandId_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetCurrentCommandId_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetCurrentCommandId_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetCurrentCommandId_Response>);
    fn dobot_msgs_v4__srv__GetCurrentCommandId_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetCurrentCommandId_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetCurrentCommandId_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetCurrentCommandId_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetCurrentCommandId_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetCurrentCommandId_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetCurrentCommandId_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetCurrentCommandId_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetCurrentCommandId_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetCurrentCommandId_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetCurrentCommandId_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetCurrentCommandId_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetCurrentCommandId_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetCurrentCommandId_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetCurrentCommandId_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetCurrentCommandId_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ServoJ_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ServoJ_Request__init(msg: *mut ServoJ_Request) -> bool;
    fn dobot_msgs_v4__srv__ServoJ_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ServoJ_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ServoJ_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ServoJ_Request>);
    fn dobot_msgs_v4__srv__ServoJ_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ServoJ_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ServoJ_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ServoJ_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for ServoJ_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ServoJ_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ServoJ_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ServoJ_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ServoJ_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ServoJ_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ServoJ_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ServoJ_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ServoJ_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ServoJ_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ServoJ_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ServoJ_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ServoJ_Response__init(msg: *mut ServoJ_Response) -> bool;
    fn dobot_msgs_v4__srv__ServoJ_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ServoJ_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ServoJ_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ServoJ_Response>);
    fn dobot_msgs_v4__srv__ServoJ_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ServoJ_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ServoJ_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ServoJ_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ServoJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ServoJ_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ServoJ_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ServoJ_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ServoJ_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ServoJ_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ServoJ_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ServoJ_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ServoJ_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ServoJ_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ServoJ_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ServoJ_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ServoP_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ServoP_Request__init(msg: *mut ServoP_Request) -> bool;
    fn dobot_msgs_v4__srv__ServoP_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ServoP_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ServoP_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ServoP_Request>);
    fn dobot_msgs_v4__srv__ServoP_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ServoP_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ServoP_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ServoP_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for ServoP_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ServoP_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ServoP_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ServoP_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ServoP_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ServoP_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ServoP_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ServoP_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ServoP_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ServoP_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ServoP_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ServoP_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ServoP_Response__init(msg: *mut ServoP_Response) -> bool;
    fn dobot_msgs_v4__srv__ServoP_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ServoP_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ServoP_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ServoP_Response>);
    fn dobot_msgs_v4__srv__ServoP_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ServoP_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ServoP_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ServoP_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ServoP_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ServoP_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ServoP_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ServoP_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ServoP_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ServoP_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ServoP_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ServoP_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ServoP_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ServoP_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ServoP_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ServoP_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__TcpDashboard_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__TcpDashboard_Request__init(msg: *mut TcpDashboard_Request) -> bool;
    fn dobot_msgs_v4__srv__TcpDashboard_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TcpDashboard_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__TcpDashboard_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TcpDashboard_Request>);
    fn dobot_msgs_v4__srv__TcpDashboard_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TcpDashboard_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<TcpDashboard_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__TcpDashboard_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TcpDashboard_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: rosidl_runtime_rs::String,

}



impl Default for TcpDashboard_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__TcpDashboard_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__TcpDashboard_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TcpDashboard_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__TcpDashboard_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__TcpDashboard_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__TcpDashboard_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TcpDashboard_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TcpDashboard_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/TcpDashboard_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__TcpDashboard_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__TcpDashboard_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__TcpDashboard_Response__init(msg: *mut TcpDashboard_Response) -> bool;
    fn dobot_msgs_v4__srv__TcpDashboard_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TcpDashboard_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__TcpDashboard_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TcpDashboard_Response>);
    fn dobot_msgs_v4__srv__TcpDashboard_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TcpDashboard_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<TcpDashboard_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__TcpDashboard_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TcpDashboard_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: rosidl_runtime_rs::String,

}



impl Default for TcpDashboard_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__TcpDashboard_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__TcpDashboard_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TcpDashboard_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__TcpDashboard_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__TcpDashboard_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__TcpDashboard_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TcpDashboard_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TcpDashboard_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/TcpDashboard_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__TcpDashboard_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EnableFTSensor_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__EnableFTSensor_Request__init(msg: *mut EnableFTSensor_Request) -> bool;
    fn dobot_msgs_v4__srv__EnableFTSensor_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EnableFTSensor_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__EnableFTSensor_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EnableFTSensor_Request>);
    fn dobot_msgs_v4__srv__EnableFTSensor_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EnableFTSensor_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<EnableFTSensor_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__EnableFTSensor_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnableFTSensor_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i32,

}



impl Default for EnableFTSensor_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__EnableFTSensor_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__EnableFTSensor_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EnableFTSensor_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableFTSensor_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableFTSensor_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableFTSensor_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EnableFTSensor_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EnableFTSensor_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/EnableFTSensor_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EnableFTSensor_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EnableFTSensor_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__EnableFTSensor_Response__init(msg: *mut EnableFTSensor_Response) -> bool;
    fn dobot_msgs_v4__srv__EnableFTSensor_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EnableFTSensor_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__EnableFTSensor_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EnableFTSensor_Response>);
    fn dobot_msgs_v4__srv__EnableFTSensor_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EnableFTSensor_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<EnableFTSensor_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__EnableFTSensor_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EnableFTSensor_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for EnableFTSensor_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__EnableFTSensor_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__EnableFTSensor_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EnableFTSensor_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableFTSensor_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableFTSensor_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EnableFTSensor_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EnableFTSensor_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EnableFTSensor_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/EnableFTSensor_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EnableFTSensor_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SixForceHome_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SixForceHome_Request__init(msg: *mut SixForceHome_Request) -> bool;
    fn dobot_msgs_v4__srv__SixForceHome_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SixForceHome_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SixForceHome_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SixForceHome_Request>);
    fn dobot_msgs_v4__srv__SixForceHome_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SixForceHome_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SixForceHome_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SixForceHome_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SixForceHome_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SixForceHome_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SixForceHome_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SixForceHome_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SixForceHome_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SixForceHome_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SixForceHome_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SixForceHome_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SixForceHome_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SixForceHome_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SixForceHome_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SixForceHome_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SixForceHome_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SixForceHome_Response__init(msg: *mut SixForceHome_Response) -> bool;
    fn dobot_msgs_v4__srv__SixForceHome_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SixForceHome_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SixForceHome_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SixForceHome_Response>);
    fn dobot_msgs_v4__srv__SixForceHome_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SixForceHome_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SixForceHome_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SixForceHome_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SixForceHome_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SixForceHome_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SixForceHome_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SixForceHome_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SixForceHome_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SixForceHome_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SixForceHome_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SixForceHome_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SixForceHome_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SixForceHome_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SixForceHome_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SixForceHome_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetForce_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetForce_Request__init(msg: *mut GetForce_Request) -> bool;
    fn dobot_msgs_v4__srv__GetForce_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetForce_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetForce_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetForce_Request>);
    fn dobot_msgs_v4__srv__GetForce_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetForce_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetForce_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetForce_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetForce_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub tool: i32,

}



impl Default for GetForce_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetForce_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetForce_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetForce_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetForce_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetForce_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetForce_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetForce_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetForce_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetForce_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetForce_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetForce_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetForce_Response__init(msg: *mut GetForce_Response) -> bool;
    fn dobot_msgs_v4__srv__GetForce_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetForce_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetForce_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetForce_Response>);
    fn dobot_msgs_v4__srv__GetForce_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetForce_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetForce_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetForce_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetForce_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetForce_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetForce_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetForce_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetForce_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetForce_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetForce_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetForce_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetForce_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetForce_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetForce_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetForce_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ForceDriveMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ForceDriveMode_Request__init(msg: *mut ForceDriveMode_Request) -> bool;
    fn dobot_msgs_v4__srv__ForceDriveMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ForceDriveMode_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ForceDriveMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ForceDriveMode_Request>);
    fn dobot_msgs_v4__srv__ForceDriveMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ForceDriveMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ForceDriveMode_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ForceDriveMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ForceDriveMode_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ForceDriveMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ForceDriveMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ForceDriveMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ForceDriveMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ForceDriveMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ForceDriveMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ForceDriveMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ForceDriveMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ForceDriveMode_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ForceDriveMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ForceDriveMode_Response__init(msg: *mut ForceDriveMode_Response) -> bool;
    fn dobot_msgs_v4__srv__ForceDriveMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ForceDriveMode_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ForceDriveMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ForceDriveMode_Response>);
    fn dobot_msgs_v4__srv__ForceDriveMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ForceDriveMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ForceDriveMode_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ForceDriveMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceDriveMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ForceDriveMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ForceDriveMode_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ForceDriveMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ForceDriveMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ForceDriveMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ForceDriveMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ForceDriveMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ForceDriveMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ForceDriveMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ForceDriveMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ForceDriveMode_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ForceDriveSpeed_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ForceDriveSpeed_Request__init(msg: *mut ForceDriveSpeed_Request) -> bool;
    fn dobot_msgs_v4__srv__ForceDriveSpeed_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ForceDriveSpeed_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ForceDriveSpeed_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ForceDriveSpeed_Request>);
    fn dobot_msgs_v4__srv__ForceDriveSpeed_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ForceDriveSpeed_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ForceDriveSpeed_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ForceDriveSpeed_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceDriveSpeed_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub speed: i32,

}



impl Default for ForceDriveSpeed_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ForceDriveSpeed_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ForceDriveSpeed_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ForceDriveSpeed_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ForceDriveSpeed_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ForceDriveSpeed_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ForceDriveSpeed_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ForceDriveSpeed_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ForceDriveSpeed_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ForceDriveSpeed_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ForceDriveSpeed_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ForceDriveSpeed_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ForceDriveSpeed_Response__init(msg: *mut ForceDriveSpeed_Response) -> bool;
    fn dobot_msgs_v4__srv__ForceDriveSpeed_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ForceDriveSpeed_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ForceDriveSpeed_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ForceDriveSpeed_Response>);
    fn dobot_msgs_v4__srv__ForceDriveSpeed_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ForceDriveSpeed_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ForceDriveSpeed_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ForceDriveSpeed_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ForceDriveSpeed_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ForceDriveSpeed_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ForceDriveSpeed_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ForceDriveSpeed_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ForceDriveSpeed_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ForceDriveSpeed_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ForceDriveSpeed_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ForceDriveSpeed_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ForceDriveSpeed_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ForceDriveSpeed_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ForceDriveSpeed_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ForceDriveSpeed_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCForceMode_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCForceMode_Request__init(msg: *mut FCForceMode_Request) -> bool;
    fn dobot_msgs_v4__srv__FCForceMode_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCForceMode_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCForceMode_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCForceMode_Request>);
    fn dobot_msgs_v4__srv__FCForceMode_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCForceMode_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FCForceMode_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCForceMode_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCForceMode_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCForceMode_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCForceMode_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCForceMode_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCForceMode_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCForceMode_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCForceMode_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCForceMode_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCForceMode_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCForceMode_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCForceMode_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCForceMode_Response__init(msg: *mut FCForceMode_Response) -> bool;
    fn dobot_msgs_v4__srv__FCForceMode_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCForceMode_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCForceMode_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCForceMode_Response>);
    fn dobot_msgs_v4__srv__FCForceMode_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCForceMode_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FCForceMode_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCForceMode_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCForceMode_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCForceMode_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCForceMode_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCForceMode_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCForceMode_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCForceMode_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCForceMode_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCForceMode_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCForceMode_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCForceMode_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCForceMode_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCForceMode_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetDeviation_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCSetDeviation_Request__init(msg: *mut FCSetDeviation_Request) -> bool;
    fn dobot_msgs_v4__srv__FCSetDeviation_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCSetDeviation_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCSetDeviation_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCSetDeviation_Request>);
    fn dobot_msgs_v4__srv__FCSetDeviation_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCSetDeviation_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FCSetDeviation_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCSetDeviation_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCSetDeviation_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCSetDeviation_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCSetDeviation_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetDeviation_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetDeviation_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetDeviation_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCSetDeviation_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCSetDeviation_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCSetDeviation_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetDeviation_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetDeviation_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCSetDeviation_Response__init(msg: *mut FCSetDeviation_Response) -> bool;
    fn dobot_msgs_v4__srv__FCSetDeviation_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCSetDeviation_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCSetDeviation_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCSetDeviation_Response>);
    fn dobot_msgs_v4__srv__FCSetDeviation_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCSetDeviation_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FCSetDeviation_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCSetDeviation_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetDeviation_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCSetDeviation_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCSetDeviation_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCSetDeviation_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCSetDeviation_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetDeviation_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetDeviation_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetDeviation_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCSetDeviation_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCSetDeviation_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCSetDeviation_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetDeviation_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetForceLimit_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCSetForceLimit_Request__init(msg: *mut FCSetForceLimit_Request) -> bool;
    fn dobot_msgs_v4__srv__FCSetForceLimit_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCSetForceLimit_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCSetForceLimit_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCSetForceLimit_Request>);
    fn dobot_msgs_v4__srv__FCSetForceLimit_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCSetForceLimit_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FCSetForceLimit_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCSetForceLimit_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCSetForceLimit_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCSetForceLimit_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCSetForceLimit_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForceLimit_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForceLimit_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForceLimit_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCSetForceLimit_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCSetForceLimit_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCSetForceLimit_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetForceLimit_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetForceLimit_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCSetForceLimit_Response__init(msg: *mut FCSetForceLimit_Response) -> bool;
    fn dobot_msgs_v4__srv__FCSetForceLimit_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCSetForceLimit_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCSetForceLimit_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCSetForceLimit_Response>);
    fn dobot_msgs_v4__srv__FCSetForceLimit_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCSetForceLimit_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FCSetForceLimit_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCSetForceLimit_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetForceLimit_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCSetForceLimit_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCSetForceLimit_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCSetForceLimit_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCSetForceLimit_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForceLimit_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForceLimit_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForceLimit_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCSetForceLimit_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCSetForceLimit_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCSetForceLimit_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetForceLimit_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetMass_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCSetMass_Request__init(msg: *mut FCSetMass_Request) -> bool;
    fn dobot_msgs_v4__srv__FCSetMass_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCSetMass_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCSetMass_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCSetMass_Request>);
    fn dobot_msgs_v4__srv__FCSetMass_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCSetMass_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FCSetMass_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCSetMass_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCSetMass_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCSetMass_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCSetMass_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetMass_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetMass_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetMass_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCSetMass_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCSetMass_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCSetMass_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetMass_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetMass_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCSetMass_Response__init(msg: *mut FCSetMass_Response) -> bool;
    fn dobot_msgs_v4__srv__FCSetMass_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCSetMass_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCSetMass_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCSetMass_Response>);
    fn dobot_msgs_v4__srv__FCSetMass_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCSetMass_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FCSetMass_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCSetMass_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetMass_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCSetMass_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCSetMass_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCSetMass_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCSetMass_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetMass_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetMass_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetMass_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCSetMass_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCSetMass_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCSetMass_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetMass_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetStiffness_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCSetStiffness_Request__init(msg: *mut FCSetStiffness_Request) -> bool;
    fn dobot_msgs_v4__srv__FCSetStiffness_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCSetStiffness_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCSetStiffness_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCSetStiffness_Request>);
    fn dobot_msgs_v4__srv__FCSetStiffness_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCSetStiffness_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FCSetStiffness_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCSetStiffness_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCSetStiffness_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCSetStiffness_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCSetStiffness_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetStiffness_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetStiffness_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetStiffness_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCSetStiffness_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCSetStiffness_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCSetStiffness_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetStiffness_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetStiffness_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCSetStiffness_Response__init(msg: *mut FCSetStiffness_Response) -> bool;
    fn dobot_msgs_v4__srv__FCSetStiffness_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCSetStiffness_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCSetStiffness_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCSetStiffness_Response>);
    fn dobot_msgs_v4__srv__FCSetStiffness_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCSetStiffness_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FCSetStiffness_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCSetStiffness_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetStiffness_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCSetStiffness_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCSetStiffness_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCSetStiffness_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCSetStiffness_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetStiffness_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetStiffness_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetStiffness_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCSetStiffness_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCSetStiffness_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCSetStiffness_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetStiffness_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetDamping_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCSetDamping_Request__init(msg: *mut FCSetDamping_Request) -> bool;
    fn dobot_msgs_v4__srv__FCSetDamping_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCSetDamping_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCSetDamping_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCSetDamping_Request>);
    fn dobot_msgs_v4__srv__FCSetDamping_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCSetDamping_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FCSetDamping_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCSetDamping_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCSetDamping_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCSetDamping_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCSetDamping_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetDamping_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetDamping_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetDamping_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCSetDamping_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCSetDamping_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCSetDamping_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetDamping_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetDamping_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCSetDamping_Response__init(msg: *mut FCSetDamping_Response) -> bool;
    fn dobot_msgs_v4__srv__FCSetDamping_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCSetDamping_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCSetDamping_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCSetDamping_Response>);
    fn dobot_msgs_v4__srv__FCSetDamping_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCSetDamping_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FCSetDamping_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCSetDamping_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetDamping_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCSetDamping_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCSetDamping_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCSetDamping_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCSetDamping_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetDamping_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetDamping_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetDamping_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCSetDamping_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCSetDamping_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCSetDamping_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetDamping_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCOff_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCOff_Request__init(msg: *mut FCOff_Request) -> bool;
    fn dobot_msgs_v4__srv__FCOff_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCOff_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCOff_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCOff_Request>);
    fn dobot_msgs_v4__srv__FCOff_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCOff_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FCOff_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCOff_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCOff_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for FCOff_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCOff_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCOff_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCOff_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCOff_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCOff_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCOff_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCOff_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCOff_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCOff_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCOff_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCOff_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCOff_Response__init(msg: *mut FCOff_Response) -> bool;
    fn dobot_msgs_v4__srv__FCOff_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCOff_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCOff_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCOff_Response>);
    fn dobot_msgs_v4__srv__FCOff_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCOff_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FCOff_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCOff_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCOff_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCOff_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCOff_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCOff_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCOff_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCOff_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCOff_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCOff_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCOff_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCOff_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCOff_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCOff_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetForceSpeedLimit_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCSetForceSpeedLimit_Request__init(msg: *mut FCSetForceSpeedLimit_Request) -> bool;
    fn dobot_msgs_v4__srv__FCSetForceSpeedLimit_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCSetForceSpeedLimit_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCSetForceSpeedLimit_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCSetForceSpeedLimit_Request>);
    fn dobot_msgs_v4__srv__FCSetForceSpeedLimit_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCSetForceSpeedLimit_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FCSetForceSpeedLimit_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCSetForceSpeedLimit_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCSetForceSpeedLimit_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCSetForceSpeedLimit_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCSetForceSpeedLimit_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForceSpeedLimit_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForceSpeedLimit_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForceSpeedLimit_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCSetForceSpeedLimit_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCSetForceSpeedLimit_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCSetForceSpeedLimit_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetForceSpeedLimit_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetForceSpeedLimit_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCSetForceSpeedLimit_Response__init(msg: *mut FCSetForceSpeedLimit_Response) -> bool;
    fn dobot_msgs_v4__srv__FCSetForceSpeedLimit_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCSetForceSpeedLimit_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCSetForceSpeedLimit_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCSetForceSpeedLimit_Response>);
    fn dobot_msgs_v4__srv__FCSetForceSpeedLimit_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCSetForceSpeedLimit_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FCSetForceSpeedLimit_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCSetForceSpeedLimit_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetForceSpeedLimit_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCSetForceSpeedLimit_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCSetForceSpeedLimit_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCSetForceSpeedLimit_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCSetForceSpeedLimit_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForceSpeedLimit_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForceSpeedLimit_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForceSpeedLimit_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCSetForceSpeedLimit_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCSetForceSpeedLimit_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCSetForceSpeedLimit_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetForceSpeedLimit_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetForce_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCSetForce_Request__init(msg: *mut FCSetForce_Request) -> bool;
    fn dobot_msgs_v4__srv__FCSetForce_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCSetForce_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCSetForce_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCSetForce_Request>);
    fn dobot_msgs_v4__srv__FCSetForce_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCSetForce_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FCSetForce_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCSetForce_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCSetForce_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCSetForce_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCSetForce_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForce_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForce_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForce_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCSetForce_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCSetForce_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCSetForce_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetForce_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetForce_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCSetForce_Response__init(msg: *mut FCSetForce_Response) -> bool;
    fn dobot_msgs_v4__srv__FCSetForce_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCSetForce_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCSetForce_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCSetForce_Response>);
    fn dobot_msgs_v4__srv__FCSetForce_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCSetForce_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FCSetForce_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCSetForce_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCSetForce_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCSetForce_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCSetForce_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCSetForce_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCSetForce_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForce_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForce_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCSetForce_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCSetForce_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCSetForce_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCSetForce_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCSetForce_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetFCCollision_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetFCCollision_Request__init(msg: *mut SetFCCollision_Request) -> bool;
    fn dobot_msgs_v4__srv__SetFCCollision_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetFCCollision_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetFCCollision_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetFCCollision_Request>);
    fn dobot_msgs_v4__srv__SetFCCollision_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetFCCollision_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetFCCollision_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetFCCollision_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetFCCollision_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetFCCollision_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetFCCollision_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetFCCollision_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetFCCollision_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetFCCollision_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetFCCollision_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetFCCollision_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetFCCollision_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetFCCollision_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetFCCollision_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetFCCollision_Response__init(msg: *mut SetFCCollision_Response) -> bool;
    fn dobot_msgs_v4__srv__SetFCCollision_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetFCCollision_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetFCCollision_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetFCCollision_Response>);
    fn dobot_msgs_v4__srv__SetFCCollision_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetFCCollision_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetFCCollision_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetFCCollision_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetFCCollision_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetFCCollision_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetFCCollision_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetFCCollision_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetFCCollision_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetFCCollision_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetFCCollision_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetFCCollision_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetFCCollision_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetFCCollision_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetFCCollision_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetFCCollision_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCCollisionSwitch_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCCollisionSwitch_Request__init(msg: *mut FCCollisionSwitch_Request) -> bool;
    fn dobot_msgs_v4__srv__FCCollisionSwitch_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCCollisionSwitch_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCCollisionSwitch_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCCollisionSwitch_Request>);
    fn dobot_msgs_v4__srv__FCCollisionSwitch_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCCollisionSwitch_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FCCollisionSwitch_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCCollisionSwitch_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCCollisionSwitch_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub enable: i32,

}



impl Default for FCCollisionSwitch_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCCollisionSwitch_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCCollisionSwitch_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCCollisionSwitch_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCCollisionSwitch_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCCollisionSwitch_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCCollisionSwitch_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCCollisionSwitch_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCCollisionSwitch_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCCollisionSwitch_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCCollisionSwitch_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCCollisionSwitch_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__FCCollisionSwitch_Response__init(msg: *mut FCCollisionSwitch_Response) -> bool;
    fn dobot_msgs_v4__srv__FCCollisionSwitch_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FCCollisionSwitch_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__FCCollisionSwitch_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FCCollisionSwitch_Response>);
    fn dobot_msgs_v4__srv__FCCollisionSwitch_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FCCollisionSwitch_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FCCollisionSwitch_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__FCCollisionSwitch_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FCCollisionSwitch_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for FCCollisionSwitch_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__FCCollisionSwitch_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__FCCollisionSwitch_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FCCollisionSwitch_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCCollisionSwitch_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCCollisionSwitch_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__FCCollisionSwitch_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FCCollisionSwitch_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FCCollisionSwitch_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/FCCollisionSwitch_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__FCCollisionSwitch_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetWorkZoneEnable_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetWorkZoneEnable_Request__init(msg: *mut SetWorkZoneEnable_Request) -> bool;
    fn dobot_msgs_v4__srv__SetWorkZoneEnable_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetWorkZoneEnable_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetWorkZoneEnable_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetWorkZoneEnable_Request>);
    fn dobot_msgs_v4__srv__SetWorkZoneEnable_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetWorkZoneEnable_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetWorkZoneEnable_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetWorkZoneEnable_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetWorkZoneEnable_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetWorkZoneEnable_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetWorkZoneEnable_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetWorkZoneEnable_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetWorkZoneEnable_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetWorkZoneEnable_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetWorkZoneEnable_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetWorkZoneEnable_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetWorkZoneEnable_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetWorkZoneEnable_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetWorkZoneEnable_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__SetWorkZoneEnable_Response__init(msg: *mut SetWorkZoneEnable_Response) -> bool;
    fn dobot_msgs_v4__srv__SetWorkZoneEnable_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetWorkZoneEnable_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__SetWorkZoneEnable_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetWorkZoneEnable_Response>);
    fn dobot_msgs_v4__srv__SetWorkZoneEnable_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetWorkZoneEnable_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetWorkZoneEnable_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__SetWorkZoneEnable_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetWorkZoneEnable_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for SetWorkZoneEnable_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__SetWorkZoneEnable_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__SetWorkZoneEnable_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetWorkZoneEnable_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetWorkZoneEnable_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetWorkZoneEnable_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__SetWorkZoneEnable_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetWorkZoneEnable_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetWorkZoneEnable_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/SetWorkZoneEnable_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__SetWorkZoneEnable_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetToolDO_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetToolDO_Request__init(msg: *mut GetToolDO_Request) -> bool;
    fn dobot_msgs_v4__srv__GetToolDO_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetToolDO_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetToolDO_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetToolDO_Request>);
    fn dobot_msgs_v4__srv__GetToolDO_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetToolDO_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetToolDO_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetToolDO_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetToolDO_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub index: i32,

}



impl Default for GetToolDO_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetToolDO_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetToolDO_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetToolDO_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetToolDO_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetToolDO_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetToolDO_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetToolDO_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetToolDO_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetToolDO_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetToolDO_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetToolDO_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetToolDO_Response__init(msg: *mut GetToolDO_Response) -> bool;
    fn dobot_msgs_v4__srv__GetToolDO_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetToolDO_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetToolDO_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetToolDO_Response>);
    fn dobot_msgs_v4__srv__GetToolDO_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetToolDO_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetToolDO_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetToolDO_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetToolDO_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetToolDO_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetToolDO_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetToolDO_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetToolDO_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetToolDO_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetToolDO_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetToolDO_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetToolDO_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetToolDO_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetToolDO_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetToolDO_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ResetRobot_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ResetRobot_Request__init(msg: *mut ResetRobot_Request) -> bool;
    fn dobot_msgs_v4__srv__ResetRobot_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ResetRobot_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ResetRobot_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ResetRobot_Request>);
    fn dobot_msgs_v4__srv__ResetRobot_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ResetRobot_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ResetRobot_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ResetRobot_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResetRobot_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ResetRobot_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ResetRobot_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ResetRobot_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ResetRobot_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ResetRobot_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ResetRobot_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ResetRobot_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ResetRobot_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ResetRobot_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ResetRobot_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ResetRobot_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ResetRobot_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__ResetRobot_Response__init(msg: *mut ResetRobot_Response) -> bool;
    fn dobot_msgs_v4__srv__ResetRobot_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ResetRobot_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__ResetRobot_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ResetRobot_Response>);
    fn dobot_msgs_v4__srv__ResetRobot_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ResetRobot_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ResetRobot_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__ResetRobot_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResetRobot_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for ResetRobot_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__ResetRobot_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__ResetRobot_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ResetRobot_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ResetRobot_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ResetRobot_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__ResetRobot_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ResetRobot_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ResetRobot_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/ResetRobot_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__ResetRobot_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RunTo_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RunTo_Request__init(msg: *mut RunTo_Request) -> bool;
    fn dobot_msgs_v4__srv__RunTo_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RunTo_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RunTo_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RunTo_Request>);
    fn dobot_msgs_v4__srv__RunTo_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RunTo_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RunTo_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RunTo_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RunTo_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RunTo_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RunTo_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RunTo_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RunTo_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RunTo_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RunTo_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RunTo_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RunTo_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RunTo_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RunTo_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RunTo_Response__init(msg: *mut RunTo_Response) -> bool;
    fn dobot_msgs_v4__srv__RunTo_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RunTo_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RunTo_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RunTo_Response>);
    fn dobot_msgs_v4__srv__RunTo_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RunTo_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RunTo_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RunTo_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RunTo_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RunTo_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RunTo_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RunTo_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RunTo_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RunTo_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RunTo_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RunTo_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RunTo_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RunTo_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RunTo_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RunTo_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StartRTOffset_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__StartRTOffset_Request__init(msg: *mut StartRTOffset_Request) -> bool;
    fn dobot_msgs_v4__srv__StartRTOffset_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StartRTOffset_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__StartRTOffset_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StartRTOffset_Request>);
    fn dobot_msgs_v4__srv__StartRTOffset_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StartRTOffset_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<StartRTOffset_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__StartRTOffset_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartRTOffset_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for StartRTOffset_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__StartRTOffset_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__StartRTOffset_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StartRTOffset_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartRTOffset_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartRTOffset_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartRTOffset_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StartRTOffset_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StartRTOffset_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/StartRTOffset_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StartRTOffset_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StartRTOffset_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__StartRTOffset_Response__init(msg: *mut StartRTOffset_Response) -> bool;
    fn dobot_msgs_v4__srv__StartRTOffset_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StartRTOffset_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__StartRTOffset_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StartRTOffset_Response>);
    fn dobot_msgs_v4__srv__StartRTOffset_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StartRTOffset_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<StartRTOffset_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__StartRTOffset_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartRTOffset_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for StartRTOffset_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__StartRTOffset_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__StartRTOffset_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StartRTOffset_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartRTOffset_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartRTOffset_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__StartRTOffset_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StartRTOffset_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StartRTOffset_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/StartRTOffset_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__StartRTOffset_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EndRTOffset_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__EndRTOffset_Request__init(msg: *mut EndRTOffset_Request) -> bool;
    fn dobot_msgs_v4__srv__EndRTOffset_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EndRTOffset_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__EndRTOffset_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EndRTOffset_Request>);
    fn dobot_msgs_v4__srv__EndRTOffset_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EndRTOffset_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<EndRTOffset_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__EndRTOffset_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EndRTOffset_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for EndRTOffset_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__EndRTOffset_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__EndRTOffset_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EndRTOffset_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EndRTOffset_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EndRTOffset_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EndRTOffset_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EndRTOffset_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EndRTOffset_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/EndRTOffset_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EndRTOffset_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EndRTOffset_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__EndRTOffset_Response__init(msg: *mut EndRTOffset_Response) -> bool;
    fn dobot_msgs_v4__srv__EndRTOffset_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EndRTOffset_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__EndRTOffset_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EndRTOffset_Response>);
    fn dobot_msgs_v4__srv__EndRTOffset_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EndRTOffset_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<EndRTOffset_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__EndRTOffset_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EndRTOffset_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for EndRTOffset_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__EndRTOffset_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__EndRTOffset_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EndRTOffset_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EndRTOffset_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EndRTOffset_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__EndRTOffset_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EndRTOffset_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EndRTOffset_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/EndRTOffset_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__EndRTOffset_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetError_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetError_Request__init(msg: *mut GetError_Request) -> bool;
    fn dobot_msgs_v4__srv__GetError_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetError_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetError_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetError_Request>);
    fn dobot_msgs_v4__srv__GetError_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetError_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetError_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetError_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetError_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub language: rosidl_runtime_rs::String,

}



impl Default for GetError_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetError_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetError_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetError_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetError_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetError_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetError_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetError_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetError_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetError_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetError_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetError_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetError_Response__init(msg: *mut GetError_Response) -> bool;
    fn dobot_msgs_v4__srv__GetError_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetError_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetError_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetError_Response>);
    fn dobot_msgs_v4__srv__GetError_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetError_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetError_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetError_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetError_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetError_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetError_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetError_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetError_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetError_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetError_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetError_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetError_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetError_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetError_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetError_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DOGroupDEC_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DOGroupDEC_Request__init(msg: *mut DOGroupDEC_Request) -> bool;
    fn dobot_msgs_v4__srv__DOGroupDEC_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DOGroupDEC_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DOGroupDEC_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DOGroupDEC_Request>);
    fn dobot_msgs_v4__srv__DOGroupDEC_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DOGroupDEC_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DOGroupDEC_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DOGroupDEC_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DOGroupDEC_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DOGroupDEC_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DOGroupDEC_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOGroupDEC_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOGroupDEC_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOGroupDEC_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DOGroupDEC_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DOGroupDEC_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DOGroupDEC_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DOGroupDEC_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DOGroupDEC_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DOGroupDEC_Response__init(msg: *mut DOGroupDEC_Response) -> bool;
    fn dobot_msgs_v4__srv__DOGroupDEC_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DOGroupDEC_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DOGroupDEC_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DOGroupDEC_Response>);
    fn dobot_msgs_v4__srv__DOGroupDEC_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DOGroupDEC_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DOGroupDEC_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DOGroupDEC_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DOGroupDEC_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DOGroupDEC_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DOGroupDEC_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DOGroupDEC_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DOGroupDEC_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOGroupDEC_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOGroupDEC_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DOGroupDEC_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DOGroupDEC_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DOGroupDEC_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DOGroupDEC_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DOGroupDEC_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetDOGroupDEC_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetDOGroupDEC_Request__init(msg: *mut GetDOGroupDEC_Request) -> bool;
    fn dobot_msgs_v4__srv__GetDOGroupDEC_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetDOGroupDEC_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetDOGroupDEC_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetDOGroupDEC_Request>);
    fn dobot_msgs_v4__srv__GetDOGroupDEC_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetDOGroupDEC_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetDOGroupDEC_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetDOGroupDEC_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetDOGroupDEC_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetDOGroupDEC_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetDOGroupDEC_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDOGroupDEC_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDOGroupDEC_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDOGroupDEC_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetDOGroupDEC_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetDOGroupDEC_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetDOGroupDEC_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetDOGroupDEC_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetDOGroupDEC_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__GetDOGroupDEC_Response__init(msg: *mut GetDOGroupDEC_Response) -> bool;
    fn dobot_msgs_v4__srv__GetDOGroupDEC_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetDOGroupDEC_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__GetDOGroupDEC_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetDOGroupDEC_Response>);
    fn dobot_msgs_v4__srv__GetDOGroupDEC_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetDOGroupDEC_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetDOGroupDEC_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__GetDOGroupDEC_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetDOGroupDEC_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for GetDOGroupDEC_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__GetDOGroupDEC_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__GetDOGroupDEC_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetDOGroupDEC_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDOGroupDEC_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDOGroupDEC_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__GetDOGroupDEC_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetDOGroupDEC_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetDOGroupDEC_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/GetDOGroupDEC_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__GetDOGroupDEC_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DIGroupDEC_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DIGroupDEC_Request__init(msg: *mut DIGroupDEC_Request) -> bool;
    fn dobot_msgs_v4__srv__DIGroupDEC_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DIGroupDEC_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DIGroupDEC_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DIGroupDEC_Request>);
    fn dobot_msgs_v4__srv__DIGroupDEC_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DIGroupDEC_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DIGroupDEC_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DIGroupDEC_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DIGroupDEC_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DIGroupDEC_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DIGroupDEC_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DIGroupDEC_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DIGroupDEC_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DIGroupDEC_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DIGroupDEC_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DIGroupDEC_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DIGroupDEC_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DIGroupDEC_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DIGroupDEC_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__DIGroupDEC_Response__init(msg: *mut DIGroupDEC_Response) -> bool;
    fn dobot_msgs_v4__srv__DIGroupDEC_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DIGroupDEC_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__DIGroupDEC_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DIGroupDEC_Response>);
    fn dobot_msgs_v4__srv__DIGroupDEC_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DIGroupDEC_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DIGroupDEC_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__DIGroupDEC_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DIGroupDEC_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for DIGroupDEC_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__DIGroupDEC_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__DIGroupDEC_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DIGroupDEC_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DIGroupDEC_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DIGroupDEC_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__DIGroupDEC_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DIGroupDEC_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DIGroupDEC_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/DIGroupDEC_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__DIGroupDEC_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RequestControl_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RequestControl_Request__init(msg: *mut RequestControl_Request) -> bool;
    fn dobot_msgs_v4__srv__RequestControl_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RequestControl_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RequestControl_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RequestControl_Request>);
    fn dobot_msgs_v4__srv__RequestControl_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RequestControl_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RequestControl_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RequestControl_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RequestControl_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for RequestControl_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RequestControl_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RequestControl_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RequestControl_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RequestControl_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RequestControl_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RequestControl_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RequestControl_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RequestControl_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RequestControl_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RequestControl_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RequestControl_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__RequestControl_Response__init(msg: *mut RequestControl_Response) -> bool;
    fn dobot_msgs_v4__srv__RequestControl_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RequestControl_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__RequestControl_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RequestControl_Response>);
    fn dobot_msgs_v4__srv__RequestControl_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RequestControl_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RequestControl_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__RequestControl_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RequestControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for RequestControl_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__RequestControl_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__RequestControl_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RequestControl_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RequestControl_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RequestControl_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__RequestControl_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RequestControl_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RequestControl_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/RequestControl_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__RequestControl_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CheckOddMovL_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__CheckOddMovL_Request__init(msg: *mut CheckOddMovL_Request) -> bool;
    fn dobot_msgs_v4__srv__CheckOddMovL_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovL_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__CheckOddMovL_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovL_Request>);
    fn dobot_msgs_v4__srv__CheckOddMovL_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckOddMovL_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovL_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__CheckOddMovL_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for CheckOddMovL_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__CheckOddMovL_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__CheckOddMovL_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckOddMovL_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovL_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovL_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovL_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckOddMovL_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckOddMovL_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/CheckOddMovL_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CheckOddMovL_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CheckOddMovL_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__CheckOddMovL_Response__init(msg: *mut CheckOddMovL_Response) -> bool;
    fn dobot_msgs_v4__srv__CheckOddMovL_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovL_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__CheckOddMovL_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovL_Response>);
    fn dobot_msgs_v4__srv__CheckOddMovL_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckOddMovL_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovL_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__CheckOddMovL_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckOddMovL_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for CheckOddMovL_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__CheckOddMovL_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__CheckOddMovL_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckOddMovL_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovL_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovL_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovL_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckOddMovL_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckOddMovL_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/CheckOddMovL_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CheckOddMovL_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CheckOddMovJ_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__CheckOddMovJ_Request__init(msg: *mut CheckOddMovJ_Request) -> bool;
    fn dobot_msgs_v4__srv__CheckOddMovJ_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovJ_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__CheckOddMovJ_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovJ_Request>);
    fn dobot_msgs_v4__srv__CheckOddMovJ_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckOddMovJ_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovJ_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__CheckOddMovJ_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for CheckOddMovJ_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__CheckOddMovJ_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__CheckOddMovJ_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckOddMovJ_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovJ_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovJ_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovJ_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckOddMovJ_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckOddMovJ_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/CheckOddMovJ_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CheckOddMovJ_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CheckOddMovJ_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__CheckOddMovJ_Response__init(msg: *mut CheckOddMovJ_Response) -> bool;
    fn dobot_msgs_v4__srv__CheckOddMovJ_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovJ_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__CheckOddMovJ_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovJ_Response>);
    fn dobot_msgs_v4__srv__CheckOddMovJ_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckOddMovJ_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovJ_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__CheckOddMovJ_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckOddMovJ_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for CheckOddMovJ_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__CheckOddMovJ_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__CheckOddMovJ_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckOddMovJ_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovJ_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovJ_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovJ_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckOddMovJ_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckOddMovJ_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/CheckOddMovJ_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CheckOddMovJ_Response() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CheckOddMovC_Request() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__CheckOddMovC_Request__init(msg: *mut CheckOddMovC_Request) -> bool;
    fn dobot_msgs_v4__srv__CheckOddMovC_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovC_Request>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__CheckOddMovC_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovC_Request>);
    fn dobot_msgs_v4__srv__CheckOddMovC_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckOddMovC_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovC_Request>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__CheckOddMovC_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    pub param_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for CheckOddMovC_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__CheckOddMovC_Request__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__CheckOddMovC_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckOddMovC_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovC_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovC_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovC_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckOddMovC_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckOddMovC_Request where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/CheckOddMovC_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CheckOddMovC_Request() }
  }
}


#[link(name = "dobot_msgs_v4__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CheckOddMovC_Response() -> *const std::ffi::c_void;
}

#[link(name = "dobot_msgs_v4__rosidl_generator_c")]
extern "C" {
    fn dobot_msgs_v4__srv__CheckOddMovC_Response__init(msg: *mut CheckOddMovC_Response) -> bool;
    fn dobot_msgs_v4__srv__CheckOddMovC_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovC_Response>, size: usize) -> bool;
    fn dobot_msgs_v4__srv__CheckOddMovC_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovC_Response>);
    fn dobot_msgs_v4__srv__CheckOddMovC_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckOddMovC_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckOddMovC_Response>) -> bool;
}

// Corresponds to dobot_msgs_v4__srv__CheckOddMovC_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckOddMovC_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub robot_return: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub res: i32,

}



impl Default for CheckOddMovC_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !dobot_msgs_v4__srv__CheckOddMovC_Response__init(&mut msg as *mut _) {
        panic!("Call to dobot_msgs_v4__srv__CheckOddMovC_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckOddMovC_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovC_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovC_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { dobot_msgs_v4__srv__CheckOddMovC_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckOddMovC_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckOddMovC_Response where Self: Sized {
  const TYPE_NAME: &'static str = "dobot_msgs_v4/srv/CheckOddMovC_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__dobot_msgs_v4__srv__CheckOddMovC_Response() }
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


