#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main="test_main"]
#![allow(clippy::needless_return)]

mod vga_buffer;
mod serial;
mod test;
mod panic;
mod exit;

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
