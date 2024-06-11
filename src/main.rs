#![feature(const_refs_to_static)]
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
pub mod interrupt;
pub mod sync;

extern crate log as log_crate;
use crate::arch::console::CONSOLE;
use crate::arch::exception::el::get_current_el;
use crate::console::interface::{Read, Write};
use arm_gic::{irq_disable, irq_enable};
use core::{alloc::Layout, arch::asm};
use log_crate::{error, info};

#[no_mangle]
pub(crate) unsafe extern "C" fn kernel_main() -> ! {
    // Initialize Exceptions
    irq_disable();
    arch::exception::init();

    // Initialize Console
    console::log::init();
    arch::console::init();

    // Initialize Timer
    info!(
        "Timer Resolution: {}ns",
        arch::timer::resolution().as_nanos()
    );
    arch::timer::init();

    irq_enable();

    println!("       _________  _________ ___  ____  _____");
    println!("      / ___/ __ \\/ ___/ __ `__ \\/ __ \\/ ___/");
    println!("     / /__/ /_/ (__  ) / / / / / /_/ (__  ) ");
    println!("     \\___/\\____/____/_/ /_/ /_/\\____/____/  ");
    println!();

    // info!("Testing Exceptions");
    // arch::test::exception::test_segfault();
    // arch::test::exception::test_sgi();
    // info!("Test Pass");

    info!("Current Exception Level: EL{}", get_current_el());

    // CPU & RAM Info
    info!("CPU Count: {} CPUs", arch::get_cpus());
    let (ram_start, ram_size) = arch::get_ramrange();
    info!("RAM: start {:#x} size {:#x}", ram_start, ram_size);

    // Memory Layout
    info!("Memory Layout");
    info!(
        "    {: <30}: [{:p} ~ {:p}]",
        "Kernel",
        &arch::kernel_start,
        &arch::kernel_end
    );
    info!(
        "    {: <30}: [{:p} ~ {:p}]",
        ".text",
        &arch::__text_start,
        &arch::__text_end
    );
    info!(
        "    {: <30}: [{:p} - {:p}]",
        ".bss",
        &arch::__bss_start,
        &arch::__bss_end_exclusive
    );
    info!(
        "    {: <30}: [{:p} ~ {:p}]",
        "boot_core_stack_start",
        &arch::__boot_core_stack_start,
        &arch::__boot_core_stack_end_exclusive
    );

    arch::exception::print_all_handlers();

    info!("Initialization Done!");
    info!("Echoing Inputs");
    info!("Waiting for events");

    let console = CONSOLE.get_mut().unwrap();
    console.clear_rx();

    loop {
        unsafe { asm!("wfe", options(nomem, nostack)) }
    }
}

#[alloc_error_handler]
fn handle_alloc_error(_layout: Layout) -> ! {
    panic!("Memory Allocation Error");
}

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo<'_>) -> ! {
    error!("KERNEL PANIC: {}", info.message().unwrap());
    let (file, line, column) = match info.location() {
        Some(location) => (location.file(), location.line(), location.column()),
        None => ("unknown", 0, 0),
    };

    error!("{}:{}:{}", file, line, column);

    loop {}
}
