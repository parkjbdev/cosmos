use crate::{
    bsp::pl011::PL011_UART,
    console::{interface, register_console, CONSOLE},
};

pub fn init() {
    // let pl011 = PL011_UART.get().unwrap();
    // register_console(pl011);

    let uart = unsafe {
        &*(PL011_UART.get().unwrap() as *const _ as *const (dyn interface::Console + Sync))
    };
    register_console(uart);
    // CONSOLE.set(PL011_UART.get().unwrap());
}
