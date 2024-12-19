use super::translation_table::interface::TranslationTable;
use crate::bsp;
use crate::bsp::memory::{physical_region_of, virtual_region_of};
use crate::memory::types::*;

pub fn kernel_map_sections() -> Result<Address<Physical>, &'static str> {
    let mut kernel_table = bsp::memory::KERNEL_TABLES.write();
    kernel_table.init();
    let phys_kernel_tables_baddr = kernel_table.phys_base_addr().unwrap();
    // kernel_table.downgrade();

    __println!("      -------------------------------------------------------------------------------------------------------------------------------------------");
    __println!(
        "       {:^32} {:^28} {:^22} {:^7} {:^35}",
        "Virtual",
        "Physical",
        "Size",
        "Attr",
        "Entity"
    );
    __println!("      -------------------------------------------------------------------------------------------------------------------------------------------");
    let sections = bsp::memory::kernel_sections();
    for section in sections.iter() {
        let virt_region = virtual_region_of(section.range.start, section.range.end);
        let phys_region = physical_region_of(virt_region);

        kernel_table.map_at(&virt_region, &phys_region, &section.attr)?;

        __println!(
            "      {} --> {} | {} | {:<3} {} | {}",
            virt_region,
            phys_region,
            virt_region.size(),
            section.attr.memory_attributes,
            section.attr.access_permissions,
            section.name
        );
    }

    __println!("      -------------------------------------------------------------------------------------------------------------------------------------------");

    Ok(phys_kernel_tables_baddr)
}
