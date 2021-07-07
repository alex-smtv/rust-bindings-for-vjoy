//! Contains information regarding vJoy devices.

#[cfg(test)]
mod tests {
    use super::super::feeding::VJDOwnership;
    use super::*;
    use crate::test_env::{TEST_DEVICE_1, TEST_DEVICE_2, TEST_DEVICE_INACTIVE};
    use serial_test::serial;

    #[test]
    #[serial]
    fn check_device_exist() {
        assert!(VJDInfo::is_exist_device(TEST_DEVICE_1));
        assert!(VJDInfo::is_exist_device(TEST_DEVICE_2));
        assert!(!VJDInfo::is_exist_device(TEST_DEVICE_INACTIVE));
    }

    #[test]
    #[serial]
    fn axis_exist() {
        assert!(VJDInfo::is_exist_axis(TEST_DEVICE_1, VJDAxis::X));
        assert!(!VJDInfo::is_exist_axis(TEST_DEVICE_1, VJDAxis::Y));
        assert!(!VJDInfo::is_exist_axis(TEST_DEVICE_1, VJDAxis::Z));
        assert!(!VJDInfo::is_exist_axis(TEST_DEVICE_1, VJDAxis::Rx));
        assert!(VJDInfo::is_exist_axis(TEST_DEVICE_1, VJDAxis::Ry));
        assert!(!VJDInfo::is_exist_axis(TEST_DEVICE_1, VJDAxis::Rz));
        assert!(VJDInfo::is_exist_axis(TEST_DEVICE_1, VJDAxis::Slider1));
        assert!(!VJDInfo::is_exist_axis(TEST_DEVICE_1, VJDAxis::Slider2));

        assert!(VJDInfo::is_exist_axis(TEST_DEVICE_2, VJDAxis::X));
        assert!(VJDInfo::is_exist_axis(TEST_DEVICE_2, VJDAxis::Y));
        assert!(VJDInfo::is_exist_axis(TEST_DEVICE_2, VJDAxis::Z));
        assert!(VJDInfo::is_exist_axis(TEST_DEVICE_2, VJDAxis::Rx));
        assert!(VJDInfo::is_exist_axis(TEST_DEVICE_2, VJDAxis::Ry));
        assert!(VJDInfo::is_exist_axis(TEST_DEVICE_2, VJDAxis::Rz));
        assert!(VJDInfo::is_exist_axis(TEST_DEVICE_2, VJDAxis::Slider1));
        assert!(VJDInfo::is_exist_axis(TEST_DEVICE_2, VJDAxis::Slider2));

        assert!(!VJDInfo::is_exist_axis(TEST_DEVICE_INACTIVE, VJDAxis::X));
        assert!(!VJDInfo::is_exist_axis(TEST_DEVICE_INACTIVE, VJDAxis::Y));
        assert!(!VJDInfo::is_exist_axis(TEST_DEVICE_INACTIVE, VJDAxis::Z));
        assert!(!VJDInfo::is_exist_axis(TEST_DEVICE_INACTIVE, VJDAxis::Rx));
        assert!(!VJDInfo::is_exist_axis(TEST_DEVICE_INACTIVE, VJDAxis::Ry));
        assert!(!VJDInfo::is_exist_axis(TEST_DEVICE_INACTIVE, VJDAxis::Rz));
        assert!(!VJDInfo::is_exist_axis(
            TEST_DEVICE_INACTIVE,
            VJDAxis::Slider1
        ));
        assert!(!VJDInfo::is_exist_axis(
            TEST_DEVICE_INACTIVE,
            VJDAxis::Slider2
        ));
    }

    #[test]
    #[serial]
    fn status_free_when_launched() {
        assert_eq!(VJDStatus::Free, VJDInfo::get_status(TEST_DEVICE_1));
        assert_eq!(VJDStatus::Free, VJDInfo::get_status(TEST_DEVICE_2));
        assert_eq!(VJDStatus::Miss, VJDInfo::get_status(TEST_DEVICE_INACTIVE));
    }

