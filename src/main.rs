#![no_main]
#![no_std]

#![allow(dead_code, unused_variables, incomplete_features)]
#![feature(
    alloc_error_handler,
    fn_align,
    naked_functions,
    slice_as_chunks,
    generic_const_exprs,
    step_trait
)]

#[macro_use]
pub mod print;

pub mod arch;
pub mod bsp;
pub mod console;
pub mod driver;
pub mod interrupt;
pub mod memory;
pub mod sync;

extern crate log as log_crate;
use crate::arch::exception::el::get_current_el;
use arch::memory::mmu::print_stat;
use bsp::memory::symbols;
use core::{alloc::Layout, arch::asm};
use log_crate::info;

#[no_mangle]
pub(crate) unsafe extern "C" fn kernel_main() -> ! {
    // Initialize Exceptions
    arch::irq::irq_disable();
    arch::exception::set_exception_handler();

    console::log::init();

    __println!("Testing Exceptions");
    arch::test::exception::test_segfault();
    // arch::test::exception::test_sgi();
    __println!("Test Pass");

    let phys_kernel_tables_base_addr = match memory::kernel_mapper::kernel_map_sections() {
        Err(string) => panic!("Error mapping kernel binary: {}", string),
        Ok(addr) => addr,
    };
    __println!(
        "kernel space: {:#x} ~ {:#x}",
        symbols::kernel_range().start,
        symbols::kernel_range().end
    );

    if let Err(e) = memory::mmu::init(phys_kernel_tables_base_addr) {
        panic!("Enabling MMU failed: {}", e);
    }

    __println!("Initializing Allocator");
    memory::mmu::init_mmio_allocator();
    __println!("Allocator Initialization Successful");

    // Initialize BSP (mmio)
    __println!("Initializing Drivers");
    bsp::init_drivers(false);
    __println!("Driver Initialization Successful");

    // Initialize Interrupts
    bsp::init_irq();

    // Initialize Timer Interrupt
    arch::timer::init_irq();

    arch::irq::irq_enable();
    arch::irq::fiq_enable();

    let ver = env!("CARGO_PKG_VERSION");

    println!("     _________  _________ ___  ____  _____");
    println!("    / ___/ __ \\/ ___/ __ `__ \\/ __ \\/ ___/");
    println!("   / /__/ /_/ (__  ) / / / / / /_/ (__  ) ");
    println!("   \\___/\\____/____/_/ /_/ /_/\\____/____/  v{}", ver);
    println!();

    // // CPU & RAM Info
    // info!("CPU Count: {} CPUs", arch::get_cpus());
    // info!("RAM Info: ");
    // arch::memory::print_ram_info();

    __println!("********* MMU Status *********");
    print_stat();

    info!("Timer Status: ");
    arch::timer::print_timer_status();
    info!(
        "Timer Resolution: {}ns",
        arch::timer::resolution().as_nanos()
    );

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
    __println!("************************************************");
    __println!("KERNEL PANIC: {}", info.message());
    let (file, line, column) = match info.location() {
        Some(location) => (location.file(), location.line(), location.column()),
        None => ("unknown", 0, 0),
    };

    __println!("{}:{}:{}", file, line, column);
    __println!("************************************************");

    #[repr(C)]
    struct QEMUParameterBlock {
        arg0: u64,
        arg1: u64,
    }

    let block = &QEMUParameterBlock {
        arg0: 0x20026,
        arg1: 1,
    };

    unsafe {
        asm!(
            "hlt #0xF000",
            in("x0") 0x18,
            in("x1") block as *const _ as u64,
            options(nostack)
        );
    }

    loop {
        unsafe { asm!("wfe", options(nomem, nostack)) };
    }
}
