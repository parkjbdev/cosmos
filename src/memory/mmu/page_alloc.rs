use super::{Address, AddressType, MemoryRegion, PageAddress, Virtual};
use crate::bsp;
use core::num::NonZeroUsize;
use spin::Mutex;

static KERNEL_VA_ALLOCATOR: Mutex<PageAllocator<Virtual>> = Mutex::new(PageAllocator::new());

pub fn kernel_va_allocator() -> &'static Mutex<PageAllocator<Virtual>> {
    &KERNEL_VA_ALLOCATOR
}

pub struct PageAllocator<T: AddressType> {
    pool: Option<MemoryRegion<T>>,
}

impl<T: AddressType> PageAllocator<T> {
    pub const fn new() -> Self {
        Self { pool: None }
    }

    pub fn init(&mut self, pool: MemoryRegion<T>) {
        self.pool = Some(pool);
    }

    pub fn alloc(&mut self, num_pages: NonZeroUsize) -> Result<MemoryRegion<T>, &'static str> {
        let num_pages: usize = num_pages.into();
        let pool = self.pool.as_mut().expect("Allocator not initialized");

        let delta: usize = num_pages * bsp::memory::KernelGranule::SIZE;
        let left_end_addr = Address::new(pool.start_page_addr().value() + delta);
        let left_end_page = PageAddress::new(left_end_addr);

        let allocation = MemoryRegion::new(pool.start_page_addr(), left_end_page);
        self.pool.as_mut().unwrap().set_start_page(left_end_page);
        Ok(allocation)
    }
}
