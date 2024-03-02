mod color;
mod buffer;
mod writer;

pub use writer::GlobalWriter;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (GlobalWriter::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
