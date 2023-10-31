#![no_std]
#![no_main]

// Panic handler
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;

// Linker looks for a function named '_start' by default
// Name-Mangling is disabled to prevent the linker from renaming the function
#[no_mangle]
// "C" calling convention is used to prevent the compiler from adding the Rust ABI
pub extern "C" fn _start() -> ! {
    vga_buffer::print_hello();
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_string("Hello Again");
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}
