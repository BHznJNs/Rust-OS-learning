#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(learning_os::test::test_runner)]
#![reexport_test_harness_main="test_main"]
#![allow(clippy::needless_return)]

extern crate learning_os;

use learning_os::vga_buffer::GlobalWriter;
use learning_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    GlobalWriter::init();
    println!("Hello World!");
    println!("Hello, {}!", 2024);

    #[cfg(test)]
    test_main();

    loop {}
}
