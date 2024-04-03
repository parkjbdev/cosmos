pub const SERIAL_PORT_ADDRESS: u32 = 0x09000000;
pub const DEVICE_TREE: u64 = 0x40000000;

extern "C" {
    pub static __boot_core_stack_start: u8;
    pub static __boot_core_stack_end_exclusive: u8;
    pub static kernel_start: u8;
    pub static kernel_end: u8;
}
