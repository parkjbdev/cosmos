use core::{cell::UnsafeCell, ops::{Add, Range}};
use super::{AccessPermissions, Address, AttributeFields, MemoryAttributes, MemorySize, Physical};

pub const RAM_START: u64 = 0x40000000;
pub const DEVICE_TREE_START: u64 = 0x40000000;

extern "Rust" {
    static __kernel_start_: UnsafeCell<()>;
    static __kernel_end_: UnsafeCell<()>;

    static __device_tree_start_: UnsafeCell<()>;
    static __device_tree_end_: UnsafeCell<()>;

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

pub struct Section {
    pub name: &'static str,
    pub range: Range<Address<Physical>>,
    pub attr: AttributeFields,
}

pub fn kernel_range() -> Range<Address<Physical>> {
    let start_addr: usize = unsafe { __kernel_start_.get() as usize };
    let end_addr: usize = unsafe { __kernel_end_.get() as usize };

    Range {
        start: Address::new(start_addr),
        end: Address::new(end_addr),
    }
}

pub fn device_tree() -> Section {
    let start_addr: usize = unsafe { __device_tree_start_.get() as usize };
    let end_addr: usize = unsafe { __device_tree_end_.get() as usize };
    Section {
        name: "Device Tree",
        range: Range {
            start: Address::new(start_addr),
            end: Address::new(end_addr),
        },
        attr: AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RO,
        },
    }
}

pub fn text() -> Section {
    let start_addr: usize = unsafe { __text_start_.get() as usize };
    let end_addr: usize = unsafe { __text_end_.get() as usize };
    Section {
        name: ".text",
        range: Range {
            start: Address::new(start_addr),
            end: Address::new(end_addr),
        },
        attr: AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RX,
        },
    }
}

pub fn rodata() -> Section {
    let start_addr: usize = unsafe { __rodata_start_.get() as usize };
    let end_addr: usize = unsafe { __rodata_end_.get() as usize };
    Section {
        name: ".rodata",
        range: Range {
            start: Address::new(start_addr),
            end: Address::new(end_addr),
        },
        attr: AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RO,
        },
    }
}

pub fn data() -> Section {
    let start_addr: usize = unsafe { __data_start_.get() as usize };
    let end_addr: usize = unsafe { __data_end_.get() as usize };
    Section {
        name: ".data",
        range: Range {
            start: Address::new(start_addr),
            end: Address::new(end_addr),
        },
        attr: AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RW,
        },
    }
}

pub fn bss() -> Section {
    let start_addr: usize = unsafe { __bss_start_.get() as usize };
    let end_addr: usize = unsafe { __bss_end_.get() as usize };
    Section {
        name: ".bss",
        range: Range {
            start: Address::new(start_addr),
            end: Address::new(end_addr),
        },
        attr: AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RW,
        },
    }
}

pub fn boot_core_stack() -> Section {
    let start_addr: usize = unsafe { __boot_core_stack_start_.get() as usize };
    let end_addr: usize = unsafe { __boot_core_stack_end_.get() as usize };
    Section {
        name: "Boot Core Stack",
        range: Range {
            start: Address::new(start_addr),
            end: Address::new(end_addr),
        },
        attr: AttributeFields {
            memory_attributes: MemoryAttributes::CacheableDRAM,
            access_permissions: AccessPermissions::RW,
        },
    }
}

pub fn mmio_remap_range() -> Range<Address<Physical>> {
    let start_addr: usize = unsafe { __mmio_remap_start_.get() as usize };
    let end_addr: usize = unsafe { __mmio_remap_end_.get() as usize };
    Range {
        start: Address::new(start_addr),
        end: Address::new(end_addr),
    }
}

pub fn page_size() -> MemorySize {
    MemorySize(unsafe { __PAGE_SIZE_.get() as usize })
}
