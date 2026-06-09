#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__AttachTool_Request() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__AttachTool_Request__init(msg: *mut AttachTool_Request) -> bool;
    fn ariac_interfaces__srv__AttachTool_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AttachTool_Request>, size: usize) -> bool;
    fn ariac_interfaces__srv__AttachTool_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AttachTool_Request>);
    fn ariac_interfaces__srv__AttachTool_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AttachTool_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AttachTool_Request>) -> bool;
}

// Corresponds to ariac_interfaces__srv__AttachTool_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AttachTool_Request {
    /// Available types are in VacuumTools.msg
    pub tool: i8,

}



impl Default for AttachTool_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__AttachTool_Request__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__AttachTool_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AttachTool_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__AttachTool_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__AttachTool_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__AttachTool_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AttachTool_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AttachTool_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/AttachTool_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__AttachTool_Request() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__AttachTool_Response() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__AttachTool_Response__init(msg: *mut AttachTool_Response) -> bool;
    fn ariac_interfaces__srv__AttachTool_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AttachTool_Response>, size: usize) -> bool;
    fn ariac_interfaces__srv__AttachTool_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AttachTool_Response>);
    fn ariac_interfaces__srv__AttachTool_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AttachTool_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AttachTool_Response>) -> bool;
}

// Corresponds to ariac_interfaces__srv__AttachTool_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AttachTool_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for AttachTool_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__AttachTool_Response__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__AttachTool_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AttachTool_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__AttachTool_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__AttachTool_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__AttachTool_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AttachTool_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AttachTool_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/AttachTool_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__AttachTool_Response() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__BidirectionalConveyorControl_Request() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__BidirectionalConveyorControl_Request__init(msg: *mut BidirectionalConveyorControl_Request) -> bool;
    fn ariac_interfaces__srv__BidirectionalConveyorControl_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BidirectionalConveyorControl_Request>, size: usize) -> bool;
    fn ariac_interfaces__srv__BidirectionalConveyorControl_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BidirectionalConveyorControl_Request>);
    fn ariac_interfaces__srv__BidirectionalConveyorControl_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BidirectionalConveyorControl_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BidirectionalConveyorControl_Request>) -> bool;
}

// Corresponds to ariac_interfaces__srv__BidirectionalConveyorControl_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__BidirectionalConveyorControl_Request__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__BidirectionalConveyorControl_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BidirectionalConveyorControl_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__BidirectionalConveyorControl_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__BidirectionalConveyorControl_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__BidirectionalConveyorControl_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BidirectionalConveyorControl_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BidirectionalConveyorControl_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/BidirectionalConveyorControl_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__BidirectionalConveyorControl_Request() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__BidirectionalConveyorControl_Response() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__BidirectionalConveyorControl_Response__init(msg: *mut BidirectionalConveyorControl_Response) -> bool;
    fn ariac_interfaces__srv__BidirectionalConveyorControl_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BidirectionalConveyorControl_Response>, size: usize) -> bool;
    fn ariac_interfaces__srv__BidirectionalConveyorControl_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BidirectionalConveyorControl_Response>);
    fn ariac_interfaces__srv__BidirectionalConveyorControl_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BidirectionalConveyorControl_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BidirectionalConveyorControl_Response>) -> bool;
}

// Corresponds to ariac_interfaces__srv__BidirectionalConveyorControl_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BidirectionalConveyorControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for BidirectionalConveyorControl_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__BidirectionalConveyorControl_Response__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__BidirectionalConveyorControl_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BidirectionalConveyorControl_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__BidirectionalConveyorControl_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__BidirectionalConveyorControl_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__BidirectionalConveyorControl_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BidirectionalConveyorControl_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BidirectionalConveyorControl_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/BidirectionalConveyorControl_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__BidirectionalConveyorControl_Response() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__CheckKitQuality_Request() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__CheckKitQuality_Request__init(msg: *mut CheckKitQuality_Request) -> bool;
    fn ariac_interfaces__srv__CheckKitQuality_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckKitQuality_Request>, size: usize) -> bool;
    fn ariac_interfaces__srv__CheckKitQuality_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckKitQuality_Request>);
    fn ariac_interfaces__srv__CheckKitQuality_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckKitQuality_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckKitQuality_Request>) -> bool;
}

// Corresponds to ariac_interfaces__srv__CheckKitQuality_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckKitQuality_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cell_type: u8,

}



