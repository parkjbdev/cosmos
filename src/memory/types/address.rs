use core::{
    fmt::{self, Debug, LowerHex},
    marker::PhantomData,
};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum Physical {}
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub enum Virtual {}

pub trait AddressType: Copy + Clone + PartialOrd + PartialEq + Ord + Eq {}
impl AddressType for Physical {}
impl AddressType for Virtual {}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Address<T: AddressType> {
    value: u64,
    _marker: PhantomData<T>,
}

impl<T: AddressType> Address<T> {
    pub fn new(value: u64) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
    pub fn value(&self) -> u64 {
        self.value
    }
}

impl fmt::Display for Address<Physical> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Physical ")?;
        write!(f, "{:#x}", self.value)
    }
}

impl fmt::Display for Address<Virtual> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Virtual ")?;
        write!(f, "{:#x}", self.value)
    }
}

impl LowerHex for Address<Physical> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.value)
    }
}

impl LowerHex for Address<Virtual> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.value)
    }
}
