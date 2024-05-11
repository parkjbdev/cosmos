use super::{Handler, IRQ_HANDLERS, IRQ_NAMES};
use crate::arch::exception::GIC;
use arm_gic::gicv3::{GicV3, IntId, SgiTarget};
use core::{arch::asm, fmt::Display};

pub fn register_sgi(sgi: SGI) {
    sgi.register();
    sgi.enable();
}

pub fn send_sgi(id: u32) {
    GicV3::send_sgi(
        IntId::sgi(id),
        SgiTarget::List {
            affinity3: 0,
            affinity2: 0,
            affinity1: 0,
            target_list: 0b1,
        },
    );
}

pub(crate) struct SGI {
    id: u32,
    prio: u8,
    handler: Handler,
    name: &'static str,
}

#[allow(dead_code)]
impl SGI {
    pub fn new(id: u32, prio: u8, handler: Handler, name: &'static str) -> Self {
        let sgi = Self {
            id,
            prio,
            handler,
            name,
        };

        sgi
    }
    
    pub fn register(&self) {
        let gic = unsafe { GIC.get_mut().unwrap() };
        let sgi_id = IntId::sgi(self.id);
        gic.set_interrupt_priority(sgi_id, self.prio);

        unsafe {
            IRQ_NAMES[self.id as usize] = Some(self.name);
            IRQ_HANDLERS[self.id as usize] = Some(self.handler);
        }

        unsafe {
            asm!("dsb nsh", "isb", options(nostack, nomem, preserves_flags));
        }
    }

    pub fn enable(&self) {
        let gic = unsafe { GIC.get_mut().unwrap() };
        gic.enable_interrupt(IntId::sgi(self.id), true);
    }

    pub fn disable(&self) {
        let gic = unsafe { GIC.get_mut().unwrap() };
        gic.enable_interrupt(IntId::sgi(self.id), false);
    }

    pub fn set_priority(&self, prio: u8) {
        let gic = unsafe { GIC.get_mut().unwrap() };
        gic.set_interrupt_priority(IntId::sgi(self.id), prio);
    }

    pub fn send(&self, target: Option<SgiTarget>) {
        let target: SgiTarget = match target {
            Some(target) => target,
            _ => SgiTarget::List {
                affinity3: 0,
                affinity2: 0,
                affinity1: 0,
                target_list: 0b1,
            },
        };
        GicV3::send_sgi(IntId::sgi(self.id), target);
    }
}

impl Display for SGI {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let id: u32 = self.id;
        let name = self.name;

        writeln!(f, "Received IRQ name: {} ({:?})", name, id)
    }
}

// pub(crate) struct PPI {

// }

// pub(crate) struct SPI {

// }


