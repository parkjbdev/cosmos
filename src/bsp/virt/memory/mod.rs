pub mod symbols;

use core::num::NonZeroUsize;

use crate::{
    bsp,
    memory::{
        address_space::{AddressSpace, AssociatedTranslationTable},
        align::{align_down, align_up},
        kernel_mapper::kernel_map_at,
        mmu::page_alloc::kernel_va_allocator,
        types::*,
    },
    sync::null_lock::NullLock,
};

type KernelTranslationTable =
    <KernelVirtAddrSpace as AssociatedTranslationTable>::TableStartFromBottom;

// 64 KiB granule size
pub type KernelGranule = TranslationGranule<{ 64 * 1024 }>;
// 16 GiB address space
pub type KernelVirtAddrSpace = AddressSpace<{ 1 << 34 }>;

pub static KERNEL_TABLES: NullLock<KernelTranslationTable> =
    NullLock::new(KernelTranslationTable::new());

pub fn kernel_tables() -> &'static NullLock<KernelTranslationTable> {
    &KERNEL_TABLES
}

fn virtual_region_of(start_addr: usize, end_addr: usize) -> MemoryRegion<Virtual> {
    let page_cnt = (end_addr - start_addr) >> KernelGranule::SHIFT;

    let start_page: PageAddress<Virtual> = PageAddress::from(start_addr);
    let end_page = start_page.offset(page_cnt as isize).unwrap();

    MemoryRegion::<Virtual>::new(start_page, end_page)
}

fn physical_region_of(virt_region: MemoryRegion<Virtual>) -> MemoryRegion<Physical> {
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

    kernel_map_at(
        name,
        &virt_region,
        &phys_region,
        &AttributeFields {
            memory_attributes: MemoryAttributes::Device,
            access_permissions: AccessPermissions::RW,
        },
    );

    virt_region.start_addr() + Address::<Virtual>::new(offset)
}

pub(crate) fn kernel_map_binary() -> Result<(), &'static str> {
    let (start_addr, end_addr) = (0x4000_0000, 0x4010_0000);
    let virt_region = virtual_region_of(start_addr, end_addr);
    let phys_region = physical_region_of(virt_region);
    kernel_map_at(
        "Kernel .devicetree Section",
        &virt_region,
        &phys_region,
        &AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RO,
        },
    );

    let (start_addr, end_addr) = self::symbols::text();
    let virt_region = virtual_region_of(start_addr, end_addr);
    let phys_region = physical_region_of(virt_region);
    kernel_map_at(
        "Kernel .text Section",
        &virt_region,
        &phys_region,
        &AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RX,
        },
    );

    let (start_addr, end_addr) = self::symbols::rodata();
    let virt_region = virtual_region_of(start_addr, end_addr);
    let phys_region = physical_region_of(virt_region);
    kernel_map_at(
        "Kernel .rodata Section",
        &virt_region,
        &phys_region,
        &AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RO,
        },
    );

    let (start_addr, end_addr) = self::symbols::data();
    let virt_region = virtual_region_of(start_addr, end_addr);
    let phys_region = physical_region_of(virt_region);
    kernel_map_at(
        "Kernel .data Section",
        &virt_region,
        &phys_region,
        &AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RW,
        },
    );

    let (start_addr, end_addr) = self::symbols::bss();
    let virt_region = virtual_region_of(start_addr, end_addr);
    let phys_region = physical_region_of(virt_region);
    kernel_map_at(
        "Kernel .bss Section",
        &virt_region,
        &phys_region,
        &AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RW,
        },
    );

    let (start_addr, end_addr) = self::symbols::boot_core_stack();
    let virt_region = virtual_region_of(start_addr, end_addr);
    let phys_region = physical_region_of(virt_region);
    kernel_map_at(
        "Kernel bootcore stack Section",
        &virt_region,
        &phys_region,
        &AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RW,
        },
    );

    Ok(())
}
