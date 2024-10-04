use crate::arch::drivers::pl011;
use crate::console::register_console;
use super::drivers::pl011::PL011_UART;

pub fn init() {
    pl011::init();
    register_console(PL011_UART.get().unwrap());
}
