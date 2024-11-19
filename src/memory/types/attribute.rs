use core::fmt::Display;

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

impl Display for MemoryAttributes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let attr = match self {
            MemoryAttributes::CacheableDRAM => "C",
            MemoryAttributes::Device => "Dev",
        };
        write!(f, "{}", attr)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq)]
pub enum AccessPermissions {
    RO = 0b100,  // 4
    RX = 0b101,  // 5
    RW = 0b110,  // 6
    RWX = 0b111, // 7
}

impl Display for AccessPermissions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let acc_p = match self {
            AccessPermissions::RO => "RO",
            AccessPermissions::RX => "RX",
            AccessPermissions::RW => "RW",
            AccessPermissions::RWX => "RWX",
        };
        write!(f, "{}", acc_p)
    }
}
