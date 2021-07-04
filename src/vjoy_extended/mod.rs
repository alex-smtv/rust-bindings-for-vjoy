//! Provides additional more complex functionalities.

#![allow(dead_code)]

use crate::vjoy_base::*;

pub struct JoystickPositionRusty {
    position: VJDPosition,
}

use variant_count::VariantCount;

#[derive(VariantCount)]
enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
    Rx = 3,
    Ry = 4,
    Rz = 5,
    Sl0 = 6,
    Sl1 = 7,
}

struct AxesState {
    axes_val: [u16; Axis::VARIANT_COUNT],
    state_changed: [bool; Axis::VARIANT_COUNT],
}

impl AxesState {
    fn set(&mut self, axis: Axis, value: u16) {
        let index = axis as usize;

        self.axes_val[index] = value;
        self.state_changed[index] = true;
    }
}

impl JoystickPositionRusty {
    pub fn new(device_index: u8) -> JoystickPositionRusty {
        JoystickPositionRusty {
            position: VJDPosition {
                bDevice: device_index,

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

                // vjoy API sets neutral to -1 in u32 type which wraps it to the max value by the 2's complement wrapping rule (overflow)
                bHats: u32::MAX,
                bHatsEx1: u32::MAX,
                bHatsEx2: u32::MAX,
                bHatsEx3: u32::MAX,

                lButtons: 0,    // Buttons 1-32
                lButtonsEx1: 0, // Buttons 33-64
                lButtonsEx2: 0, // Buttons 65-96
                lButtonsEx3: 0, // Buttons 97-128
            },
        }
    }

    pub fn position_as_mut(&mut self) -> &mut VJDPosition {
        &mut self.position
    }

    pub fn set_axis_x(&mut self, value: i32) {
        self.position.wAxisX = value;
    }

    fn push_button(&mut self, button: u32) {}

    fn release_button() {}

    pub fn publish(&mut self) -> bool {
        vjoy_update_position(self.position.bDevice as u32, &mut self.position)
    }
}

// TODO: make it singleton, otherwise opening/closing devices from multiple instance is tricky
pub struct VJoyDeviceManager {
    driver_ver: u16,
    dll_ver: u16,
    opened_devices: Vec<u8>,
}

impl VJoyDeviceManager {
    pub fn new() -> Result<VJoyDeviceManager, &'static str> {
        if vjoy_is_enabled() {
            return Err("vJoy driver is not enabled.");
        }

        if vjoy_get_version().is_none() {
            return Err("Getting the version number of the installed vJoy driver failed.");
        }

        let (driver_ver, dll_ver) = vjoy_get_driver_dll_version();

        if driver_ver.is_none() {
            panic!("Could not determine the vJoy driver version. Is vJoy installed?");
        }

        if dll_ver.is_none() {
            panic!("Could not determine the vJoy DLL version. Is the vJoy interface DLL provided?");
        }

        let mut opened_devices: Vec<u8> = Vec::with_capacity(16);

        // TODO: use API to get all owned and already opened devices

        Ok(VJoyDeviceManager {
            driver_ver: driver_ver.unwrap(),
            dll_ver: dll_ver.unwrap(),
            opened_devices,
        })
    }

    pub fn open_device(&mut self, id: u8) -> Result<(), VJDStatus> {
        if !vjoy_acquire(id as u32) {
            Err(vjoy_get_status(id as u32))
        } else {
            self.opened_devices.insert(id as usize, id);
            Ok(())
        }
    }

    pub fn close_device(&mut self, id: u8) -> Result<(), &'static str> {
        if vjoy_get_status(id as u32) != VJDStatus::Own {
            Err("")
        } else {
            vjoy_relinquish(id as u32);
            self.opened_devices.remove(id as usize);
            Ok(())
        }
    }
}
