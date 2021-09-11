use super::{
    type_conversion::{BYTE, DWORD, LONG, ULONG},
    VJDButtonState,
};

/**
    Holds data that describes a position of a vJoy device.

    The data layout and data types follows what is set in the vJoy C API.
*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PositionV2 {
    /**
        Device id (1-based).

        Note that the internal implementation of
        `UpdateVJD(UINT rID, PVOID pData)` in the C API will for some reason
        always reassign this field to the value received in `rID`, so any change we make won't be saved in practice...
    */
    bDevice: BYTE,

    wThrottle: LONG,
    wRudder: LONG,
    wAileron: LONG,

    /// Axis X
    wAxisX: LONG,

    /// Axis Y
    wAxisY: LONG,

    /// Axis Z
    wAxisZ: LONG,

    /// Axis X Rotation
    wAxisXRot: LONG,

    /// Axis Y Rotation
    wAxisYRot: LONG,

    /// Axis Z Rotation
    wAxisZRot: LONG,

    /// Slider 1
    wSlider: LONG,

    /// Slider 2
    wDial: LONG,

    wWheel: LONG,
    wAxisVX: LONG,
    wAxisVY: LONG,
    wAxisVZ: LONG,
    wAxisVBRX: LONG,
    wAxisVBRY: LONG,
    wAxisVBRZ: LONG,

    /**
        Buttons 1-32. Each bit position represents a button state. Ascending button number from right to left.

        0b00000000000000000000000000000001 (0x00000001) -> button1  is pressed.
        0b10000000000000000000000000000000 (0x80000000) -> button32 is pressed.
    */
    lButtons: ULONG,

    /**
        Continuous-type POV: lower 16 bits, POV #1

        Discrete-type POV: lower 16 bits is divided in 4 chunks of 4 bits each, one
        chunk of 4 bits represents one discrete-type POV. Ascending POV number from right to left:

        0000_0000_0000_0000\
        POV4_POV3_POV2_POV1
    */
    bHats: DWORD,

    /// Continuous-type POV: lower 16 bits, POV #2\
    /// Discrete-type POV: none
    bHatsEx1: DWORD,

    /// Continuous-type POV: lower 16 bits, POV #3\
    /// Discrete-type POV: none
    bHatsEx2: DWORD,

    /// Continuous-type POV: lower 16 bits, POV #4\
    /// Discrete-type POV: none
    bHatsEx3: DWORD,

    /**
        Buttons 33-64. Each bit position represents a button state. Ascending button number from right to left.

        0b00000000000000000000000000000001 (0x00000001) -> button 33 is pressed.
        0b10000000000000000000000000000000 (0x80000000) -> button 64 is pressed.
    */
    lButtonsEx1: ULONG,

    /**
        Buttons 65-96. Each bit position represents a button state. Ascending button number from right to left.

        0b00000000000000000000000000000001 (0x00000001) -> button 65 is pressed.
        0b10000000000000000000000000000000 (0x80000000) -> button 96 is pressed.
    */
    lButtonsEx2: ULONG,

    /**
        Buttons 97-128. Each bit position represents a button state. Ascending button number from right to left.

        0b00000000000000000000000000000001 (0x00000001) -> button 97  is pressed.
        0b10000000000000000000000000000000 (0x80000000) -> button 128 is pressed.
    */
    lButtonsEx3: ULONG,
}

/**
    Holds data that describes a position of a vJoy device. This is a container of information that won't
    do anything until it is send to vJoy.
*/
pub struct VJDPosition {
    device: VJDevice,
    position: PositionV2,
}

impl VJDPosition {
    pub fn new(device: VJDevice) -> VJDPosition {
        VJDPosition {
            device,
            position: PositionV2 {
                bDevice: device as u8,

                wThrottle: 0,
                wRudder: 0,
                wAileron: 0,

                wAxisX: 16384,
                wAxisY: 16384,
                wAxisZ: 16384,

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

                // vJoy C API sets neutral to -1 in u32 type which wraps it to the max value by the 2's complement wrapping rule (overflow)
                bHats: u32::MAX,
                bHatsEx1: u32::MAX,
                bHatsEx2: u32::MAX,
                bHatsEx3: u32::MAX,

                lButtons: 0,
                lButtonsEx1: 0,
                lButtonsEx2: 0,
                lButtonsEx3: 0,
            },
        }
    }

    // TODO: doc methods

    pub fn get_device(&self) -> VJDevice {
        self.device
    }

    pub fn get_position(&self) -> PositionV2 {
        self.position
    }

    pub fn set_button(&mut self, button: u32, state: VJDButtonState) {
        match state {
            VJDButtonState::Pressed => self.set_button_pressed(button),
            VJDButtonState::Released => self.set_button_released(button),
        }
    }

    pub fn set_button_pressed(&mut self, button: u32) {
        let mask: u32 = 0b1 << ((button - 1) % 32);

        match (button - 1) / 32 {
            0 => self.position.lButtons |= mask,
            1 => self.position.lButtonsEx1 |= mask,
            2 => self.position.lButtonsEx2 |= mask,
            3 => self.position.lButtonsEx3 |= mask,
            _ => (),
        }
    }

