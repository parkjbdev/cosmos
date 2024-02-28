#![no_main]
#![no_std]

use log::info;
use core::fmt::Write;
use uefi::prelude::*;

#[entry]
fn efi_main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    let stdout = system_table.stdout();
    stdout.clear().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();

    info!("This is an info message");

    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}
