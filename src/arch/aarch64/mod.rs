pub mod console;
pub mod exception;
pub mod memory;
pub mod start;
pub mod test;
pub mod timer;
pub mod semihosting;

use aarch64_cpu::asm;
use crate::drivers::devicetree;

pub fn get_cpus() -> usize {
    devicetree::enum_subnodes("/cpus")
        .filter(|cpu| cpu.split('@').next().unwrap() == "cpu")
        .count()
}

pub fn halt() {
    asm::wfi();
}
