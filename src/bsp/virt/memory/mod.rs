use crate::{
    memory::{
        address_space::{AddressSpace, AssociatedTranslationTable},
        translation_granule::TranslationGranule,
        translation_table::kernel_map_at,
        types::memory::{AccessPermissions, AttributeFields, MemoryAttributes},
    },
    sync::null_lock::NullLock,
};

type KernelTranslationTable =
    <KernelVirtAddrSpace as AssociatedTranslationTable>::TableStartFromBottom;

pub type KernelGranule = TranslationGranule<{ 64 * 1024 }>;
pub type KernelVirtAddrSpace = AddressSpace<{ 1 << 39 }>;

pub static KERNEL_TABLES: NullLock<KernelTranslationTable> =
    NullLock::new(KernelTranslationTable::new());

pub fn kernel_tables() -> &'static NullLock<KernelTranslationTable> {
    &KERNEL_TABLES
}

pub(crate) fn kernel_map_binary() -> Result<(), &'static str> {
    kernel_map_at(
        "Kernel .text Section",
        virt_region,
        phys_region,
        &AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RO,
        },
    );

    kernel_map_at(
        "Kernel .rodata Section",
        virt_region,
        phys_region,
        &AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RO,
        },
    );

    kernel_map_at(
        "Kernel .data Section",
        virt_region,
        phys_region,
        &AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RW,
        },
    );
    kernel_map_at(
        "Kernel .bss Section",
        virt_region,
        phys_region,
        &AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RW,
        },
    );
    kernel_map_at(
        "Kernel BootCore Stack",
        virt_region,
        phys_region,
        &AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RW,
        },
    );

    // TODO: set kernel map
    Ok(())
}
