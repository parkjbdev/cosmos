use core::fmt::{Display, Formatter, Result};

#[derive(Debug)]
#[repr(C)]
pub struct ExceptionState {
    // General Purpose Registers (x0-x29)
    gpr: [u64; 30],

    // Link Register AKA. x30
    lr: u64,

    // Exception Link Register
    pub elr_el1: u64,

    // Program Status Register
    spsr_el1: u64,

    // Exception Syndrome Register
    esr_el1: u64,
}

impl Display for ExceptionState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "ELR_EL1: {:#018x}", self.elr_el1)?;
        writeln!(f, "SPSR_EL1: {:#018x}", self.spsr_el1)?;
        writeln!(f)?;
        writeln!(f, "General Purpose Registers")?;

        #[rustfmt::skip]
        let alternating = |x| -> _ {
            if x % 4 != 3 { "   " } else { "\n" }
        };

        // Print two registers per line.
        for (i, reg) in self.gpr.iter().enumerate() {
            write!(f, "      x{: <2}: {:#016x}{}", i, reg, alternating(i))?;
        }
        write!(f, "      lr : {:#016x}", self.lr)?;
        writeln!(f)
    }
}
