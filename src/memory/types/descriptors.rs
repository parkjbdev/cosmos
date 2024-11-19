use super::*;

#[derive(Copy, Clone)]
pub struct MMIODescriptor {
    start_addr: Address<Physical>,
    end_addr: Address<Physical>,
}

impl MMIODescriptor {
    /// Create an instance.
    pub fn new(start_addr: Address<Physical>, end_addr: Address<Physical>) -> Self {
        assert!(start_addr < end_addr);

        Self {
            start_addr,
            end_addr,
        }
    }

    /// Return the start address.
    pub const fn start_addr(&self) -> Address<Physical> {
        self.start_addr
    }

    /// Return the exclusive end address.
    pub fn end_addr(&self) -> Address<Physical> {
        self.end_addr
    }
}
