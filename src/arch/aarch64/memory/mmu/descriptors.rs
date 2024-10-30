use crate::memory::types::Granule64KB;
use crate::memory::types::*;
use aarch64_cpu::registers::{Readable, Writeable};
use tock_registers::{register_bitfields, registers::InMemoryRegister};

// A level 3 page descriptor, as per ARMv8-A Architecture Reference Manual Figure D5-17.
register_bitfields! {u64,
    pub STAGE1_PAGE_DESCRIPTOR [
        /// Unprivileged execute-never.
        UXN      OFFSET(54) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Privileged execute-never.
        PXN      OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Physical address of the next table descriptor (lvl2) or the page descriptor (lvl3).
        OUTPUT_ADDR_64KiB OFFSET(16) NUMBITS(32) [], // [47:16]

        /// Access flag.
        AF       OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Shareability field.
        SH       OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],

        /// Access Permissions.
        AP       OFFSET(6) NUMBITS(2) [
            RW_EL1 = 0b00,
            RW_EL1_EL0 = 0b01,
            RO_EL1 = 0b10,
            RO_EL1_EL0 = 0b11
        ],

        /// Memory attributes index into the MAIR_EL1 register.
        AttrIndx OFFSET(2) NUMBITS(3) [],

        TYPE     OFFSET(1) NUMBITS(1) [
            Reserved_Invalid = 0,
            Page = 1
        ],

        VALID    OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageDescriptor {
    value: u64,
}

impl PageDescriptor {
    pub const fn new() -> Self {
        Self { value: 0 }
    }

    /// Create an instance.
    pub fn from_output_page_addr(
        phys_output_page_addr: PageAddress<Physical>,
        attribute_fields: &AttributeFields,
    ) -> Self {
        let val = InMemoryRegister::<u64, STAGE1_PAGE_DESCRIPTOR::Register>::new(0);

        let shifted = phys_output_page_addr.value() >> Granule64KB::SHIFT;
        val.write(
            STAGE1_PAGE_DESCRIPTOR::OUTPUT_ADDR_64KiB.val(shifted as u64)
                + STAGE1_PAGE_DESCRIPTOR::AF::True
                + STAGE1_PAGE_DESCRIPTOR::TYPE::Page
                + STAGE1_PAGE_DESCRIPTOR::VALID::True
                + (*attribute_fields).into(),
        );

        Self { value: val.get() }
    }

    pub fn is_valid(&self) -> bool {
        InMemoryRegister::<u64, STAGE1_PAGE_DESCRIPTOR::Register>::new(self.value)
            .is_set(STAGE1_PAGE_DESCRIPTOR::VALID)
    }
}

// A table descriptor, as per ARMv8-A Architecture Reference Manual Figure D5-15.
register_bitfields! {u64,
    pub STAGE1_TABLE_DESCRIPTOR [
        /// Physical address of the next descriptor.
        NEXT_LEVEL_TABLE_ADDR_64KiB OFFSET(16) NUMBITS(32) [], // [47:16]

        TYPE  OFFSET(1) NUMBITS(1) [
            Block = 0,
            Table = 1
        ],

        VALID OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableDescriptor {
    value: u64,
}

impl TableDescriptor {
    pub const fn new() -> Self {
        Self { value: 0 }
    }

    pub fn from_next_level_table_addr(phys_next_level_table_addr: Address<Physical>) -> Self {
        let val = InMemoryRegister::<u64, STAGE1_TABLE_DESCRIPTOR::Register>::new(0);
        let shifted = phys_next_level_table_addr.value() >> Granule64KB::SHIFT;
        val.write(
            STAGE1_TABLE_DESCRIPTOR::NEXT_LEVEL_TABLE_ADDR_64KiB.val(shifted as u64)
                + STAGE1_TABLE_DESCRIPTOR::TYPE::Table
                + STAGE1_TABLE_DESCRIPTOR::VALID::True,
        );
        Self { value: val.get() }
    }
}
