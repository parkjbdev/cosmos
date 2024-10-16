use super::descriptors::{PageDescriptor, TableDescriptor};
use crate::{
    arch::memory::translation_granule::Granule512MiB,
    memory::{
        self,
        translation_table::interface::TranslationTable, types::address::{Address, Physical},
    },
};

pub(super) trait StartAddr {
    fn phys_start_addr(&self) -> Address<Physical>;
}

impl<T, const N: usize> StartAddr for [T; N] {
    fn phys_start_addr(&self) -> Address<Physical> {
        Address::new(self as *const _ as u64)
    }
}

#[repr(C)]
#[repr(align(65536))]
pub struct FixedSizeTranslationTable<const NUM_ENTRIES: usize> {
    l3: [[PageDescriptor; 8192]; NUM_ENTRIES],
    l2: [TableDescriptor; NUM_ENTRIES],
    initialized: bool,
}

impl<const NUM_TABLES: usize> TranslationTable for FixedSizeTranslationTable<NUM_TABLES> {
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
    [u8; Self::SIZE >> Granule512MiB::SHIFT]: Sized,
{
    type TableStartFromBottom = FixedSizeTranslationTable<{ Self::SIZE >> Granule512MiB::SHIFT }>;
}
