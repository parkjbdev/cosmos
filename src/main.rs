#![allow(unused_variables)]
#![allow(dead_code)]
#![feature(alloc_error_handler)]
#![feature(exposed_provenance)]
#![feature(fn_align)]
#![feature(naked_functions)]
#![feature(slice_as_chunks)]
#![feature(strict_provenance)]
// #![feature(asm_const)]
// #![feature(const_refs_to_static)]
// #![feature(core_intrinsics)]
#![no_main]
#![no_std]

#[macro_use]
pub mod print;
pub mod arch;
pub mod console;
pub mod interrupt;
pub mod memory;
pub mod sync;
pub mod utils;
pub mod driver;

extern crate log as log_crate;
use crate::arch::exception::el::get_current_el;
use core::alloc::Layout;
use log_crate::{error, info};

#[no_mangle]
pub(crate) unsafe extern "C" fn kernel_main() -> ! {
    // Initialize Exceptions
    arch::irq::irq_disable();
    arch::exception::init();

    // Initialize Console
    console::log::init();
    arch::console::init();

    // Initialize Timer
    arch::timer::init();
    arch::irq::irq_enable();

    let ver = env!("CARGO_PKG_VERSION");

    println!("     _________  _________ ___  ____  _____");
    println!("    / ___/ __ \\/ ___/ __ `__ \\/ __ \\/ ___/");
    println!("   / /__/ /_/ (__  ) / / / / / /_/ (__  ) ");
    println!("   \\___/\\____/____/_/ /_/ /_/\\____/____/  v{}", ver);
    println!();

    info!("Timer Status: ");
    arch::timer::print_timer_status();
    info!(
        "Timer Resolution: {}ns",
        arch::timer::resolution().as_nanos()
    );

    info!("CPU Count: {} CPUs", arch::get_cpus());

    // info!("Testing Exceptions");
    // arch::test::exception::test_segfault();
    // arch::test::exception::test_sgi();
    // info!("Test Pass");

    info!("Current Page Size: {}", arch::memory::get_page_size());
    info!("[TIP] You can change the PAGE_SIZE in kernel.ld");

    info!("Initializing Memory...");
    // arch::memory::mmu::init();

    // CPU & RAM Info
    info!("RAM Info: ");
    arch::memory::print_ram_info();

    arch::memory::print_memory_layout();
    info!("Memory Layout: ");

    info!("Current Exception Level: {}", get_current_el());

    info!("Exception handling state:");
    arch::exception::print_state();

    info!("Registered IRQ handlers:");
    arch::irq::print_interrupts();

    info!("Echoing Inputs");
    info!("Waiting for interrupts...");

    let console = console::console();
    console.clear_rx();

    loop {
        arch::halt();
    }
}

#[alloc_error_handler]
fn handle_alloc_error(_layout: Layout) -> ! {
    panic!("Memory Allocation Error");
}

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo<'_>) -> ! {
    error!("KERNEL PANIC: {}", info.message());
    let (file, line, column) = match info.location() {
        Some(location) => (location.file(), location.line(), location.column()),
        None => ("unknown", 0, 0),
    };

    error!("{}:{}:{}", file, line, column);

    loop {}
}
