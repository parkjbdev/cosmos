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
use log_crate::info;

#[alloc_error_handler]
fn handle_alloc_error(_layout: Layout) -> ! {
    panic!("Memory Allocation Error");
}

#[panic_handler]
fn handle_panic(info: &core::panic::PanicInfo<'_>) -> ! {
    // console이 미등록이거나 재진입 상황에서도 semihosting으로 출력
    use core::fmt::Write;
    struct SemihostWriter;

    impl Write for SemihostWriter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let bytes = s.as_bytes();
            let mut buf = [0u8; 256];
            let len = bytes.len().min(buf.len() - 1);
            buf[..len].copy_from_slice(&bytes[..len]);

            unsafe {
                asm!(
                    "hlt #0xF000",
                    in("x0") 0x04u64,
                    in("x1") buf.as_ptr() as u64,
                    options(readonly, nostack)
                );
            }
            Ok(())
        }
    }

    let mut w = SemihostWriter;

    writeln!(w, "** KERNEL PANIC **");
    writeln!(w, "{}", info.message());
    if let Some(loc) = info.location() {
        writeln!(w, " at {}:{}:{}", loc.file(), loc.line(), loc.column());
    }

    // Killing kernel

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
