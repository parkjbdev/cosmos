pub mod el;
pub mod handlers;
pub mod irq;
pub mod state;

use crate::sync::spinlock::RawSpinlock;
use aarch64_cpu::asm::barrier;
use aarch64_cpu::registers::*;
use arm_gic::gicv3::GicV3;
use core::arch::global_asm;
use core::cell::UnsafeCell;
use generic_once_cell::OnceCell;
use state::ExceptionState;

global_asm!(include_str!("vector_table.s"));

type Handler = fn(state: &ExceptionState) -> bool;

pub(crate) static mut GIC: OnceCell<RawSpinlock, GicV3> = OnceCell::new();

pub fn init() {
    // TODO: Assert interrupts are disabled

    // Set Exception Vector Table
    extern "Rust" {
        static __exception_vector: UnsafeCell<()>;
    }

    VBAR_EL1.set(unsafe { __exception_vector.get() as u64 });

    barrier::isb(barrier::SY);
}

pub fn print_state() {
    use crate::info;

    #[rustfmt::skip]
    let to_mask_str = |x| -> _ {
        if x { "Masked" } else { "Unmasked" }
    };

    info!("      Debug:  {}", to_mask_str(DAIF.is_set(DAIF::D)));
    info!("      SError: {}", to_mask_str(DAIF.is_set(DAIF::A)));
    info!("      IRQ:    {}", to_mask_str(DAIF.is_set(DAIF::I)));
    info!("      FIQ:    {}", to_mask_str(DAIF.is_set(DAIF::F)));
}
