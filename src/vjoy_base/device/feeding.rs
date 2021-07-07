//! Contains logics to feed/update vJoy devices.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_env::TEST_DEVICE_1;
    use crate::test_env::TEST_DEVICE_2;
    use serial_test::serial;

    #[test]
    #[serial]
    fn acquire_relinquish() {
        assert!(VJDOwnership::acquire(TEST_DEVICE_1));
        assert!(VJDOwnership::acquire(TEST_DEVICE_2));
        assert_eq!(VJDStatus::Own, VJDInfo::get_status(TEST_DEVICE_1));
        assert_eq!(VJDStatus::Own, VJDInfo::get_status(TEST_DEVICE_2));

        assert!(VJDOwnership::relinquish(TEST_DEVICE_1));
        assert!(VJDOwnership::relinquish(TEST_DEVICE_2));
        assert_eq!(VJDStatus::Free, VJDInfo::get_status(TEST_DEVICE_1));
        assert_eq!(VJDStatus::Free, VJDInfo::get_status(TEST_DEVICE_2));
    }

    #[test]
    #[serial]
    fn set_axis_checked_success() {
        VJDOwnership::acquire(TEST_DEVICE_1);
        VJDOwnership::acquire(TEST_DEVICE_2);

        //assert!(vjoy_set_axis(DEVICE_TEST_1, VJoyAxis::X, 0));
        //assert!(!vjoy_set_axis(DEVICE_TEST_1, VJoyAxis::Y, 16000));
        //assert!(vjoy_set_axis(DEVICE_TEST_1, VJoyAxis::Ry, 16000));
        //assert!(vjoy_set_axis(DEVICE_TEST_1, VJoyAxis::Slider1, 32000));

        VJDOwnership::relinquish(TEST_DEVICE_1);
        VJDOwnership::relinquish(TEST_DEVICE_2);
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

use super::info::VJDInfo;
use crate::ffi::*;

/**
    Holder of utility methods to manage devices acquisition and relinquishment.
*/
pub struct VJDOwnership(());

impl VJDOwnership {
    /**
        Acquire the specified device and change his state to [`VJDStatus::Own`].\
        Only a device in state [`VJDStatus::Free`] can be acquired.

        Returns `true` if acquisition was authorized, `false` otherwise.
    */
    pub fn acquire(device: VJDevice) -> bool {
        unsafe { AcquireVJD(device) }
    }

    /**
        Relinquish the previously acquired specified device and change his state to [`VJDStatus::Free`].\
        Only a device in state [`VJDStatus::Own`] can be relinquish.

        Returns `true` if relinquishment was authorized, `false` otherwise.
    */
    pub fn relinquish(device: VJDevice) -> bool {
        if VJDInfo::get_status(device) != VJDStatus::Own {
            false
        } else {
            unsafe { RelinquishVJD(device) }
            true
        }
    }
}

/**
    Holder of utility methods to feed/update vJoy devices in the most efficient way by
    using [`VJDPosition`].

    Feeds vJoy devices by directly providing [`VJDPosition`]s. This is the most efficient way, because you can make changes in batch to a position and then push the updated position at the proper time.

    Another strategy exist with [`VJDSeqFeed`] which won't force you keep track of a position, but is less efficient. See [`VJDSeqFeed`] for more details.
*/
pub struct VJDPosFeed(());

impl VJDPosFeed {
    /**
        Update the position data of the specified device.\
        Returns `true` if the device is successfully updated, or `false` otherwise.

        This is the checked version. It will make sure you acquired the specified device
        prior to calling this function and that the device id encoded
        in the specified position matches the device id provided to this function. For an unchecked version, use [`VJDPosFeed::update_position_unchecked`].
    */
    pub fn update_position_checked(device: VJDevice, position: &mut VJDPosition) -> bool {
        // TODO: also check position.device_id match specified id
        if VJDInfo::get_status(device) != VJDStatus::Own {
            false
        } else {
            unsafe { UpdateVJD(device, position) }
        }
    }
    // TODO: better use device_id + position, or just position and extract encoded device_id?
    /**
        Update the position data of the specified device.\
        Returns `true` if the device is successfully updated or `false` otherwise.

        This is the unchecked version. You are responsible to make sure you acquired
        the specified device prior to calling this function and that the device id encoded
        in the specified position matches the device id provided to this function. For a checked version,
        use [`VJDPosFeed::update_position_checked`].
    */
    pub fn update_position_unchecked(device: VJDevice, position: &mut VJDPosition) -> bool {
        unsafe { UpdateVJD(device, position) }
    }
}

/**
    Holder of utility methods to feed/update vJoy devices in a less efficient way by
    using sequential updates.

    Feeds vJoy devices sequentially. This is the less efficient way, each functions call will in the internal implementation make a call to UpdateVJD(...).

    Another strategy exist with [`VJDPosFeed`] which will force you keep track of a [`VJDPosition`], but is more efficient. See [`VJDPosFeed`] for more details.
*/
pub struct VJDSeqFeed(());

impl VJDSeqFeed {
    /**
        Resets all the controls of the specified device to a set of values. Returns
        `true` if operation succeed, `false` otherwise.

        These values are hard coded in the vJoy interface DLL and are currently set as follows:
        - Axes X, Y & Z: Middle point.
        - All other axes: 0.
        - POV Switches: Neutral (-1).
        - Buttons: Not Pressed (0).
    */
    pub fn reset(device: VJDevice) -> bool {
        unsafe { ResetVJD(device) }
    }

    /**
        Resets all the controls of the all devices to a set of values.

        See [`VJDSeqFeed::reset`] for details.
    */
    pub fn reset_all() {
        unsafe { ResetAll() }
    }

    /**
        Resets all buttons to an inactive state in the specified device.
    */
    pub fn reset_btns(device: VJDevice) {
        unsafe { ResetButtons(device) };
    }

    /**
        Resets all POV switches to a neutral state in the specified device.
    */
    pub fn reset_povs(device: VJDevice) {
        unsafe { ResetPovs(device) };
    }

    /**
        Write a value for the given axis to the specified device. Value can be in the range of 0x1-0x8000.

        This is the checked version. It will make sure you acquired the specified device prior to calling this function. For an unchecked version, use [`VJDSeqFeed::set_axis_unchecked`].
    */
    pub fn set_axis_checked(device: VJDevice, axis: VJDAxis, value: i32) -> bool {
        if VJDInfo::get_status(device) != VJDStatus::Own {
            false
        } else {
            unsafe { SetAxis(value, device, axis) }
        }
    }

    /**
        Write a value for the given axis to the specified device. Value can be in the range of 0x1-0x8000.

        This is the unchecked version. You are responsible to make sure you acquired
        the specified device prior to calling this function. For a checked version,
        use [`VJDSeqFeed::set_axis_checked`].
    */
    pub fn set_axis_unchecked(device: VJDevice, axis: VJDAxis, value: i32) {
        unsafe { SetAxis(value, device, axis) };
    }

    /**
        Write an activation state (`true` or `false`) in a given button to the specified device.\
        Returns `true` if the device is successfully updated or `false` otherwise.

        Button number can in the range 1-128.
    */
    pub fn set_btn(device: VJDevice, button_number: u8, is_activate: bool) -> bool {
        unsafe { SetBtn(is_activate, device, button_number) }
    }

    /**
        Write a discrete direction in a given discrete POV to the specified device.\
        Returns `true` if the device is successfully updated or `false` otherwise.\
    */
    pub fn set_disc_pov(device: VJDevice, pov_number: VJDPovNumber, value: VJDPovDisc) -> bool {
        unsafe { SetDiscPov(value, device, pov_number) }
    }

    /**
        Write a value in a given continuous POV to the specified device.\
        Returns `true` if the device is successfully updated or `false` otherwise.

        Value can be in the range: -1 to 35999./
        It is measured in units of one-hundredth a degree. -1 means neutral.
    */
    pub fn set_cont_pov(device: VJDevice, pov_number: VJDPovNumber, value: u32) -> bool {
        unsafe { SetContPov(value, device, pov_number) }
    }
}
