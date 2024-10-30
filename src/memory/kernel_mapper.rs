use super::translation_table::interface::TranslationTable;
use crate::memory::types::*;
use crate::{bsp, sync::interface::Mutex};

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
    bsp::memory::kernel_tables().lock(|tables| tables.map_at(virt_region, phys_region, attributes));
}
