//! Provides simple safe wrappers around vJoy public C API. Remark: vJoy C API is not thread-safe!

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
        Some(format!("{:X}", version).parse::<u16>().unwrap())
    } else {
        None
    }
}

/**
    Handle a special case in the vJoy C API where a string is constructed into a void pointer which holds a PWSTR (= wchar_t string pointer; each char is 16 bits on Windows: cf Unicode).\

    Return a [`String`], or [`None`] if the pointer is null.
*/
fn widestring_ptr_to_string(raw_ptr: *const u16) -> Option<String> {
    if raw_ptr.is_null() {
        None
    } else {
        unsafe {
            use widestring::U16CString;

            Some(U16CString::from_ptr_str(raw_ptr).to_string_lossy())
        }
    }
}

/**
    Get the Product [`String`] of the installed vJoy driver,
    or [`None`] if it fails (vJoy version 2.x is not installed and enabled).
*/
pub fn vjoy_get_product() -> Option<String> {
    if !vjoy_is_enabled() {
        return None;
    }

    // Important: to be used only after vJoyEnabled()
    widestring_ptr_to_string(unsafe { GetvJoyProductString() } as *const u16)
}

/**
    Get the Manufacturer [`String`] of the installed vJoy driver,
    or [`None`] if it fails (vJoy version 2.x is not installed and enabled).
*/
pub fn vjoy_get_manufacturer() -> Option<String> {
    if !vjoy_is_enabled() {
        return None;
    }

    // Important: to be used only after vJoyEnabled()
    widestring_ptr_to_string(unsafe { GetvJoyManufacturerString() as *const u16 })
}

/**
    Get the Serial Number [`String`] of the installed vJoy driver,
    or [`None`] if it fails (vJoy version 2.x is not installed and enabled).
*/
pub fn vjoy_get_serial_number() -> Option<String> {
    if !vjoy_is_enabled() {
        return None;
    }

    // Important: to be used only after vJoyEnabled()
    widestring_ptr_to_string(unsafe { GetvJoySerialNumberString() as *const u16 })
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
        None
    } else {
        Some(driver_ver)
    };

    let dll_ver = if dll_ver == 0 { None } else { Some(dll_ver) };

    (driver_ver, dll_ver)
}

/**
    Returns `true` if the vJoy Driver version matches the vJoyInterface.dll file version, or `false` if it fails.\
    Use [`vjoy_get_driver_dll_version`] instead if the version numbers should be kept.
*/
fn vjoy_is_driver_match_dll() -> bool {
    unsafe { DriverMatch(std::ptr::null_mut(), std::ptr::null_mut()) }
}

fn vjoy_register_callback() {}

/**
    Returns the status of the specified device as a [`VJDStatus`] enum.
*/
pub fn vjoy_get_status(device_id: u32) -> VJDStatus {
    unsafe { GetVJDStatus(device_id) }
}

/**
    Returns `true` if the specified device exists (configured and enabled).\

    Returns `false` otherwise (including the following cases: device does not exist, disabled, driver not installed).
*/
fn vjoy_exist_device(device_id: u32) -> bool {
    unsafe { isVJDExists(device_id) }
}

/**
    Describe a negative state of [`vjoy_get_owner_pid`].
*/
#[derive(Debug)]
pub enum PIDFailed {
    /// Usually indicates a FREE device (no owner).
    NoFileExist,

    /// Usually indicates a MISSING device.
    NoDevExist,

    /// Indicates some internal problem.
    BadDevStat,

    /// Unknown
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
            _ => Err(PIDFailed::Unknown(format!(
                "Device {} not owned by a process and its state is unknown.",
                device_id,
            ))),
        }
    }
}

/**
    Acquire the specified device.\

    Only a device in state [`VJDStatus::Free`] can be acquired.\
    If acquisition is successful the function returns `true` and the device status becomes [`VJDStatus::Own`].
*/
pub fn vjoy_acquire(device_id: u32) -> bool {
    unsafe { AcquireVJD(device_id) }
}

/**
    Relinquish the previously acquired specified device.\

    Use only when device is in state [`VJDStatus::Own`].\
    State becomes [`VJDStatus::Free`] immediately after this function returns.\
    Returns `true` if relinquish is successful, or `false` if it fails.
*/
pub fn vjoy_relinquish(device_id: u32) -> bool {
    if vjoy_get_status(device_id) != VJDStatus::Own {
        false
    } else {
    unsafe { RelinquishVJD(device_id as u32) }
        vjoy_get_status(device_id) == VJDStatus::Free
    }
}

/**
    Update the position data of the specified device.\
    Returns `true` if the device is updated or `false` otherwise.\

    This function is unchecked, meaning you must use it only after the device has been successfully acquired and this requirement is on your responsability.\

    A checked version exist at [`vjoy_update_position`].
*/
pub fn vjoy_update_position_unchecked(device_id: u32, position: &mut VJDPosition) -> bool {
    unsafe { UpdateVJD(device_id, position) }
}

/**
    Update the position data of the specified device.\
    Returns `true` if the device is updated, or `false` otherwise.\

    Should be used only after the device has been successfully acquired. The function will check
    for this requirement and return [`None`] if the device is not owned.
*/
pub fn vjoy_update_position(device_id: u32, position: &mut VJDPosition) -> Option<bool> {
    if vjoy_get_status(device_id) != VJDStatus::Own {
        None
    } else {
        Some(unsafe { UpdateVJD(device_id, position) })
    }
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
