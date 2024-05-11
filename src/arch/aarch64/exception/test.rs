use crate::arch::{exception::sgi, state::ExceptionState};
use arm_gic::gicv3::IntId;
use log::info;

pub fn test_segfault() {
    let addr: u64 = 4 * 1024 * 1024 * 1024;
    info!("Trying to read from address {:#}GiB", addr >> 30);
    unsafe { core::ptr::read_volatile(addr as *mut u64) };
    info!("Survived");

    let addr: u64 = 8 * 1024 * 1024 * 1024;
    info!("Trying to read from address {:#}GiB", addr >> 30);
    unsafe { core::ptr::read_volatile(addr as *mut u64) };
    info!("Survived");
}

pub fn test_sgi() {
    // Testing Interrupt
    info!("Testing Software Generated Interrupt(SGI)");

    fn test_sgi_handler(state: &ExceptionState) -> bool {
        println!("test_sgi handler called");
        true
    }

    // Configure an SGI(Software Generated Interrupt) and then send it to ourself.
    let sgi = sgi::SGI::new(3, 0x00, test_sgi_handler, "test");

    sgi::register_sgi(sgi);
    sgi::send_sgi(3);
}
