use super::descriptors::{PageDescriptor, TableDescriptor};
use crate::memory::{self, types::*};

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
pub struct FixedSizeTranslationTable<const NUM_ENTRIES: usize> {
    l3: [[PageDescriptor; 8192]; NUM_ENTRIES],
    l2: [TableDescriptor; NUM_ENTRIES],
    initialized: bool,
}

impl<const NUM_TABLES: usize> FixedSizeTranslationTable<NUM_TABLES> {
    /// Helper to calculate the lvl2 and lvl3 indices from an address.
    #[inline(always)]
    fn lvl2_lvl3_index_from_page_addr(
        &self,
        virt_page_addr: PageAddress<Virtual>,
    ) -> Result<(usize, usize), &'static str> {
        let addr = virt_page_addr.value();
        let lvl2_index = addr >> Granule512MB::SHIFT;
        let lvl3_index = (addr & Granule512MB::MASK) >> Granule64KB::SHIFT;

        if lvl2_index > (NUM_TABLES - 1) {
            return Err("Virtual page is out of bounds of translation table");
        }

        Ok((lvl2_index, lvl3_index))
    }

    /// Sets the PageDescriptor corresponding to the supplied page address.
    ///
    /// Doesn't allow overriding an already valid page.
    #[inline(always)]
    fn set_page_descriptor_from_page_addr(
        &mut self,
        virt_page_addr: PageAddress<Virtual>,
        new_desc: &PageDescriptor,
    ) -> Result<(), &'static str> {
        let (lvl2_index, lvl3_index) = self.lvl2_lvl3_index_from_page_addr(virt_page_addr)?;
        let desc = &mut self.l3[lvl2_index][lvl3_index];

        if desc.is_valid() {
            return Err("Virtual page is already mapped");
        }

        *desc = *new_desc;
        Ok(())
    }
}

impl<const NUM_TABLES: usize> memory::translation_table::interface::TranslationTable
    for FixedSizeTranslationTable<NUM_TABLES>
{
    fn init(&mut self) {
        if self.initialized {
            return;
        }
        self.populate();
        self.initialized = true;
    }
    fn phys_base_addr(&self) -> Address<Physical> {
        self.l2.phys_start_addr()
    }

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

        let iter = phys_region.into_iter().zip(virt_region.into_iter());
        for (phys_page_addr, virt_page_addr) in iter {
            let new_desc = PageDescriptor::from_output_page_addr(phys_page_addr, attributes);
            let virt_page = virt_page_addr;
            self.set_page_descriptor_from_page_addr(virt_page, &new_desc)?;
        }

        Ok(())
    }
}

impl<const NUM_TABLES: usize> FixedSizeTranslationTable<NUM_TABLES> {
    pub const fn new() -> Self {
        Self {
            l3: [[PageDescriptor::new(); 8192]; NUM_TABLES],
            l2: [TableDescriptor::new(); NUM_TABLES],
            initialized: false,
        }
    }

    fn populate(&mut self) -> Result<(), &'static str> {
        for (l2_idx, l2_entry) in self.l2.iter_mut().enumerate() {
            let phys_table_addr = self.l3[l2_idx].phys_start_addr();
            *l2_entry = TableDescriptor::from_next_level_table_addr(phys_table_addr);
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
