#![no_std]
#![no_main]
#![allow(clippy::needless_return)]

use learning_os::println;
use learning_os::hlt::hlt_loop;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    learning_os::init();

    println!("Hello World!");
    println!("Hello, {}!", 2024);

    #[cfg(test)]
    test_main();

    hlt_loop();
}
