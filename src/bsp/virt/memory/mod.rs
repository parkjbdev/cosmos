pub mod symbols;

use crate::memory::{
    address_space::{AddressSpace, AssociatedTranslationTable},
    mmu::page_alloc::kernel_va_allocator,
    translation_table::interface::TranslationTable,
    types::*,
};
use core::num::NonZeroUsize;
use spin::RwLock;
use symbols::Section;

pub type KernelTranslationTable =
    <KernelVirtAddrSpace as AssociatedTranslationTable>::TableStartFromBottom;

// 64 KiB granule size
pub type KernelGranule = TranslationGranule<{ 64 * 1024 }>;
// 4 GiB address space
pub type KernelVirtAddrSpace = AddressSpace<{ 1 << 42 }>;

pub static KERNEL_TABLES: RwLock<KernelTranslationTable> =
    RwLock::new(KernelTranslationTable::new());

pub fn virtual_region_of(
    start_addr: Address<Physical>,
    end_addr: Address<Physical>,
) -> MemoryRegion<Virtual> {
    let page_cnt = (end_addr - start_addr).0 >> KernelGranule::SHIFT;

    let start_page = PageAddress::from(start_addr.into_virtual());
    let end_page = start_page.offset(page_cnt as isize).unwrap();

    MemoryRegion::<Virtual>::new(start_page, end_page)
}

pub fn physical_region_of(virt_region: MemoryRegion<Virtual>) -> MemoryRegion<Physical> {
    MemoryRegion::<Physical>::new(
        PageAddress::from(virt_region.start_page_addr().inner().value()),
        PageAddress::from(virt_region.end_page_addr().inner().value()),
    )
}

pub fn kernel_map_mmio(
    name: &'static str,
    start_addr: Address<Physical>,
    end_addr: Address<Physical>,
) -> Address<Virtual> {

    let start_addr = start_addr.align_down_page();
    let end_addr = end_addr.align_up_page();

    let start_page = PageAddress::from(start_addr);
    let end_page = PageAddress::from(end_addr);

    let offset = usize::from(start_addr) & KernelGranule::MASK;

    let phys_region = MemoryRegion::<Physical>::new(start_page, end_page);
    let num_pages = NonZeroUsize::new(usize::from(end_addr - start_addr) >> KernelGranule::SHIFT)
        .expect("num_pages are not NonZero");

    let virt_region = kernel_va_allocator().lock().alloc(num_pages).unwrap();


    __println!(
        "Mapping MMIO: {} [{:#x} ~ {:#x}], {} pages, {}",
        name,
        start_addr,
        end_addr,
        num_pages,
        virt_region
    );

    let _ = KERNEL_TABLES.write().map_at(
        &virt_region,
        &phys_region,
        &AttributeFields {
            memory_attributes: MemoryAttributes::Device,
            access_permissions: AccessPermissions::RW,
        },
    );

    virt_region.start_addr() + Address::<Virtual>::new(offset)
}

pub fn kernel_sections() -> [Section; 6] {
    let device_tree = symbols::device_tree();
    let text = symbols::text();
    let rodata = symbols::rodata();
    let data = symbols::data();
    let bss = symbols::bss();
    let boot_core_stack = symbols::boot_core_stack();

    [device_tree, text, rodata, data, bss, boot_core_stack]
}