    #[test]
    #[serial]
    fn pid_value_is_valid() {
        assert!(VJDOwnership::acquire(TEST_DEVICE_1));
        assert!(VJDOwnership::acquire(TEST_DEVICE_2));
        assert!(!VJDOwnership::acquire(TEST_DEVICE_INACTIVE));

        let vjoy_pid1 = VJDInfo::get_owner_pid(TEST_DEVICE_1);
        let vjoy_pid2 = VJDInfo::get_owner_pid(TEST_DEVICE_2);
        let vjoy_pid3 = VJDInfo::get_owner_pid(TEST_DEVICE_INACTIVE);

        assert!(vjoy_pid1.is_ok());
        assert!(vjoy_pid2.is_ok());
        assert!(vjoy_pid3.is_err());
        assert_eq!(std::process::id() as i32, vjoy_pid1.unwrap());
        assert_eq!(std::process::id() as i32, vjoy_pid2.unwrap());
        assert_eq!(Some(PIDFailed::NoDevExist), vjoy_pid3.err());

        assert!(VJDOwnership::relinquish(TEST_DEVICE_1));
        assert!(VJDOwnership::relinquish(TEST_DEVICE_2));
        assert!(!VJDOwnership::relinquish(TEST_DEVICE_INACTIVE));
    }

    #[test]
    #[serial]
    fn btns_detected() {
        assert!(VJDInfo::get_total_btns(TEST_DEVICE_1).is_ok());
        assert!(VJDInfo::get_total_btns(TEST_DEVICE_2).is_ok());
        assert!(VJDInfo::get_total_btns(TEST_DEVICE_INACTIVE).is_ok());
    }

    #[test]
    #[serial]
    fn disc_povs_detected() {
        assert!(VJDInfo::get_total_disc_povs(TEST_DEVICE_1).is_ok());
        assert!(VJDInfo::get_total_disc_povs(TEST_DEVICE_2).is_ok());
        assert!(VJDInfo::get_total_disc_povs(TEST_DEVICE_INACTIVE).is_ok());
    }

    #[test]
    #[serial]
    fn cont_povs_detected() {
        assert!(VJDInfo::get_total_cont_povs(TEST_DEVICE_1).is_ok());
        assert!(VJDInfo::get_total_cont_povs(TEST_DEVICE_2).is_ok());
        assert!(VJDInfo::get_total_cont_povs(TEST_DEVICE_INACTIVE).is_ok());
    }

    #[test]
    #[serial]
    fn total_buttons_valid() {
        assert_eq!(5, VJDInfo::get_total_btns(TEST_DEVICE_1).unwrap());
        assert_eq!(1, VJDInfo::get_total_btns(TEST_DEVICE_2).unwrap());
    }

    #[test]
    #[serial]
    fn total_disc_povs_valid() {
        assert_eq!(0, VJDInfo::get_total_disc_povs(TEST_DEVICE_1).unwrap());
        assert_eq!(1, VJDInfo::get_total_disc_povs(TEST_DEVICE_2).unwrap());
    }

    #[test]
    #[serial]
    fn total_cont_povs_valid() {
        assert_eq!(2, VJDInfo::get_total_cont_povs(TEST_DEVICE_1).unwrap());
        assert_eq!(0, VJDInfo::get_total_cont_povs(TEST_DEVICE_2).unwrap());
    }
}

use crate::ffi::*;

/**
    Describes a negative state of [`VJDInfo::get_owner_pid`].
*/
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PIDFailed {
    /// Usually indicates a free device (no owner).
    NoFileExist,

    /// Usually indicates a missing device.
    NoDevExist,

    /// Indicates some internal problem.
    BadDevStat,

    /// Unknown failure.
    Unknown,
}

