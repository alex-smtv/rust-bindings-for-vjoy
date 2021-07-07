//! Contains general data regarding the installed vJoy device driver.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_env::{TEST_MANUFACTURER, TEST_PRODUCT, TEST_SERIAL_NUMBER, TEST_VERSION};
    use serial_test::serial;

    #[test]
    fn widestring_ptr_to_string_works() {
        // buf represents the string "Test"
        let buf = vec![84_u16, 101, 115, 116, 0];
        let raw_ptr = buf.as_ptr();

        let result = widestring_ptr_to_string(raw_ptr);

        assert_eq!("Test", result.unwrap());
    }

    #[test]
    #[serial]
    fn check_vjoy_enabled() {
        assert!(VJGeneral::is_enabled());
    }

    #[test]
    #[serial]
    fn version_detected() {
        assert!(VJGeneral::get_version().is_some());
    }

    #[test]
    #[serial]
    fn product_detected() {
        assert!(VJGeneral::get_product().is_some());
    }

    #[test]
    #[serial]
    fn manufacturer_detected() {
        assert!(VJGeneral::get_manufacturer().is_some());
    }

    #[test]
    #[serial]
    fn serial_number_detected() {
        assert!(VJGeneral::get_serial_number().is_some());
    }

    #[test]
    #[serial]
    fn driver_dll_detected() {
        let (driver, dll) = VJGeneral::get_driver_dll_version();
        assert!(driver.is_some());
        assert!(dll.is_some());
    }

    #[test]
    #[serial]
    fn version_is_correct() {
        assert_eq!(TEST_VERSION, VJGeneral::get_version().unwrap());
    }

    #[test]
    #[serial]
    fn product_is_correct() {
        assert_eq!(TEST_PRODUCT, VJGeneral::get_product().unwrap());
    }

    #[test]
    #[serial]
    fn manufacturer_is_correct() {
        assert_eq!(TEST_MANUFACTURER, VJGeneral::get_manufacturer().unwrap());
    }

    #[test]
    #[serial]
    fn serial_number_is_correct() {
        assert_eq!(TEST_SERIAL_NUMBER, VJGeneral::get_serial_number().unwrap());
    }

    #[test]
    #[serial]
    fn driver_match_dll_consistency() {
        let (driver, dll) = VJGeneral::get_driver_dll_version();

        assert_eq!(
            driver.unwrap() == dll.unwrap(),
            VJGeneral::is_driver_match_dll()
        );
    }

    #[test]
    #[serial]
    fn driver_version_consistency() {
        let (driver, _) = VJGeneral::get_driver_dll_version();
        assert!(driver.is_some());

        let driver_alternative = VJGeneral::get_version();
        assert!(driver_alternative.is_some());

        assert_eq!(driver.unwrap(), driver_alternative.unwrap());
    }
}

use crate::ffi::*;

/**
    Handle the case in the vJoy C API where a string is constructed from a void pointer
    which is PWSTR (= 'wchar_t' = wide string pointer; each char is 16 bits on Windows:
    cf Unicode).

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
    Holder of utility methods to retrieve general data regarding the installed vJoy driver.
*/
pub struct VJGeneral(());

impl VJGeneral {
    /// Describes the maximum number of vJoy devices that can potentially exist.
    pub const MAX_DEVICES: u8 = 16;

    /**
        Returns `true` if vJoy version 2.x is installed and enabled, `false` otherwise.
    */
    pub fn is_enabled() -> bool {
        unsafe { vJoyEnabled() }
    }

    /**
        Returns the version number of the installed vJoy driver,
        or [`None`] if no vJoy 2.x is installed and enabled.
    */
    pub fn get_version() -> Option<u16> {
        if !Self::is_enabled() {
            return None;
        }

        // Important: GetvJoyVersion() to be used only after vJoyEnabled().
        let version = unsafe { GetvJoyVersion() };

        // 'version' is not a decimal representation, but a hexadecimal one
        // e.g. version 219 will be stored as 0x0219 in the vJoy C API
        // (which corresponds to 537 if we consider it as a decimal representation)
        // we convert 'version' back to a decimal representation
        let version = format!("{:X}", version).parse::<u16>().unwrap();

        if version != 0 {
            Some(version)
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
        Returns `true` if the vJoy driver version and the vJoyInterface.dll file version
        are identical, `false` otherwise.

        Use [`VJGeneral::get_driver_dll_version`] if the version numbers should be kept.
    */
    pub fn is_driver_match_dll() -> bool {
        unsafe { DriverMatch(std::ptr::null_mut(), std::ptr::null_mut()) }
    }

    /**
        Returns the vJoy Driver version and the vJoyInterface.dll file version, or [`None`]
        for either of one if the version cannot be determined.

        The return format is a tuple: (driver_version, dll_version).
    */
    pub fn get_driver_dll_version() -> (Option<u16>, Option<u16>) {
        let mut dll_ver = 0;
        let mut driver_ver = 0;

        unsafe { DriverMatch(&mut dll_ver, &mut driver_ver) };

        // 'dll_ver' and 'driver_ver' are not a decimal representation,
        // but a hexadecimal one
        // e.g. version 219 will be stored as 0x0219 in the vJoy C API
        // (which corresponds to 537 if we consider it as a decimal representation)
        // we convert 'dll_ver' and 'driver_ver' back to a decimal representation
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

    #[allow(unused)]
    // TODO: investigate
    fn register_callback() {}
}
