use core::arch::asm;

use arm_gic::gicv3::{GicV3, IntId};
use log::{error, info};

use crate::arch::{dtb::get_dtb, state::State};

fn timer_handler(_state: &State) -> bool {
    info!("Handle Timer Interrupt");

    // TODO: Handle timer
    unsafe {
        asm!(
            "msr cntp_cval_el0, xzr",
            "msr cntp_ctl_el0, xzr",
            options(nostack, nomem)
        );
    }
    true
}

pub fn init_timer(gic: &GicV3) {
    let dtb = get_dtb();
    let timer_compatible =
        core::str::from_utf8(dtb.get_property("/timer", "compatible").unwrap()).unwrap();
    if !timer_compatible.contains("armv8-timer") {
        error!("Compatible Timer (armv8-timer) Not Found");
        return;
    }
    // armv8-timer found..
    // parse timer interrupts
    let timer_interrupts = dtb.get_property("/timer", "interrupts").unwrap();
    const split_size: usize = core::mem::size_of::<u32>();

    let chunks: &[[u8; split_size]] = unsafe { timer_interrupts.as_chunks_unchecked() };
    let timer_secure = Timer {
        irq_type: u32::from_be_bytes(chunks[0]),
        irq_num: u32::from_be_bytes(chunks[1]),
        irq_flag: u32::from_be_bytes(chunks[2]),
    };
    let timer_nonsecure = Timer {
        irq_type: u32::from_be_bytes(chunks[3]),
        irq_num: u32::from_be_bytes(chunks[4]),
        irq_flag: u32::from_be_bytes(chunks[5]),
    };
    let timer_virtual = Timer {
        irq_type: u32::from_be_bytes(chunks[6]),
        irq_num: u32::from_be_bytes(chunks[7]),
        irq_flag: u32::from_be_bytes(chunks[8]),
    };
    let timer_hypervisor = Timer {
        irq_type: u32::from_be_bytes(chunks[9]),
        irq_num: u32::from_be_bytes(chunks[10]),
        irq_flag: u32::from_be_bytes(chunks[11]),
    };
}

pub struct Timer {
    irq_type: u32,
    irq_num: u32,
    irq_flag: u32,
}

impl Timer {
    fn to_intid(self) -> IntId {
        if self.irq_type == 0 {
            IntId::spi(self.irq_num)
        } else if self.irq_type == 1 {
            IntId::ppi(self.irq_num)
        } else {
            IntId::sgi(self.irq_num)
        }
    }
    fn set_prio(self, prio: u8, gic: &GicV3) {
        // gic.set_interrupt_priority(self.to_intid(), prio);
    }
}
