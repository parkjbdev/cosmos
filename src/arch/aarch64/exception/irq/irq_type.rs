use core::fmt::Display;

#[derive(Eq, PartialEq)]
pub enum InterruptType {
    PPI,
    SPI,
    SGI,
}

impl InterruptType {}

impl Display for InterruptType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InterruptType::PPI => write!(f, "PPI"),
            InterruptType::SPI => write!(f, "SPI"),
            InterruptType::SGI => write!(f, "SGI"),
        }
    }
}

