use vjoy_wrapper::vjoy_base::driver::VJGeneral;

fn main() {
    print!("vJoy enabled     | ");
    print_unwrap(Some(VJGeneral::is_enabled()));

    print!("Product          | ");
    print_unwrap(VJGeneral::get_product());

    print!("Manufacturer     | ");
    print_unwrap(VJGeneral::get_manufacturer());

    print!("Serial number    | ");
    print_unwrap(VJGeneral::get_serial_number());

    print!("version          | ");
    print_unwrap(VJGeneral::get_version());

    print!("driver           | ");
    print_unwrap(VJGeneral::get_driver_dll_version().0);

    print!("dll              | ");
    print_unwrap(VJGeneral::get_driver_dll_version().1);

    print!("driver/dll match | ");
    print_unwrap(Some(VJGeneral::is_driver_match_dll()));
}

fn print_unwrap<T: std::fmt::Display>(val: Option<T>) {
    if let Some(val) = val {
        println!("{}", val);
    } else {
        println!("Unknown");
    }
}
