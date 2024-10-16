use super::error::MMUEnableError;
use crate::memory::types::address::*;

pub trait MMU {
    /// init should enable the MMU and caching
    fn init(&self, phys_table_baddr: Address<Physical>) -> Result<(), MMUEnableError>;

    /// Returns whether the MMU is enabled
    fn is_enabled(&self) -> bool;
}
