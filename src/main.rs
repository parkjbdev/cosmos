#![no_std]
#![no_main]

// Panic handler
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Linker looks for a function named '_start' by default
// Name-Mangling is disabled to prevent the linker from renaming the function
#[no_mangle]
// "C" calling convention is used to prevent the compiler from adding the Rust ABI
pub extern "C" fn _start() -> ! {
    static HELLO: &[u8] = b"Hello World!";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
