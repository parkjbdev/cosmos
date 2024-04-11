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

#[no_mangle]
pub(crate) unsafe extern "C" fn loader_main() -> ! {
    log::init();
    arch::stdout::init();
    info!("Hello World from cosmos");

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
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    info!("PANIC {}", info);
    loop {}
}