impl Default for CheckKitQuality_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__CheckKitQuality_Request__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__CheckKitQuality_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckKitQuality_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__CheckKitQuality_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__CheckKitQuality_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__CheckKitQuality_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckKitQuality_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckKitQuality_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/CheckKitQuality_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__CheckKitQuality_Request() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__CheckKitQuality_Response() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__CheckKitQuality_Response__init(msg: *mut CheckKitQuality_Response) -> bool;
    fn ariac_interfaces__srv__CheckKitQuality_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CheckKitQuality_Response>, size: usize) -> bool;
    fn ariac_interfaces__srv__CheckKitQuality_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CheckKitQuality_Response>);
    fn ariac_interfaces__srv__CheckKitQuality_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CheckKitQuality_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CheckKitQuality_Response>) -> bool;
}

// Corresponds to ariac_interfaces__srv__CheckKitQuality_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CheckKitQuality_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub is_good: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for CheckKitQuality_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__CheckKitQuality_Response__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__CheckKitQuality_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CheckKitQuality_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__CheckKitQuality_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__CheckKitQuality_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__CheckKitQuality_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CheckKitQuality_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CheckKitQuality_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/CheckKitQuality_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__CheckKitQuality_Response() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__ControlCellFeeder_Request() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__ControlCellFeeder_Request__init(msg: *mut ControlCellFeeder_Request) -> bool;
    fn ariac_interfaces__srv__ControlCellFeeder_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ControlCellFeeder_Request>, size: usize) -> bool;
    fn ariac_interfaces__srv__ControlCellFeeder_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ControlCellFeeder_Request>);
    fn ariac_interfaces__srv__ControlCellFeeder_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ControlCellFeeder_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ControlCellFeeder_Request>) -> bool;
}

// Corresponds to ariac_interfaces__srv__ControlCellFeeder_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ControlCellFeeder_Request {
    /// available types are in CellTypes.msg
    pub cell_type: u8,

}



impl Default for ControlCellFeeder_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__ControlCellFeeder_Request__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__ControlCellFeeder_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ControlCellFeeder_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__ControlCellFeeder_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__ControlCellFeeder_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__ControlCellFeeder_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ControlCellFeeder_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ControlCellFeeder_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/ControlCellFeeder_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__ControlCellFeeder_Request() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__ControlCellFeeder_Response() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__ControlCellFeeder_Response__init(msg: *mut ControlCellFeeder_Response) -> bool;
    fn ariac_interfaces__srv__ControlCellFeeder_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ControlCellFeeder_Response>, size: usize) -> bool;
    fn ariac_interfaces__srv__ControlCellFeeder_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ControlCellFeeder_Response>);
    fn ariac_interfaces__srv__ControlCellFeeder_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ControlCellFeeder_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ControlCellFeeder_Response>) -> bool;
}

// Corresponds to ariac_interfaces__srv__ControlCellFeeder_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ControlCellFeeder_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for ControlCellFeeder_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__ControlCellFeeder_Response__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__ControlCellFeeder_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ControlCellFeeder_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__ControlCellFeeder_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__ControlCellFeeder_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__ControlCellFeeder_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ControlCellFeeder_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ControlCellFeeder_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/ControlCellFeeder_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__ControlCellFeeder_Response() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__ConveyorControl_Request() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__ConveyorControl_Request__init(msg: *mut ConveyorControl_Request) -> bool;
    fn ariac_interfaces__srv__ConveyorControl_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConveyorControl_Request>, size: usize) -> bool;
    fn ariac_interfaces__srv__ConveyorControl_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConveyorControl_Request>);
    fn ariac_interfaces__srv__ConveyorControl_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConveyorControl_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ConveyorControl_Request>) -> bool;
}

// Corresponds to ariac_interfaces__srv__ConveyorControl_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConveyorControl_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub speed: f64,

}



impl Default for ConveyorControl_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__ConveyorControl_Request__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__ConveyorControl_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConveyorControl_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__ConveyorControl_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__ConveyorControl_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__ConveyorControl_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConveyorControl_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConveyorControl_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/ConveyorControl_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__ConveyorControl_Request() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__ConveyorControl_Response() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__ConveyorControl_Response__init(msg: *mut ConveyorControl_Response) -> bool;
    fn ariac_interfaces__srv__ConveyorControl_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ConveyorControl_Response>, size: usize) -> bool;
    fn ariac_interfaces__srv__ConveyorControl_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ConveyorControl_Response>);
    fn ariac_interfaces__srv__ConveyorControl_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ConveyorControl_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ConveyorControl_Response>) -> bool;
}

