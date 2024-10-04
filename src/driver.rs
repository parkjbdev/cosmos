pub mod interface {
    pub trait DeviceDriver {
        fn init(&self) -> Result<(), &'static str> {
            Ok(())
        }
    }
}
