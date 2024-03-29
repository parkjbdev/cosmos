pub mod entry;
pub mod serial;

use core::ptr;
use goblin::elf::header::header64::{Header, EI_DATA, ELFDATA2LSB, ELFMAG, SELFMAG};
use hermit_dtb::Dtb;
use log::info;

use serial::SerialPort;

const DEVICE_TREE: u64 = 0x40000000;
const SERIAL_PORT_ADDRESS: u32 = 0x09000000;

static mut COM1: SerialPort = SerialPort::new(SERIAL_PORT_ADDRESS);

pub fn init_stdout() {
    let dtb = unsafe {
        Dtb::from_raw(ptr::from_exposed_addr(DEVICE_TREE as usize)).expect("dtb invalid")
    };
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

pub fn find_kernel() -> &'static [u8] {
    let dtb = unsafe {
        Dtb::from_raw(ptr::from_exposed_addr(DEVICE_TREE as usize)).expect("invalid DTB")
    };
    let module_start = dtb
        .enum_subnodes("/chosen")
        .find(|node| node.starts_with("module@"))
        .map(|node| {
            let value = node.strip_prefix("module@").unwrap();
            usize::from_str_radix(value, 16).unwrap_or(0x48000000)
        })
        .unwrap();

    info!("module_start: {}", module_start);

    let header = unsafe {
        &*core::mem::transmute::<*const u8, *const Header>(ptr::from_exposed_addr(module_start))
    };

    info!("ELF Magic: {:?}", header.e_ident);
    if header.e_ident[0..SELFMAG] != ELFMAG[..] {
        panic!("Not an ELF file - wrong magic number");
    }

    // Assuming target is little endian system
    let file_size = if header.e_ident[EI_DATA] == ELFDATA2LSB {
        header.e_shoff + (header.e_shentsize as u64 * header.e_shnum as u64)
    } else {
        header.e_shoff.to_le() + (header.e_shentsize.to_le() as u64 * header.e_shnum.to_le() as u64)
    };
    info!("ELF File Size: {}", file_size);

    unsafe { core::slice::from_raw_parts(ptr::from_exposed_addr(module_start), file_size as usize) }
}