    pub fn set_button_released(&mut self, button: u32) {
        let mut mask: u32 = 0b1 << ((button - 1) % 32);

        mask = !mask;

        match (button - 1) / 32 {
            0 => self.position.lButtons &= mask,
            1 => self.position.lButtonsEx1 &= mask,
            2 => self.position.lButtonsEx2 &= mask,
            3 => self.position.lButtonsEx3 &= mask,
            _ => (),
        }
    }

    pub fn set_disc_pov(&mut self, pov: VJDPovNumber, direction: VJDPovDisc) {
        let shift = 4 * (pov as u32 - 1);

        // Keep all bits except for the POV number given in argument
        self.position.bHats &= !(0b1111 << shift);

        // Inject direction to the POV number given in argument
        self.position.bHats |= (direction as u32) << shift;
    }

    pub fn set_cont_pov(&mut self, pov: VJDPovNumber, value: u32) {
        match pov {
            VJDPovNumber::Pov1 => self.position.bHats = value,
            VJDPovNumber::Pov2 => self.position.bHatsEx1 = value,
            VJDPovNumber::Pov3 => self.position.bHatsEx2 = value,
            VJDPovNumber::Pov4 => self.position.bHatsEx3 = value,
        }
    }

    pub fn set_axis_x(&mut self, value: i32) {
        self.position.wAxisX = value;
    }

    pub fn set_axis_y(&mut self, value: i32) {
        self.position.wAxisY = value;
    }

    pub fn set_axis_z(&mut self, value: i32) {
        self.position.wAxisZ = value;
    }

    pub fn set_axis_xr(&mut self, value: i32) {
        self.position.wAxisXRot = value;
    }

    pub fn set_axis_yr(&mut self, value: i32) {
        self.position.wAxisYRot = value;
    }

    pub fn set_axis_zr(&mut self, value: i32) {
        self.position.wAxisZRot = value;
    }

    pub fn set_slider1(&mut self, value: i32) {
        self.position.wSlider = value;
    }

    pub fn set_slider2(&mut self, value: i32) {
        self.position.wDial = value;
    }
}

/// Describes the status of a vJoy device.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VJDStatus {
    /// The vJoy device is owned by this application.
    Own,

    /// The vJoy device is not owned by any application (including this one).
    Free,

    /// The vJoy device is owned by another application. It cannot be acquired by this
    /// application.
    Busy,

    /// The vJoy device is missing. It either does not exist or the driver is disabled.
    Miss,

    /// Unknown.
    Unknown,
}

/// Describes an axis of a vJoy device.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VJDAxis {
    X = 0x30,
    Y = 0x31,
    Z = 0x32,
    Rx = 0x33,
    Ry = 0x34,
    Rz = 0x35,
    Slider1 = 0x36,
    Slider2 = 0x37,
}

// TODO: test it contains device from range [1; MAX]; may need custom macro
/// Describes a vJoy device number ("id").
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VJDevice {
    // index is 1-based
    /// Device #1.
    D1 = 1,
    /// Device #2.
    D2 = 2,
    /// Device #3.
    D3 = 3,
    /// Device #4.
    D4 = 4,
    /// Device #5.
    D5 = 5,
    /// Device #6.
    D6 = 6,
    /// Device #7.
    D7 = 7,
    /// Device #8.
    D8 = 8,
    /// Device #9.
    D9 = 9,
    /// Device #10.
    D10 = 10,
    /// Device #11.
    D11 = 11,
    /// Device #12.
    D12 = 12,
    /// Device #13.
    D13 = 13,
    /// Device #14.
    D14 = 14,
    /// Device #15.
    D15 = 15,
    /// Device #16.
    D16 = 16,
}
// use std::hash::{Hash, Hasher};

// impl Hash for VJDevice {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         (*self as u8).hash(state);
//     }
// }

// impl PartialEq for VJDevice {
//     fn eq(&self, other: &VJDevice) -> bool {
//         *self as u8 == *other as u8
//     }
// }

impl VJDevice {
    pub fn get_from(value: u8) -> Option<VJDevice> {
        if !(1..=crate::vjoy_base::driver::VJGeneral::MAX_DEVICES).contains(&value) {
            return None;
        }

        match value {
            1 => Some(VJDevice::D1),
            2 => Some(VJDevice::D2),
            3 => Some(VJDevice::D3),
            4 => Some(VJDevice::D4),
            5 => Some(VJDevice::D5),
            6 => Some(VJDevice::D6),
            7 => Some(VJDevice::D7),
            8 => Some(VJDevice::D8),
            9 => Some(VJDevice::D9),
            10 => Some(VJDevice::D10),
            11 => Some(VJDevice::D11),
            12 => Some(VJDevice::D12),
            13 => Some(VJDevice::D13),
            14 => Some(VJDevice::D14),
            15 => Some(VJDevice::D15),
            16 => Some(VJDevice::D16),
            _ => None,
        }
    }
}

/**
    Describes a discrete POV direction of a vJoy device.
*/
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VJDPovDisc {
    Neutral = -1,
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

/**
    Describes a POV number ("id") of a vJoy device.
*/
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VJDPovNumber {
    /// POV #1.
    Pov1 = 1,
    /// POV #2.
    Pov2 = 2,
    /// POV #3.
    Pov3 = 3,
    /// POV #4.
    Pov4 = 4,
}
