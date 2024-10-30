pub mod error;
pub mod interface;

pub use error::*;

use crate::arch;
use interface::MMU;

use super::types::*;

pub fn init(phys_table_baddr: Address<Physical>) -> Result<(), self::error::MMUEnableError> {
    arch::memory::mmu().init(phys_table_baddr)
}

pub fn post_init() {}
