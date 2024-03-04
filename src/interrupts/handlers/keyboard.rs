use x86_64::structures::idt::InterruptStackFrame;
use x86_64::instructions::port::Port;
use lazy_static::lazy_static;
use pc_keyboard::{DecodedKey, layouts, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use crate::print;
use crate::serial_println;
use super::PICS;
use super::InterruptIndex;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore,
        ));
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    fn end_interrupt() {
        unsafe {
            PICS.lock()
                .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
        }
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    let Ok(Some(key_event)) = keyboard.add_byte(scancode) else {
        return end_interrupt();
    };
    let Some(key) = keyboard.process_keyevent(key_event) else {
        return end_interrupt();
    };
    match key {
        DecodedKey::Unicode(character) => print!("{}", character),
        DecodedKey::RawKey(key) => print!("{:?}", key),
    }

    end_interrupt();
}