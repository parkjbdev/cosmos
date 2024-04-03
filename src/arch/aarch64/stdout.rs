use hermit_dtb::Dtb;

use super::constants::SERIAL_PORT_ADDRESS;
use super::get_dtb;
use super::serial::SerialPort;

pub static mut COM1: SerialPort = SerialPort::new(SERIAL_PORT_ADDRESS);

pub fn init() {
    let dtb = get_dtb();
    let prop = dtb.get_property("/chosen", "stdout-path");
    let uart_addr = if let Some(stdout) = prop {
        let stdout = core::str::from_utf8(stdout)
            .unwrap()
            .trim_matches(char::from(0));
        if let Some(pos) = stdout.find('@') {
            let len = stdout.len();
            u32::from_str_radix(&stdout[pos + 1..len], 16).unwrap_or(SERIAL_PORT_ADDRESS)
        } else {
            SERIAL_PORT_ADDRESS
        }
    } else {
        SERIAL_PORT_ADDRESS
    };

    unsafe {
        COM1.set_port(uart_addr);
    }
}

pub fn output_message_byte(byte: u8) {
    unsafe {
        COM1.write_byte(byte);
    }
}
