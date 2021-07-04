#![doc(hidden)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod force_feedback;

pub type BOOL = bool;
pub type BYTE = std::os::raw::c_uchar;
pub type UCHAR = std::os::raw::c_uchar;
pub type SHORT = std::os::raw::c_short;
pub type WORD = std::os::raw::c_ushort;
pub type DWORD = std::os::raw::c_ulong;
pub type UINT = std::os::raw::c_uint;
pub type ULONG = std::os::raw::c_ulong;
pub type LONG = std::os::raw::c_long;
pub type PVOID = *mut std::os::raw::c_void;

// pub const DEVICENAME_STRING: &'static [u8; 5usize] = b"vJoy\0";
// pub const NTDEVICE_NAME_STRING: &'static [u8; 13usize] = b"\\Device\\vJoy\0";
// pub const SYMBOLIC_NAME_STRING: &'static [u8; 17usize] = b"\\DosDevices\\vJoy\0";
// pub const DOS_FILE_NAME: &'static [u8; 9usize] = b"\\\\.\\vJoy\0";
// pub const VJOY_INTERFACE: &'static [u8; 8usize] = b"Device_\0";
// pub const VER_X_: u32 = 0;
// pub const VER_H_: u32 = 2;
// pub const VER_M_: u32 = 1;
// pub const VER_L_: u32 = 8;
// pub const VENDOR_N_ID: u32 = 4660;
// pub const PRODUCT_N_ID: u32 = 48813;
// pub const VERSION_N: u32 = 536;
// pub const VENDOR_STR_ID: &'static [u8; 17usize] = b"Shaul Eizikovich\0";
// pub const PRODUCT_STR_ID: &'static [u8; 24usize] = b"vJoy - Virtual Joystick\0";
pub const MAX_N_DEVICES: u32 = 16;
pub const HID_USAGE_X: u32 = 48;
pub const HID_USAGE_Y: u32 = 49;
pub const HID_USAGE_Z: u32 = 50;
pub const HID_USAGE_RX: u32 = 51;
pub const HID_USAGE_RY: u32 = 52;
pub const HID_USAGE_RZ: u32 = 53;
pub const HID_USAGE_SL0: u32 = 54;
pub const HID_USAGE_SL1: u32 = 55;
pub const HID_USAGE_WHL: u32 = 56;
pub const HID_USAGE_POV: u32 = 57;
// pub const NO_HANDLE_BY_INDEX: i32 = -1;
// pub const BAD_PREPARSED_DATA: i32 = -2;
// pub const NO_CAPS: i32 = -3;
// pub const BAD_N_BTN_CAPS: i32 = -4;
// pub const BAD_CALLOC: i32 = -5;
// pub const BAD_BTN_CAPS: i32 = -6;
// pub const BAD_BTN_RANGE: i32 = -7;
// pub const BAD_N_VAL_CAPS: i32 = -8;
// pub const BAD_ID_RANGE: i32 = -9;
// pub const NO_SUCH_AXIS: i32 = -10;
// pub const BAD_DEV_STAT: i32 = -11;
// pub const NO_DEV_EXIST: i32 = -12;
// pub const NO_FILE_EXIST: i32 = -13;
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
    Holds data that describe a vJoy device position.
*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VJDPosition {
    pub bDevice: BYTE,
    pub wThrottle: LONG,
    pub wRudder: LONG,
    pub wAileron: LONG,
    pub wAxisX: LONG,
    pub wAxisY: LONG,
    pub wAxisZ: LONG,
    pub wAxisXRot: LONG,
    pub wAxisYRot: LONG,
    pub wAxisZRot: LONG,
    pub wSlider: LONG,
    pub wDial: LONG,
    pub wWheel: LONG,
    pub wAxisVX: LONG,
    pub wAxisVY: LONG,
    pub wAxisVZ: LONG,
    pub wAxisVBRX: LONG,
    pub wAxisVBRY: LONG,
    pub wAxisVBRZ: LONG,
    pub lButtons: ULONG,
    pub bHats: DWORD,
    pub bHatsEx1: DWORD,
    pub bHatsEx2: DWORD,
    pub bHatsEx3: DWORD,
    pub lButtonsEx1: ULONG,
    pub lButtonsEx2: ULONG,
    pub lButtonsEx3: ULONG,
}

pub type JOYSTICK_POSITION_V2 = VJDPosition;

/**
    Describes the status of a device.
*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VJDStatus {
    /// The vJoy Device is owned by this application.
    Own,

    /// The vJoy Device is NOT owned by any application  (including this one).
    Free,

    /// The vJoy Device is owned by another application. It cannot be acquired by this application.
    Busy,

    /// The vJoy Device is missing. It either does not   exist or the driver is disabled.
    Miss,

    /// Unknown.
    Unknown,
}

extern "C" {
    pub fn AcquireVJD(rID: UINT) -> BOOL;
    pub fn DriverMatch(DllVer: *mut WORD, DrvVer: *mut WORD) -> BOOL;
    pub fn GetNumberExistingVJD(n: *mut std::os::raw::c_int) -> BOOL;
    pub fn GetOwnerPid(rID: UINT) -> std::os::raw::c_int;
    pub fn GetVJDAxisExist(rID: UINT, Axis: UINT) -> BOOL;
    pub fn GetVJDAxisMax(rID: UINT, Axis: UINT, Max: *mut LONG) -> BOOL;
    pub fn GetVJDAxisMin(rID: UINT, Axis: UINT, Min: *mut LONG) -> BOOL;
    pub fn GetVJDButtonNumber(rID: UINT) -> std::os::raw::c_int;
    pub fn GetVJDContPovNumber(rID: UINT) -> std::os::raw::c_int;
    pub fn GetVJDDiscPovNumber(rID: UINT) -> std::os::raw::c_int;
    pub fn GetVJDStatus(rID: UINT) -> VJDStatus;
    pub fn GetvJoyManufacturerString() -> PVOID;
    pub fn GetvJoyMaxDevices(n: *mut std::os::raw::c_int) -> BOOL;
    pub fn GetvJoyProductString() -> PVOID;
    pub fn GetvJoySerialNumberString() -> PVOID;
    pub fn GetvJoyVersion() -> SHORT;
    pub fn isVJDExists(rID: UINT) -> BOOL;
    pub fn RelinquishVJD(rID: UINT);
    pub fn ResetAll();
    pub fn ResetButtons(rID: UINT) -> BOOL;
    pub fn ResetPovs(rID: UINT) -> BOOL;
    pub fn ResetVJD(rID: UINT) -> BOOL;
    pub fn SetAxis(Value: LONG, rID: UINT, Axis: UINT) -> BOOL;
    pub fn SetBtn(Value: BOOL, rID: UINT, nBtn: UCHAR) -> BOOL;
    pub fn SetContPov(Value: DWORD, rID: UINT, nPov: UCHAR) -> BOOL;
    pub fn SetDiscPov(Value: std::os::raw::c_int, rID: UINT, nPov: UCHAR) -> BOOL;
    pub fn UpdateVJD(rID: UINT, pData: *mut JOYSTICK_POSITION_V2) -> BOOL;
    pub fn vJoyEnabled() -> BOOL;
}
