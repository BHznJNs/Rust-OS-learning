use core::fmt::{self, Write};
use spin::Mutex;
use super::color::{Color, ColorCode};
use super::buffer::{Buffer, ScreenChar};

struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.new_line();
            return;
        }

        if self.column_position >= Buffer::WIDTH {
            self.new_line();
        }

        let row = Buffer::HEIGHT - 1;
        let col = self.column_position;

        let color_code = self.color_code;
        let char = ScreenChar::new(byte, color_code);
        self.buffer.put(char, row, col);
        self.column_position += 1;
    }

    fn new_line(&mut self) {
        for row in 1..Buffer::HEIGHT {
            for col in 0..Buffer::WIDTH {
                let ch = self.buffer.get(row, col);
                self.buffer.put(ch, row - 1, col);
            }
        }
        self.clear_row(Buffer::HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank_ch = ScreenChar::new(b' ', self.color_code);
        for col in 0..Buffer::WIDTH {
            self.buffer.put(blank_ch, row, col);
        }
    } 
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        return Ok(());
    }
}


// --- --- --- --- --- ---

static mut GLOBAL_WRITER: Option<Mutex<Writer>> = None;
pub struct GlobalWriter;

impl GlobalWriter {
    pub fn init() {
        let writer_inst = Writer {
            column_position: 0,
            color_code: ColorCode::new(
                Color::Yellow,
                Color::Black,
            ),
            buffer: Buffer::new(),
        };
        unsafe { GLOBAL_WRITER = Some(Mutex::new(writer_inst)) }
    }

    pub fn print(args: fmt::Arguments) {
        let writer = unsafe {
            GLOBAL_WRITER.as_ref().unwrap()
        };
        writer.lock().write_fmt(args).expect("Unexpected IO error");
    }

    pub fn get_ch(row: usize, col: usize) -> ScreenChar {
        let writer = unsafe {
            GLOBAL_WRITER.as_ref().unwrap()
        };
        let ch = writer.lock().buffer.get(row, col);
        return ch;
    }
}
