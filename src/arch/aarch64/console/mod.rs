mod pl011;

use log::info;
use crate::arch::constants::SERIAL_PORT_ADDRESS;
use crate::arch::dtb;
use crate::arch::console::pl011::PL011Uart;
use crate::interrupt::interface::RegisterInterrupt;
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
    pl011.init_irq();

    unsafe {
        let _ = CONSOLE.set(pl011);
    }
    info!("UART ADDR: {:#x}", uart_addr);
}
