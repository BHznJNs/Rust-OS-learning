#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(learning_os::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use learning_os::panic;
use learning_os::vga_buffer::GlobalWriter;
use learning_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    GlobalWriter::init();
    println!("test_println output");
}
