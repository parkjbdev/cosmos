#![feature(strict_provenance)]
#![feature(exposed_provenance)]
#![feature(alloc_error_handler)]
#![feature(asm_const)]

#![no_main]
#![no_std]

pub mod acpi;
pub mod irq;
pub mod init;
pub mod sync;

use core::alloc::Layout;
use log::info;
use uefi::prelude::*;

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
    // info!("Locating ACPI RSDP Table from UEFI Configuration Table");
    // let rsdp_addr = locate_acpi_rsdp_table(&system_table).unwrap();
    // info!("rsdp addr: {:?}", rsdp_addr);

    // Initialize Interrupt
    info!("Initializing Interrupt");
    irq::init();
    irq::enable();

    system_table.boot_services().stall(10_000_000);

    Status::SUCCESS
}

#[alloc_error_handler]
fn handle_alloc_error(_layout: Layout) -> ! {
    panic!("Memory Allocation Error");
}
