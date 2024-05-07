pub mod handlers;
pub mod test;
pub mod timer;

use crate::sync::spin::RawSpinlock;

use aarch64_cpu::registers::*;
use arm_gic::{
    gicv3::{GicV3, IntId},
    irq_enable,
};
use hermit_dtb::Dtb;
use core::{arch::global_asm, cell::UnsafeCell};
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

    timer::init_timer();
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

    // error here
    // ptr::write_volatile requires that the pointer argument is aligned and non-null
    // TODO: allocate gicd and gicr to virtualmem
    let mut gic = unsafe { GicV3::new(&mut gicd_start, &mut gicr_start) };
    gic.setup();
    GicV3::set_priority_mask(0xff);

    gic
}

use arm_gic::gicv3::Trigger;

pub struct RawInterrupt {
    pub irq_type: u32,
    pub id: u32,
    pub trigger: u32,
    pub prio: u8,
}

impl From<RawInterrupt> for Interrupt {
    fn from(raw: RawInterrupt) -> Self {
        // Interrupt ID
        let irq_id = if raw.irq_type == 0 {
            IntId::spi(raw.id)
        } else if raw.irq_type == 1 {
            IntId::ppi(raw.id)
        } else {
            IntId::sgi(raw.id)
        };

        // Set Trigger Type from Flag
        let irq_flag = if raw.trigger == 4 || raw.trigger == 8 {
            Trigger::Level
        } else if raw.trigger == 2 || raw.trigger == 1 {
            Trigger::Edge
        } else {
            panic!("Invalid interrupt level!");
        };

        Interrupt {
            id: irq_id,
            trigger: irq_flag,
            prio: raw.prio,
            irq_type: raw.irq_type,
        }
    }
}

pub struct Interrupt {
    pub irq_type: u32,
    pub id: IntId,
    pub trigger: Trigger,
    pub prio: u8,
}

impl Interrupt {
    fn register_gic(&self) {
        let gic = unsafe { GIC.get_mut().unwrap() };

        gic.set_trigger(self.id, self.trigger);
        gic.set_interrupt_priority(self.id, self.prio);
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
