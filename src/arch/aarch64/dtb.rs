use crate::sync::spinlock::RawSpinlock;

use super::constants::*;
use generic_once_cell::OnceCell;
use hermit_dtb::Dtb;

static DTB: OnceCell<RawSpinlock, Dtb> = OnceCell::new();

fn init_dtb() -> Result<(), Dtb<'static>> {
    DTB.set(unsafe { Dtb::from_raw(sptr::from_exposed_addr(DEVICE_TREE as usize)).unwrap() })
}

pub fn get_dtb<'a>() -> &'a Dtb<'a> {
    match DTB.get() {
        Some(dtb) => dtb,
        None => {
            // Lazy Initialization
            match init_dtb() {
                Err(e) => panic!("Cannot Initialize DTB"),
                _ => (),
            }

            DTB.get().unwrap()
        }
    }
}