// Corresponds to ariac_interfaces__srv__ConveyorControl_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ConveyorControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for ConveyorControl_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__ConveyorControl_Response__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__ConveyorControl_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ConveyorControl_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__ConveyorControl_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__ConveyorControl_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__ConveyorControl_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ConveyorControl_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ConveyorControl_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/ConveyorControl_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__ConveyorControl_Response() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__EndCompetition_Request() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__EndCompetition_Request__init(msg: *mut EndCompetition_Request) -> bool;
    fn ariac_interfaces__srv__EndCompetition_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EndCompetition_Request>, size: usize) -> bool;
    fn ariac_interfaces__srv__EndCompetition_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EndCompetition_Request>);
    fn ariac_interfaces__srv__EndCompetition_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EndCompetition_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<EndCompetition_Request>) -> bool;
}

// Corresponds to ariac_interfaces__srv__EndCompetition_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EndCompetition_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub shutdown_gazebo: bool,

}



impl Default for EndCompetition_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__EndCompetition_Request__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__EndCompetition_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EndCompetition_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__EndCompetition_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__EndCompetition_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__EndCompetition_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EndCompetition_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EndCompetition_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/EndCompetition_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__EndCompetition_Request() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__EndCompetition_Response() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__EndCompetition_Response__init(msg: *mut EndCompetition_Response) -> bool;
    fn ariac_interfaces__srv__EndCompetition_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EndCompetition_Response>, size: usize) -> bool;
    fn ariac_interfaces__srv__EndCompetition_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EndCompetition_Response>);
    fn ariac_interfaces__srv__EndCompetition_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EndCompetition_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<EndCompetition_Response>) -> bool;
}

// Corresponds to ariac_interfaces__srv__EndCompetition_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EndCompetition_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for EndCompetition_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__EndCompetition_Response__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__EndCompetition_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EndCompetition_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__EndCompetition_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__EndCompetition_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__EndCompetition_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EndCompetition_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EndCompetition_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/EndCompetition_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__EndCompetition_Response() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__SubmitHighPriorityOrder_Request() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__SubmitHighPriorityOrder_Request__init(msg: *mut SubmitHighPriorityOrder_Request) -> bool;
    fn ariac_interfaces__srv__SubmitHighPriorityOrder_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SubmitHighPriorityOrder_Request>, size: usize) -> bool;
    fn ariac_interfaces__srv__SubmitHighPriorityOrder_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SubmitHighPriorityOrder_Request>);
    fn ariac_interfaces__srv__SubmitHighPriorityOrder_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SubmitHighPriorityOrder_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SubmitHighPriorityOrder_Request>) -> bool;
}

// Corresponds to ariac_interfaces__srv__SubmitHighPriorityOrder_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmitHighPriorityOrder_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: rosidl_runtime_rs::String,

}



impl Default for SubmitHighPriorityOrder_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__SubmitHighPriorityOrder_Request__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__SubmitHighPriorityOrder_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SubmitHighPriorityOrder_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__SubmitHighPriorityOrder_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__SubmitHighPriorityOrder_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__SubmitHighPriorityOrder_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SubmitHighPriorityOrder_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SubmitHighPriorityOrder_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/SubmitHighPriorityOrder_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__SubmitHighPriorityOrder_Request() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__SubmitHighPriorityOrder_Response() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__SubmitHighPriorityOrder_Response__init(msg: *mut SubmitHighPriorityOrder_Response) -> bool;
    fn ariac_interfaces__srv__SubmitHighPriorityOrder_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SubmitHighPriorityOrder_Response>, size: usize) -> bool;
    fn ariac_interfaces__srv__SubmitHighPriorityOrder_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SubmitHighPriorityOrder_Response>);
    fn ariac_interfaces__srv__SubmitHighPriorityOrder_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SubmitHighPriorityOrder_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SubmitHighPriorityOrder_Response>) -> bool;
}

