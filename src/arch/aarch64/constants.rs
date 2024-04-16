pub const SERIAL_PORT_ADDRESS: u32 = 0x09000000;
pub const RAM_START: u64 = 0x40000000;
pub const DEVICE_TREE: u64 = RAM_START;

extern "C" {
    pub static __boot_core_stack_start: u64;
    pub static __boot_core_stack_end_exclusive: u64;
    pub static kernel_start: u64;
    pub static kernel_end: u64;
    pub static __text_start: u64;
    pub static __text_end: u64;
    pub static __exception_vector_table_start: u64;
    pub static __exception_vector_table_end: u64;
    pub static __bss_start: u64;
    pub static __bss_end_exclusive: u64;
}
