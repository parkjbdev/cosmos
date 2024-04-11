#![allow(dead_code)]

use core::{
    arch::{asm, global_asm},
    sync::atomic::AtomicU64,
};

use arm_gic::irq_disable;

use crate::loader_main;

global_asm!(include_str!("entry.s"));
global_asm!(include_str!("el.s"));

extern "C" {
    static vector_table: u64;
}

pub(crate) static CURRENT_STACK_ADDRESS: AtomicU64 = AtomicU64::new(0);

#[inline(never)]
#[no_mangle]
#[link_section = ".text._start"]
pub unsafe fn _start_cosmos() -> ! {
    unsafe {
        // Disable Interrupts
        irq_disable();
        // asm!("msr daifset, 0xf", options(nostack));

        // Reset Thread id registers
        asm!("msr tpidr_el0, xzr", "msr tpidr_el1, xzr", options(nostack));

        // Reset debug control register
        asm!("msr mdscr_el1, xzr", options(nostack));

        // Set Exception Vector Table
        asm!(
            "adrp x4, {vector_table}",
            "add  x4, x4, #:lo12:{vector_table}",
            "msr vbar_el1, x4",
            vector_table = sym vector_table,
            out("x4") _,
            options(nostack),
        );

        // Memory barrier
        asm!("dsb sy", options(nostack));

        // Enter loader
        loader_main();
    }
}
