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
    RO = 0b100,  // 4
    RX = 0b101,  // 5
    RW = 0b110,  // 6
    RWX = 0b111, // 7
}
