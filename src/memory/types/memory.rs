use super::{address::AddressType, page::PageAddress};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq)]
pub struct MemoryRegion<ADDRESS_TYPE: AddressType> {
    start: PageAddress<ADDRESS_TYPE>,
    end: PageAddress<ADDRESS_TYPE>,
}

#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq)]
pub struct AttributeFields {
    pub memory_attributes: MemoryAttributes,
    pub access_permissions: AccessPermissions,
}

#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq)]
pub enum MemoryAttributes {
    CacheableDRAM,
    Device,
}

#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq)]
pub enum AccessPermissions {
    RO, // 4
    RX, // 5
    RW, // 6
}


