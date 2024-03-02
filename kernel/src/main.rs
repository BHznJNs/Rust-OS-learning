#![no_std]
#![no_main]
#![allow(clippy::needless_return)]

mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::GlobalWriter;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    GlobalWriter::init();
    println!("Hello World!");
    println!("A number of {}", 1234);
    loop {}
}
