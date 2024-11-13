use crate::driver::interface::DeviceDriver;

pub mod devicetree;
pub mod gic;
pub mod pl011;
pub mod timer;

pub fn init_drivers() {
    // 순서 변경금지
    // 1. DTB
    // 2. GOC
    // 3. PL011
    // 그 이후 무관

    devicetree::DeviceTreeDriver.init();
    gic::GicDriver.init();
    pl011::PL011UartDriver.init();

    // pl011::PL011UartDriver.init();
    // pl011::PL011UartDriver::init();
    // pl011::PL011UartDriver.init();
    // pl011::PL011Driver.register_from_devicetree_and_enable_irq_handler();
    // pl011::PL011Driver.register_and_enable_irq_handler();
    timer::TimerDriver.init();
}
