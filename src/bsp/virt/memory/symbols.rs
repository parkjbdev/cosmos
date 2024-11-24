use core::cell::UnsafeCell;

pub const SERIAL_PORT_ADDRESS: u32 = 0x09000000;
pub const RAM_START: u64 = 0x40000000;
pub const DEVICE_TREE_START: u64 = 0x40000000;

extern "Rust" {
    static __kernel_start_: UnsafeCell<()>;
    static __kernel_end_: UnsafeCell<()>;

    static __text_start_: UnsafeCell<()>;
    static __text_end_: UnsafeCell<()>;

    static __rodata_start_: UnsafeCell<()>;
    static __rodata_end_: UnsafeCell<()>;

    static __data_start_: UnsafeCell<()>;
    static __data_end_: UnsafeCell<()>;

    static __bss_start_: UnsafeCell<()>;
    static __bss_end_: UnsafeCell<()>;

    static __boot_core_stack_start_: UnsafeCell<()>;
    static __boot_core_stack_end_: UnsafeCell<()>;

    static __mmio_remap_start_: UnsafeCell<()>;
    static __mmio_remap_end_: UnsafeCell<()>;

    static __PAGE_SIZE_: UnsafeCell<()>;
}

pub fn kernel() -> (usize, usize) {
    let start_addr: usize = unsafe { __kernel_start_.get() as usize };
    let end_addr: usize = unsafe { __kernel_end_.get() as usize };
    (start_addr, end_addr)
}

pub fn text() -> (usize, usize) {
    let start_addr: usize = unsafe { __text_start_.get() as usize };
    let end_addr: usize = unsafe { __text_end_.get() as usize };
    (start_addr, end_addr)
}

pub fn rodata() -> (usize, usize) {
    let start_addr: usize = unsafe { __rodata_start_.get() as usize };
    let end_addr: usize = unsafe { __rodata_end_.get() as usize };
    (start_addr, end_addr)
}

pub fn data() -> (usize, usize) {
    let start_addr: usize = unsafe { __data_start_.get() as usize };
    let end_addr: usize = unsafe { __data_end_.get() as usize };
    (start_addr, end_addr)
}

pub fn bss() -> (usize, usize) {
    let start_addr: usize = unsafe { __bss_start_.get() as usize };
    let end_addr: usize = unsafe { __bss_end_.get() as usize };
    (start_addr, end_addr)
}

pub fn boot_core_stack() -> (usize, usize) {
    let start_addr: usize = unsafe { __boot_core_stack_start_.get() as usize };
    let end_addr: usize = unsafe { __boot_core_stack_end_.get() as usize };
    (start_addr, end_addr)
}

pub fn mmio_remap_region() -> (usize, usize) {
    let start_addr: usize = unsafe { __mmio_remap_start_.get() as usize };
    let end_addr: usize = unsafe { __mmio_remap_end_.get() as usize };
    (start_addr, end_addr)
}

pub fn page_size() -> usize {
    unsafe { __PAGE_SIZE_.get() as usize }
}
