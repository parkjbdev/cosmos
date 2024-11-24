use hermit_dtb::{Dtb, EnumSubnodesIter};
use spin::Mutex;

pub static DEVICE_TREE: Mutex<Option<Dtb>> = Mutex::new(None);

pub fn init(base: u32) {
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
