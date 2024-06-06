use crate::console;
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

// PL011 UART registers.
// Descriptions taken from "PrimeCell UART (PL011) Technical Reference Manual" r1p5.
register_bitfields! {
    u32,

    /// Flag Register.
    FR [
        TXFE OFFSET(7) NUMBITS(1) [],
        TXFF OFFSET(5) NUMBITS(1) [],
        RXFE OFFSET(4) NUMBITS(1) [],
        BUSY OFFSET(3) NUMBITS(1) []
    ],

    /// Integer Baud Rate Divisor.
    IBRD [
        BAUD_DIVINT OFFSET(0) NUMBITS(16) []
    ],

    /// Fractional Baud Rate Divisor.
    FBRD [
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) []
    ],

    /// Line Control Register.
    LCR_H [
        #[allow(clippy::enum_variant_names)]
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBit = 0b00,
            SixBit = 0b01,
            SevenBit = 0b10,
            EightBit = 0b11
        ],
        FEN  OFFSET(4) NUMBITS(1) [
            FifosDisabled = 0,
            FifosEnabled = 1
        ]
    ],

    /// Control Register.
    CR [
        RXE OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        TXE OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        UARTEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Interrupt FIFO Level Select Register.
    IFLS [
        RXIFLSEL OFFSET(3) NUMBITS(5) [
            OneEigth = 0b000,
            OneQuarter = 0b001,
            OneHalf = 0b010,
            ThreeQuarters = 0b011,
            SevenEights = 0b100
        ]
    ],

    /// Interrupt Mask Set/Clear Register.
    IMSC [
        RTIM OFFSET(6) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        RXIM OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Masked Interrupt Status Register.
    MIS [
        RTMIS OFFSET(6) NUMBITS(1) [],
        RXMIS OFFSET(4) NUMBITS(1) []
    ],

    /// Interrupt Clear Register.
    ICR [
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => DR: ReadWrite<u32>),
        (0x04 => _reserved1),
        (0x18 => FR: ReadOnly<u32, FR::Register>),
        (0x1c => _reserved2),
        (0x24 => IBRD: WriteOnly<u32, IBRD::Register>),
        (0x28 => FBRD: WriteOnly<u32, FBRD::Register>),
        (0x2c => LCR_H: WriteOnly<u32, LCR_H::Register>),
        (0x30 => CR: WriteOnly<u32, CR::Register>),
        (0x34 => IFLS: ReadWrite<u32, IFLS::Register>),
        (0x38 => IMSC: ReadWrite<u32, IMSC::Register>),
        (0x3C => _reserved3),
        (0x40 => MIS: ReadOnly<u32, MIS::Register>),
        (0x44 => ICR: WriteOnly<u32, ICR::Register>),
        (0x48 => @END),
    }
}

use core::{fmt, marker::PhantomData, ops};

pub struct MMIODerefWrapper<T> {
    start_addr: usize,
    phantom: PhantomData<fn() -> T>,
}

impl<T> MMIODerefWrapper<T> {
    /// Create an instance.
    pub const unsafe fn new(start_addr: usize) -> Self {
        Self {
            start_addr,
            phantom: PhantomData,
        }
    }
}

impl<T> ops::Deref for MMIODerefWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.start_addr as *const _) }
    }
}

type Registers = MMIODerefWrapper<RegisterBlock>;

pub struct PL011Uart {
    registers: Registers,
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
        self.flush();

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

    fn flush(&self) {
        while self.registers.FR.matches_all(FR::BUSY::SET) {
            unsafe { core::arch::asm!("nop", options(nomem, nostack)) }
        }
    }

    fn _write_char(&mut self, c: char) {
        self.flush();
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
        self.flush()
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
