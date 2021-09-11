//! Provides additional functionalities.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vjoy_base::driver::VJGeneral;

    #[test]
    fn sdl2_vjoys_correct() {}

    #[test]
    fn axis_scale_sdl2_to_vjoy_consistency() {
        // Strategy of the test:
        // SDL2 values are 16bit precision in i16, vJoy values are 15bit precision in u16.
        // There's twice more values in SDL2 than in vJoy.
        // This means a pair of values of SDL2 should be scaled to a similar vJoy value.
        //
        // E.g. -32,768 (sdl2) -> 0 (vjoy); -32,767 (sdl2) -> 0 (vjoy);
        // E.g. -32,766 (sdl2) -> 1 (vjoy); -32,765 (sdl2) -> 1 (vjoy);
        // Etc.
        //
        // The test will:
        //      - Create a Vec for both SDL2 and vJoy holding all possible values for each.
        //      - Iterate through SDL2 Vec, apply the scaling function and check the returned
        //        value match what it should be with the help of vJoy Vec.

        // Save how many possible values there should be for SDL2 and vJoy.
        let sdl2_values_len = i16::MAX as i32 + (-(i16::MIN as i32)) + 1;
        let vjoy_values_len = VJGeneral::MAX_AXIS_VALUE - VJGeneral::MIN_AXIS_VALUE + 1;

        // Create Vecs with exact capacities
        let mut sdl2_values: Vec<i16> = Vec::with_capacity(sdl2_values_len as usize);
        let mut vjoy_values: Vec<i32> = Vec::with_capacity(vjoy_values_len as usize);

        // Fill all SDL2 possible values
        for value in i16::MIN..=i16::MAX {
            sdl2_values.push(value);
        }

        // Check the vec has been correctly populated
        assert_eq!(sdl2_values_len, sdl2_values.len() as i32);

        // Fill all vJoy possible values
        for value in VJGeneral::MIN_AXIS_VALUE..=VJGeneral::MAX_AXIS_VALUE {
            vjoy_values.push(value);
        }

        // Check the vec has been correctly populated
        assert_eq!(vjoy_values_len as usize, vjoy_values.len());

        // Create counters that will help make comparison of a pair of SDL2 values with a vJoy value.
        // See comments at the beginning of the function for a detailed explanation.
        let mut sdl2_counter = 0;
        let mut vjoy_counter = 0;

        for sdl2_value in sdl2_values {
            assert_eq!(
                SDL2Helper::axis_scale_sdl2_to_vjoy_unchecked(sdl2_value),
                vjoy_values[vjoy_counter]
            );

            sdl2_counter += 1;

            if sdl2_counter % 2 == 0 {
                vjoy_counter += 1;
            }
        }
    }
}

use crate::vjoy_base::device::VJDevice;
use crate::vjoy_base::driver::VJGeneral;
use std::cmp::Ordering;
use winreg::enums::*;
use winreg::RegKey;

/// Describes an error state of [`get_vjoy_devices_reg`].
#[derive(Debug)]
pub enum VJDRegistryError {
    /// A device number was not in the range [1, [`VJGeneral::MAX_DEVICES`]]. This
    /// is a sign of malformed entries.
    InvalidDevice,

    /// A device number found in registry appeared in an unordered fashion. This is
    /// a sign of malformed entries.
    InvalidOrder,

    /// A device number was found at least twice in registry. This is a sign of
    /// malformed entries.
    DuplicateEntry,

    /// The access to the registry path where devices are registered has failed.
    /// [`std::io::Error`] is provided for further investigation.
    PathError(std::io::Error),
}

