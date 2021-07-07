//! Provides wrappers around vJoy public C API. Please note that the vJoy C API is not thread-safe.
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

pub mod device;
pub mod driver;
pub mod force_feedback;
