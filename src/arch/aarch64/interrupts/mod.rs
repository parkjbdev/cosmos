pub mod handlers;
pub mod timer;

use super::{dtb::get_dtb, state::State};
use arm_gic::{
    gicv3::{GicV3, IntId, SgiTarget},
    irq_enable,
};
use core::arch::{asm, global_asm};
use log::{error, info};

global_asm!(include_str!("irq.s"));

// const MAX_HANDLERS: usize = 1024;

// type Handler = fn(state: &State) -> bool;
// static mut IRQ_NAMES: [Option<&'static str>; MAX_HANDLERS] = [None; MAX_HANDLERS];
// static mut IRQ_HANDLERS: [Option<Handler>; MAX_HANDLERS] = [None; MAX_HANDLERS];

// pub(crate) static mut GIC: OnceCell<RawSpinlock, GicV3> = OnceCell::new();
// static mut TIMER_INTERRUPT: IntId = IntId::sgi(0);
// const RESCHED_SGI: u32 = 1;

// extern "C" {
//     static vector_table: u8;
// }

pub fn init() {
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

    // info!("Initializing GIC.. GICD: {:#x} ({:#x}), GICC: {:#x} ({:#x})", gicd_start, gicd_size, gicc_start, gicc_size);
    // error here
    // ptr::write_volatile requires that the pointer argument is aligned and non-null
    // TODO: allocate gicd and gicc to virtualmem
    // let mut gic = unsafe { GicV3::new(&mut gicd_start, &mut gicc_start) };
    // gic.setup();
    GicV3::set_priority_mask(0xff);

    // timer::init_timer(&gic);

    // unsafe {
    //     GIC.set(gic).unwrap();
    // }
    // // Scheduler Interrupt
    // let resched_sgi = IntId::sgi(RESCHED_SGI);
    // gic.set_interrupt_priority(resched_sgi, 0x00);
    // gic.enable_interrupt(resched_sgi, true);
    // unsafe {
    //     IRQ_NAMES[u32::from(resched_sgi) as usize] = Some("Scheduler");
    // }

    irq_enable();
    info!("Interrupt Enabled.. Sending Test Interrupt");
    unsafe {
        asm!("svc 0x80");
    }
    // test_irq(&mut gic);
}

fn test_irq(gic: &mut GicV3) {
    // Testing Interrupt
    // Configure an SGI(Software Generated Interrupt) and then send it to ourself.
    let sgi_id = IntId::sgi(3);

    // let gic = unsafe { GIC.get_mut().unwrap() };
    gic.set_interrupt_priority(sgi_id, 0x00);
    irq_enable();
    gic.enable_interrupt(sgi_id, true);
    GicV3::send_sgi(
        sgi_id,
        SgiTarget::List {
            affinity3: 0,
            affinity2: 0,
            affinity1: 0,
            target_list: 0b1,
        },
    );
    info!("SGI(id: {}) Sent", u32::from(sgi_id));
}
