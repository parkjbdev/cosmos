#![no_main]
#![no_std]
#![allow(dead_code, unused_variables, incomplete_features)]
#![feature(alloc_error_handler, fn_align, generic_const_exprs, step_trait)]

#[macro_use]
pub mod print;

pub mod arch;
pub mod bsp;
pub mod console;
pub mod driver;
pub mod drivers;
pub mod init;
pub mod interrupt;
pub mod memory;
pub mod sync;

extern crate log as log_crate;
use core::{alloc::Layout, arch::asm};
use log_crate::{debug, error, info, warn};

#[alloc_error_handler]
fn handle_alloc_error(_layout: Layout) -> ! {
    panic!("Memory Allocation Error");
}

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("************************************************");
    println!("KERNEL PANIC: {}", info.message());
    let (file, line, column) = match info.location() {
        Some(location) => (location.file(), location.line(), location.column()),
        None => ("unknown", 0, 0),
    };

    println!("{}:{}:{}", file, line, column);
    println!("************************************************");

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
