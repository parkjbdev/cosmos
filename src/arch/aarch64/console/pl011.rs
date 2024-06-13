use super::pl011_regs::*;
use crate::{console, interrupt};
use core::fmt;
use tock_registers::interfaces::{Readable, Writeable};

pub struct PL011Uart {
    pub registers: Registers,
    chars_written: usize,
    chars_read: usize,
}

impl PL011Uart {
    pub const fn new(base: usize) -> Self {
        Self {
            registers: unsafe { Registers::new(base) },
            chars_written: 0,
            chars_read: 0,
        }
    }

    pub fn init(&mut self) {
        self._flush();

        self.registers.CR.set(0);
        self.registers.ICR.write(ICR::ALL::CLEAR);
        self.registers.IBRD.write(IBRD::BAUD_DIVINT.val(3));
        self.registers.FBRD.write(FBRD::BAUD_DIVFRAC.val(16));
        self.registers
            .LCR_H
            .write(LCR_H::WLEN::EightBit + LCR_H::FEN::FifosEnabled);

        // Set RX FIFO fill level at 1/8.
        self.registers.IFLS.write(IFLS::RXIFLSEL::OneEigth);

        // Enable RX IRQ + RX timeout IRQ.
        self.registers
            .IMSC
            .write(IMSC::RXIM::Enabled + IMSC::RTIM::Enabled);

        self.registers
            .CR
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);
    }

    fn _flush(&self) {
        while self.registers.FR.matches_all(FR::BUSY::SET) {
            unsafe { core::arch::asm!("nop", options(nomem, nostack)) }
        }
    }

    fn _write_char(&mut self, c: char) {
        self._flush();
        self.registers.DR.set(c as u32);
        self.chars_written += 1;
    }

    fn _read_char(&mut self, blocking: bool) -> Option<char> {
        if self.registers.FR.matches_all(FR::RXFE::SET) {
            if blocking {
                return None;
            }
            while self.registers.FR.matches_all(FR::RXFE::SET) {
                unsafe { core::arch::asm!("nop", options(nomem, nostack)) }
            }
        }

        let mut c = self.registers.DR.get() as u8 as char;
        if c == '\r' {
            c = '\n';
        }

        self.chars_read += 1;

        Some(c)
    }
    pub fn echo(&mut self) {
        while let Some(c) = self._read_char(true) {
            self._write_char(c)
        }
    }
}

impl fmt::Write for PL011Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self._write_char(c);
        }

        Ok(())
    }
}

impl console::interface::Write for PL011Uart {
    fn write_fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result {
        fmt::Write::write_fmt(self, args)
    }

    fn write_char(&mut self, c: char) {
        self._write_char(c);
    }

    fn flush(&self) {
        self._flush()
    }
}

impl console::interface::Read for PL011Uart {
    fn read_char(&mut self) -> char {
        self._read_char(false).unwrap()
    }

    fn clear_rx(&mut self) {
        while self._read_char(true).is_some() {}
    }
}

impl interrupt::interface::IRQHandler for PL011Uart {
    fn handler(&mut self, cb: fn()) {
        let pending = self.registers.MIS.extract();
        self.registers.ICR.write(ICR::ALL::CLEAR);
        if pending.matches_any(MIS::RXMIS::SET + MIS::RTMIS::SET) {
            cb();
            // while let Some(c) = self._read_char(true) {
            //     self._write_char(c)
            // }
        }
    }
}
