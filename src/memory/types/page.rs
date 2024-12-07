use super::address::{Address, AddressType};
use crate::bsp;
use core::{fmt::Display, iter::Step};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq)]
pub struct PageAddress<T: AddressType> {
    inner: Address<T>,
}

impl<T: AddressType> Display for PageAddress<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#x}", self.inner.value())
    }
}

impl<T: AddressType> PageAddress<T> {
    pub fn new(addr: Address<T>) -> Self {
        Self { inner: addr }
    }
    pub fn inner(&self) -> Address<T> {
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

impl<T: AddressType> From<Address<T>> for PageAddress<T> {
    fn from(address: Address<T>) -> Self {
        assert!(
            crate::memory::align::is_aligned(address.into(), bsp::memory::KernelGranule::SIZE),
            "Input usize not page aligned"
        );
        Self { inner: address }
    }
}

impl<T: AddressType> From<usize> for PageAddress<T> {
    fn from(addr: usize) -> Self {
        assert!(
            crate::memory::align::is_aligned(addr, bsp::memory::KernelGranule::SIZE),
            "Input usize not page aligned"
        );

        Self {
            inner: Address::new(addr),
        }
    }
}

impl<T: AddressType> Step for PageAddress<T> {
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        if start > end {
            return (0, None);
        }

        // Since start <= end, do unchecked arithmetic.
        (end.value() - start.value(), Some((end.value() - start.value()) >> bsp::memory::KernelGranule::SHIFT))
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        start.offset(count as isize)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        start.offset(-(count as isize))
    }
}
