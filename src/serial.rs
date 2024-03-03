use core::fmt::{self, Write};
use spin::Mutex;
use uart_16550::SerialPort;

static mut SERIAL: Option<Mutex<SerialPort>> = None;

pub struct SerialController;
impl SerialController {
    pub fn init() {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        unsafe { SERIAL = Some(Mutex::new(serial_port)) };
    }

    pub fn print(args: fmt::Arguments) {
        let serial = unsafe { SERIAL.as_ref().unwrap() };
        serial.lock().write_fmt(args).expect("Printing to serial failed");
    }
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        SerialController::print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => (serial_print!("\n"));
    ($fmt:expr) => (serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
