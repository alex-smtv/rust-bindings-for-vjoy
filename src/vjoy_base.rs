//! Provides simple safe wrappers around vJoy public C API. Remark: vJoy C API is not thread-safe!

#![allow(dead_code)]

#[cfg(test)]
mod tests {
    // Important: The vJoy C API is not thread-safe. This kind of error
    // can occur: https://vjoy.freeforums.net/thread/28/call-registerclassex-failed-disabled-device
    // Use `cargo test -- --test-threads 1` to run every test serially.
    // Another solution is to use `serial_test` crate (solution chosen here).

    use super::*;
    use serial_test::serial;

    // Set of devices used for tests. At least two must exist to test discrete
    // POVs and continoues POVs (both cannot reside in one device).
    // The set of tests will be based on this development environment:
    //  - Device of test #1:
    //      - Device id: 9 (editable)
    //      - Activated axes: X, Ry, Slider 1 (others are deactivated)
    //      - Number of buttons: 5
    //      - # of Disc POVs: 0
    //      - # of Cont POVs: 2
    //      - Activated force feedback: constant, ramp, square, sine, triangle,
    //        sawtooth up, sawtooth down, spring, damper, inertia, friction
    //
    //  - Device of test #2:
    //      - Device id: 10 (editable)
    //      - Activated axes: X, Y, Z, Rx, Ry, Rz, Slider 1, Slider 2
    //      - Number of buttons: 1
    //      - # of Disc POVs: 1
    //      - # of Cont POVs: 0
    //      - Activated force feedback: none, effects disabled
    const DEVICE_1: VJoyDevice = VJoyDevice::D9; // Device of test #1
    const DEVICE_2: VJoyDevice = VJoyDevice::D10; // Device of test #2

    #[test]
    #[serial]
    fn check_vjoy_enabled() {
        assert!(VJoy::is_enabled());
    }

    #[test]
    #[serial]
    fn check_device_exist() {
        assert!(VJoy::exist_device(DEVICE_1));
        assert!(VJoy::exist_device(DEVICE_2));
    }

    #[test]
    #[serial]
    fn status_free_when_launched() {
        assert_eq!(VJDStatus::Free, VJoy::get_status(DEVICE_1));
        assert_eq!(VJDStatus::Free, VJoy::get_status(DEVICE_2));
    }

    #[test]
    #[serial]
    fn driver_dll_detected() {
        let (driver, dll) = VJoy::get_driver_dll_version();
        assert!(driver.is_some());
        assert!(dll.is_some());
    }

    #[test]
    #[serial]
    fn driver_dll_match_consistency() {
        let (driver, dll) = VJoy::get_driver_dll_version();
        let is_match = driver.unwrap_or(0) == dll.unwrap_or(0);

        assert_eq!(is_match, VJoy::is_driver_match_dll());
    }

    #[test]
    #[serial]
    fn get_version_works() {
        assert!(VJoy::get_version().is_some());
    }

    #[test]
    #[serial]
    fn get_driver_get_version_match() {
        let (driver, _) = VJoy::get_driver_dll_version();
        assert!(driver.is_some());

        let driver_alternative = VJoy::get_version();
        assert!(driver_alternative.is_some());

        assert_eq!(driver.unwrap(), driver_alternative.unwrap());
    }

    #[test]
    #[serial]
    fn acquire_relinquish() {
        assert!(VJoy::acquire(DEVICE_1));
        assert!(VJoy::acquire(DEVICE_2));
        assert_eq!(VJDStatus::Own, VJoy::get_status(DEVICE_1));
        assert_eq!(VJDStatus::Own, VJoy::get_status(DEVICE_2));

        assert!(VJoy::relinquish(DEVICE_1));
        assert!(VJoy::relinquish(DEVICE_2));
        assert_eq!(VJDStatus::Free, VJoy::get_status(DEVICE_1));
        assert_eq!(VJDStatus::Free, VJoy::get_status(DEVICE_2));
    }

    #[test]
    #[serial]
    fn pid_match_vjoy_own() {
        VJoy::acquire(DEVICE_1);
        VJoy::acquire(DEVICE_2);

        let vjoy_pid1 = VJoy::get_owner_pid(DEVICE_1);
        let vjoy_pid2 = VJoy::get_owner_pid(DEVICE_2);

        assert!(vjoy_pid1.is_ok());
        assert!(vjoy_pid2.is_ok());
        assert_eq!(std::process::id() as i32, vjoy_pid1.unwrap());
        assert_eq!(std::process::id() as i32, vjoy_pid2.unwrap());

        VJoy::relinquish(DEVICE_1);
        VJoy::relinquish(DEVICE_2);
    }

