use core::fmt::{self, Display, Formatter};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemorySize(pub usize);

impl Display for MemorySize {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let kb = self.0 / 1024;
        let mb = kb / 1024;
        let gb = mb / 1024;
        if gb > 0 {
            write!(f, "{} GB ({})", gb, self.0)
        } else if mb > 0 {
            write!(f, "{} MB ({})", mb, self.0)
        } else if kb > 0 {
            write!(f, "{} KB ({})", kb, self.0)
        } else {
            write!(f, "{} B ({})", self.0, self.0)
        }
    }
}
