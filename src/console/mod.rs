pub mod log;

pub mod interface {
    use core::fmt;

    pub trait Write {
        fn write_char(&self, c: char);
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
        fn flush(&self);
    }

    pub trait Read {
        fn read_char(&self) -> char;
        fn clear_rx(&self);
    }

    pub trait Echo {
        fn echo(&self);
    }

    pub trait Console: Write + Read + Echo {}
}
