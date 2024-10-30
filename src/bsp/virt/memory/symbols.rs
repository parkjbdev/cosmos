use core::cell::UnsafeCell;

pub const SERIAL_PORT_ADDRESS: u32 = 0x09000000;
pub const RAM_START: u64 = 0x40000000;
pub const DEVICE_TREE_START: u64 = 0x40000000;

extern "Rust" {
    pub static kernel_start: UnsafeCell<()>;
    pub static kernel_end: UnsafeCell<()>;

    pub static __text_start: UnsafeCell<()>;
    pub static __text_end: UnsafeCell<()>;

    pub static __rodata_start: UnsafeCell<()>;
    pub static __rodata_end: UnsafeCell<()>;

    pub static __data_start: UnsafeCell<()>;
    pub static __data_end: UnsafeCell<()>;

    pub static __bss_start: UnsafeCell<()>;
    pub static __bss_end: UnsafeCell<()>;

    pub static __boot_core_stack_start: UnsafeCell<()>;
    pub static __boot_core_stack_end: UnsafeCell<()>;

    pub static PAGE_SIZE: UnsafeCell<()>;
}

pub fn page_size() -> usize {
    unsafe { PAGE_SIZE.get() as usize }
}

pub fn kernel() -> (usize, usize) {
    let start_addr: usize = unsafe { kernel_start.get() as usize };
    let end_addr: usize = unsafe { kernel_end.get() as usize };
    (start_addr, end_addr)
}

pub fn text() -> (usize, usize) {
    let start_addr: usize = unsafe { __text_start.get() as usize };
    let end_addr: usize = unsafe { __text_end.get() as usize };
    (start_addr, end_addr)
}

pub fn rodata() -> (usize, usize) {
    let start_addr: usize = unsafe { __rodata_start.get() as usize };
    let end_addr: usize = unsafe { __rodata_end.get() as usize };
    (start_addr, end_addr)
}

pub fn data() -> (usize, usize) {
    let start_addr: usize = unsafe { __data_start.get() as usize };
    let end_addr: usize = unsafe { __data_end.get() as usize };
    (start_addr, end_addr)
}

pub fn bss() -> (usize, usize) {
    let start_addr: usize = unsafe { __bss_start.get() as usize };
    let end_addr: usize = unsafe { __bss_end.get() as usize };
    (start_addr, end_addr)
}

pub fn boot_core_stack() -> (usize, usize) {
    let start_addr: usize = unsafe { __boot_core_stack_start.get() as usize };
    let end_addr: usize = unsafe { __boot_core_stack_end.get() as usize };
    (start_addr, end_addr)
}
