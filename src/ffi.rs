#![doc(hidden)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod force_feedback;

mod rusty_structure;
pub use rusty_structure::{VJDAxis, VJDPosition, VJDPovDisc, VJDPovNumber, VJDStatus, VJDevice};

mod type_conversion;
use type_conversion::{BOOL, DWORD, LONG, PVOID, SHORT, UCHAR, UINT, WORD};

type JOYSTICK_POSITION_V2 = VJDPosition;
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

extern "C" {
    pub fn AcquireVJD(rID: VJDevice) -> BOOL;
    pub fn DriverMatch(DllVer: *mut WORD, DrvVer: *mut WORD) -> BOOL;
    pub fn GetNumberExistingVJD(n: *mut std::os::raw::c_int) -> BOOL;
    pub fn GetOwnerPid(rID: VJDevice) -> std::os::raw::c_int;
    pub fn GetVJDAxisExist(rID: VJDevice, Axis: VJDAxis) -> BOOL;
    pub fn GetVJDAxisMax(rID: VJDevice, Axis: UINT, Max: *mut LONG) -> BOOL;
    pub fn GetVJDAxisMin(rID: VJDevice, Axis: UINT, Min: *mut LONG) -> BOOL;
    pub fn GetVJDButtonNumber(rID: VJDevice) -> std::os::raw::c_int;
    pub fn GetVJDContPovNumber(rID: VJDevice) -> std::os::raw::c_int;
    pub fn GetVJDDiscPovNumber(rID: VJDevice) -> std::os::raw::c_int;
    pub fn GetVJDStatus(rID: VJDevice) -> VJDStatus;
    pub fn GetvJoyManufacturerString() -> PVOID;
    pub fn GetvJoyMaxDevices(n: *mut std::os::raw::c_int) -> BOOL;
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
    pub fn SetBtn(Value: BOOL, rID: VJDevice, nBtn: UCHAR) -> BOOL;
    pub fn SetContPov(Value: DWORD, rID: VJDevice, nPov: VJDPovNumber) -> BOOL;
    pub fn SetDiscPov(Value: VJDPovDisc, rID: VJDevice, nPov: VJDPovNumber) -> BOOL;
    pub fn UpdateVJD(rID: VJDevice, pData: *mut JOYSTICK_POSITION_V2) -> BOOL;
    pub fn vJoyEnabled() -> BOOL;
}
