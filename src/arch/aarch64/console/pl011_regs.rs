use core::{marker::PhantomData, ops};
use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

// PL011 UART registers.
// Descriptions taken from "PrimeCell UART (PL011) Technical Reference Manual" r1p5.
register_bitfields! {
    u32,

    /// Flag Register.
    pub FR [
        TXFE OFFSET(7) NUMBITS(1) [],
        TXFF OFFSET(5) NUMBITS(1) [],
        RXFE OFFSET(4) NUMBITS(1) [],
        BUSY OFFSET(3) NUMBITS(1) []
    ],

    /// Integer Baud Rate Divisor.
    pub IBRD [
        BAUD_DIVINT OFFSET(0) NUMBITS(16) []
    ],

    /// Fractional Baud Rate Divisor.
    pub FBRD [
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) []
    ],

    /// Line Control Register.
    pub LCR_H [
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
    pub CR [
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
    pub IFLS [
        RXIFLSEL OFFSET(3) NUMBITS(5) [
            OneEigth = 0b000,
            OneQuarter = 0b001,
            OneHalf = 0b010,
            ThreeQuarters = 0b011,
            SevenEights = 0b100
        ]
    ],

    /// Interrupt Mask Set/Clear Register.
    pub IMSC [
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
    pub MIS [
        RTMIS OFFSET(6) NUMBITS(1) [],
        RXMIS OFFSET(4) NUMBITS(1) []
    ],

    /// Interrupt Clear Register.
    pub ICR [
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => pub DR: ReadWrite<u32>),
        (0x04 => _reserved1),
        (0x18 => pub FR: ReadOnly<u32, FR::Register>),
        (0x1c => _reserved2),
        (0x24 => pub IBRD: WriteOnly<u32, IBRD::Register>),
        (0x28 => pub FBRD: WriteOnly<u32, FBRD::Register>),
        (0x2c => pub LCR_H: WriteOnly<u32, LCR_H::Register>),
        (0x30 => pub CR: WriteOnly<u32, CR::Register>),
        (0x34 => pub IFLS: ReadWrite<u32, IFLS::Register>),
        (0x38 => pub IMSC: ReadWrite<u32, IMSC::Register>),
        (0x3C => _reserved3),
        (0x40 => pub MIS: ReadOnly<u32, MIS::Register>),
        (0x44 => pub ICR: WriteOnly<u32, ICR::Register>),
        (0x48 => @END),
    }
}

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

pub type Registers = MMIODerefWrapper<RegisterBlock>;
