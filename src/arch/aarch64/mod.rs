pub mod console;
pub mod constants;
pub mod drivers;
pub mod dtb;
pub mod exception;
pub mod mm;
pub mod start;
pub mod test;
pub mod timer;

pub use constants::*;
pub use exception::irq;

use aarch64_cpu::asm;

pub fn get_cpus() -> usize {
    dtb::get_dtb()
        .enum_subnodes("/cpus")
        .filter(|cpu| cpu.split('@').next().unwrap() == "cpu")
        .count()
}

pub fn halt() {
    asm::wfi();
}
