use super::registers::*;
use crate::driver;
use aarch64_cpu::asm;
use core::fmt;
use tock_registers::interfaces::{Readable, Writeable};

pub(super) struct PL011UartInner {
    pub registers: Registers,
    pub chars_written: usize,
    pub chars_read: usize,
}

impl PL011UartInner {
    pub(super) const fn new(base: u32) -> Self {
        Self {
            registers: unsafe { Registers::new(base as usize) },
            chars_written: 0,
            chars_read: 0,
        }
    }

    pub(super) fn flush(&self) {
        while self.registers.FR.matches_all(FR::BUSY::SET) {
            asm::nop();
        }
    }

    pub(super) fn write_char(&mut self, c: char) {
        self.flush();
        self.registers.DR.set(c as u32);
        self.chars_written += 1;
    }

    pub(super) fn read_char(&mut self, nonblocking: bool) -> Option<char> {
        if nonblocking && self.registers.FR.matches_all(FR::RXFE::SET) {
            return None;
        }

        while self.registers.FR.matches_all(FR::RXFE::SET) {
            asm::nop();
        }

        let mut c = self.registers.DR.get() as u8 as char;
        if c == '\r' {
            c = '\n';
        }

        self.chars_read += 1;

        Some(c)
    }

    pub(super) fn echo(&mut self) {
        while let Some(c) = self.read_char(true) {
            self.write_char(c)
        }
    }
}

impl driver::interface::DeviceDriver for PL011UartInner {
    fn init(&mut self) -> Result<(), &'static str> {
        self.flush();

        // Clear
        self.registers.CR.set(0);
        self.registers.ICR.write(ICR::ALL::CLEAR);

        // Set baud rate
        self.registers.IBRD.write(IBRD::BAUD_DIVINT.val(3));
        self.registers.FBRD.write(FBRD::BAUD_DIVFRAC.val(16));

        // Set Data Frame
        self.registers
            .LCR_H
            .write(LCR_H::WLEN::EightBit + LCR_H::FEN::FifosEnabled);

        // Set RX FIFO fill level at 1/8.
        self.registers.IFLS.write(IFLS::RXIFLSEL::OneEigth);

        // Enable RX IRQ + RX timeout IRQ.
        self.registers
            .IMSC
            .write(IMSC::RXIM::Enabled + IMSC::RTIM::Enabled);

        // Set Control Register
        // Enable UART, RX, TX
        self.registers
            .CR
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);

        Ok(())
    }
}

impl fmt::Write for PL011UartInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }

        Ok(())
    }
}
