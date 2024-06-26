use super::constants::*;
use hermit_dtb::Dtb;

pub fn get_dtb<'a>() -> Dtb<'a> {
    unsafe { Dtb::from_raw(sptr::from_exposed_addr(DEVICE_TREE as usize)).unwrap() }
}
