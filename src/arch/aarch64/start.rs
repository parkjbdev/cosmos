use crate::{drivers::devicetree::BOOT_DTB_ADDR, init::kernel_main};
use aarch64_cpu::{
    asm::{barrier, eret},
    registers::*,
};
use core::{arch::global_asm, sync::atomic::Ordering};
use tock_registers::interfaces::ReadWriteable;

global_asm!(include_str!("entry.s"));

#[no_mangle]
pub unsafe fn _start_cosmos(dtb_addr: u64, boot_core_stack_end_exclusive_addr: u64) {
    BOOT_DTB_ADDR.store(dtb_addr, Ordering::Relaxed);
    // Change EL2 to EL1 and jump to kernel_main

    // Enable timer counter registers for EL1.
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

    // No offset for reading the counters.
    CNTVOFF_EL2.set(0);

    // Set EL1 execution state to AArch64.
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    // Set up a simulated exception return.
    //
    // First, fake a saved program status where all interrupts were masked and SP_EL1 was used as a
    // stack pointer.
    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h,
    );

    // Second, let the link register point to kernel_main().
    ELR_EL2.set(kernel_main as *const () as u64);

    // Set up SP_EL1 (stack pointer), which will be used by EL1 once we "return" to it. Since there
    // are no plans to ever return to EL2, just re-use the same stack.
    // SP_EL1.set(virt_boot_core_stack_end_exclusive_addr);
    SP_EL1.set(boot_core_stack_end_exclusive_addr);

    // Disable EL1 MMU/caches before transitioning.
    // QEMU's bootloader may leave SCTLR_EL1.M=1, causing translation faults
    // on physical memory accesses before the kernel sets up its own page tables.
    SCTLR_EL1.modify(
        SCTLR_EL1::M::Disable + // MMU enable for EL1 and EL0 stage 1 address translation.
            SCTLR_EL1::A::Disable +
            SCTLR_EL1::C::NonCacheable + // Cacheability control, for data accesses.
            SCTLR_EL1::I::NonCacheable, // Instruction access Cacheability control, for accesses at EL0 and EL1
    );
    barrier::isb(barrier::SY);

    eret();
}
