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
