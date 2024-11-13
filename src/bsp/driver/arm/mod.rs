use aarch64_cpu::asm::barrier;
use pl011::CONSOLE;
use crate::{arch, driver::interface::DeviceDriver};

pub mod devicetree;
pub mod gic;
pub mod pl011;
pub mod timer;

pub fn init_drivers() {
    devicetree::init(0x40000000);
    gic::GicDriver.init();
    barrier::isb(barrier::SY);

    let uart_addr = {
        let stdout = devicetree::get_property("/chosen", "stdout-path").unwrap();
        core::str::from_utf8(stdout)
            .unwrap()
            .trim_matches(char::from(0))
            .split_once('@')
            .map(|(_, addr)| u32::from_str_radix(addr, 16).unwrap())
            .unwrap()
    }; // 0x0900_0000
    pl011::init(uart_addr);
    pl011::init_irq();

    timer::TimerDriver.init();
    timer::TimerDriver.register_from_devicetree_and_enable_irq_handler();
}
