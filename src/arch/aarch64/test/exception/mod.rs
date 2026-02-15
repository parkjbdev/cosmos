use crate::arch::exception::{irq, state::ExceptionState};
use log::info;

pub fn test_segfault() {
    let addr: u64 = 4 * 1024 * 1024 * 1024;
    println!("Trying to read from address {:#}GiB", addr >> 30);
    unsafe { core::ptr::read_volatile(addr as *mut u64) };
    println!("Survived");

    let addr: u64 = 8 * 1024 * 1024 * 1024;
    println!("Trying to read from address {:#}GiB", addr >> 30);
    unsafe { core::ptr::read_volatile(addr as *mut u64) };
    println!("Survived");
}

pub fn test_sgi() {
    // Testing Interrupt
    info!("Testing Software Generated Interrupt(SGI)");

    fn test_sgi_handler(state: &ExceptionState) -> bool {
        info!("SGI Test Success");
        true
    }

    // Configure an SGI(Software Generated Interrupt) and then send it to ourself.
    let sgi_id = 3;
    irq::Interrupt::new(sgi_id, 0x01, 0x00, test_sgi_handler, "test")
        .register()
        .enable_irq(true);
    irq::send_sgi(sgi_id);
}
