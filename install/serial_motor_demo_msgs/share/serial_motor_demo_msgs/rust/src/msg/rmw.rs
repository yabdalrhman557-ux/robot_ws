#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "serial_motor_demo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__serial_motor_demo_msgs__msg__MotorCommand() -> *const std::ffi::c_void;
}

#[link(name = "serial_motor_demo_msgs__rosidl_generator_c")]
extern "C" {
    fn serial_motor_demo_msgs__msg__MotorCommand__init(msg: *mut MotorCommand) -> bool;
    fn serial_motor_demo_msgs__msg__MotorCommand__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorCommand>, size: usize) -> bool;
    fn serial_motor_demo_msgs__msg__MotorCommand__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorCommand>);
    fn serial_motor_demo_msgs__msg__MotorCommand__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorCommand>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorCommand>) -> bool;
}

// Corresponds to serial_motor_demo_msgs__msg__MotorCommand
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorCommand {

    // This member is not documented.
    #[allow(missing_docs)]
    pub is_pwm: bool,

    /// in pwm mode this is cast to int and is pwm value
    pub mot_1_req_rad_sec: f32,

    /// in pwm mode this is cast to int and is pwm value
    pub mot_2_req_rad_sec: f32,

}



impl Default for MotorCommand {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !serial_motor_demo_msgs__msg__MotorCommand__init(&mut msg as *mut _) {
        panic!("Call to serial_motor_demo_msgs__msg__MotorCommand__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorCommand {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { serial_motor_demo_msgs__msg__MotorCommand__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { serial_motor_demo_msgs__msg__MotorCommand__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { serial_motor_demo_msgs__msg__MotorCommand__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorCommand {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorCommand where Self: Sized {
  const TYPE_NAME: &'static str = "serial_motor_demo_msgs/msg/MotorCommand";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__serial_motor_demo_msgs__msg__MotorCommand() }
  }
}


#[link(name = "serial_motor_demo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__serial_motor_demo_msgs__msg__MotorVels() -> *const std::ffi::c_void;
}

#[link(name = "serial_motor_demo_msgs__rosidl_generator_c")]
extern "C" {
    fn serial_motor_demo_msgs__msg__MotorVels__init(msg: *mut MotorVels) -> bool;
    fn serial_motor_demo_msgs__msg__MotorVels__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MotorVels>, size: usize) -> bool;
    fn serial_motor_demo_msgs__msg__MotorVels__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MotorVels>);
    fn serial_motor_demo_msgs__msg__MotorVels__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MotorVels>, out_seq: *mut rosidl_runtime_rs::Sequence<MotorVels>) -> bool;
}

// Corresponds to serial_motor_demo_msgs__msg__MotorVels
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MotorVels {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mot_1_rad_sec: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mot_2_rad_sec: f32,

}



impl Default for MotorVels {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !serial_motor_demo_msgs__msg__MotorVels__init(&mut msg as *mut _) {
        panic!("Call to serial_motor_demo_msgs__msg__MotorVels__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MotorVels {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { serial_motor_demo_msgs__msg__MotorVels__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { serial_motor_demo_msgs__msg__MotorVels__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { serial_motor_demo_msgs__msg__MotorVels__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MotorVels {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MotorVels where Self: Sized {
  const TYPE_NAME: &'static str = "serial_motor_demo_msgs/msg/MotorVels";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__serial_motor_demo_msgs__msg__MotorVels() }
  }
}


#[link(name = "serial_motor_demo_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__serial_motor_demo_msgs__msg__EncoderVals() -> *const std::ffi::c_void;
}

#[link(name = "serial_motor_demo_msgs__rosidl_generator_c")]
extern "C" {
    fn serial_motor_demo_msgs__msg__EncoderVals__init(msg: *mut EncoderVals) -> bool;
    fn serial_motor_demo_msgs__msg__EncoderVals__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EncoderVals>, size: usize) -> bool;
    fn serial_motor_demo_msgs__msg__EncoderVals__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EncoderVals>);
    fn serial_motor_demo_msgs__msg__EncoderVals__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EncoderVals>, out_seq: *mut rosidl_runtime_rs::Sequence<EncoderVals>) -> bool;
}

// Corresponds to serial_motor_demo_msgs__msg__EncoderVals
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EncoderVals {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mot_1_enc_val: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mot_2_enc_val: i32,

}



impl Default for EncoderVals {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !serial_motor_demo_msgs__msg__EncoderVals__init(&mut msg as *mut _) {
        panic!("Call to serial_motor_demo_msgs__msg__EncoderVals__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EncoderVals {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { serial_motor_demo_msgs__msg__EncoderVals__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { serial_motor_demo_msgs__msg__EncoderVals__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { serial_motor_demo_msgs__msg__EncoderVals__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EncoderVals {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EncoderVals where Self: Sized {
  const TYPE_NAME: &'static str = "serial_motor_demo_msgs/msg/EncoderVals";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__serial_motor_demo_msgs__msg__EncoderVals() }
  }
}


