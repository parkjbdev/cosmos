pub mod error;
pub mod interface;
pub mod page_alloc;

pub use error::*;

use crate::{arch, bsp};
use interface::MMU;

use super::types::*;

pub fn init(phys_table_baddr: Address<Physical>) -> Result<(), self::error::MMUEnableError> {
    arch::memory::mmu().init(phys_table_baddr)
}

pub fn init_mmio_allocator() {
    let region = {
        let (start_addr, end_addr) = bsp::memory::symbols::mmio_remap_region();
        let start_page_addr: PageAddress<Virtual> = PageAddress::from(start_addr);
        let end_page_addr: PageAddress<Virtual> = PageAddress::from(end_addr);

        MemoryRegion::new(start_page_addr, end_page_addr)
    };

    page_alloc::kernel_va_allocator().lock().init(region)
}
