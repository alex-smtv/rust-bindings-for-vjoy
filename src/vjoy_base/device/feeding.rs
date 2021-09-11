//! Contains logics to feed/update vJoy devices.

// TODO: review feedings into unused axes (C API will still accept such call with no error)

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

        assert!(VJDSeqFeed::set_axis(TEST_DEVICE_1, VJDAxis::X, 0));
        //assert!(!VJDSeqFeed::set_axis(TEST_DEVICE_1, VJDAxis::Y, 16000));
        assert!(VJDSeqFeed::set_axis(TEST_DEVICE_1, VJDAxis::Ry, 16000));
        assert!(VJDSeqFeed::set_axis(TEST_DEVICE_1, VJDAxis::Slider1, 32000));

        VJDOwnership::relinquish(TEST_DEVICE_1);
        VJDOwnership::relinquish(TEST_DEVICE_2);
    }
}

use super::info::VJDInfo;
use crate::{ffi::*, vjoy_base::driver::VJGeneral};

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
        Send the position data of the device encoded in [`VJDPosition`] to vJoy. Only a device in state [`VJDStatus::Own`] can have his position updated.

        Returns `true` if the operation succeeds, `false` otherwise.
    */
    pub fn send_position(position: &VJDPosition) -> bool {
        unsafe { UpdateVJD(position.get_device(), &mut position.get_position()) }
    }
}

/**
    Holder of utility methods to feed/update vJoy devices in a less efficient way by
    using sequential updates.

    Feeds vJoy devices sequentially. This is the less efficient way, each methods' call will in the internal implementation make a call to UpdateVJD(...).

    Another strategy exist with [`VJDPosFeed`] which will force you keep track of a [`VJDPosition`], but is more efficient. See [`VJDPosFeed`] for more details.
*/
pub struct VJDSeqFeed(());

impl VJDSeqFeed {
    /**
        Resets all the controls of the specified device to a set of values.\
        Returns `true` if the operation succeeds, `false` otherwise.

        These values are hard coded in the vJoy interface DLL and are currently set as follows:
        - Axes X, Y & Z: middle point.
        - All other axes: 0.
        - POV switches: neutral.
        - Buttons: not pressed.
    */
    pub fn reset(device: VJDevice) -> bool {
        // ResetVJD() is bugged... implement a custom code
        // unsafe { ResetVJD(device) }

        let was_owned = match VJDInfo::get_status(device) {
            VJDStatus::Own => true,
            VJDStatus::Free => false,
            _ => return false,
        };

        if !was_owned {
            VJDOwnership::acquire(device);
        }

        VJDPosFeed::send_position(&VJDPosition::new(device));

        if !was_owned {
            VJDOwnership::relinquish(device);
        }

        true
    }

    /**
        Resets all the controls of all devices to a set of values.

        Please note: vJoy public API doesn't tell us if it succeeds or not.
    */
    pub fn reset_all() {
        // ResetAll() is bugged... implement a custom code
        // unsafe { ResetAll() }

        // TODO: ?return bool with which devices failed (non blocking loop: if a device
        // fails, the following may succeed)
        for n in 1..=VJGeneral::MAX_DEVICES {
            VJDSeqFeed::reset(VJDevice::get_from(n).unwrap());
        }
    }

    /**
        Resets all buttons to a released state in the specified device.
    */
    pub fn reset_btns(device: VJDevice) {
        // return bool is not needed, internal implementation only return
        // false when providing a wrong device number, which is impossible
        // because we provide it by a controlled enum
        unsafe { ResetButtons(device) };
    }

    /**
        Resets all POV switches to a neutral state in the specified device.
    */
    pub fn reset_povs(device: VJDevice) {
        // return bool is not needed, internal implementation only return
        // false when providing a wrong device number, which is impossible
        // because we provide it by a controlled enum
        unsafe { ResetPovs(device) };
    }

    // TODO: check range 0x1-0x8000 for setaxis and update doc

    /**
        Write a value to the given axis of the specified device. Only a device in state [`VJDStatus::Own`] can have his axes altered.

        Returns `true` if the operation succeeds, `false` otherwise.

        Value can be in the range of 0 to 32767. Middle point is at 16384.
    */
    // Value range is annonced 1 to 32768 in the vJoy doc, but the reality
    // when tested is 0 to 32767. See this thread for more details:
    // https://vjoy.freeforums.net/thread/15/axis-value-range
    pub fn set_axis(device: VJDevice, axis: VJDAxis, value: i32) -> bool {
        unsafe { SetAxis(value, device, axis) }
    }

    /**
        Set a given button of the specified device pressed or released. Only a device in state [`VJDStatus::Own`] can have his buttons altered.

        Returns `true` if the operation succeeds, `false` otherwise.

        Button number can be in the range 1 to 128.
    */
    pub fn set_btn(device: VJDevice, button_number: VJDButton, state: VJDButtonState) -> bool {
        unsafe { SetBtn(state, device, button_number) }
    }

    /**
        Write a discrete direction to a given discrete POV of the specified device.

        Returns `true` if the operation succeeds, `false` otherwise.
    */
    pub fn set_disc_pov(
        device: VJDevice,
        pov_number: VJDPovNumber,
        disc_direction: VJDPovDisc,
    ) -> bool {
        unsafe { SetDiscPov(disc_direction, device, pov_number) }
    }

    /**
        Write a value to a given continuous POV of the specified device.

        Returns `true` if the operation succeeds, `false` otherwise.

        Value can be in the range 0 to 35999, neutral is [`u32::MAX`].\
        A value is measured in units of one-hundredth a degree.
    */
    pub fn set_cont_pov(device: VJDevice, pov_number: VJDPovNumber, value: u32) -> bool {
        unsafe { SetContPov(value, device, pov_number) }
    }
}
