use crate::arch::drivers::pl011::PL011Uart;
use crate::arch::dtb;
use crate::arch::exception::irq::Interrupt;
use crate::interrupt::interface::IRQHandler;
use crate::sync::spinlock::RawSpinlock;
use generic_once_cell::OnceCell;

pub static mut CONSOLE: OnceCell<RawSpinlock, PL011Uart> = OnceCell::new();

pub fn init() {
    let mut pl011 = {
        // use crate::arch::constants::SERIAL_PORT_ADDRESS;

        let stdout = dtb::get_dtb()
            .get_property("/chosen", "stdout-path")
            .unwrap();
        let uart_addr = core::str::from_utf8(stdout)
            .unwrap()
            .trim_matches(char::from(0))
            .split_once('@')
            .map(|(_, addr)| u32::from_str_radix(addr, 16).unwrap())
            .unwrap();
        // .map(|(_, addr)| u32::from_str_radix(addr, 16).unwrap_or(SERIAL_PORT_ADDRESS))
        // .unwrap_or(SERIAL_PORT_ADDRESS);
        PL011Uart::new(uart_addr as usize)
    };
    pl011.init();

    let _ = unsafe { CONSOLE.set(pl011) };

    let pl011_dt = dtb::get_dtb().get_property("/pl011", "interrupts").unwrap();

    const SPLIT_SIZE: usize = core::mem::size_of::<u32>();
    let chunks: &[[u8; SPLIT_SIZE]] = unsafe { pl011_dt.as_chunks_unchecked() };

    Interrupt::from_raw(
        u32::from_be_bytes(chunks[0]),
        u32::from_be_bytes(chunks[1]),
        u32::from_be_bytes(chunks[2]),
        0x00,
        |state| {
            unsafe {
                CONSOLE.get_mut().unwrap().handler(|| {
                    CONSOLE.get_mut().unwrap().echo();
                })
            };
            true
        },
        "Keyboard Interrupt",
    )
    .register();
}
