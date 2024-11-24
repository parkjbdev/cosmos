use super::translation_table::interface::TranslationTable;
use crate::memory::types::*;
use crate::{bsp, sync::interface::Mutex};

pub fn kernel_map_binary() -> Result<Address<Physical>, &'static str> {
    let phys_kernel_tables_baddr = bsp::memory::kernel_tables().lock(|table| {
        table.init();
        table.phys_base_addr()
    });

    __println!("      -------------------------------------------------------------------------------------------------------------------------------------------");
    __println!(
        "       {:<30}    {:<30}   {:^7}   {:^9}   {:^35}",
        "Virtual", "Physical", "Size", "Attr", "Entity"
    );
    __println!("      -------------------------------------------------------------------------------------------------------------------------------------------");
    bsp::memory::kernel_map_binary()?;

    __println!("      -------------------------------------------------------------------------------------------------------------------------------------------");

    Ok(phys_kernel_tables_baddr)
}

pub fn kernel_map_at(
    name: &'static str,
    virt_region: &MemoryRegion<Virtual>,
    phys_region: &MemoryRegion<Physical>,
    attributes: &AttributeFields,
) {
    __println!(
        "      {} --> {} | {} | {:<3} {} | {}",
        virt_region,
        phys_region,
        virt_region.size(),
        attributes.memory_attributes,
        attributes.access_permissions,
        name
    );

    bsp::memory::kernel_tables().lock(|tables| tables.map_at(virt_region, phys_region, attributes));
}
