use super::devicetree;
use crate::{arch::irq::GIC, driver};
use arm_gic::gicv3::GicV3;

pub struct GicDriver;

impl driver::interface::DeviceDriver for GicDriver {
    fn init(&mut self) -> Result<(), &'static str> {
        let compat = self.compatible();
        if !compat.contains("arm,gic-v3") {
            panic!("Compatible GIC (arm,gic-v3) Not Found");
        }

        // Parse GICD & GICC from the devicetree /intc reg
        let reg = devicetree::get_property("/intc", "reg").unwrap();

        // GIC Distributor interface (GICD)
        let (slice, residual_slice) = reg.split_at(core::mem::size_of::<u64>());
        let gicd_start = u64::from_be_bytes(slice.try_into().unwrap());
        let (slice, residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
        let gicd_size = u64::from_be_bytes(slice.try_into().unwrap());

        // GIC Redistributors (GICR), one range per redistributor region
        let (slice, residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
        let gicr_start = u64::from_be_bytes(slice.try_into().unwrap());
        let (slice, _residual_slice) = residual_slice.split_at(core::mem::size_of::<u64>());
        let gicr_size = u64::from_be_bytes(slice.try_into().unwrap());

        let gicd_start: *mut u64 = gicd_start as _;
        let gicr_start: *mut u64 = gicr_start as _;

        // TODO: allocate gicd and gicr to virtualmem
        let mut gic = unsafe { GicV3::new(gicd_start, gicr_start) };
        GicV3::set_priority_mask(0xff);
        gic.setup();

        let result = unsafe { GIC.set(gic) };

        Ok(())
    }

    fn compatible(&self) -> &str {
        core::str::from_utf8(devicetree::get_property("/intc", "compatible").unwrap()).unwrap()
        // "arm,gic-v3"
    }

    // GIC itself does not have IRQ handler
    fn register_from_devicetree_and_enable_irq_handler(&self) {}
    fn register_and_enable_irq_handler(&self, interrupt: crate::arch::irq::Interrupt) {}
}
