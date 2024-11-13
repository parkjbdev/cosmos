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
impl<ADDRESS_TYPE: AddressType> MemoryRegion<ADDRESS_TYPE> {
    pub fn new(start: PageAddress<ADDRESS_TYPE>, end: PageAddress<ADDRESS_TYPE>) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    pub fn start_addr(&self) -> Address<ADDRESS_TYPE> {
        self.start.inner()
    }

    pub fn start_page_addr(&self) -> PageAddress<ADDRESS_TYPE> {
        self.start
    }

    pub fn end_addr(&self) -> Address<ADDRESS_TYPE> {
        self.end.inner()
    }

    pub fn end_page_addr(&self) -> PageAddress<ADDRESS_TYPE> {
        self.end
    }

    pub fn size(&self) -> usize {
        // Invariant: start <= end_exclusive, so do unchecked arithmetic.
        let end_exclusive = self.end.value();
        let start = self.start.value();

        end_exclusive - start
    }
}

impl<ADDRESS_TYPE: AddressType> IntoIterator for MemoryRegion<ADDRESS_TYPE> {
    type Item = PageAddress<ADDRESS_TYPE>;
    type IntoIter = Range<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Range {
            start: self.start,
            end: self.end,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemorySize(pub usize);

impl Display for MemorySize {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let kb = self.0 / 1024;
        let mb = kb / 1024;
        let gb = mb / 1024;
        if gb > 0 {
            write!(f, "{} GB ({:#x})", gb, self.0)
        } else if mb > 0 {
            write!(f, "{} MB ({:#x})", mb, self.0)
        } else if kb > 0 {
            write!(f, "{} KB ({:#x})", kb, self.0)
        } else {
            write!(f, "{} B ({:#x})", self.0, self.0)
        }
    }
}
