// Describes the characteristics of a translation granule.

pub struct TranslationGranule<const SIZE: usize>;
impl<const GRANULE_SIZE: usize> TranslationGranule<GRANULE_SIZE> {
    pub const SIZE: usize = Self::size_checked();
    pub const SHIFT: usize = Self::SIZE.trailing_zeros() as usize;
    pub const MASK: usize = Self::SIZE - 1;

    const fn size_checked() -> usize {
        assert!(GRANULE_SIZE.is_power_of_two());
        GRANULE_SIZE
    }
}

pub type Granule512MB = TranslationGranule<{512 * 1024 * 1024}>;
pub type Granule64KB = TranslationGranule<{64 * 1024}>;
