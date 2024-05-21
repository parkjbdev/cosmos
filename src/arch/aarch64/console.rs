use hermit_dtb::Dtb;
use log::info;

use crate::sync::spin::RawSpinlock;
use generic_once_cell::OnceCell;

use super::constants::SERIAL_PORT_ADDRESS;
use super::pl011::PL011Uart;

pub static mut CONSOLE: OnceCell<RawSpinlock, PL011Uart> = OnceCell::new();

pub fn init(dtb: &Dtb) {
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
        info!("UART ADDR: {:#x}", uart_addr);
    }
}
