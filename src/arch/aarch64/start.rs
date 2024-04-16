#![allow(dead_code)]

use crate::kernel_main;
use core::arch::{asm, global_asm};

global_asm!(include_str!("entry.s"));
global_asm!(include_str!("el.s"));

#[no_mangle]
pub unsafe fn _start_cosmos(boot_core_stack_end_exclusive_addr: u64) {
    // Change EL2 to EL1 and jump to loader_main
    // Translated to pure asm code from aarch64_cpu library dependency
    // https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/09_privilege_level
    asm!(
        // Enable timer counter registers for EL1.
        // CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET
        "mov x1, 0x1",
        "msr CNTHCTL_EL2, x1",
        // No offset for reading the counters.
        "msr CNTVOFF_EL2, xzr",
        // Set EL1 execution state to AArch64.
        // HCR_EL2::RW::EL1IsAarch64
        "mov x1, 0x80000000",
        "msr HCR_EL2, x1",
        // Set up a simulated exception return.
        // First, fake a saved program status where all interrupts were masked and SP_EL1 was used as a stack pointer.
        // SPSR_EL2: DAIF is masked, M: EL1h = 0x3c5
        "mov x1, 0x3c5",
        "msr SPSR_EL2, x1",
        // Second, let the link register point to kernel_init().
        "msr ELR_EL2, {}",
        // Set up SP_EL1 (stack pointer), which will be used by EL1 once we "return" to it. Since there are no plans to ever return to EL2, just re-use the same stack.
        "msr SP_EL1, {}",
        // Use `eret` to "return" to EL1. Execution will continue at the address set by ELR_EL2.
        "eret",
        in(reg) (kernel_main as *const () as u64),
        in(reg) boot_core_stack_end_exclusive_addr,
    );
}
