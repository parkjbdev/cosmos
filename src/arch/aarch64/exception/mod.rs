pub mod handlers;
pub mod timer;
pub mod test;

use crate::sync::spin::RawSpinlock;

use super::dtb::get_dtb;
use aarch64_cpu::registers::*;
use arm_gic::{
    gicv3::{GicV3, IntId, SgiTarget},
    irq_enable,
};
use core::{
    arch::{asm, global_asm},
    cell::UnsafeCell,
};
use generic_once_cell::OnceCell;
use log::info;

extern "C" {
    pub fn current_el() -> u8;
}

global_asm!(include_str!("vector_table.s"));

// const MAX_HANDLERS: usize = 1024;

// type Handler = fn(state: &State) -> bool;
// static mut IRQ_NAMES: [Option<&'static str>; MAX_HANDLERS] = [None; MAX_HANDLERS];
// static mut IRQ_HANDLERS: [Option<Handler>; MAX_HANDLERS] = [None; MAX_HANDLERS];
// static mut TIMER_INTERRUPT: IntId = IntId::sgi(0);
// const RESCHED_SGI: u32 = 1;

pub(crate) static mut GIC: OnceCell<RawSpinlock, GicV3> = OnceCell::new();

pub fn init() {
    // Set Exception Vector Table
    info!("Current Exception Level: EL{}", unsafe { current_el() });

    extern "Rust" {
        static __exception_vector: UnsafeCell<()>;
    }

    unsafe {
        VBAR_EL1.set(__exception_vector.get() as u64);
    }

    let gic = init_gic();
    timer::init_timer(&gic);
    unsafe { GIC.set(gic).unwrap() };
    irq_enable();

    test::test_segfault();

    // // Scheduler Interrupt
    // let resched_sgi = IntId::sgi(RESCHED_SGI);
    // gic.set_interrupt_priority(resched_sgi, 0x00);
    // gic.enable_interrupt(resched_sgi, true);
    // unsafe {
    //     IRQ_NAMES[u32::from(resched_sgi) as usize] = Some("Scheduler");
    // }

    // BUG: Why is test_sgi not calling interrupt handler?
    // maybe it is sending interrupt to the wrong cpu
    test::test_sgi();
}

fn init_gic() -> GicV3 {
    // Parse GICD & GICC from the dtb /intc reg
    let reg = get_dtb().get_property("/intc", "reg").unwrap();
    let (slice, residual_slice) = reg.split_at(core::mem::size_of::<u64>());
    let mut gicd_start = u64::from_be_bytes(slice.try_into().unwrap());
    let (slice, residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
    let gicd_size = u64::from_be_bytes(slice.try_into().unwrap());
    let (slice, residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
    let mut gicc_start = u64::from_be_bytes(slice.try_into().unwrap());
    let (slice, _residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
    let gicc_size = u64::from_be_bytes(slice.try_into().unwrap());

    info!(
        "Initializing GIC.. GICD: {:#x} ({:#x}), GICC: {:#x} ({:#x})",
        gicd_start, gicd_size, gicc_start, gicc_size
    );

    // error here
    // ptr::write_volatile requires that the pointer argument is aligned and non-null
    // TODO: allocate gicd and gicc to virtualmem
    let mut gic = unsafe { GicV3::new(&mut gicd_start, &mut gicc_start) };
    gic.setup();
    GicV3::set_priority_mask(0xff);

    gic
}

