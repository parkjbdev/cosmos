use spin::Mutex;

pub mod log;

pub mod interface {
    use core::fmt;

    use crate::interrupt;

    pub trait Write {
        fn write_char(&self, c: char);
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
        fn flush(&self);
    }

    pub trait Read {
        fn read_char(&self) -> char;
        fn clear_rx(&self);
    }

    pub trait Statistics {
        fn chars_written(&self) -> usize {
            0
        }
        fn chars_read(&self) -> usize {
            0
        }
    }

    pub trait Echo {
        fn echo(&self);
    }

    pub trait Console: Write + Read + Statistics + Echo + interrupt::interface::IRQHandler {}
}

static CONSOLE: Mutex<Option<&'static (dyn interface::Console + Sync)>> = Mutex::new(None);

pub fn register_console(console: &'static (dyn interface::Console + Sync)) {
    *CONSOLE.lock() = Some(console);
}

pub fn console() -> &'static dyn interface::Console {
    CONSOLE.lock().unwrap()
}

#[cfg(target_os = "none")]
#[doc(hidden)]
pub fn print(args: core::fmt::Arguments<'_>) {
    console().write_fmt(args).unwrap();
}
