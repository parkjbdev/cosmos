use hermit_dtb::{Dtb, EnumSubnodesIter};
use spin::Mutex;

/// FDT magic number: 0xd00dfeed (big-endian)
const FDT_MAGIC: u32 = 0xd00dfeed;

pub static DEVICE_TREE: Mutex<Option<Dtb>> = Mutex::new(None);

/// Find the DTB by checking the firmware-provided address first,
/// then scanning known locations for the FDT magic.
pub fn find_dtb(firmware_addr: u64, ram_start: u64) -> u64 {
    // 1. Trust the bootloader if it provided a non-zero address with valid magic
    if firmware_addr != 0 && has_fdt_magic(firmware_addr as usize) {
        return firmware_addr;
    }

    // 2. Scan known offsets from RAM start (QEMU virt places DTB at RAM base)
    for offset in [0x0, 0x100] {
        let addr = ram_start as usize + offset;
        if has_fdt_magic(addr) {
            return addr as u64;
        }
    }

    panic!("DTB not found");
}

fn has_fdt_magic(addr: usize) -> bool {
    let magic = unsafe { core::ptr::read_volatile(addr as *const u32) };
    u32::from_be(magic) == FDT_MAGIC
}

pub fn init(base: u64) {
    let mut device_tree = DEVICE_TREE.lock();

    *device_tree = Some(unsafe {
        Dtb::from_raw(sptr::from_exposed_addr(base as usize)).expect("Error Initializing DT")
    });
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