/**
    Returns a list of device numbers registered in the windows registry, or [`VJDRegistryError`] if it fails. The list is ordered.
*/
pub fn reg_vjoy_devices() -> Result<Vec<VJDevice>, VJDRegistryError> {
    let mut captured_device_numbers: Vec<VJDevice> = Vec::new();

    let devices_path =
        match RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(VJGeneral::REG_DEVICES_PATH) {
            Ok(reg_key) => reg_key,
            Err(error) => return Err(VJDRegistryError::PathError(error)),
        };

    let reg_device_prefix = VJGeneral::REG_DEVICE_PREFIX;

    for i in devices_path.enum_keys().map(|x| x.unwrap()) {
        // First check reg key starts with 'Device' with a potential following number.
        if i.starts_with(reg_device_prefix) && i.len() > reg_device_prefix.len() {
            // Then check the potential number is really a valid number.
            if let Ok(val) = i[reg_device_prefix.len()..].parse::<u8>() {
                // A device number should be in the correct range in registry
                if !(1..=VJGeneral::MAX_DEVICES).contains(&val) {
                    return Err(VJDRegistryError::InvalidDevice);
                } else {
                    for i in &captured_device_numbers {
                        match (*i as u8).cmp(&val) {
                            // A device number should not appear twice in registry
                            Ordering::Equal => return Err(VJDRegistryError::DuplicateEntry),

                            // A device number should not appear in unordered fashion in registry
                            Ordering::Greater => return Err(VJDRegistryError::InvalidOrder),
                            _ => {}
                        }
                    }
                }

                let device = match VJDevice::get_from(val) {
                    Some(vjd) => vjd,
                    None => return Err(VJDRegistryError::InvalidDevice),
                };

                captured_device_numbers.push(device);
            }
        };
    }

    Ok(captured_device_numbers)
}

use sdl2::{joystick::Joystick, IntegerOrSdlError, JoystickSubsystem};
use std::collections::HashMap;

/// Maps a vJoy device ID to a SDL2 joystick.
pub type SDL2Vjoys = HashMap<VJDevice, Joystick>;

/**
    Describes error from [`sdl2_get_vjoys`].
*/
#[derive(Debug)]
pub enum SDL2VjoyError {
    /// The number of vjoy joysticks found by SDL2 doesn't match the number of vjoy devices found in
    /// the windows registry.
    DeviceCountError,

    /// Registry error when trying to capture vjoy registered devices in the windows registry.
    RegError(VJDRegistryError),

    /// A given integer was so big that its representation as a C integer would be negative.
    IntegerOverflows(&'static str, u32),

    /// SDL2 error.
    SdlError(String),
}

/**
    Provides utilities to handle and recognize vJoy devices inside SDL2.
*/
pub struct SDL2Helper(());

impl SDL2Helper {
    #[allow(dead_code)]
    /// Describes the maximum value of a SDL2 axis.
    const MAX_AXIS_VALUE: i32 = 32767;

    #[allow(dead_code)]
    /// Describes the neutral value of a SDL2 axis.
    const NEUTRAL_AXIS_VALUE: i32 = 0;

    #[allow(dead_code)]
    /// Describes the minimum value of a SDL2 axis.
    const MIN_AXIS_VALUE: i32 = -32768;

