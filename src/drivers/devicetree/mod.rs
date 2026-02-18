use core::sync::atomic::AtomicU64;

use hermit_dtb::{Dtb, EnumSubnodesIter};
use spin::Mutex;

/// FDT magic number: 0xd00dfeed (big-endian)
const FDT_MAGIC: u32 = 0xd00dfeed;

pub static BOOT_DTB_ADDR: AtomicU64 = AtomicU64::new(0);
pub static DEVICE_TREE: Mutex<Option<Dtb>> = Mutex::new(None);

pub fn has_fdt_magic(addr: usize) -> bool {
    let magic = unsafe { core::ptr::read_volatile(addr as *const u32) };
    u32::from_be(magic) == FDT_MAGIC
}

pub fn init(base: u64) {
    let mut device_tree = DEVICE_TREE.lock();

    *device_tree = Some(unsafe {
        Dtb::from_raw(sptr::from_exposed_addr(base as usize)).expect("Error Initializing DT")
    });
}

pub fn get_dtb_size() -> usize {
    // TODO: This is a temporary hardcoded value until we can reliably read the size from the DTB header.
    1024 * 1024

    // cannot access to totalsize in hermit-dtb
    // DEVICE_TREE
    //     .lock()
    //     .as_ref()
    //     .unwrap().header.totalsize as usize
}

pub fn update_base_address(new_base: u32) {
    let mut device_tree = DEVICE_TREE.lock();
    *device_tree =
        Some(unsafe { Dtb::from_raw(sptr::from_exposed_addr(new_base as usize)).unwrap() });
}

pub fn get_property<'a>(path: &'a str, property: &'a str) -> Option<&'a [u8]> {
    DEVICE_TREE
        .lock()
        .as_ref()
        .unwrap()
        .get_property(path, property)
}

pub fn enum_subnodes(path: &str) -> EnumSubnodesIter {
    DEVICE_TREE.lock().as_ref().unwrap().enum_subnodes(path)
}

/// Parse one big-endian u64 from a byte slice.
/// Returns (value, remaining_bytes).
pub fn dt_read_u64(bytes: &[u8]) -> (usize, &[u8]) {
    let (slice, rest) = bytes.split_at(core::mem::size_of::<u64>());
    (u64::from_be_bytes(slice.try_into().unwrap()) as usize, rest)
}
