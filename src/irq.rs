use core::arch::asm;
use arm_gic::{
    gicv3::{GicV3, IntId, SgiTarget}, irq_disable, irq_enable
};
use log::{info, debug};

pub fn init() {
    const GICD_BASE_ADDRESS: *mut u64 = 0x800_0000 as _;
    const GICR_BASE_ADDRESS: *mut u64 = 0x80A_0000 as _;

    let mut gic = unsafe { GicV3::new(GICD_BASE_ADDRESS, GICR_BASE_ADDRESS)};
    gic.setup();

    // Testing Interrupt
    // Configure an SGI and then send it to ourself.
    let sgi_intid = IntId::sgi(3);
    GicV3::set_priority_mask(0xff);
    gic.set_interrupt_priority(sgi_intid, 0x80);
    gic.enable_interrupt(sgi_intid, true);
    irq_enable();
    GicV3::send_sgi(
        sgi_intid,
        SgiTarget::List {
            affinity3: 0,
            affinity2: 0,
            affinity1: 0,
            target_list: 0b1,
        },
    );
}

// ARM DAIF Documentation
// https://developer.arm.com/documentation/ddi0601/2023-12/AArch64-Registers/DAIF--Interrupt-Mask-Bits?lang=en#fieldset_0-63_10

#[inline]
pub fn disable() {
    unsafe {
        asm!("msr DAIFSet, #0xf", options(nomem, nostack));
    }
}

pub fn enable() {
    unsafe {
        asm!("msr DAIFClr, #0xf", options(nomem, nostack));
    }
}

pub fn halt() {
    unsafe {
        asm!("wfi");
    }
}

pub fn safe_halt() {
    unsafe {
        asm!("sti;hlt");
    }
}

const MAX_HANDLERS: usize = 256;
type Handler = fn() -> bool;

pub fn install_handler(irq_number: u8, handler: Handler) {
    debug!("Install handler for interrupt {}", irq_number);

}
