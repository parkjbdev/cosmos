use crate::memory::types::*;

pub trait TranslationTable {
    fn init(&mut self);
    fn phys_base_addr(&self) -> Result<Address<Physical>, &'static str>;
    fn map_at(
        &mut self,
        virt_region: &MemoryRegion<Virtual>,
        phys_region: &MemoryRegion<Physical>,
        attributes: &AttributeFields,
    ) -> Result<(), &'static str>;
}
