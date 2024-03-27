pub mod irq;

use core::arch::global_asm;
use log::info;

global_asm!(include_str!("start.s"));

pub fn processor_init() {
    info!("Initializing Interrupt");
    irq::init();
    irq::enable();
}
