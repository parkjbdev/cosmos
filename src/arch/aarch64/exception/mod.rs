pub mod handlers;
pub mod sgi;
pub mod test;
pub mod timer;

use super::state::ExceptionState;
use crate::sync::spin::RawSpinlock;
use aarch64_cpu::registers::*;
use arm_gic::gicv3::Trigger;
use arm_gic::{
    gicv3::{GicV3, IntId},
    irq_enable,
};
use core::{arch::global_asm, cell::UnsafeCell};
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

    irq_enable();

    timer::init_timer();
    test::test_segfault();
    test::test_sgi();

    // // Scheduler Interrupt
    // let resched_sgi = IntId::sgi(RESCHED_SGI);
    // gic.set_interrupt_priority(resched_sgi, 0x00);
    // gic.enable_interrupt(resched_sgi, true);
    // unsafe {
    //     IRQ_NAMES[u32::from(resched_sgi) as usize] = Some("Scheduler");
    // }
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

pub struct Interrupt {
    pub id: IntId,
    pub trigger: Trigger,
    pub prio: u8,
    pub handler: Handler,
    pub name: &'static str,
}

#[allow(dead_code)]
impl Interrupt {
    fn raw_new(
        irq_type: u32,
        id: u32,
        trigger: u32,
        prio: u8,
        name: &'static str,
        handler: Handler,
    ) -> Self {
        // Interrupt ID
        let irq_id = match irq_type {
            0 => IntId::spi(id),
            1 => IntId::ppi(id),
            _ => IntId::sgi(id),
        };

        // Set Trigger Type from Flag
        let irq_flag = if trigger == 4 || trigger == 8 {
            Trigger::Level
        } else if trigger == 2 || trigger == 1 {
            Trigger::Edge
        } else {
            panic!("Invalid interrupt level!");
        };

        Self {
            id: irq_id,
            trigger: irq_flag,
            prio,
            handler,
            name,
        }
    }

    fn new(id: IntId, trigger: Trigger, prio: u8, name: &'static str, handler: Handler) -> Self {
        Self {
            id,
            trigger,
            prio,
            handler,
            name,
        }
    }

    fn register(&self) {
        let gic = unsafe { GIC.get_mut().unwrap() };

        gic.set_trigger(self.id, self.trigger);
        gic.set_interrupt_priority(self.id, self.prio);

        let id = u32::from(self.id) as usize;

        unsafe {
            IRQ_NAMES[id] = Some(self.name);
            IRQ_HANDLERS[id] = Some(self.handler);
        }
    }

    fn enable(&self) {
        let gic = unsafe { GIC.get_mut().unwrap() };
        gic.enable_interrupt(self.id, true);
    }

    fn disable(&self) {
        let gic = unsafe { GIC.get_mut().unwrap() };
        gic.enable_interrupt(self.id, false);
    }
}
