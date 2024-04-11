#[derive(Debug)]
#[repr(C, packed)]
pub(crate) struct State {
    /// stack selector
    pub spsel: u64,
    /// Exception Link Register
    pub elr_el1: u64,
    /// Program Status Register
    pub spsr_el1: u64,
    /// User-level stack
    pub sp_el0: u64,
    /// Thread ID Register
    pub tpidr_el0: u64,
    /// X0 register
    pub x0: u64,
    /// X1 register
    pub x1: u64,
    /// X2 register
    pub x2: u64,
    /// X3 register
    pub x3: u64,
    /// X4 register
    pub x4: u64,
    /// X5 register
    pub x5: u64,
    /// X6 register
    pub x6: u64,
    /// X7 register
    pub x7: u64,
    /// X8 register
    pub x8: u64,
    /// X9 register
    pub x9: u64,
    /// X10 register
    pub x10: u64,
    /// X11 register
    pub x11: u64,
    /// X12 register
    pub x12: u64,
    /// X13 register
    pub x13: u64,
    /// X14 register
    pub x14: u64,
    /// X15 register
    pub x15: u64,
    /// X16 register
    pub x16: u64,
    /// X17 register
    pub x17: u64,
    /// X18 register
    pub x18: u64,
    /// X19 register
    pub x19: u64,
    /// X20 register
    pub x20: u64,
    /// X21 register
    pub x21: u64,
    /// X22 register
    pub x22: u64,
    /// X23 register
    pub x23: u64,
    /// X24 register
    pub x24: u64,
    /// X25 register
    pub x25: u64,
    /// X26 register
    pub x26: u64,
    /// X27 register
    pub x27: u64,
    /// X28 register
    pub x28: u64,
    /// X29 register
    pub x29: u64,
    /// X30 register
    pub x30: u64,
}
