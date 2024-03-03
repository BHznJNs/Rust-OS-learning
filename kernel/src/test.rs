use crate::serial_print;
use crate::serial_println;
use super::serial::SerialController;
use super::exit;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    SerialController::init();
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }
    exit::exit_qemu(exit::QemuExitCode::Success);
}