// Corresponds to ariac_interfaces__srv__SubmitHighPriorityOrder_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmitHighPriorityOrder_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SubmitHighPriorityOrder_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__SubmitHighPriorityOrder_Response__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__SubmitHighPriorityOrder_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SubmitHighPriorityOrder_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__SubmitHighPriorityOrder_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__SubmitHighPriorityOrder_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__SubmitHighPriorityOrder_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SubmitHighPriorityOrder_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SubmitHighPriorityOrder_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/SubmitHighPriorityOrder_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__SubmitHighPriorityOrder_Response() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__SubmitInspectionReport_Request() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__SubmitInspectionReport_Request__init(msg: *mut SubmitInspectionReport_Request) -> bool;
    fn ariac_interfaces__srv__SubmitInspectionReport_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SubmitInspectionReport_Request>, size: usize) -> bool;
    fn ariac_interfaces__srv__SubmitInspectionReport_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SubmitInspectionReport_Request>);
    fn ariac_interfaces__srv__SubmitInspectionReport_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SubmitInspectionReport_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SubmitInspectionReport_Request>) -> bool;
}

// Corresponds to ariac_interfaces__srv__SubmitInspectionReport_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmitInspectionReport_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub timestamp: builtin_interfaces::msg::rmw::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    pub report: super::super::msg::rmw::InspectionReport,

}



impl Default for SubmitInspectionReport_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__SubmitInspectionReport_Request__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__SubmitInspectionReport_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SubmitInspectionReport_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__SubmitInspectionReport_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__SubmitInspectionReport_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__SubmitInspectionReport_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SubmitInspectionReport_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SubmitInspectionReport_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/SubmitInspectionReport_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__SubmitInspectionReport_Request() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__SubmitInspectionReport_Response() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__SubmitInspectionReport_Response__init(msg: *mut SubmitInspectionReport_Response) -> bool;
    fn ariac_interfaces__srv__SubmitInspectionReport_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SubmitInspectionReport_Response>, size: usize) -> bool;
    fn ariac_interfaces__srv__SubmitInspectionReport_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SubmitInspectionReport_Response>);
    fn ariac_interfaces__srv__SubmitInspectionReport_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SubmitInspectionReport_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SubmitInspectionReport_Response>) -> bool;
}

// Corresponds to ariac_interfaces__srv__SubmitInspectionReport_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmitInspectionReport_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for SubmitInspectionReport_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__SubmitInspectionReport_Response__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__SubmitInspectionReport_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SubmitInspectionReport_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__SubmitInspectionReport_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__SubmitInspectionReport_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__SubmitInspectionReport_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SubmitInspectionReport_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SubmitInspectionReport_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/SubmitInspectionReport_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__SubmitInspectionReport_Response() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__Trigger_Request() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__Trigger_Request__init(msg: *mut Trigger_Request) -> bool;
    fn ariac_interfaces__srv__Trigger_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Trigger_Request>, size: usize) -> bool;
    fn ariac_interfaces__srv__Trigger_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Trigger_Request>);
    fn ariac_interfaces__srv__Trigger_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Trigger_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Trigger_Request>) -> bool;
}

// Corresponds to ariac_interfaces__srv__Trigger_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Trigger_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Trigger_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__Trigger_Request__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__Trigger_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Trigger_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__Trigger_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__Trigger_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__Trigger_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Trigger_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Trigger_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/Trigger_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__Trigger_Request() }
  }
}


#[link(name = "ariac_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__Trigger_Response() -> *const std::ffi::c_void;
}

#[link(name = "ariac_interfaces__rosidl_generator_c")]
extern "C" {
    fn ariac_interfaces__srv__Trigger_Response__init(msg: *mut Trigger_Response) -> bool;
    fn ariac_interfaces__srv__Trigger_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Trigger_Response>, size: usize) -> bool;
    fn ariac_interfaces__srv__Trigger_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Trigger_Response>);
    fn ariac_interfaces__srv__Trigger_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Trigger_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Trigger_Response>) -> bool;
}

// Corresponds to ariac_interfaces__srv__Trigger_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Trigger_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for Trigger_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ariac_interfaces__srv__Trigger_Response__init(&mut msg as *mut _) {
        panic!("Call to ariac_interfaces__srv__Trigger_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Trigger_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__Trigger_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__Trigger_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ariac_interfaces__srv__Trigger_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Trigger_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Trigger_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ariac_interfaces/srv/Trigger_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ariac_interfaces__srv__Trigger_Response() }
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


