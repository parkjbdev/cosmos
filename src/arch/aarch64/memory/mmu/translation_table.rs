use super::descriptors::{PageDescriptor, TableDescriptor};
use crate::{
    bsp::memory::symbols,
    memory::{self, types::*},
};

pub(super) trait StartAddr {
    fn phys_start_addr(&self) -> Address<Physical>;
}

impl<T, const N: usize> StartAddr for [T; N] {
    fn phys_start_addr(&self) -> Address<Physical> {
        Address::new(self as *const _ as usize)
    }
}

#[repr(C)]
#[repr(align(65536))]
pub struct FixedSizeTranslationTable<const NUM_TABLES: usize> {
    /// Table descriptors, covering 512 MiB windows.
    l2: [TableDescriptor; NUM_TABLES],

    /// Page descriptors, covering 64 KiB windows per entry.
    l3: [[PageDescriptor; 8192]; NUM_TABLES],

    /// Have the tables been initialized?
    pub initialized: bool,
}

impl<const NUM_TABLES: usize> FixedSizeTranslationTable<NUM_TABLES> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            l2: [TableDescriptor::new(); NUM_TABLES],
            l3: [[PageDescriptor::new(); 8192]; NUM_TABLES],
            initialized: false,
        }
    }

    /// Helper to calculate the lvl1 indices from an address.
    #[inline(always)]
    fn l1_idx(virt_page_addr: PageAddress<Virtual>) -> Result<usize, &'static str> {
        let addr = virt_page_addr.value();
        let idx = addr >> Granule4TB::SHIFT;
        if idx > (NUM_TABLES - 1) {
            return Err("Virtual page is out of bounds of translation table");
        }
        Ok(idx)
    }

    /// Helper to calculate the lvl2 indices from an address.
    #[inline(always)]
    fn l2_idx(virt_page_addr: PageAddress<Virtual>) -> Result<usize, &'static str> {
        let addr = virt_page_addr.value();
        let idx = (addr & Granule4TB::MASK) >> Granule512MB::SHIFT;
        if idx > (NUM_TABLES - 1) {
            return Err("Virtual page is out of bounds of translation table");
        }
        Ok(idx)
    }

    /// Helper to calculate the lvl3 indices from an address.
    #[inline(always)]
    fn l3_idx(virt_page_addr: PageAddress<Virtual>) -> Result<usize, &'static str> {
        let addr = virt_page_addr.value();
        let idx = (addr & Granule512MB::MASK) >> Granule64KB::SHIFT;
        Ok(idx)
    }

    /// Sets the PageDescriptor corresponding to the supplied page address.
    ///
    /// Doesn't allow overriding an already valid page.
    #[inline(always)]
    fn set_page_from_page_addr(
        &mut self,
        virt_page_addr: PageAddress<Virtual>,
        page: &PageDescriptor,
    ) -> Result<(), &'static str> {
        // let l1_idx = Self::l1_idx(virt_page_addr).unwrap();
        let l2_idx = Self::l2_idx(virt_page_addr).unwrap();
        let l3_idx = Self::l3_idx(virt_page_addr).unwrap();
        let offset = virt_page_addr.value() & Granule64KB::MASK;

        let desc = &mut self.l3[l2_idx][l3_idx];

        if desc.is_valid() {
            return Err("Virtual page is already mapped");
        }

        *desc = *page;
        Ok(())
    }

    #[inline(always)]
    fn get_page(&self, virt_page_addr: PageAddress<Virtual>) -> PageDescriptor {
        let l2_idx = Self::l2_idx(virt_page_addr).unwrap();
        let l3_idx = Self::l3_idx(virt_page_addr).unwrap();
        let desc = &self.l3[l2_idx][l3_idx];
        *desc
    }
}

impl<const NUM_TABLES: usize> memory::translation_table::interface::TranslationTable
    for FixedSizeTranslationTable<NUM_TABLES>
{
    #[inline(always)]
    fn init(&mut self) {
        if self.initialized {
            return;
        }

        // Populate the l2 entries.
        for (l2_idx, l2_entry) in self.l2.iter_mut().enumerate() {
            let phys_table_addr = self.l3[l2_idx].phys_start_addr();
            *l2_entry = TableDescriptor::from_next_level_table_addr(phys_table_addr);
        }

        self.initialized = true;
    }

    #[inline(always)]
    fn phys_base_addr(&self) -> Result<Address<Physical>, &'static str> {
        if !self.initialized {
            return Err("Translation table is not initialized");
        }
        Ok(self.l2.phys_start_addr())
    }

    #[inline(always)]
    fn map_at(
        &mut self,
        virt_region: &MemoryRegion<Virtual>,
        phys_region: &MemoryRegion<Physical>,
        attributes: &AttributeFields,
    ) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("Translation table is not initialized");
        }

        if virt_region.size() != phys_region.size() {
            return Err("Tried to map memory regions with unequal sizes");
        }

        if phys_region.end_page_addr().value() > symbols::kernel_range().end.value() {
            return Err("Physical region is out of bounds of translation table");
        }

        let iter = phys_region.into_iter().zip(virt_region.into_iter());
        for (phys_page_addr, virt_page_addr) in iter {
            let new_page_descriptor =
                PageDescriptor::from_output_page_addr(phys_page_addr, attributes);
            self.set_page_from_page_addr(virt_page_addr, &new_page_descriptor)?;
        }

        Ok(())
    }
}

impl<const SIZE: usize> memory::address_space::AssociatedTranslationTable
    for memory::address_space::AddressSpace<SIZE>
where
    [u8; Self::SIZE >> Granule512MB::SHIFT]: Sized,
{
    type TableStartFromBottom = FixedSizeTranslationTable<{ Self::SIZE >> Granule512MB::SHIFT }>;
}
