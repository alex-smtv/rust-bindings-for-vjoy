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
// pub const HID_USAGE_X: u32 = 48;
// pub const HID_USAGE_Y: u32 = 49;
// pub const HID_USAGE_Z: u32 = 50;
// pub const HID_USAGE_RX: u32 = 51;
// pub const HID_USAGE_RY: u32 = 52;
// pub const HID_USAGE_RZ: u32 = 53;
// pub const HID_USAGE_SL0: u32 = 54;
// pub const HID_USAGE_SL1: u32 = 55;
// pub const HID_USAGE_WHL: u32 = 56;
// pub const HID_USAGE_POV: u32 = 57;
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
    Holds data that describes a position of a vJoy device.\

    The data layout and data types follows what is set in the vJoy C API.
*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VJDPosition {
    /**
        This field holds the device id (1-based).\

        However, please note that the internal implementation of
        `UpdateVJD(UINT rID, PVOID pData)` in the C API will for some reason
        reassign this field to the value received in `rID`.
    */
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

    /// Slider 1
    pub wSlider: LONG,

    /// Slider 2
    pub wDial: LONG,

    pub wWheel: LONG,
    pub wAxisVX: LONG,
    pub wAxisVY: LONG,
    pub wAxisVZ: LONG,
    pub wAxisVBRX: LONG,
    pub wAxisVBRY: LONG,
    pub wAxisVBRZ: LONG,

    /**
        Buttons 1-32./

        0x00000001 means button1 is pressed, 0x80000000 -> button32 is pressed.
    */
    pub lButtons: ULONG,

    /// Lower 4 bits: HAT switch or 16-bit of continuous HAT switch.
    pub bHats: DWORD,

    /// Lower 4 bits: HAT switch or 16-bit of continuous HAT switch.
    pub bHatsEx1: DWORD,

    /// Lower 4 bits: HAT switch or 16-bit of continuous HAT switch.
    pub bHatsEx2: DWORD,

    /// Lower 4 bits: HAT switch or 16-bit of continuous HAT switch.
    pub bHatsEx3: DWORD,

    /// Buttons 33-64
    pub lButtonsEx1: ULONG,

    /// Buttons 65-96
    pub lButtonsEx2: ULONG,

    /// Buttons 97-128
    pub lButtonsEx3: ULONG,
}

impl VJDPosition {
    pub fn new(device_id: u8) -> VJDPosition {
        VJDPosition {
            bDevice: device_id,

            wThrottle: 0,
            wRudder: 0,
            wAileron: 0,

            wAxisX: 0,
            wAxisY: 0,
            wAxisZ: 0,

            wAxisXRot: 0,
            wAxisYRot: 0,
            wAxisZRot: 0,

            wSlider: 0,
            wDial: 0,

            wWheel: 0,
            wAxisVX: 0,
            wAxisVY: 0,
            wAxisVZ: 0,
            wAxisVBRX: 0,
            wAxisVBRY: 0,
            wAxisVBRZ: 0,

            // vJoy C API sets neutral to -1 in u32 type which wraps it to the max value by the 2's complement wrapping rule (overflow)
            bHats: u32::MAX,
            bHatsEx1: u32::MAX,
            bHatsEx2: u32::MAX,
            bHatsEx3: u32::MAX,

            lButtons: 0,
            lButtonsEx1: 0,
            lButtonsEx2: 0,
            lButtonsEx3: 0,
        }
    }
}

pub type JOYSTICK_POSITION_V2 = VJDPosition;

/// Describes the status of a vJoy device.
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

/// Describes an axis of a vJoy device.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VJoyAxis {
    X = 0x30,
    Y = 0x31,
    Z = 0x32,
    Rx = 0x33,
    Ry = 0x34,
    Rz = 0x35,
    Slider1 = 0x36,
    Slider2 = 0x37,
}

/// Describes a vJoy device (index is 1-based).
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VJoyDevice {
    D1 = 1,
    D2 = 2,
    D3 = 3,
    D4 = 4,
    D5 = 5,
    D6 = 6,
    D7 = 7,
    D8 = 8,
    D9 = 9,
    D10 = 10,
    D11 = 11,
    D12 = 12,
    D13 = 13,
    D14 = 14,
    D15 = 15,
    D16 = 16,
}

extern "C" {
    pub fn AcquireVJD(rID: VJoyDevice) -> BOOL;
    pub fn DriverMatch(DllVer: *mut WORD, DrvVer: *mut WORD) -> BOOL;
    pub fn GetNumberExistingVJD(n: *mut std::os::raw::c_int) -> BOOL;
    pub fn GetOwnerPid(rID: VJoyDevice) -> std::os::raw::c_int;
    pub fn GetVJDAxisExist(rID: VJoyDevice, Axis: VJoyAxis) -> BOOL;
    pub fn GetVJDAxisMax(rID: VJoyDevice, Axis: UINT, Max: *mut LONG) -> BOOL;
    pub fn GetVJDAxisMin(rID: VJoyDevice, Axis: UINT, Min: *mut LONG) -> BOOL;
    pub fn GetVJDButtonNumber(rID: VJoyDevice) -> std::os::raw::c_int;
    pub fn GetVJDContPovNumber(rID: VJoyDevice) -> std::os::raw::c_int;
    pub fn GetVJDDiscPovNumber(rID: VJoyDevice) -> std::os::raw::c_int;
    pub fn GetVJDStatus(rID: VJoyDevice) -> VJDStatus;
    pub fn GetvJoyManufacturerString() -> PVOID;
    pub fn GetvJoyMaxDevices(n: *mut std::os::raw::c_int) -> BOOL;
    pub fn GetvJoyProductString() -> PVOID;
    pub fn GetvJoySerialNumberString() -> PVOID;
    pub fn GetvJoyVersion() -> SHORT;
    pub fn isVJDExists(rID: VJoyDevice) -> BOOL;
    pub fn RelinquishVJD(rID: VJoyDevice);
    pub fn ResetAll();
    pub fn ResetButtons(rID: VJoyDevice) -> BOOL;
    pub fn ResetPovs(rID: VJoyDevice) -> BOOL;
    pub fn ResetVJD(rID: VJoyDevice) -> BOOL;
    pub fn SetAxis(Value: LONG, rID: VJoyDevice, Axis: VJoyAxis) -> BOOL;
    pub fn SetBtn(Value: BOOL, rID: VJoyDevice, nBtn: UCHAR) -> BOOL;
    pub fn SetContPov(Value: DWORD, rID: VJoyDevice, nPov: UCHAR) -> BOOL;
    pub fn SetDiscPov(Value: std::os::raw::c_int, rID: VJoyDevice, nPov: UCHAR) -> BOOL;
    pub fn UpdateVJD(rID: VJoyDevice, pData: *mut JOYSTICK_POSITION_V2) -> BOOL;
    pub fn vJoyEnabled() -> BOOL;
}
