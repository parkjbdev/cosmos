use core::iter::Step;

use crate::bsp;

use super::address::{Address, AddressType};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq)]
pub struct PageAddress<ADDRESS_TYPE: AddressType> {
    inner: Address<ADDRESS_TYPE>,
}

#[allow(non_camel_case_types)]
impl<ADDRESS_TYPE: AddressType> PageAddress<ADDRESS_TYPE> {
    pub fn inner(&self) -> Address<ADDRESS_TYPE> {
        self.inner
    }

    pub fn value(&self) -> usize {
        self.inner.value()
    }

    pub fn offset(self, count: isize) -> Option<Self> {
        if count == 0 {
            return Some(self);
        }
        let delta = count
            .unsigned_abs()
            .checked_mul(bsp::memory::KernelGranule::SIZE)
            .unwrap() as usize;
        let result = if count.is_positive() {
            self.value().checked_add(delta)
        } else {
            self.value().checked_sub(delta)
        };

        Some(Self {
            inner: Address::new(result?),
        })
    }
}

#[allow(non_camel_case_types)]
impl<ADDRESS_TYPE: AddressType> From<Address<ADDRESS_TYPE>> for PageAddress<ADDRESS_TYPE> {
    fn from(address: Address<ADDRESS_TYPE>) -> Self {
        Self { inner: address }
    }
}

#[allow(non_camel_case_types)]
impl<ADDRESS_TYPE: AddressType> From<usize> for PageAddress<ADDRESS_TYPE> {
    fn from(addr: usize) -> Self {
        assert!(
            super::super::align::is_aligned(addr, bsp::memory::KernelGranule::SIZE),
            "Input usize not page aligned"
        );

        Self {
            inner: Address::new(addr),
        }
    }
}

impl<ATYPE: AddressType> Step for PageAddress<ATYPE> {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if start > end {
            return None;
        }

        // Since start <= end, do unchecked arithmetic.
        Some((end.value() - start.value()) >> bsp::memory::KernelGranule::SHIFT)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        start.offset(count as isize)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        start.offset(-(count as isize))
    }
}
