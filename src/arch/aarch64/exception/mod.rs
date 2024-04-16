pub mod handlers;
pub mod timer;

use crate::sync::spin::RawSpinlock;

use super::dtb::get_dtb;
use arm_gic::{
    gicv3::{GicV3, IntId, SgiTarget},
    irq_enable,
};
use core::arch::{asm, global_asm};
use generic_once_cell::OnceCell;
use log::info;

extern "C" {
    pub static vector_table: u64;
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

    unsafe {
        info!(
            "Initializing Exception Vector Table: {:p} ~ {:p}",
            &super::__exception_vector_table_start,
            &super::__exception_vector_table_end
        );
    }

    // unsafe {
    //     asm!(
    //         "msr VBAR_EL1, {vector_addr}",
    //         "isb sy",
    //         "dsb sy",
    //         vector_addr = in(reg) &super::__exception_vector_table_start,
    //         options(nomem, nostack),
    //     );
    // }

    // ERROR: If `__exception_vector_table_start` == `__exception_vector_table_end` it panics
    // successfully, but if not, it will wait forever (maybe) because CPU couldn't find the proper
    // exception vector table.. `__exception_vector_table_start` == `__exception_vector_table_end`
    // means that the exception vector table is optimized out by the compiler
    // If the script below is enabled, compiler won't optimize out the exception vector
    // table.. since exception vector table is 'KEEP'ed by the linker script now it will never
    // optimize out the vector table

    // WARNING: This won't retrieve proper address.. why?
    // Is the sym vector_table retriving proper address of the vector table?
    // __exception_vector_table_start dismatches sym vector_table..

    // maybe this is right..
    let vts: u64;
    unsafe {
        asm!(
                "adrp x4, {vector_table}",
                "add  x4, x4, #:lo12:{vector_table}",
                "msr VBAR_EL1, x4",
                "isb sy",
                "dsb sy",
                vector_table = sym vector_table,
                out("x4") vts,
                options(nomem, nostack),
        )
    }
    println!("vector_table_symbol (hmm..): {:#x}", vts);

    let gic = init_gic();
    timer::init_timer(&gic);
    irq_enable();

    // // Scheduler Interrupt
    // let resched_sgi = IntId::sgi(RESCHED_SGI);
    // gic.set_interrupt_priority(resched_sgi, 0x00);
    // gic.enable_interrupt(resched_sgi, true);
    // unsafe {
    //     IRQ_NAMES[u32::from(resched_sgi) as usize] = Some("Scheduler");
    // }

    unsafe {
        GIC.set(gic).unwrap();
    }
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
    GicV3::set_priority_mask(0xff);
    gic.setup();

    gic
}

pub fn test_segfault() {
    let addr: u64 = 4 * 1024 * 1024 * 1024;
    info!("Trying to read from address {:#}GiB", addr >> 30);
    unsafe { core::ptr::read_volatile(addr as *mut u64) };
    info!("Survived");

    let addr: u64 = 8 * 1024 * 1024 * 1024;
    info!("Trying to read from address {:#}GiB", addr >> 30);
    unsafe { core::ptr::read_volatile(addr as *mut u64) };
    info!("Survived");
}

pub fn test_sgi() {
    // Testing Interrupt
    info!("Testing Software Generated Interrupt(SGI)");

    // Configure an SGI(Software Generated Interrupt) and then send it to ourself.
    let sgi_id = IntId::sgi(3);

    let gic = unsafe { GIC.get_mut().unwrap() };
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

pub fn test_svc() {
    unsafe {
        asm!("svc 0x80");
    }
}
