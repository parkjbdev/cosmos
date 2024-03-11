#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
#![feature(asm_const)]

pub mod acpi;
pub mod irq;

use core::alloc::Layout;
use log::info;
use uefi::prelude::*;
use uefi::table::cfg;
use core::ffi::c_void;

extern crate alloc;

#[entry]
fn efi_main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let mut system_table = uefi_services::system_table();

    info!("Welcome to cosmos");

    system_table
        .stdout()
        .reset(false)
        .expect("Failed to reset stdout");

    // UEFI Revision
    let uefi_rev = system_table.uefi_revision();
    info!("UEFI Revision {}.{}", uefi_rev.major(), uefi_rev.minor());

    // Time
    let time = system_table.runtime_services().get_time().unwrap();
    info!("Time: {:?}", time);

    // Locate the ACPI RSDP Table
    info!("Locating ACPI RSDP Table from UEFI Configuration Table");
    let rsdp_addr = locate_acpi_rsdp_table(&system_table);
    info!("rsdp addr: {:?}", rsdp_addr);

    system_table.boot_services().stall(10_000_000);

    Status::SUCCESS
}

fn locate_acpi_rsdp_table(system_table: &SystemTable<Boot>) -> Option<*const c_void> {
    let mut config_entries = system_table.config_table().iter();
    let rsdp_addr = config_entries
        .find(|entry| matches!(entry.guid, cfg::ACPI_GUID | cfg::ACPI2_GUID))
        .map(|entry| entry.address);

    rsdp_addr
}

#[alloc_error_handler]
fn handle_alloc_error(_layout: Layout) -> ! {
    panic!("Memory Allocation Error");
}
