pub mod handlers;
// pub mod sgi;
pub mod irq;
pub mod test;
pub mod timer;

use super::state::ExceptionState;
use crate::{arch::exception::irq::Interrupt, sync::spin::RawSpinlock};
use aarch64_cpu::registers::*;
use arm_gic::{gicv3::GicV3, irq_disable, irq_enable};
use core::arch::{asm, global_asm};
use core::cell::UnsafeCell;
use generic_once_cell::OnceCell;
use hermit_dtb::Dtb;
use log::info;

extern "C" {
    pub fn current_el() -> u8;
}

global_asm!(include_str!("vector_table.s"));

const MAX_HANDLERS: usize = 1024;

type Handler = fn(state: &ExceptionState) -> bool;

static mut IRQ_NAMES: [Option<&'static str>; MAX_HANDLERS] = [None; MAX_HANDLERS];
static mut IRQ_HANDLERS: [Option<Handler>; MAX_HANDLERS] = [None; MAX_HANDLERS];

pub(crate) static mut GIC: OnceCell<RawSpinlock, GicV3> = OnceCell::new();

pub fn init(dtb: &Dtb) {
    irq_disable();
    // Set Exception Vector Table
    info!("Current Exception Level: EL{}", unsafe { current_el() });

    extern "Rust" {
        static __exception_vector: UnsafeCell<()>;
    }

    unsafe {
        VBAR_EL1.set(__exception_vector.get() as u64);
    }

    // Set GIC
    let gic = init_gic(dtb);
    unsafe { GIC.set(gic).unwrap() };

    timer::init_timer();

    test::test_segfault();
    test::test_sgi();

    // Scheduler Interrupt
    const RESCHED_SGI: u32 = 0;
    let scheduler = Interrupt::new(
        RESCHED_SGI,
        1,
        0x00,
        Some(|state| {
            println!("Scheduler Interrupt");
            true
        }),
        Some("Scheduler"),
    )
    .register()
    .enable();

    irq_enable();
}

fn init_gic(dtb: &Dtb) -> GicV3 {
    // Check Compatible GIC
    let compat = core::str::from_utf8(dtb.get_property("/intc", "compatible").unwrap()).unwrap();
    if !compat.contains("arm,gic-v3") {
        panic!("Compatible GIC (arm,gic-v3) Not Found");
    }
    info!("GIC Compatible: {}", compat);

    // Parse GICD & GICC from the dtb /intc reg
    let reg = dtb.get_property("/intc", "reg").unwrap();

    // GIC Distributor interface (GICD)
    let (slice, residual_slice) = reg.split_at(core::mem::size_of::<u64>());
    let mut gicd_start = u64::from_be_bytes(slice.try_into().unwrap());
    let (slice, residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
    let gicd_size = u64::from_be_bytes(slice.try_into().unwrap());

    // GIC Redistributors (GICR), one range per redistributor region
    let (slice, residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
    let mut gicr_start = u64::from_be_bytes(slice.try_into().unwrap());
    let (slice, _residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
    let gicr_size = u64::from_be_bytes(slice.try_into().unwrap());

    info!(
        "Initializing GIC.. GICD: {:#x} ({:#x}), GICR: {:#x} ({:#x})",
        gicd_start, gicd_size, gicr_start, gicr_size
    );

    let gicd_start: *mut u64 = gicd_start as _;
    let gicr_start: *mut u64 = gicr_start as _;

    // TODO: allocate gicd and gicr to virtualmem
    let mut gic = unsafe { GicV3::new(gicd_start, gicr_start) };
    gic.setup();
    GicV3::set_priority_mask(0xff);

    gic
}

pub fn mask_irq() {
    unsafe {
        asm!(
            "msr DAIFSet, {0}",
            const 0b0010,
            options(nostack, nomem, preserves_flags)
        );
    }
}

pub fn unmask_irq() {
    unsafe {
        asm!(
            "msr DAIFClr, {0}",
            const 0b0010,
            options(nostack, nomem, preserves_flags)
        );
    }
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