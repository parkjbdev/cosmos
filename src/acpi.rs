use core::ffi::c_void;

use uefi::table::{cfg, Boot, SystemTable};

pub fn locate_acpi_rsdp_table(system_table: &SystemTable<Boot>) -> Option<*const c_void> {
    let mut config_entries = system_table.config_table().iter();
    let rsdp_addr = config_entries
        .find(|entry| matches!(entry.guid, cfg::ACPI_GUID | cfg::ACPI2_GUID))
        .map(|entry| entry.address);

    rsdp_addr
}

