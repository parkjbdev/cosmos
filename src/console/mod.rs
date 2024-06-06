pub mod interface;
pub mod log;

#[cfg(target_os = "none")]
#[doc(hidden)]
pub fn print(args: core::fmt::Arguments<'_>) {
    use crate::{arch::console::CONSOLE, console::interface::Write};

    let console = unsafe { CONSOLE.get_mut().unwrap() };
    console.write_fmt(args).unwrap();
}

