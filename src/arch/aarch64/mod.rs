pub mod constants;
pub mod dtb;
pub mod interrupts;
pub mod serial;
pub mod start;
pub mod state;
pub mod stdout;

pub use constants::*;

use crate::arch::stdout::COM1;
use log::info;

extern "C" {
    pub fn get_el() -> u8;
}

pub fn init() {
    let dtb = dtb::get_dtb();
    // CPU
    let cpus = dtb.enum_subnodes("/cpus");
    let cpu_cnt = cpus
        .filter(|cpu| cpu.split('@').next().unwrap() == "cpu")
        .count();
    info!("CPU Count: {} CPUs", cpu_cnt);

    // UART
    let uart_addr: u32 = unsafe { COM1.get_port() };
    info!("UART ADDR: {:#x}", uart_addr);

    // RAM
    let mem_devt = dtb.get_property("/memory", "device_type").unwrap();
    assert!(
        core::str::from_utf8(mem_devt)
            .unwrap()
            .trim_matches(char::from(0))
            == "memory"
    );

    let mem_reg = dtb.get_property("/memory", "reg").unwrap();
    let (start, size) = mem_reg.split_at(core::mem::size_of::<u64>());
    let ram_start = u64::from_be_bytes(start.try_into().unwrap());
    let ram_size = u64::from_be_bytes(size.try_into().unwrap());
    info!("RAM: start {:#x} size {:#x}", ram_start, ram_size);
}
