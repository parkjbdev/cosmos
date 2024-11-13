pub mod symbols;

use crate::{
    memory::{
        address_space::{AddressSpace, AssociatedTranslationTable},
        kernel_mapper::kernel_map_at,
        types::*,
    },
    sync::null_lock::NullLock,
};
use log_crate::info;

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
    let text_page_count = (end_addr - start_addr) >> KernelGranule::SHIFT;
    let start_page: PageAddress<Virtual> = PageAddress::from(start_addr);
    let end_page = start_page.offset(text_page_count as isize).unwrap();
    MemoryRegion::<Virtual>::new(start_page, end_page)
}

fn physical_region_of(virt_region: MemoryRegion<Virtual>) -> MemoryRegion<Physical> {
    MemoryRegion::<Physical>::new(
        PageAddress::from(virt_region.start_page_addr().inner().value()),
        PageAddress::from(virt_region.end_page_addr().inner().value()),
    )
}

pub(crate) fn kernel_map_binary() -> Result<(), &'static str> {
    let virt_region = virtual_region_of(0, 0x4010_0000);
    let phys_region = physical_region_of(virt_region);
    info!("      {: <15}: [{:#x} ~ {:#x}]", "mmio", 0, 0x4010_0000);
    kernel_map_at(
        "MMIO",
        &virt_region,
        &phys_region,
        &AttributeFields {
            access_permissions: AccessPermissions::RO,
            memory_attributes: MemoryAttributes::Device,
        },
    );

    let (start_addr, end_addr) = self::symbols::text();
    let virt_region = virtual_region_of(start_addr, end_addr);
    let phys_region = physical_region_of(virt_region);
    info!(
        "      {: <15}: [{:#x} ~ {:#x}]",
        ".text", start_addr, end_addr
    );

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
    info!(
        "      {: <15}: [{:#x} ~ {:#x}]",
        ".rodata", start_addr, end_addr
    );

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
    info!(
        "      {: <15}: [{:#x} ~ {:#x}]",
        ".data", start_addr, end_addr
    );

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
    info!(
        "      {: <15}: [{:#x} ~ {:#x}]",
        ".bss", start_addr, end_addr
    );

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
    info!(
        "      {: <15}: [{:#x} ~ {:#x}]",
        "bootcore", start_addr, end_addr
    );

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
