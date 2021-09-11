mod ffi;
pub mod vjoy_base;
pub mod vjoy_extra;

mod test_env {
    #![allow(unused)]

    use super::vjoy_base::device::VJDevice;

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
    pub const TEST_DEVICE_1: VJDevice = VJDevice::D9; // Device of test #1
    pub const TEST_DEVICE_2: VJDevice = VJDevice::D10; // Device of test #2
    pub const TEST_DEVICE_INACTIVE: VJDevice = VJDevice::D16; // Device not activated

    pub const TEST_VERSION: u16 = 219;
    pub const TEST_PRODUCT: &str = "vJoy - Virtual Joystick";
    pub const TEST_MANUFACTURER: &str = "Shaul Eizikovich";
    pub const TEST_SERIAL_NUMBER: &str = "2.1.9";
}
