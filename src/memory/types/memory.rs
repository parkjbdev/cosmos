use super::{
    address::{Address, AddressType},
    page::PageAddress,
};
use core::{
    fmt::{self, Display, Formatter},
    ops::Range,
};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq)]
pub struct MemoryRegion<ADDRESS_TYPE: AddressType> {
    start: PageAddress<ADDRESS_TYPE>,
    end: PageAddress<ADDRESS_TYPE>,
}

#[allow(non_camel_case_types)]
impl<T: AddressType> MemoryRegion<T> {
    pub fn new(start: PageAddress<T>, end: PageAddress<T>) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    pub fn start_addr(&self) -> Address<T> {
        self.start.inner()
    }

    pub fn start_page_addr(&self) -> PageAddress<T> {
        self.start
    }

    pub fn end_addr(&self) -> Address<T> {
        self.end.inner()
    }

    pub fn end_page_addr(&self) -> PageAddress<T> {
        self.end
    }

    pub fn size(&self) -> MemorySize {
        // Invariant: start <= end_exclusive, so do unchecked arithmetic.
        let end_exclusive = self.end.value();
        let start = self.start.value();

        MemorySize(end_exclusive - start)
    }
}

impl<T: AddressType> IntoIterator for MemoryRegion<T> {
    type Item = PageAddress<T>;
    type IntoIter = Range<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Range {
            start: self.start,
            end: self.end,
        }
    }
}

impl<T: AddressType> Display for MemoryRegion<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            " {} {}..{}",
            T::PREFIX,
            self.start_addr(),
            self.end_addr()
        )
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct MemorySize(pub usize);

impl Display for MemorySize {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let kb = self.0 / 1024;
        let mb = kb / 1024;
        let gb = mb / 1024;
        if gb > 0 {
            write!(f, "{:>3} GB ({:#x})", gb, self.0)
        } else if mb > 0 {
            write!(f, "{:>3} MB ({:#x})", mb, self.0)
        } else if kb > 0 {
            write!(f, "{:>3} KB ({:#x})", kb, self.0)
        } else {
            write!(f, "{:>3}  B ({:#x})", self.0, self.0)
        }
    }
}
