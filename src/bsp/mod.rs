#[cfg(feature = "qemu-virt")]
pub mod driver;
pub mod virt;

#[cfg(feature = "qemu-virt")]
pub use driver::arm::*;
pub use virt::*;
