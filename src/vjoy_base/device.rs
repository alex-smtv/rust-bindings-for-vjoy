//! Contains logics to operate on vJoy devices.

pub use crate::ffi::{
    VJDAxis, VJDButton, VJDButtonState, VJDPosition, VJDPovDisc, VJDPovNumber, VJDStatus, VJDevice,
};

pub mod feeding;
pub mod info;
