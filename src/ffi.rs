#![doc(hidden)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod force_feedback;

mod rusty_structure;
use rusty_structure::PositionV2;
pub use rusty_structure::{VJDAxis, VJDPosition, VJDPovDisc, VJDPovNumber, VJDStatus, VJDevice};

mod type_conversion;
use type_conversion::{BOOL, DWORD, LONG, PVOID, SHORT, WORD};

type RemovalCB = std::option::Option<unsafe extern "C" fn(arg1: BOOL, arg2: BOOL, arg3: PVOID)>;

// TODO: investigate if something interesting can be done with these REG/LOG values
// pub const REG_PARAM: &'static [u8; 50usize] =
//     b"SYSTEM\\CurrentControlSet\\services\\vjoy\\Parameters\0";
// pub const REG_PARAM_DEV0: &'static [u8; 58usize] =
//     b"SYSTEM\\CurrentControlSet\\services\\vjoy\\Parameters\\Device0\0";
// pub const REG_PARAM_DEV: &'static [u8; 57usize] =
//     b"SYSTEM\\CurrentControlSet\\services\\vjoy\\Parameters\\Device\0";
// pub const REG_DEVICE: &'static [u8; 7usize] = b"Device\0";
// pub const REG_INIT: &'static [u8; 5usize] = b"Init\0";
// pub const BTN_INIT: &'static [u8; 5usize] = b"BTNS\0";
// pub const INTERFACE_LOG_LEVEL: &'static [u8; 22usize] = b"VJOYINTERFACELOGLEVEL\0";
// pub const INTERFACE_LOG_FILE: &'static [u8; 21usize] = b"VJOYINTERFACELOGFILE\0";
// pub const INTERFACE_DEF_LOG_FILE: &'static [u8; 18usize] = b"vJoyInterface.log\0";

/**
    Describes a button state of a vJoy device.
*/
// We cannot use bool, so we simulate it
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VJDButtonState {
    Pressed = 1,
    Released = 0,
}

/**
    Describes a button of a vJoy device.
*/
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[rustfmt::skip]
pub enum VJDButton {
    // vJoy button starts at index 1 and not 0
    B1 = 1, B2, B3, B4, B5, B6, B7, B8, B9, B10,
    B11, B12, B13, B14, B15, B16, B17, B18, B19, B20,
    B21, B22, B23, B24, B25, B26, B27, B28, B29, B30,
    B31, B32, B33, B34, B35, B36, B37, B38, B39, B40,
    B41, B42, B43, B44, B45, B46, B47, B48, B49, B50,
    B51, B52, B53, B54, B55, B56, B57, B58, B59, B60,
    B61, B62, B63, B64, B65, B66, B67, B68, B69, B70,
    B71, B72, B73, B74, B75, B76, B77, B78, B79, B80,
    B81, B82, B83, B84, B85, B86, B87, B88, B89, B90,
    B91, B92, B93, B94, B95, B96, B97, B98, B99, B100,
    B101, B102, B103, B104, B105, B106, B107, B108, B109, B110,
    B111, B112, B113, B114, B115, B116, B117, B118, B119, B120,
    B121, B122, B123, B124, B125, B126, B127, B128,
}

extern "C" {
    pub fn AcquireVJD(rID: VJDevice) -> BOOL;
    pub fn DriverMatch(DllVer: *mut WORD, DrvVer: *mut WORD) -> BOOL;
    //pub fn GetNumberExistingVJD(n: *mut std::os::raw::c_int) -> BOOL;
    pub fn GetOwnerPid(rID: VJDevice) -> std::os::raw::c_int;
    pub fn GetVJDAxisExist(rID: VJDevice, Axis: VJDAxis) -> BOOL;
    // pub fn GetVJDAxisMax(rID: VJDevice, Axis: UINT, Max: *mut LONG) -> BOOL;
    // pub fn GetVJDAxisMin(rID: VJDevice, Axis: UINT, Min: *mut LONG) -> BOOL;
    pub fn GetVJDButtonNumber(rID: VJDevice) -> std::os::raw::c_int;
    pub fn GetVJDContPovNumber(rID: VJDevice) -> std::os::raw::c_int;
    pub fn GetVJDDiscPovNumber(rID: VJDevice) -> std::os::raw::c_int;
    pub fn GetVJDStatus(rID: VJDevice) -> VJDStatus;
    pub fn GetvJoyManufacturerString() -> PVOID;
    //pub fn GetvJoyMaxDevices(n: *mut std::os::raw::c_int) -> BOOL;
    pub fn GetvJoyProductString() -> PVOID;
    pub fn GetvJoySerialNumberString() -> PVOID;
    pub fn GetvJoyVersion() -> SHORT;
    pub fn isVJDExists(rID: VJDevice) -> BOOL;
    pub fn RegisterRemovalCB(cb: RemovalCB, data: PVOID);
    pub fn RelinquishVJD(rID: VJDevice);
    pub fn ResetAll();
    pub fn ResetButtons(rID: VJDevice) -> BOOL;
    pub fn ResetPovs(rID: VJDevice) -> BOOL;
    pub fn ResetVJD(rID: VJDevice) -> BOOL;
    pub fn SetAxis(Value: LONG, rID: VJDevice, Axis: VJDAxis) -> BOOL;
    pub fn SetBtn(Value: VJDButtonState, rID: VJDevice, nBtn: VJDButton) -> BOOL;
    pub fn SetContPov(Value: DWORD, rID: VJDevice, nPov: VJDPovNumber) -> BOOL;
    pub fn SetDiscPov(Value: VJDPovDisc, rID: VJDevice, nPov: VJDPovNumber) -> BOOL;
    pub fn UpdateVJD(rID: VJDevice, pData: *mut PositionV2) -> BOOL;
    pub fn vJoyEnabled() -> BOOL;
}
