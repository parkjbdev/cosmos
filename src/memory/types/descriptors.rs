use super::*;

#[derive(Copy, Clone)]
pub struct MMIODescriptor {
    start_addr: Address<Physical>,
    end_addr_exclusive: Address<Physical>,
}

impl MMIODescriptor {
    /// Create an instance.
    pub fn new(start_addr: Address<Physical>, size: usize) -> Self {
        assert!(size > 0);
        let end_addr_exclusive = Address::new(start_addr.value() + size);

        Self {
            start_addr,
            end_addr_exclusive,
        }
    }

    /// Return the start address.
    pub const fn start_addr(&self) -> Address<Physical> {
        self.start_addr
    }

    /// Return the exclusive end address.
    pub fn end_addr_exclusive(&self) -> Address<Physical> {
        self.end_addr_exclusive
    }
}
