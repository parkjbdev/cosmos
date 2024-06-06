// Exception Level

use aarch64_cpu::registers::*;
use core::fmt::Display;

#[derive(Eq, PartialEq)]
pub enum ExceptionLevel {
    EL3,
    EL2,
    EL1,
    EL0,
}

impl Display for ExceptionLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ExceptionLevel::EL3 => write!(f, "EL3"),
            ExceptionLevel::EL2 => write!(f, "EL2"),
            ExceptionLevel::EL1 => write!(f, "EL1"),
            ExceptionLevel::EL0 => write!(f, "EL0"),
        }
    }
}

pub fn get_current_el() -> ExceptionLevel {
    let el = CurrentEL.read(CurrentEL::EL);
    match el {
        3 => ExceptionLevel::EL3,
        2 => ExceptionLevel::EL2,
        1 => ExceptionLevel::EL1,
        0 => ExceptionLevel::EL0,
        _ => panic!("Invalid Exception Level"),
    }
}
