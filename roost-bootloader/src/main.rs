// roost-bootloader/src/main.rs

#![no_std]
#![no_main]

extern crate roost_kernel;

/// Bare Metal Entry Point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Bootloader initialization logic
    // Kernel initialization call
    loop {}
}