use interface::TranslationTable;

use crate::memory::types::address::*;
use crate::{bsp, sync::interface::Mutex};

use super::types::memory::{AttributeFields, MemoryRegion};

pub mod interface {
    use super::*;

    pub trait TranslationTable {
        fn init(&mut self);
        fn phys_base_addr(&self) -> Address<Physical>;
    }
}

pub fn kernel_map_binary() -> Result<Address<Physical>, &'static str> {
    let phys_kernel_tables_baddr = bsp::memory::kernel_tables().lock(|table| {
        table.init();
        table.phys_base_addr()
    });

    bsp::memory::kernel_map_binary()?;

    Ok(phys_kernel_tables_baddr)
}

pub fn kernel_map_at(
    name: &'static str,
    virt_region: &MemoryRegion<Virtual>,
    phys_region: &MemoryRegion<Physical>,
    attributes: &AttributeFields,
) {
}
