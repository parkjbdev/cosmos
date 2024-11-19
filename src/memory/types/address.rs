use core::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops,
};

use crate::{bsp, memory::align};
use super::MemorySize;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Physical;
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Virtual;

pub trait AddressType: Copy + Clone + PartialOrd + PartialEq + Ord + Eq {
    const PREFIX: &'static str;
}

impl AddressType for Physical {
    const PREFIX: &'static str = "PA";
}

impl AddressType for Virtual {
    const PREFIX: &'static str = "VA";
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Address<T: AddressType> {
    value: usize,
    _marker: PhantomData<T>,
}

impl<T: AddressType> Address<T> {
    pub const fn new(value: usize) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    pub const fn value(&self) -> usize {
        self.value
    }

    pub const fn align_down_page(self) -> Self {
        let aligned = align::align_down(self.value, bsp::memory::KernelGranule::SIZE);

        Self::new(aligned)
    }

    pub const fn align_up_page(self) -> Self {
        let aligned = align::align_up(self.value, bsp::memory::KernelGranule::SIZE);

        Self::new(aligned)
    }
}

impl<T: AddressType> ops::Add<Address<T>> for Address<T> {
    type Output = Address<T>;

    fn add(self, rhs: Address<T>) -> Self::Output {
        Address::new(self.value + rhs.value)
    }
}

impl<T: AddressType> ops::Sub<Address<T>> for Address<T> {
    type Output = MemorySize;

    fn sub(self, rhs: Address<T>) -> Self::Output {
        MemorySize(self.value - rhs.value)
    }
}

impl<T: AddressType> fmt::Display for Address<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let q4: u16 = ((self.value >> 48) & 0xffff) as u16;
        let q3: u16 = ((self.value >> 32) & 0xffff) as u16;
        let q2: u16 = ((self.value >> 16) & 0xffff) as u16;
        let q1: u16 = (self.value & 0xffff) as u16;

        write!(f, "0x")?;
        if q4 != 0 {
            write!(f, "{:04x}_", q4)?;
        }
        if q3 != 0 {
            write!(f, "{:04x}_", q3)?;
        }
        if q2 != 0 {
            write!(f, "{:04x}_", q2)?;
        }
        write!(f, "{:04x}", q1)
    }
}

impl<T: AddressType> fmt::LowerHex for Address<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.value, f) // delegate to i32's implementation
    }
}
