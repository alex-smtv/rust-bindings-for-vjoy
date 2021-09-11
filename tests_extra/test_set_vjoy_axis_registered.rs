use vjoy_wrapper::test_env::TEST_DEVICE_1;
use vjoy_wrapper::vjoy_base::device::feeding::{VJDOwnership, VJDSeqFeed};
use vjoy_wrapper::vjoy_base::device::VJDAxis;
use vjoy_wrapper::vjoy_base::driver::VJGeneral;
use vjoy_wrapper::vjoy_extra::SDL2Helper;

fn main() {
    //test_conversion();
    test_bulk(false);
}

// Test vJoy axis values from vJoy MIN to MAX. For each value, we'll check with SDL2 if they are
// correctly registered.
fn test_bulk(debug_mode: bool) {
    VJDOwnership::acquire(TEST_DEVICE_1);

    let sdl_context = sdl2::init().unwrap();
    let joystick_subsystem = sdl_context.joystick().unwrap();

    let vjoys_read = SDL2Helper::get_vjoys(&joystick_subsystem).unwrap();
    let sdl2_vjoy = vjoys_read.get(&TEST_DEVICE_1).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Required initialization. 'Wake up' SDL2 to capture changes.
    VJDSeqFeed::set_axis(TEST_DEVICE_1, VJDAxis::X, VJGeneral::MAX_AXIS_VALUE);
    event_pump.pump_events();

    if debug_mode {
        println!("SDL axis read after set: {}", sdl2_vjoy.axis(0).unwrap());
    }

    for i in VJGeneral::MIN_AXIS_VALUE..=VJGeneral::MAX_AXIS_VALUE {
        VJDSeqFeed::set_axis(TEST_DEVICE_1, VJDAxis::X, i);

        // TODO: investigate for better handling.
        // Dirty necessary time gate. vJoy changes may not be seen by SDL in later checks if it
        // goes too fast.
        //
        // When vJoy modifies a joystick's position, the OS may put the operation in a pending
        // state and the original vJoy library then waits the write to complete (separate thread).
        // This is suspected to be the reason why SDL is sometimes not able to correctly read
        // changes when it's going too fast (when there's change A and B, when B occurs SDL still
        // reads value A instead of B). See 'BOOL Update(UINT rID)' function in vJoyInterface.cpp
        // (vJoy source code).
        //
        // Wait value set by empirical local tests, may be different in another machine.
        std::thread::sleep(std::time::Duration::from_nanos(10));
        event_pump.pump_events();

        let sdl2_axis_read = match sdl2_vjoy.axis(0) {
            Ok(val) => val,
            Err(_) => panic!("SDL2 could not read the axis 0."),
        };

        if debug_mode {
            println!(
                "i: {}, axis_read: {}, scaled: {}",
                i,
                sdl2_axis_read,
                SDL2Helper::axis_scale_sdl2_to_vjoy_unchecked(sdl2_axis_read)
            );
        }

        // Bit precision of SDL2(16bits) and vJoy(15bits) differs (so are the ranges), so scaling
        // must occurs. This implies that when we change an axis value of vJoy, SDL2 gives us a
        // scaled value when reading the axis (e.g. vJoy value of 16384 is 0 when reading with SDL2).
        //
        // However, for some reason SDL2 is scaling wrongly between read value of 1 included and
        // 28671 included. The scaled value is inappropriately rounded down when it should be
        // rounded up (e.g. setting a vjoy axis value to 28671 will provoke SDL2 to have a scaled
        // value of 24574 even though the correct computed value is around 24574.875, meaning SDL2
        // should read 24575).
        //
        // This anomaly doesn't occur in other ranges. For this reason, when we are in the anomaly
        // range we'll alter the assertion to make it pass in normal condition.
        if (0..28672).contains(&i) {
            assert_eq!(
                i,
                SDL2Helper::axis_scale_sdl2_to_vjoy_unchecked(sdl2_axis_read + 1)
            );
        } else {
            assert_eq!(
                i,
                SDL2Helper::axis_scale_sdl2_to_vjoy_unchecked(sdl2_axis_read)
            );
        }
    }

    VJDOwnership::relinquish(TEST_DEVICE_1);
}

#[allow(dead_code)]
fn test_conversion() {
    let sdl2 = 24577;

    println!(
        "sdl2: {}, scaled vjoy: {}",
        sdl2,
        SDL2Helper::axis_scale_sdl2_to_vjoy_unchecked(sdl2)
    );

    let vjoy = 28672;

    println!(
        "vjoy: {}, scaled sdl2: {}",
        vjoy,
        SDL2Helper::axis_scale_vjoy_to_sdl2_precise_unchecked(vjoy)
    );
}
