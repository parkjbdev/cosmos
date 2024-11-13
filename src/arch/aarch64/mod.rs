pub mod console;
pub mod devicetree;
pub mod drivers;
pub mod exception;
pub mod memory;
pub mod start;
pub mod test;
pub mod timer;

pub use exception::irq;

use aarch64_cpu::asm;

pub fn get_cpus() -> usize {
    devicetree::enum_subnodes("/cpus")
        .filter(|cpu| cpu.split('@').next().unwrap() == "cpu")
        .count()
}

pub fn halt() {
    asm::wfi();
}
