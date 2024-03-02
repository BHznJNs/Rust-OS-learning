use core::panic::PanicInfo;
use super::GlobalWriter;
use super::exit::{exit_qemu, QemuExitCode};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::println;

    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::serial::SerialController;
    use crate::serial_print;
    use crate::serial_println;

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