    #[test]
    fn widestring_ptr_to_string_works() {
        // buf represents the string "Test"
        let buf = vec![84_u16, 101, 115, 116, 0];
        let raw_ptr = buf.as_ptr();

        let str = widestring_ptr_to_string(raw_ptr);

        assert_eq!("Test", str.unwrap());
    }

    #[test]
    #[serial]
    fn axis_exist() {
        assert!(VJoy::is_axis_exist(DEVICE_1, VJoyAxis::X));
        assert!(VJoy::is_axis_exist(DEVICE_1, VJoyAxis::Ry));
        assert!(VJoy::is_axis_exist(DEVICE_1, VJoyAxis::Slider1));

        assert!(VJoy::is_axis_exist(DEVICE_2, VJoyAxis::X));
        assert!(VJoy::is_axis_exist(DEVICE_2, VJoyAxis::Y));
        assert!(VJoy::is_axis_exist(DEVICE_2, VJoyAxis::Z));
        assert!(VJoy::is_axis_exist(DEVICE_2, VJoyAxis::Rx));
        assert!(VJoy::is_axis_exist(DEVICE_2, VJoyAxis::Ry));
        assert!(VJoy::is_axis_exist(DEVICE_2, VJoyAxis::Rz));
        assert!(VJoy::is_axis_exist(DEVICE_2, VJoyAxis::Slider1));
        assert!(VJoy::is_axis_exist(DEVICE_2, VJoyAxis::Slider2));
    }

    #[test]
    #[serial]
    fn total_buttons() {
        let result1 = VJoy::get_total_btns(DEVICE_1);
        let result2 = VJoy::get_total_btns(DEVICE_2);

        assert!(result1.is_ok());
        assert!(result2.is_ok());

        assert_eq!(5, result1.unwrap());
        assert_eq!(1, result2.unwrap());
    }

    #[test]
    #[serial]
    fn total_disc_povs() {
        assert_eq!(0, VJoy::get_total_disc_povs(DEVICE_1));
        assert_eq!(1, VJoy::get_total_disc_povs(DEVICE_2));
    }

    #[test]
    #[serial]
    fn total_cont_povs() {
        assert_eq!(2, VJoy::get_total_cont_povs(DEVICE_1));
        assert_eq!(0, VJoy::get_total_cont_povs(DEVICE_2));
    }

    #[test]
    #[serial]
    fn set_axis_checked_success() {
        VJoy::acquire(DEVICE_1);
        VJoy::acquire(DEVICE_2);

        //assert!(vjoy_set_axis(DEVICE_TEST_1, VJoyAxis::X, 0));
        //assert!(!vjoy_set_axis(DEVICE_TEST_1, VJoyAxis::Y, 16000));
        //assert!(vjoy_set_axis(DEVICE_TEST_1, VJoyAxis::Ry, 16000));
        //assert!(vjoy_set_axis(DEVICE_TEST_1, VJoyAxis::Slider1, 32000));

        VJoy::relinquish(DEVICE_1);
        VJoy::relinquish(DEVICE_2);
    }

    // #[test]
    // #[serial]
    // fn set_axis_unchecked_success() {
    //     vjoy_acquire(DEVICE_1);
    //     vjoy_acquire(DEVICE_2);

    //     vjoy_set_axis_unchecked(DEVICE_1, VJoyAxis::X, 0);
    //     vjoy_relinquish(DEVICE_1);
    //     vjoy_relinquish(DEVICE_2);
    // }
}

use crate::ffi::*;
pub use crate::ffi::{VJDPosition, VJDStatus, VJoyAxis, VJoyDevice};

#[derive(Debug)]
pub struct DeviceNotAcquired;

/**
    Describes a negative state of [`VJoy::get_owner_pid`].
*/
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    Describes a negative state of [`VJoy::get_total_btns`].
*/
#[repr(i32)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TotalBtnsFailed {
    NoHandleByIndex = -1,
    BadPreparsedData = -2,
    NoCaps = -3,
    BadNBtnCaps = -4,
    BadBtnCaps = -6,
    BadBtnRange = -7,
    Unknown = 0,
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
    Wraps operations on vJoy.\

    This struct is a zero-sized type that no one outside his module can construct.
    It is meant to wraps static accesses.
