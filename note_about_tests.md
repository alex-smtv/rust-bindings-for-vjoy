The vJoy C API is not thread-safe. This kind of error can occur: [https://vjoy.freeforums.net/thread/28/call-registerclassex-failed-disabled-device](https://vjoy.freeforums.net/thread/28/call-registerclassex-failed-disabled-device).

By default Rust runs tests in parallel, so to avoid problems with thread safety we have two options:
    1. Use `cargo test -- --test-threads 1` to run every test serially.
    2. Use `serial_test` crate and `[serial]` attribute.

The projects makes use of option 2.