/**
    Describes a negative state of [`VJDInfo::get_total_btns`].
*/
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TotalBtnsFailed {
    /// Failed to get a handle to a specified HID device index.
    NoHandleByIndex,

    /// Failed to get device's pre-parsed data.
    BadPreparsedData,

    /// Failed to get device's capabilities.
    NoCaps,

    /// Failed to get the "Number of Buttons" field in the device's
    /// capabilities structure.
    BadNBtnCaps,

    /// Failed to extract the "Button Capabilities" from the device's
    /// capabilities structure.
    BadBtnCaps,

    /// Failed to extract the "Button Range" from device's capabilities
    /// structure.
    BadBtnRange,

    /// Unknown failure.
    Unknown,
}

/**
    Describes an error state of [`VJDInfo::get_total_disc_povs`] or
    [`VJDInfo::get_total_cont_povs`].
*/
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TotalPOVFailed {
    /// Unknown failure.
    Unknown,
}

/**
    Holder of utility methods to retrieve information regarding vJoy devices.
*/
pub struct VJDInfo(());

impl VJDInfo {
    /**
        Returns `true` if the specified device exists (configured and enabled), `false` otherwise (including the following cases: device does not exist, disabled, driver not installed).
    */
    pub fn is_exist_device(device: VJDevice) -> bool {
        unsafe { isVJDExists(device) }
    }

    /**
        Returns `true` if the specified axis exists in the specified device, `false` otherwise.
    */
    pub fn is_exist_axis(device: VJDevice, axis: VJDAxis) -> bool {
        unsafe { GetVJDAxisExist(device, axis) }
    }

    /**
        Returns the status of the specified device as one variant of the [`VJDStatus`] enum.
    */
    pub fn get_status(device: VJDevice) -> VJDStatus {
        unsafe { GetVJDStatus(device) }
    }

    /**
        Returns the process id (PID) of the process that owns the specified device.

        If the device is owned by a process, then the function returns a positive integer which is the PID of the owner.

        Otherwise, the function returns one variant of the [`PIDFailed`] enum to describe
        the resulting negative state.
    */
    pub fn get_owner_pid(device: VJDevice) -> Result<i32, PIDFailed> {
        let result = unsafe { GetOwnerPid(device) };

        if result > 0 {
            Ok(result)
        } else {
            match result {
                -11 => Err(PIDFailed::BadDevStat),
                -12 => Err(PIDFailed::NoDevExist),
                -13 => Err(PIDFailed::NoFileExist),
                _ => Err(PIDFailed::Unknown), // should not happen, but make it exhaustive
            }
        }
    }

    /**
        On success, returns the number of buttons in the specified device. Valid values are 0 to 128.

        On failure, returns one variant of [`TotalBtnsFailed`] enum.
    */
    pub fn get_total_btns(device: VJDevice) -> Result<u8, TotalBtnsFailed> {
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
                _ => Err(TotalBtnsFailed::Unknown), // should not happen, but make it exhaustive
            }
        }
    }

    /**
        Returns the number of discrete-type POV hats in the specified device, or one
        variant of [`TotalPOVFailed`] if it fails.

        Valid number value is 0 to 4.
    */
    pub fn get_total_disc_povs(device: VJDevice) -> Result<u8, TotalPOVFailed> {
        let result = unsafe { GetVJDDiscPovNumber(device) };

        if result >= 0 {
            Ok(result as u8)
        } else {
            Err(TotalPOVFailed::Unknown)
        }
    }

    /**
        Returns the number of continuous-type POV hats in the specified device, or one
        variant of [`TotalPOVFailed`] if it fails.

        Valid number value is 0 to 4.
    */
    pub fn get_total_cont_povs(device: VJDevice) -> Result<u8, TotalPOVFailed> {
        let result = unsafe { GetVJDContPovNumber(device) };

        if result >= 0 {
            Ok(result as u8)
        } else {
            Err(TotalPOVFailed::Unknown)
        }
    }
}
