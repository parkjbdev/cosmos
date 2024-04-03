extern crate log as log_crate;
use log_crate::info;

use crate::{arch, log};

extern "C" {
    static __boot_core_stack_start: u8;
    static __boot_core_stack_end_exclusive: u8;
    static kernel_start: u8;
    static kernel_end: u8;
}

#[no_mangle]
pub(crate) unsafe extern "C" fn loader_main() -> ! {
    log::init();
    arch::stdout::init();
    arch::init();

    info!("Hello World from cosmos");
    println!("boot_core_stack_start: {:p}", &__boot_core_stack_start);
    println!(
        "boot_core_stack_end_exclusive: {:p}",
        &__boot_core_stack_end_exclusive
    );
    println!("Loader: [{:p} - {:p}]", &kernel_start, &kernel_end);

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    info!("PANIC {}", info);
    loop {}
}
