#![no_main]
#![no_std]
#![feature(exposed_provenance)]

#[macro_use]
mod macros;
mod arch;
mod console;
mod log;
mod loader;

use core::fmt::Write;

#[cfg(target_os = "none")]
#[doc(hidden)]
fn _print(args: core::fmt::Arguments<'_>) {
    unsafe {
        console::CONSOLE.write_fmt(args).unwrap();
    }
}
