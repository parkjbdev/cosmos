pub use super::arch::console::CONSOLE;
use core::fmt::{Arguments, Result};

pub trait Console {
    fn write_fmt(&mut self, args: Arguments) -> Result;
    fn write_char(&mut self, c: char);
    fn flush(&self);
    fn read_char(&mut self) -> char;
    fn clear_rx(&mut self);
}
