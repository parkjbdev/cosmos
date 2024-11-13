use generic_once_cell::OnceCell;
use pl011::PL011Uart;

use crate::{arch::irq::Interrupt, driver, sync::spinlock::RawSpinlock};

use super::devicetree::DEVICE_TREE;

mod pl011;
mod pl011_inner;
mod registers;

pub const PL011_UART: OnceCell<RawSpinlock, PL011Uart> = OnceCell::new();

pub struct PL011UartDriver;

impl driver::interface::DeviceDriver for PL011UartDriver {
    fn init(&mut self) -> Result<(), &'static str> {
        let uart_addr = {
            let stdout = DEVICE_TREE
                .get()
                .unwrap()
                .get_property("/chosen", "stdout-path")
                .unwrap();
            core::str::from_utf8(stdout)
                .unwrap()
                .trim_matches(char::from(0))
                .split_once('@')
                .map(|(_, addr)| u32::from_str_radix(addr, 16).unwrap())
                .unwrap()
        };

        let mut pl011 = PL011Uart::new(uart_addr);

        pl011.init();
        PL011_UART.set(pl011);

        Ok(())
    }

    fn compatible(&self) -> &str {
        core::str::from_utf8(
            DEVICE_TREE
                .get()
                .unwrap()
                .get_property("/pl011", "compatible")
                .unwrap(),
        )
        .unwrap()
        // "arm,pl011", "arm,primecell"
    }

    fn register_from_devicetree_and_enable_irq_handler(&self) {
        // Interrupt
        let pl011_dt = DEVICE_TREE
            .get()
            .unwrap()
            .get_property("/pl011", "interrupts")
            .unwrap();

        const SPLIT_SIZE: usize = core::mem::size_of::<u32>();
        let chunks: &[[u8; SPLIT_SIZE]] = unsafe { pl011_dt.as_chunks_unchecked() };

        Interrupt::from_raw(
            u32::from_be_bytes(chunks[0]),
            u32::from_be_bytes(chunks[1]),
            u32::from_be_bytes(chunks[2]),
            0x00,
            |state| {
                // console().handler(|| {
                //     console().echo();
                // });
                true
            },
            "Keyboard Interrupt",
        )
        .register();
    }
}
