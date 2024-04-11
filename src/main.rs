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
pub mod start;
pub mod sync;

use core::alloc::Layout;

#[alloc_error_handler]
fn handle_alloc_error(_layout: Layout) -> ! {
    panic!("Memory Allocation Error");
}
