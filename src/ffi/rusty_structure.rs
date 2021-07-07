use super::type_conversion::{BYTE, DWORD, LONG, ULONG};

/**
    Holds data that describes a position of a vJoy device.\

    The data layout and data types follows what is set in the vJoy C API.
*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VJDPosition {
    /**
        This field holds the device id (1-based).\

        However, please note that the internal implementation of
        `UpdateVJD(UINT rID, PVOID pData)` in the C API will for some reason
        reassign this field to the value received in `rID`.
    */
    pub bDevice: BYTE,

    pub wThrottle: LONG,
    pub wRudder: LONG,
    pub wAileron: LONG,

    pub wAxisX: LONG,
    pub wAxisY: LONG,
    pub wAxisZ: LONG,

    pub wAxisXRot: LONG,
    pub wAxisYRot: LONG,
    pub wAxisZRot: LONG,

    /// Slider 1
    pub wSlider: LONG,

    /// Slider 2
    pub wDial: LONG,

    pub wWheel: LONG,
    pub wAxisVX: LONG,
    pub wAxisVY: LONG,
    pub wAxisVZ: LONG,
    pub wAxisVBRX: LONG,
    pub wAxisVBRY: LONG,
    pub wAxisVBRZ: LONG,

    /**
        Buttons 1-32./

        0x00000001 means button1 is pressed, 0x80000000 -> button32 is pressed.
    */
    pub lButtons: ULONG,

    /// Lower 4 bits: HAT switch or 16-bit of continuous HAT switch.
    pub bHats: DWORD,

    /// Lower 4 bits: HAT switch or 16-bit of continuous HAT switch.
    pub bHatsEx1: DWORD,

    /// Lower 4 bits: HAT switch or 16-bit of continuous HAT switch.
    pub bHatsEx2: DWORD,

    /// Lower 4 bits: HAT switch or 16-bit of continuous HAT switch.
    pub bHatsEx3: DWORD,

    /// Buttons 33-64
    pub lButtonsEx1: ULONG,

    /// Buttons 65-96
    pub lButtonsEx2: ULONG,

    /// Buttons 97-128
    pub lButtonsEx3: ULONG,
}

impl VJDPosition {
    pub fn new(device_id: u8) -> VJDPosition {
        VJDPosition {
            bDevice: device_id,

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

            // vJoy C API sets neutral to -1 in u32 type which wraps it to the max value by the 2's complement wrapping rule (overflow)
            bHats: u32::MAX,
            bHatsEx1: u32::MAX,
            bHatsEx2: u32::MAX,
            bHatsEx3: u32::MAX,

            lButtons: 0,
            lButtonsEx1: 0,
            lButtonsEx2: 0,
            lButtonsEx3: 0,
        }
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

/// Describes a vJoy device.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VJDevice {
    // index is 1-based
    D1 = 1,
    D2 = 2,
    D3 = 3,
    D4 = 4,
    D5 = 5,
    D6 = 6,
    D7 = 7,
    D8 = 8,
    D9 = 9,
    D10 = 10,
    D11 = 11,
    D12 = 12,
    D13 = 13,
    D14 = 14,
    D15 = 15,
    D16 = 16,
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
    Describes a POV number of a vJoy device.
*/
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VJDPovNumber {
    Pov1 = 1,
    Pov2 = 2,
    Pov3 = 3,
    Pov4 = 4,
}
