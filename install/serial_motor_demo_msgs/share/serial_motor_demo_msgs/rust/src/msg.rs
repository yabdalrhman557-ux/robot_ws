#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to serial_motor_demo_msgs__msg__MotorCommand

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotorCommand::default())
  }
}

impl rosidl_runtime_rs::Message for MotorCommand {
  type RmwMsg = super::msg::rmw::MotorCommand;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        is_pwm: msg.is_pwm,
        mot_1_req_rad_sec: msg.mot_1_req_rad_sec,
        mot_2_req_rad_sec: msg.mot_2_req_rad_sec,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      is_pwm: msg.is_pwm,
      mot_1_req_rad_sec: msg.mot_1_req_rad_sec,
      mot_2_req_rad_sec: msg.mot_2_req_rad_sec,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      is_pwm: msg.is_pwm,
      mot_1_req_rad_sec: msg.mot_1_req_rad_sec,
      mot_2_req_rad_sec: msg.mot_2_req_rad_sec,
    }
  }
}


// Corresponds to serial_motor_demo_msgs__msg__MotorVels

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MotorVels::default())
  }
}

impl rosidl_runtime_rs::Message for MotorVels {
  type RmwMsg = super::msg::rmw::MotorVels;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mot_1_rad_sec: msg.mot_1_rad_sec,
        mot_2_rad_sec: msg.mot_2_rad_sec,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mot_1_rad_sec: msg.mot_1_rad_sec,
      mot_2_rad_sec: msg.mot_2_rad_sec,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mot_1_rad_sec: msg.mot_1_rad_sec,
      mot_2_rad_sec: msg.mot_2_rad_sec,
    }
  }
}


// Corresponds to serial_motor_demo_msgs__msg__EncoderVals

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::EncoderVals::default())
  }
}

impl rosidl_runtime_rs::Message for EncoderVals {
  type RmwMsg = super::msg::rmw::EncoderVals;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mot_1_enc_val: msg.mot_1_enc_val,
        mot_2_enc_val: msg.mot_2_enc_val,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mot_1_enc_val: msg.mot_1_enc_val,
      mot_2_enc_val: msg.mot_2_enc_val,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mot_1_enc_val: msg.mot_1_enc_val,
      mot_2_enc_val: msg.mot_2_enc_val,
    }
  }
}