*/
pub struct VJoy(());

impl VJoy {
    /// Describes the maximum number of vJoy devices that can potentially exist.
    pub const MAX_DEVICES: u8 = 16;

    /**
        Returns `true` if vJoy version 2.x is installed and enabled.
    */
    pub fn is_enabled() -> bool {
        unsafe { vJoyEnabled() }
    }

    /**
        Get the version number of the installed vJoy driver,
        or [`None`] if no vJoy 2.x is installed and enabled.
    */
    pub fn get_version() -> Option<u16> {
        if !Self::is_enabled() {
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
        Get the Product [`String`] of the installed vJoy driver,
        or [`None`] if it fails (vJoy version 2.x is not installed and enabled).
    */
    pub fn get_product() -> Option<String> {
        if !Self::is_enabled() {
            return None;
        }

        // Important: to be used only after vJoyEnabled()
        widestring_ptr_to_string(unsafe { GetvJoyProductString() } as *const u16)
    }

    /**
        Get the Manufacturer [`String`] of the installed vJoy driver,
        or [`None`] if it fails (vJoy version 2.x is not installed and enabled).
    */
    pub fn get_manufacturer() -> Option<String> {
        if !Self::is_enabled() {
            return None;
        }

        // Important: to be used only after vJoyEnabled()
        widestring_ptr_to_string(unsafe { GetvJoyManufacturerString() as *const u16 })
    }

    /**
        Get the Serial Number [`String`] of the installed vJoy driver,
        or [`None`] if it fails (vJoy version 2.x is not installed and enabled).
    */
    pub fn get_serial_number() -> Option<String> {
        if !Self::is_enabled() {
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
    pub fn get_driver_dll_version() -> (Option<u16>, Option<u16>) {
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
        Use [`VJoy::get_driver_dll_version`] instead if the version numbers should be kept.
    */
    pub fn is_driver_match_dll() -> bool {
        unsafe { DriverMatch(std::ptr::null_mut(), std::ptr::null_mut()) }
    }

    fn register_callback() {}

    /**
        Returns the status of the specified device as a [`VJDStatus`] enum.
    */
    pub fn get_status(device: VJoyDevice) -> VJDStatus {
        unsafe { GetVJDStatus(device) }
    }

    /**
        Returns `true` if the specified device exists (configured and enabled).\

        Returns `false` otherwise (including the following cases: device does not exist, disabled, driver not installed).
    */
    pub fn exist_device(device: VJoyDevice) -> bool {
        unsafe { isVJDExists(device) }
    }

    /**
        Returns the Process ID (PID) of the process that owns the specified device.\

        If the device is owned by a process, then the function returns a positive integer which is the PID of the owner.\

        Otherwise, the function returns an [`PIDFailed`] enum to describe the resulting state.
    */
    pub fn get_owner_pid(device: VJoyDevice) -> Result<i32, PIDFailed> {
        let result = unsafe { GetOwnerPid(device) };

        if result >= 0 {
            Ok(result)
        } else {
            match result {
                -11 => Err(PIDFailed::BadDevStat),
                -12 => Err(PIDFailed::NoDevExist),
                -13 => Err(PIDFailed::NoFileExist),
                _ => Err(PIDFailed::Unknown(format!(
                    "Device {:?} not owned by a process and its state is unknown.",
                    device,
                ))),
            }
        }
    }

    /**
        Acquire the specified device.\

        Only a device in state [`VJDStatus::Free`] can be acquired.\
        If acquisition is successful the function returns `true` and the device status becomes [`VJDStatus::Own`].
    */
    pub fn acquire(device: VJoyDevice) -> bool {
        unsafe { AcquireVJD(device) }
    }

    /**
        Relinquish the previously acquired specified device.\

        Use only when device is in state [`VJDStatus::Own`].\
        State becomes [`VJDStatus::Free`] immediately after this function returns.\
        Returns `true` if relinquish is successful, or `false` if it fails.
    */
    pub fn relinquish(device: VJoyDevice) -> bool {
        if Self::get_status(device) != VJDStatus::Own {
            false
        } else {
            unsafe { RelinquishVJD(device) }
            Self::get_status(device) == VJDStatus::Free
        }
    }

    /**
        Update the position data of the specified device.\
        Returns `true` if the device is successfully updated, or `false` otherwise.\

        This is the checked version. It will make sure you acquired the specified device
        prior to calling this function and that the device id encoded
        in the specified position matches the device id provided to this function. For an unchecked version, use [`VJoy::update_position_unchecked`].
    */
    pub fn update_position_checked(
        device: VJoyDevice,
        position: &mut VJDPosition,
    ) -> Result<bool, DeviceNotAcquired> {
        // TODO: also check position.device_id match specified id
        if Self::get_status(device) != VJDStatus::Own {
            Err(DeviceNotAcquired)
        } else {
            Ok(unsafe { UpdateVJD(device, position) })
        }
    }
    // TODO: better use device_id + position, or just position and extract encoded device_id?
    /**
        Update the position data of the specified device.\
        Returns `true` if the device is successfully updated or `false` otherwise.\

        This is the unchecked version. You are responsible to make sure you acquired
        the specified device prior to calling this function and that the device id encoded
        in the specified position matches the device id provided to this function. For a checked version,
        use [`VJoy::update_position_checked`].
    */
    pub fn update_position_unchecked(device: VJoyDevice, position: &mut VJDPosition) -> bool {
        unsafe { UpdateVJD(device, position) }
    }

    /**
        On success, returns the number of buttons in the specified device. Valid values are 0 to 128. \

        On failure, returns one variant of [`TotalBtnsFailed`] enum.
    */
    pub fn get_total_btns(device: VJoyDevice) -> Result<u8, TotalBtnsFailed> {
        let result = unsafe { GetVJDButtonNumber(device) };

        if result >= 0 {
            Ok(result as u8)
        } else {
            match result {
                -1 => Err(TotalBtnsFailed::NoHandleByIndex),
                -2 => Err(TotalBtnsFailed::BadPreparsedData),
                -3 => Err(TotalBtnsFailed::NoCaps),
                -4 => Err(TotalBtnsFailed::BadNBtnCaps),
                -6 => Err(TotalBtnsFailed::BadBtnCaps),
                -7 => Err(TotalBtnsFailed::BadBtnRange),
                _ => Err(TotalBtnsFailed::Unknown),
            }
        }
    }

    /**
        Returns the number of discrete-type POV hats in the specified device.\

        Discrete-type POV Hat values may be North, East, South, West or neutral.
    */
    pub fn get_total_disc_povs(device: VJoyDevice) -> u8 {
        unsafe { GetVJDDiscPovNumber(device) as u8 }
    }

    /**
        Returns the number of continuous-type POV hats in the specified device.\

        Continuous-type POV Hat values may be 0 to 35900.
    */
    pub fn get_total_cont_povs(device: VJoyDevice) -> u8 {
        unsafe { GetVJDContPovNumber(device) as u8 }
    }

    /**
        Returns `true` if the specified axis exists in the specified device, `false` otherwise.
    */
    pub fn is_axis_exist(device: VJoyDevice, axis: VJoyAxis) -> bool {
        unsafe { GetVJDAxisExist(device, axis) }
    }

    fn reset() {}
    fn reset_all() {}
    fn reset_btns() {}
    fn reset_povs() {}

    /**
        Write a value for the given axis to the specified device. Value can be in the range of 0x1-0x8000.\

        This is the checked version. It will make sure you acquired the specified device
        prior to calling this function. For an unchecked version, use [`VJoy::set_axis_unchecked`].
    */
    pub fn set_axis_checked(
        device: VJoyDevice,
        axis: VJoyAxis,
        value: i32,
    ) -> Result<(), DeviceNotAcquired> {
        if Self::get_status(device) != VJDStatus::Own {
            Err(DeviceNotAcquired)
        } else {
            unsafe { SetAxis(value, device, axis) };
            Ok(())
        }
    }

    /**
        Write a value for the given axis to the specified device. Value can be in the range of 0x1-0x8000.\

        This is the unchecked version. You are responsible to make sure you acquired
        the specified device prior to calling this function. For a checked version,
        use [`VJoy::set_axis_checked`].
    */
    pub fn set_axis_unchecked(device: VJoyDevice, axis: VJoyAxis, value: i32) {
        unsafe { SetAxis(value, device, axis) };
    }

    fn set_btn() {}
    fn set_disc_pov() {}
    fn set_cont_pov() {}
}
