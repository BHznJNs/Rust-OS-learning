#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main="test_main"]
#![allow(clippy::needless_return)]

mod vga_buffer;
mod serial;
mod panic;
mod exit;

use serial::SerialController;
use vga_buffer::GlobalWriter;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    GlobalWriter::init();
    println!("Hello World!");
    println!("Hello, {}!", 2024);

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 2);
    serial_println!("[ok]");
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    SerialController::init();
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }
    exit::exit_qemu(exit::QemuExitCode::Success);
}
