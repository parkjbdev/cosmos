use crate::arch::drivers::pl011;

pub fn init_irq() {
    pl011::init_irq();
}
