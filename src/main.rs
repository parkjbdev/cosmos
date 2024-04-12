#![feature(panic_info_message)]
#![feature(slice_as_chunks)]
#![feature(strict_provenance)]
#![feature(exposed_provenance)]
#![feature(alloc_error_handler)]
#![feature(asm_const)]
#![feature(naked_functions)]
#![no_main]
#![no_std]

#[macro_use]
pub mod print;
pub mod arch;
pub mod console;
pub mod log;
pub mod sync;

extern crate log as log_crate;
use core::alloc::Layout;
use log_crate::info;

use crate::arch::get_el;

#[no_mangle]
pub(crate) unsafe extern "C" fn loader_main() -> ! {
    log::init();
    arch::stdout::init();

    println!("  _________  _________ ___  ____  _____");
    println!(" / ___/ __ \\/ ___/ __ `__ \\/ __ \\/ ___/");
    println!("/ /__/ /_/ (__  ) / / / / / /_/ (__  ) ");
    println!("\\___/\\____/____/_/ /_/ /_/\\____/____/  ");
    println!();

    info!("Current Exception Level: EL{}", get_el());

    arch::init();
    arch::interrupts::init();

    println!(
        "boot_core_stack_start: {:p}",
        &arch::__boot_core_stack_start
    );
    println!(
        "boot_core_stack_end_exclusive: {:p}",
        &arch::__boot_core_stack_end_exclusive
    );
    println!(
        "Kernel: [{:p} - {:p}]",
        &arch::kernel_start,
        &arch::kernel_end
    );

    loop {}
}

#[alloc_error_handler]
fn handle_alloc_error(_layout: Layout) -> ! {
    panic!("Memory Allocation Error");
}

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("KERNEL PANIC!!!");

    let (file, line, column) = match info.location() {
        Some(location) => (location.file(), location.line(), location.column()),
        None => ("unknown", 0, 0),
    };

    println!(
        "[PANIC] {} ({}, line {}, column {})",
        info.message().unwrap_or(&format_args!("Unknown Error")),
        file,
        line,
        column,
    );

    loop {}
}
