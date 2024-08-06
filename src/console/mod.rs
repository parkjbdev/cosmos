pub mod log;

pub mod interface {
    use core::fmt;

    pub trait Write {
        fn write_char(&mut self, c: char);
        fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result;
        fn flush(&self);
    }

    pub trait Read {
        fn read_char(&mut self) -> char;
        fn clear_rx(&mut self);
    }

    pub trait Statistics {
        fn chars_written(&self) -> usize {
            0
        }
        fn chars_read(&self) -> usize {
            0
        }
    }
}

#[cfg(target_os = "none")]
#[doc(hidden)]
pub fn print(args: core::fmt::Arguments<'_>) {
    use crate::{arch::console::CONSOLE, console::interface::Write};

    let console = unsafe { CONSOLE.get_mut().unwrap() };
    console.write_fmt(args).unwrap();
}
