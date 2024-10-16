pub struct AddressSpace<const SIZE: usize>;

impl<const SIZE: usize> AddressSpace<SIZE> {
    /// The address space size.
    pub const SIZE: usize = Self::size_checked();

    /// The address space shift, aka log2(size).
    pub const SIZE_SHIFT: usize = Self::SIZE.trailing_zeros() as usize;

    const fn size_checked() -> usize {
        assert!(SIZE.is_power_of_two());

        // Check for architectural restrictions as well.
        Self::arch_address_space_size_sanity_checks();

        SIZE
    }
}

pub trait AssociatedTranslationTable {
    type TableStartFromBottom;
}

