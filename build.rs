extern crate bindgen;

use std::env;
use std::fs;
use std::path::PathBuf;

// static link still generates STATUS_DLL_NOT_FOUND:
// https://stackoverflow.com/questions/4074176/included-openssl-as-a-static-library-but-its-still-looking-for-a-dll
// .cargo/config -> https://www.reddit.com/r/rust/comments/7mif9i/how_to_compile_binaries_without_dependencies_on/
fn main() {
    let should_gen_bindings = false;

    let package_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut target_dir = package_dir.clone();
    target_dir.push("target");
    target_dir.push(env::var("PROFILE").unwrap());

    let vjoy_version = "2.1.8";
    let vjoy_lib_name = "vJoyInterface";

    let mut lib_dir = package_dir.clone();

    lib_dir.push("vjoy_libs");
    lib_dir.push(vjoy_version);
    lib_dir.push("lib");
    lib_dir.push("amd64");

    let mut headers_dir = package_dir;
    headers_dir.push("vjoy_libs");
    headers_dir.push(vjoy_version);
    headers_dir.push("headers");

    let mut bindgen_dir = headers_dir.clone();
    bindgen_dir.push("bindgen");

    let mut wrapper_file = bindgen_dir.clone();
    wrapper_file.push("wrapper");
    wrapper_file.set_extension("h");

    // Tell cargo to tell rustc to link the vJoyInterface library.
    println!("cargo:rustc-link-search={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static={}", vjoy_lib_name);

    // Re-run the build script when changes occur on these files
    println!("cargo:rerun-if-changed={}", lib_dir.display());
    println!("cargo:rerun-if-changed={}", headers_dir.display());
    println!("cargo:rerun-if-changed={}", bindgen_dir.display());

    if should_gen_bindings {
        // The bindgen::Builder is the main entry point
        // to bindgen, and lets you build up options for
        // the resulting bindings.
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header(wrapper_file.into_os_string().into_string().unwrap())
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("vjoy_bindings.rs"))
            .expect("Couldn't write bindings!");

        // To access generated file, use the following line in source code:
        // include!(concat!(env!("OUT_DIR"), "/vjoy_bindings.rs"))
    }

    // Copy DLL file
    let mut lib_path = lib_dir;
    lib_path.push(vjoy_lib_name);
    lib_path.set_extension("dll");

    let mut out_lib_path = target_dir;
    out_lib_path.push(vjoy_lib_name);
    out_lib_path.set_extension("dll");

    if let Err(e) = fs::copy(lib_path.clone(), out_lib_path.clone()) {
        panic!(
            "Could not copy the DLL file, error: {}\nLib path: {}\nOut path: {}\n",
            e,
            lib_path.display(),
            out_lib_path.display()
        );
    }
}
