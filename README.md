# vjoy-wrapper: Rust Wrapper for vJoy
A Rust wrapper around [vJoy](https://sourceforge.net/projects/vjoystick/) C API.

This project is still on the works.

## Who is this for?
This is meant to be used by Rust developers. It is not meant to be used by end-users ("customers") who have vJoy installed, unless they feel adventurous to code with Rust.

## Usage
Please refer to the documentation to understand how to use the project. Read below to access the documentation.

## Documentation
The project's documentation can be found at [https://alex-smtv.github.io/rust-vjoy-wrapper/vjoy_wrapper/index.html](https://alex-smtv.github.io/rust-vjoy-wrapper/vjoy_wrapper/index.html).

Additionally you can generate the documentation yourself with the command line `cargo doc --no-deps --open`.


## Releases information
Be mindful of [semver](https://semver.org/) which the project makes use of. Releases will have numbers as MAJOR.MINOR.PATCH starting from 1.0.0. Before release 1.0.0, the convention will be 0.X.Y where X means new functionalities is available and Y means things changed (e.g. fixes) but nothing new is available to use. As a final note, releases before 1.0.0 are considered unstable and backwards compatibility between releases is not guaranteed.

## Tests
Before tweaking and/or running tests, be mindful of [complementary  notes](./note_about_tests.md).

Also a special development setup is required with vJoy in order to perform tests correctly. Basically, some vJoy devices are reserved for testing purpose. The `test_env` module (found at [./src/lib.rs](./src/lib.rs)) will provide you further information of the needed setup.

## License
The project is released under the [MIT](./LICENSE.md) license.