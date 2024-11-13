pub mod interface {
    use crate::arch::irq::Interrupt;

    pub trait DeviceDriver {
        fn init(&mut self) -> Result<(), &'static str>;

        fn compatible(&self) -> &str {
            todo!()
        }

        fn register_from_devicetree_and_enable_irq_handler(&self) {
            todo!()
        }

        fn register_and_enable_irq_handler(&self, interrupt: Interrupt) {
            todo!()
        }
    }
}
