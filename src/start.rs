extern crate log as log_crate;
use log_crate::info;

use crate::{arch, log};

#[no_mangle]
pub(crate) unsafe extern "C" fn loader_main() -> ! {
    log::init();
    arch::stdout::init();
    info!("Hello World from cosmos");

    arch::init();
    arch::interrupts::init();

    println!(
        "boot_core_stack_start: {:p}",
        &arch::__boot_core_stack_start
    );
    println!(
        "boot_core_stack_end_exclusive: {:p}",
        &arch::__boot_core_stack_end_exclusive
    );
    println!(
        "Kernel: [{:p} - {:p}]",
        &arch::kernel_start,
        &arch::kernel_end
    );

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    info!("PANIC {}", info);
    loop {}
}
