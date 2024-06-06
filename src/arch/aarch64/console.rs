use log::info;

use crate::arch::constants::SERIAL_PORT_ADDRESS;
use crate::arch::dtb;
use crate::arch::exception::irq::irq::Interrupt;
use crate::arch::pl011::PL011Uart;
use crate::sync::spinlock::RawSpinlock;
use generic_once_cell::OnceCell;

pub static mut CONSOLE: OnceCell<RawSpinlock, PL011Uart> = OnceCell::new();

pub fn init() {
    let dtb = &dtb::get_dtb();

    let stdout = dtb.get_property("/chosen", "stdout-path").unwrap();
    let uart_addr = core::str::from_utf8(stdout)
        .unwrap()
        .trim_matches(char::from(0))
        .split_once('@')
        .map(|(_, addr)| u32::from_str_radix(addr, 16).unwrap_or(SERIAL_PORT_ADDRESS))
        .unwrap_or(SERIAL_PORT_ADDRESS);

    // UART
    let mut pl011 = PL011Uart::new(uart_addr as usize);
    pl011.init();
    unsafe {
        let _ = CONSOLE.set(pl011);
    }
    info!("UART ADDR: {:#x}", uart_addr);

    let pl011_dt = dtb.get_property("/pl011", "interrupts").unwrap();

    const SPLIT_SIZE: usize = core::mem::size_of::<u32>();

    let chunks: &[[u8; SPLIT_SIZE]] = unsafe { pl011_dt.as_chunks_unchecked() };

    let _keyboard: Interrupt = Interrupt::from_raw(
        u32::from_be_bytes(chunks[0]),
        u32::from_be_bytes(chunks[1]),
        u32::from_be_bytes(chunks[2]),
        0xff,
        Some(|state| true),
        Some("Keyboard Interrupt"),
    );
    _keyboard.register();
}
