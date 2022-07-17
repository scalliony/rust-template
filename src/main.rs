#![no_main]
use scalliony_api::*;

/// Called at boot-time (optional)
#[no_mangle]
pub extern "C" fn _start() {
    io::log("booting...");
}

/// Called at each tick (required)
#[no_mangle]
pub extern "C" fn tick() {
    match sensors::contact() {
        Some(_) => motor::rotate_left(),
        None => {},
    }
    motor::go_forward(3);
}
