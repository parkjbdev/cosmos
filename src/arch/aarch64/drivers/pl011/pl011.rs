use crate::{
    console, driver, interrupt,
    sync::{interface::Mutex, null_lock::NullLock},
};
use core::fmt;

// PL011UartInner
use super::pl011_inner::PL011UartInner;
use super::registers::*;
use aarch64_cpu::registers::{Readable, Writeable};

// PL011Uart is a wrapper of PL011UartInner
pub struct PL011Uart {
    inner: NullLock<PL011UartInner>,
}

impl PL011Uart {
    pub const fn new(base: usize, clock_hz: u32, baud_rate: u32) -> Self {
        Self {
            inner: NullLock::new(PL011UartInner::new(base, clock_hz, baud_rate)),
        }
    }
}

impl driver::interface::DeviceDriver for PL011Uart {
    fn init(&self) -> Result<(), &'static str> {
        self.inner.lock(|inner| inner.init())
    }
}

impl console::interface::Write for PL011Uart {
    fn write_char(&self, c: char) {
        self.inner.lock(|inner| inner.write_char(c))
    }

    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        self.inner.lock(|inner| fmt::Write::write_fmt(inner, args))
    }

    fn flush(&self) {
        self.inner.lock(|inner| inner.flush())
    }
}

impl console::interface::Read for PL011Uart {
    fn read_char(&self) -> char {
        self.inner.lock(|inner| inner.read_char(false).unwrap())
    }

    fn clear_rx(&self) {
        while self.inner.lock(|inner| inner.read_char(true).is_some()) {}
    }
}

impl console::interface::Statistics for PL011Uart {
    fn chars_written(&self) -> usize {
        self.inner.lock(|inner| inner.chars_written)
    }

    fn chars_read(&self) -> usize {
        self.inner.lock(|inner| inner.chars_read)
    }
}

impl console::interface::Echo for PL011Uart {
    fn echo(&self) {
        self.inner.lock(|inner| inner.echo())
    }
}

impl console::interface::Console for PL011Uart {}

impl interrupt::interface::IRQHandler for PL011Uart {
    fn handler(&self, cb: fn()) {
        self.inner.lock(|inner| {
            let pending = inner.registers.MIS.extract();
            inner.registers.ICR.write(ICR::ALL::CLEAR);
            if pending.matches_any(MIS::RXMIS::SET + MIS::RTMIS::SET) {
                cb();
            }
        });
    }
}
