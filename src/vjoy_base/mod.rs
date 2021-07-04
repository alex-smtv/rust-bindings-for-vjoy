//! Provides simple safe wrappers around vJoy public C API.

#![allow(dead_code)]

use crate::ffi::*;
pub use crate::ffi::{VJDPosition, VJDStatus};

/**
    Returns `true` if vJoy version 2.x is installed and enabled.
*/
pub fn vjoy_is_enabled() -> bool {
    unsafe { vJoyEnabled() }
}

/**
    Get the version number of the installed vJoy driver,
    or [`None`] if no vJoy 2.x is installed and enabled.
*/
pub fn vjoy_get_version() -> Option<u16> {
    if !vjoy_is_enabled() {
        return None;
    }

    // Important: GetvJoyVersion() to be used only after vJoyEnabled().
    let version = unsafe { GetvJoyVersion() };
    if version != 0 {
        Some(version as u16)
    } else {
        None
    }
}

/**
    Handle a special case in the vJoy C API where a string is constructed into a void pointer which holds a PWSTR (= wchar_t string pointer; each char is 16 bits on Windows)
*/
fn c_widestring_to_string(c_ptr: *mut std::os::raw::c_void) -> Option<String> {
    if c_ptr.is_null() {
        None
    } else {
        unsafe {
            use widestring::U16CString;

            Some(U16CString::from_ptr_str(c_ptr as *const _).to_string_lossy())
        }
    }
}

/**
    Get the Product String of the installed vJoy driver,
    or [`None`] if it fails (vJoy version 2.x is not installed and enabled).
*/
pub fn vjoy_get_product() -> Option<String> {
    if !vjoy_is_enabled() {
        return None;
    }

    // Important: to be used only after vJoyEnabled()
    c_widestring_to_string(unsafe { GetvJoyProductString() })
}

/**
    Get the Manufacturer String of the installed vJoy driver,
    or [`None`] if it fails (vJoy version 2.x is not installed and enabled).
*/
pub fn vjoy_get_manufacturer() -> Option<String> {
    if !vjoy_is_enabled() {
        return None;
    }

    // Important: to be used only after vJoyEnabled()
    c_widestring_to_string(unsafe { GetvJoyManufacturerString() })
}

/**
    Get the Serial Number String of the installed vJoy driver,
    or [`None`] if it fails (vJoy version 2.x is not installed and enabled).
*/
pub fn vjoy_get_serial_number() -> Option<String> {
    if !vjoy_is_enabled() {
        return None;
    }

    // Important: to be used only after vJoyEnabled()
    c_widestring_to_string(unsafe { GetvJoySerialNumberString() })
}

/**
    Returns the vJoy Driver version and vJoyInterface.dll file version, or [`None`]
    for either of one if the version cannot be determined.\

    The return format is a tuple: (driver_version, dll_version).
*/
pub fn vjoy_get_driver_dll_version() -> (Option<u16>, Option<u16>) {
    let mut dll_ver = 0;
    let mut driver_ver = 0;

    unsafe { DriverMatch(&mut dll_ver, &mut driver_ver) };

    dll_ver = format!("{:X}", dll_ver).parse::<u16>().unwrap();
    driver_ver = format!("{:X}", driver_ver).parse::<u16>().unwrap();

    let driver_ver = if driver_ver == 0 {
        Some(driver_ver)
    } else {
        None
    };

    let dll_ver = if dll_ver == 0 { Some(dll_ver) } else { None };

    (driver_ver, dll_ver)
}

/**
    Returns `true` if the vJoy Driver version matches the vJoyInterface.dll file version, `false` otherwise.\
    Use [`vjoy_get_driver_dll_version`] instead if the version numbers should be kept.
*/
fn vjoy_is_driver_match_dll() -> bool {
    unsafe { DriverMatch(std::ptr::null_mut(), std::ptr::null_mut()) }
}

fn vjoy_register_callback() {}

/**
    Returns the status of the specified device.\
    Use [`VJDStatus`] enum to determine the result.
*/
pub fn vjoy_get_status(device_id: u32) -> VJDStatus {
    unsafe { GetVJDStatus(device_id) }
}

/**
    Returns `true` if the specified device exists (configured and enabled).\

    Returns `false` otherwise (including the following cases: device does not exist, disabled, driver not installed).
*/
fn vjoy_is_exist(device_id: u32) -> bool {
    unsafe { isVJDExists(device_id) }
}

/**
    Used to describe a negative state of [`vjoy_get_owner_pid`].\

    - NoFileExist: Usually indicates a FREE device (no owner).
    - NoDevExist : Usually indicates a MISSING device.
    - BadDevStat : Indicates some internal problem.
*/
pub enum PIDFailed {
    NoFileExist,
    NoDevExist,
    BadDevStat,
    Unknown(String),
}

/**
    Returns the Process ID (PID) of the process that owns the specified device.\

    If the device is owned by a process, then the function returns a positive integer which is the PID of the owner.\

    Otherwise, the function returns an [`PIDFailed`] enum to describe the resulting state.
*/
fn vjoy_get_owner_pid(device_id: u32) -> Result<i32, PIDFailed> {
    let result = unsafe { GetOwnerPid(device_id) };

    if result >= 0 {
        Ok(result)
    } else {
        match result {
            -11 => Err(PIDFailed::BadDevStat),
            -12 => Err(PIDFailed::NoDevExist),
            -13 => Err(PIDFailed::NoFileExist),
            i => Err(PIDFailed::Unknown(format!(
                "Device {} not owned by a process and its state is unknown.",
                device_id,
            ))),
        }
    }
}

/**
    Acquire the specified device (id).\

    Only a device in state VJD_STAT_FREE can be acquired.
    If acquisition is successful the function returns TRUE and the device status becomes VJD_STAT_OWN.
*/
pub fn vjoy_acquire(device_id: u32) -> bool {
    unsafe { AcquireVJD(device_id) }
}

/**
    Relinquish the previously acquired specified device (id).\

    Use only when device is state VJD_STAT_OWN.
    State becomes VJD_STAT_FREE immediately after this function returns.
*/
pub fn vjoy_relinquish(device_id: u32) {
    unsafe { RelinquishVJD(device_id as u32) }
}

/**
    Update the position data of the specified device (id).
    Returns `true` if device updated.\

    Use only after device has been successfully acquired.
*/
pub fn vjoy_update_position(device_id: u32, position: &mut VJDPosition) -> bool {
    unsafe { UpdateVJD(device_id, position) }
}

fn vjoy_get_total_btns() {}
fn vjoy_get_total_dics_povs() {}
fn vjoy_get_total_cont_povs() {}
fn vjoy_is_axis_exist() {}

fn vjoy_reset() {}
fn vjoy_reset_all() {}
fn vjoy_reset_btns() {}
fn vjoy_reset_povs() {}
fn vjoy_set_axis() {}
fn vjoy_set_btn() {}
fn vjoy_set_disc_pov() {}
fn vjoy_set_cont_pov() {}
