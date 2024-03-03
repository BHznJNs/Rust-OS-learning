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

#[test_case]
fn test_println_simple() {
    GlobalWriter::init();
    println!("test_println_simple output");
}
#[test_case]
fn test_println_many() {
    GlobalWriter::init();
    for _ in 0..200 {
        println!("test_println_many output");
    }
}
#[test_case]
fn test_println_output() {
    use buffer::Buffer;

    GlobalWriter::init();

    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let ch = GlobalWriter::get_ch(Buffer::HEIGHT - 2, i);
        assert_eq!(char::from(ch.ascii_character), c);
    }
}
