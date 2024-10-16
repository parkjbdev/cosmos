#[cfg(feature = "qemu-virt")]
mod virt;

#[cfg(feature = "qemu-virt")]
pub use virt::*;


