use crate::{
    arch::memory::mmu::descriptors::STAGE1_PAGE_DESCRIPTOR,
    memory::types::{AccessPermissions, AttributeFields, MemoryAttributes},
};
use core::convert;

/// Constants for indexing the MAIR_EL1.
#[allow(dead_code)]
pub mod mair {
    pub const DEVICE: u64 = 0;
    pub const NORMAL: u64 = 1;
}

/// Convert the kernel's generic memory attributes to HW-specific attributes of the MMU.
impl convert::From<AttributeFields>
    for tock_registers::fields::FieldValue<u64, STAGE1_PAGE_DESCRIPTOR::Register>
{
    fn from(attribute_fields: AttributeFields) -> Self {
        // Memory attributes.
        let mut desc = match attribute_fields.memory_attributes {
            MemoryAttributes::CacheableDRAM => {
                STAGE1_PAGE_DESCRIPTOR::SH::InnerShareable
                    + STAGE1_PAGE_DESCRIPTOR::AttrIndx.val(mair::NORMAL)
            }
            MemoryAttributes::Device => {
                STAGE1_PAGE_DESCRIPTOR::SH::OuterShareable
                    + STAGE1_PAGE_DESCRIPTOR::AttrIndx.val(mair::DEVICE)
            }
        };

        // Access Permissions.
        desc += match attribute_fields.access_permissions {
            AccessPermissions::RO => STAGE1_PAGE_DESCRIPTOR::AP::RO_EL1,
            AccessPermissions::RW => STAGE1_PAGE_DESCRIPTOR::AP::RW_EL1,
            AccessPermissions::RX => STAGE1_PAGE_DESCRIPTOR::AP::RO_EL1,
            AccessPermissions::RWX => STAGE1_PAGE_DESCRIPTOR::AP::RW_EL1,
        };

        // The execute-never attribute is mapped to PXN in AArch64.
        desc += if attribute_fields.access_permissions as u8 & 0b001 == 0b001 {
            STAGE1_PAGE_DESCRIPTOR::PXN::True
        } else {
            STAGE1_PAGE_DESCRIPTOR::PXN::False
        };

        // Always set unprivileged exectue-never as long as userspace is not implemented yet.
        desc += STAGE1_PAGE_DESCRIPTOR::UXN::True;

        desc
    }
}