    /**
        Returns a list of SDL2 devices which are recognized as vJoy devices, or [`SDL2VjoyError`] if it fails. The list is mapped as [`SDL2Vjoys`].
    */
    pub fn get_vjoys(joy_subsystem: &JoystickSubsystem) -> Result<SDL2Vjoys, SDL2VjoyError> {
        // Total number of joysticks found by SDL2
        let num_joys = match joy_subsystem.num_joysticks() {
            Ok(num) => num,
            Err(msg) => return Err(SDL2VjoyError::SdlError(msg)),
        };

        // Will hold vjoy joysticks found by SDL2, MUST be ordered by SDL2 device index
        let mut joys: Vec<Joystick> = Vec::with_capacity(num_joys as usize);

        // We'll cycle through all joysticks found by SDL2 to find vjoy joysticks
        for i in 0..num_joys {
            let joy_name = match joy_subsystem.name_for_index(i) {
                Ok(name) => name,
                Err(error) => match error {
                    IntegerOrSdlError::IntegerOverflows(msg, int) => {
                        return Err(SDL2VjoyError::IntegerOverflows(msg, int))
                    }
                    IntegerOrSdlError::SdlError(msg) => return Err(SDL2VjoyError::SdlError(msg)),
                },
            };

            if joy_name == "vJoy Device" {
                let joy = match joy_subsystem.open(i) {
                    Ok(joystick) => joystick,
                    Err(error) => match error {
                        IntegerOrSdlError::IntegerOverflows(msg, int) => {
                            return Err(SDL2VjoyError::IntegerOverflows(msg, int))
                        }
                        IntegerOrSdlError::SdlError(msg) => {
                            return Err(SDL2VjoyError::SdlError(msg))
                        }
                    },
                };

                joys.push(joy);
            }
        }

        // TODO: think of edge cases to use instance id for sorting...
        joys.sort_by_key(|joy| joy.instance_id());

        let reg_devices = match reg_vjoy_devices() {
            Ok(vjdevices) => vjdevices,
            Err(error) => return Err(SDL2VjoyError::RegError(error)),
        };

        if joys.len() != reg_devices.len() {
            return Err(SDL2VjoyError::DeviceCountError);
        }

        let mut vjoys: SDL2Vjoys = HashMap::new();

        // Remainder of the function goal: we have a list of vjoy joysticks found by SDL2, but we don't know
        // for which vjoy device each one corresponds to (all have the same name and GUID).
        // We want to map a SDL2 vjoy joystick to his corresponding vjoy device number.
        //
        // At this point: the number of vjoys joysticks ('joys') found by SDL2 is the same as the number
        // of vjoy devices ('reg_devices') found in the windows registry.
        //
        // 'joys' is ordered in ascending order (found vjoy #1, found vjoy #2, etc.).
        // 'reg_devices' is ordered by activated device number (device #1, device #3, device #4, etc.).
        // What is left to do is to make a direct connection between 'joys' and 'reg_devices'.
        for device in reg_devices {
            // joys.remove(0) is always valid because reg_devices and joys have the same size.
            vjoys.insert(device, joys.remove(0));
        }

        Ok(vjoys)
    }

    fn general_scaling(
        value: i16,
        source_rmin: i32,
        source_rmax: i32,
        target_rmin: i32,
        target_rmax: i32,
    ) -> i32 {
        (((value as i32 - source_rmin) * (target_rmax - target_rmin)) as f32
            / (source_rmax - source_rmin) as f32)
            .round() as i32
            + target_rmin
    }

    #[allow(dead_code)]
    // debug purpose atm; f32 precise enough
    fn precise_scaling(
        value: i16,
        source_rmin: i32,
        source_rmax: i32,
        target_rmin: i32,
        target_rmax: i32,
    ) -> f32 {
        (((value as i32 - source_rmin) * (target_rmax - target_rmin)) as f32
            / (source_rmax - source_rmin) as f32)
            + target_rmin as f32
    }

    /**
        Scales a SDL2 axis value to fit into a vJoy axis. The scaled value is rounded.
    */
    pub fn axis_scale_sdl2_to_vjoy_unchecked(value: i16) -> i32 {
        Self::general_scaling(
            value,
            Self::MIN_AXIS_VALUE,
            Self::MAX_AXIS_VALUE,
            VJGeneral::MIN_AXIS_VALUE,
            VJGeneral::MAX_AXIS_VALUE,
        )
    }

    /**
        Scales a vJoy axis value to fit into a SDL2 axis. The scaled value is of f32 precision.
    */
    pub fn axis_scale_vjoy_to_sdl2_precise_unchecked(value: i16) -> f32 {
        Self::precise_scaling(
            value,
            VJGeneral::MIN_AXIS_VALUE,
            VJGeneral::MAX_AXIS_VALUE,
            Self::MIN_AXIS_VALUE,
            Self::MAX_AXIS_VALUE,
        )
    }
}
