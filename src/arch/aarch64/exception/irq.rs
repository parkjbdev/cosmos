use aarch64_cpu::registers::*;
use tock_registers::interfaces::ReadWriteable;

pub fn exec_with_irq_disabled<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let daif = DAIF.get();
    let ret = f();
    DAIF.set(daif);
    ret
}

pub fn irq_enable() {
    DAIF.modify(DAIF::I::Unmasked);
}

pub fn irq_disable() {
    DAIF.modify(DAIF::I::Masked);
}

pub fn fiq_enable() {
    DAIF.modify(DAIF::F::Unmasked);
}

pub fn fiq_disable() {
    DAIF.modify(DAIF::F::Masked);
}
