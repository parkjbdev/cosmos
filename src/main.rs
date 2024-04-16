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
pub mod log;
pub mod sync;

extern crate log as log_crate;
use core::{alloc::Layout, arch::asm};

#[no_mangle]
pub(crate) unsafe extern "C" fn kernel_main() -> ! {
    log::init();
    arch::init();

    println!("       _________  _________ ___  ____  _____");
    println!("      / ___/ __ \\/ ___/ __ `__ \\/ __ \\/ ___/");
    println!("     / /__/ /_/ (__  ) / / / / / /_/ (__  ) ");
    println!("     \\___/\\____/____/_/ /_/ /_/\\____/____/  ");
    println!();

    println!(
        "{: <30}: [{:p} ~ {:p}]",
        "Kernel",
        &arch::kernel_start,
        &arch::kernel_end
    );
    println!(
        "{: <30}: [{:p} ~ {:p}]",
        ".text",
        &arch::__text_start,
        &arch::__text_end
    );
    println!(
        "{: <30}: [{:p} ~ {:p}]",
        ".text._exception_vector_table",
        &arch::__exception_vector_table_start,
        &arch::__exception_vector_table_end
    );
    println!(
        "{: <30}: [{:p} - {:p}]",
        ".bss",
        &arch::__bss_start,
        &arch::__bss_end_exclusive
    );
    println!(
        "{: <30}: [{:p} ~ {:p}]",
        "boot_core_stack_start",
        &arch::__boot_core_stack_start,
        &arch::__boot_core_stack_end_exclusive
    );

    arch::exception::test_sgi();

    arch::exception::test_segfault();
    // arch::exception::test_svc();
    // arch::exception::test_exception();


    println!("Waiting for interrupts...");
    unsafe {
        asm!("wfi");
    }


    loop {}
}

#[alloc_error_handler]
fn handle_alloc_error(_layout: Layout) -> ! {
    panic!("Memory Allocation Error");
}

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo<'_>) -> ! {
    log_crate::error!("KERNEL PANIC");
    let (file, line, column) = match info.location() {
        Some(location) => (location.file(), location.line(), location.column()),
        None => ("unknown", 0, 0),
    };

    log_crate::error!
    ("{}:{}:{}", file, line, column);

    loop {}
}
