mod color;
mod buffer;
mod writer;

pub use writer::GlobalWriter;

pub const _PRINT: for<'a> fn(core::fmt::Arguments<'a>) = GlobalWriter::print;
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_PRINT(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}