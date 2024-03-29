use core::fmt::Write;
use core::panic::PanicInfo;
extern crate log as log_crate;
use log_crate::info;

use crate::{arch, console};

extern "C" {
    static kernel_start: u8;
    static kernel_end: u8;
}

#[no_mangle]
pub(crate) unsafe extern "C" fn loader_main() -> ! {
    arch::init_stdout();
    crate::log::init();

    info!("Hello, world from cosmos-loader!");
    println!("Loader: [{:p} - {:p}]", &kernel_start, &kernel_end);

    // Find Kernel from DTB
    let kernel = arch::find_kernel();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo<'_>) -> ! {
    writeln!(unsafe { &mut console::CONSOLE }, "[LOADER] {info}").ok();
    loop {}
}
