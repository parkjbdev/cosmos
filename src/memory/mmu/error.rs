use core::fmt::{Debug, Display};

pub enum MMUEnableError {
    AlreadyEnabled,
    InvalidGranuleSize(usize),
    GranuleNotSupported(usize),
    Other(&'static str),
}

impl Debug for MMUEnableError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MMUEnableError::AlreadyEnabled => write!(f, "MMU is already enabled"),
            MMUEnableError::InvalidGranuleSize(size) => write!(f, "Invalid Granule Size: {}", size),
            MMUEnableError::GranuleNotSupported(size) => {
                write!(f, "Granule size {} is not supported", size)
            }
            MMUEnableError::Other(e) => write!(f, "{}", e),
        }
    }
}

impl Display for MMUEnableError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MMUEnableError::AlreadyEnabled => write!(f, "MMU is already enabled"),
            MMUEnableError::InvalidGranuleSize(size) => write!(f, "Invalid Granule Size: {}", size),
            MMUEnableError::GranuleNotSupported(size) => {
                write!(f, "Granule size {} is not supported", size)
            }
            MMUEnableError::Other(e) => write!(f, "{}", e),
        }
    }
}
