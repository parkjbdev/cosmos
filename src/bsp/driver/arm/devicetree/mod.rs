use crate::{bsp::memory::symbols::DEVICE_TREE_START, driver, sync::spinlock::RawSpinlock};
use generic_once_cell::OnceCell;
use hermit_dtb::Dtb;

pub const DEVICE_TREE: OnceCell<RawSpinlock, Dtb> = OnceCell::new();

pub struct DeviceTreeDriver;

impl driver::interface::DeviceDriver for DeviceTreeDriver {
    fn init(&mut self) -> Result<(), &'static str> {
        DEVICE_TREE.set(unsafe {
            Dtb::from_raw(sptr::from_exposed_addr(DEVICE_TREE_START as usize)).unwrap()
        });
        Ok(())
    }

    fn compatible(&self) -> &str {
        "Device Tree"
    }

    // DTB does not have IRQ handler
    fn register_from_devicetree_and_enable_irq_handler(&self) {}
    fn register_and_enable_irq_handler(&self, interrupt: crate::arch::irq::Interrupt) {}
}
