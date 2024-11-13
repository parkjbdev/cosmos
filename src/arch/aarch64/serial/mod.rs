mod registers;

use aarch64_cpu::asm;
use core::fmt;
use core::ptr::Unique;
use registers::*;
use spin::Mutex;
use tock_registers::interfaces::{Readable, Writeable};

use crate::driver;

const UART0: *mut u8 = 0x09000000 as *mut u8;
pub static CONSOLE: Mutex<Console> = Mutex::new(Console {
    target: unsafe { Unique::new_unchecked(UART0) },
});

const REGISTERS: Registers = unsafe { Registers::new(0x09000000) };

/// Abstracts around the [`PrimeCell UART (PL011)`][UART-Spec] and implements `fmt::Write`
/// [UART-Spec]: http://infocenter.arm.com/help/topic/com.arm.doc.ddi0183f/DDI0183.pdf
pub struct Console {
    target: Unique<u8>,
}

impl Console {
    pub fn write_byte(byte: u8) {
        Self::flush();
        REGISTERS.DR.set(byte as u32);
    }

    pub fn flush() {
        while REGISTERS.FR.matches_all(FR::BUSY::SET) {
            asm::nop();
        }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for byte in string.bytes() {
            // self.write_byte(byte)
            Console::write_byte(byte)
        }
        Ok(())
    }
}

impl driver::interface::DeviceDriver for Console {
    fn init(&mut self) -> Result<(), &'static str> {
        todo!()
    }
}
