use super::color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: color::ColorCode,
}

impl ScreenChar {
    pub fn new(ch: u8, color_code: color::ColorCode) -> Self {
        Self {
            ascii_character: ch,
            color_code,
        }
    }
}

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[ScreenChar; Self::WIDTH]; Self::HEIGHT],
}

impl Buffer {
    pub const HEIGHT: usize = 25;
    pub const WIDTH: usize = 80;

    pub fn new() -> &'static mut Self {
        unsafe { &mut *(0xb8000 as *mut Self) }
    }

    pub fn get(&self, row: usize, col: usize) -> ScreenChar {
        let target_ptr = & self.chars[row][col] as *const ScreenChar;
        return unsafe { target_ptr.read_volatile() };
    }

    pub fn put(&mut self, ch: ScreenChar, row: usize, col: usize) {
        let target_ptr = &mut self.chars[row][col] as *mut ScreenChar;
        unsafe { target_ptr.write_volatile(ch) };
    }
}
