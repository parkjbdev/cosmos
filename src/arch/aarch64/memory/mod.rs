pub mod mmu;
pub mod attribute;
use mmu::MMU;

use crate::bsp::memory::symbols;
use crate::memory::types::MemorySize;
use crate::{arch::devicetree, memory};
use log::info;

pub fn mmu() -> &'static impl memory::mmu::interface::MMU {
    &MMU
}

pub fn get_page_size() -> MemorySize {
    MemorySize(unsafe { symbols::PAGE_SIZE.get() as usize })
}

pub fn get_ramrange() -> (u64, MemorySize) {
    let mem_devt = devicetree::dtb()
        .get_property("/memory", "device_type")
        .unwrap();

    assert!(
        core::str::from_utf8(mem_devt)
            .unwrap()
            .trim_matches(char::from(0))
            == "memory"
    );

    let mem_reg = devicetree::dtb().get_property("/memory", "reg").unwrap();
    let (start, size) = mem_reg.split_at(core::mem::size_of::<u64>());
    let ram_start = u64::from_be_bytes(start.try_into().unwrap());
    let ram_size = usize::from_be_bytes(size.try_into().unwrap());

    (ram_start, MemorySize(ram_size))
}

pub fn print_ram_info() {
    let (ram_start, ram_size) = get_ramrange();
    info!("      Start Address {:#x}", ram_start);
    info!("      Size {}", ram_size);
}
