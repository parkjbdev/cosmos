pub mod el;
pub mod handlers;
pub mod irq;
pub mod state;

use crate::arch::dtb;
use crate::sync::spinlock::{RawSpinlock, Spinlock};
use aarch64_cpu::asm::barrier;
use aarch64_cpu::registers::*;
use arm_gic::gicv3::GicV3;
use core::arch::{asm, global_asm};
use core::cell::UnsafeCell;
use generic_once_cell::OnceCell;
use log::info;
use state::ExceptionState;

global_asm!(include_str!("vector_table.s"));

const MAX_HANDLERS: usize = 1024;

type Handler = fn(state: &ExceptionState) -> bool;

static IRQ_NAMES: Spinlock<[Option<&'static str>; MAX_HANDLERS]> =
    Spinlock::new([None; MAX_HANDLERS]);
static IRQ_HANDLERS: Spinlock<[Option<Handler>; MAX_HANDLERS]> =
    Spinlock::new([None; MAX_HANDLERS]);

pub(crate) static mut GIC: OnceCell<RawSpinlock, GicV3> = OnceCell::new();

pub fn init() {
    // TODO: Assert that interrupts are disabled

    // Set Exception Vector Table
    extern "Rust" {
        static __exception_vector: UnsafeCell<()>;
    }

    VBAR_EL1.set(unsafe { __exception_vector.get() as u64 });

    let dtb = &dtb::get_dtb();
    // Set GIC
    let gic = irq::init_gic(dtb);
    unsafe { GIC.set(gic).unwrap() };

    barrier::isb(barrier::SY);
}

pub fn set_irq(daif: u64) {
    unsafe {
        asm!(
            "msr DAIFSet, {0}",
            in(reg) daif,
            options(nostack, nomem, preserves_flags)
        );
    }
}

pub fn exec_with_irq_disabled<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let daif = DAIF.get();
    let ret = f();
    set_irq(daif);
    ret
}

pub fn print_all_handlers() {
    info!("IRQ Handlers");
    for (i, name) in IRQ_NAMES.lock().iter().enumerate() {
        if let Some(handler) = name {
            info!("    {: >3}. {}", i, handler);
        }
    }
}
