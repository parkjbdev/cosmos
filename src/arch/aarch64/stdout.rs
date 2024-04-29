use super::constants::SERIAL_PORT_ADDRESS;
use super::dtb::get_dtb;
use super::serial::SerialPort;

pub static mut COM1: SerialPort = SerialPort::new(SERIAL_PORT_ADDRESS);

pub fn init() {
    let dtb = get_dtb();
    let stdout = dtb.get_property("/chosen", "stdout-path").unwrap();
    let uart_addr = core::str::from_utf8(stdout)
        .unwrap()
        .trim_matches(char::from(0))
        .split_once('@')
        .map(|(_, addr)| u32::from_str_radix(addr, 16).unwrap_or(SERIAL_PORT_ADDRESS))
        .unwrap_or(SERIAL_PORT_ADDRESS);

    unsafe {
        COM1.set_port(uart_addr);
    }
}

pub fn output_message_byte(byte: u8) {
    unsafe {
        COM1.write_byte(byte);
    }
}
