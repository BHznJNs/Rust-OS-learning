#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::needless_return)]

pub mod vga_buffer;
pub mod gdt;
pub mod interrupts;
pub mod hlt;
pub mod serial;
pub mod panic;
pub mod exit;
pub mod test;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    test_main();
    hlt::hlt_loop();
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}
