use core::arch::asm;

use arm_gic::gicv3::{GicV3, IntId, SgiTarget};
use log::info;

use crate::arch::exception::GIC;

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

struct Sgi(u32);
impl Sgi {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    pub fn enable(&self, gic: &mut GicV3, prio: u8) {
        gic.enable_interrupt(IntId::sgi(self.0), true);
    }
    pub fn disable(&self, gic: &mut GicV3) {
        gic.enable_interrupt(IntId::sgi(self.0), false);
    }
    pub fn set_priority(&self, gic: &mut GicV3, prio: u8) {
        gic.set_interrupt_priority(IntId::sgi(self.0), prio);
    }
    pub fn send(&self, target: SgiTarget) {
        GicV3::send_sgi(IntId::sgi(self.0), target);
    }
}

pub fn test_sgi() {
    // Testing Interrupt
    info!("Testing Software Generated Interrupt(SGI)");

    // Configure an SGI(Software Generated Interrupt) and then send it to ourself.
    let sgi_id = IntId::sgi(3);

    let gic = unsafe { GIC.get_mut().unwrap() };
    gic.set_interrupt_priority(sgi_id, 0x00);
    gic.enable_interrupt(sgi_id, true);

    unsafe {
        asm!("dsb nsh", "isb", options(nostack, nomem, preserves_flags));
    }

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
