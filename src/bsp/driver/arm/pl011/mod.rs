mod registers;

use crate::driver::interface::DeviceDriver;
use crate::{
    arch::irq::Interrupt,
    bsp::devicetree,
    driver,
    interrupt::{self, interface::IRQHandler},
};
use aarch64_cpu::asm;
use core::fmt;
use registers::*;
use spin::Mutex;
use tock_registers::interfaces::{Readable, Writeable};

pub static CONSOLE: Mutex<Option<PL011Uart>> = Mutex::new(None);

pub fn init(base: u32) {
    let mut uart = PL011Uart::new(base);
    uart.init();

    let mut console = CONSOLE.lock();
    *console = Some(uart);
}

pub fn init_irq() {
    // Interrupt
    let pl011_dt = devicetree::get_property("/pl011", "interrupts").unwrap();

    const SPLIT_SIZE: usize = core::mem::size_of::<u32>();
    let chunks: &[[u8; SPLIT_SIZE]] = unsafe { pl011_dt.as_chunks_unchecked() };

    Interrupt::from_raw(
        u32::from_be_bytes(chunks[0]),
        u32::from_be_bytes(chunks[1]),
        u32::from_be_bytes(chunks[2]),
        0x00,
        |state| {
            CONSOLE.lock().as_mut().unwrap().handler(|| {
                CONSOLE.lock().as_mut().unwrap().echo();
                // CONSOLE.lock().as_mut().unwrap().write_char('h');
            });
            true
        },
        "Keyboard Interrupt",
    )
    .register();
}

pub fn update_base_address(new_base: u32) {
    let mut console = CONSOLE.lock();
    if let Some(ref mut uart) = *console {
        uart.update_base(new_base); // Assuming a method to update the base address
    } else {
        *console = Some(PL011Uart::new(new_base));
    }
}

pub struct PL011Uart {
    pub registers: Registers,
}

impl PL011Uart {
    pub(super) const fn new(base: u32) -> Self {
        Self {
            registers: unsafe { Registers::new(base as usize) },
        }
    }

    pub fn update_base(&mut self, base: u32) {
        self.registers = unsafe { Registers::new(base as usize) };
    }

    pub(super) fn flush(&self) {
        while self.registers.FR.matches_all(FR::BUSY::SET) {
            asm::nop();
        }
    }

    pub(super) fn write_char(&mut self, c: char) {
        self.flush();
        self.registers.DR.set(c as u32);
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

        Some(c)
    }

    pub(super) fn echo(&mut self) {
        while let Some(c) = self.read_char(true) {
            self.write_char(c)
        }
    }

    pub fn clear_rx(&mut self) {
        println!("Clearing RX");
        while self.read_char(true).is_some() {}
    }
}

impl driver::interface::DeviceDriver for PL011Uart {
    fn init(&mut self) -> Result<(), &'static str> {
        // phys_println!("Initializing PL011 UART");
        self.flush();

        // Clear
        // phys_println!("Clearing");
        self.registers.CR.set(0);
        self.registers.ICR.write(ICR::ALL::CLEAR);

        // Set baud rate
        // phys_println!("Setting Baud Rate");
        self.registers.IBRD.write(IBRD::BAUD_DIVINT.val(3));
        self.registers.FBRD.write(FBRD::BAUD_DIVFRAC.val(16));

        // Set Data Frame
        // phys_println!("Setting Data Frame");
        self.registers
            .LCR_H
            .write(LCR_H::WLEN::EightBit + LCR_H::FEN::FifosEnabled);

        // Set RX FIFO fill level at 1/8.
        // phys_println!("Setting RX FIFO Level");
        self.registers.IFLS.write(IFLS::RXIFLSEL::OneEigth);

        // Enable RX IRQ + RX timeout IRQ.
        // phys_println!("Enabling RX IRQ");
        self.registers
            .IMSC
            .write(IMSC::RXIM::Enabled + IMSC::RTIM::Enabled);

        // Set Control Register
        // Enable UART, RX, TX
        // phys_println!("Enabling UART");
        self.registers
            .CR
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);

        phys_println!("PL011 UART Initialized");

        Ok(())
    }
}

impl fmt::Write for PL011Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }

        Ok(())
    }
}

impl interrupt::interface::IRQHandler for PL011Uart {
    fn handler(&mut self, cb: fn()) {
        let pending = self.registers.MIS.extract();
        self.registers.ICR.write(ICR::ALL::CLEAR);
        if pending.matches_any(MIS::RXMIS::SET + MIS::RTMIS::SET) {
            println!("Interrupt from PL011 UART");
            cb();
        }
    }
}
