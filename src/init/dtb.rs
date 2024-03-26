use core::ptr;
use log::info;
use hermit_dtb::Dtb;

pub const RAM_START_ADDR: u64 = 0x4000_0000;
pub const DEVICE_TREE_ADDR: u64 = RAM_START_ADDR;

pub fn get<'a>() -> Dtb<'a> {
    unsafe {
        Dtb::from_raw(ptr::from_exposed_addr(DEVICE_TREE_ADDR as _))
            .expect(".dtb file has invalid header")
    }
}

pub fn print_nodes() {
    let dtb = get();

    for node in dtb.enum_subnodes("/") {
        info!("{node}");
    }
}

