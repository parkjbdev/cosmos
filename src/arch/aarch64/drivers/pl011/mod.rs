mod pl011;
mod pl011_inner;
mod registers;

use crate::{
    arch::{drivers::devicetree, irq::Interrupt},
    console::console,
    driver::interface::DeviceDriver,
    sync::spinlock::RawSpinlock,
};
use generic_once_cell::OnceCell;
pub use pl011::PL011Uart;

pub static PL011_UART: OnceCell<RawSpinlock, PL011Uart> = OnceCell::new();

pub fn init(base: usize, clock_hz: u32, baud_rate: u32) {
    #![allow(unused_must_use)]
    PL011_UART.set(PL011Uart::new(base, clock_hz, baud_rate));
    PL011_UART.get().unwrap().init();
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
            console().handler(|| {
                console().echo();
            });
            true
        },
        "Keyboard Interrupt",
    )
    .register();
}
