use crate::arch::exception::irq;
use core::cell::UnsafeCell;

// Simple Null Lock
pub struct NullLock<T>
where
    T: ?Sized,
{
    data: UnsafeCell<T>,
}

unsafe impl<T> Send for NullLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for NullLock<T> where T: ?Sized + Send {}

impl<T> NullLock<T> {
    /// Create an instance.
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

impl<T> super::interface::Mutex for NullLock<T> {
    type Data = T;

    fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R {
        // In a real lock, there would be code encapsulating this line that ensures that this
        // mutable reference will ever only be given out once at a time.
        let data = unsafe { &mut *self.data.get() };

        f(data)
    }
}

// IRQ Safe Null Lock
pub struct IRQSafeNullLock<T>
where
    T: ?Sized,
{
    data: UnsafeCell<T>,
}

unsafe impl<T> Send for IRQSafeNullLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for IRQSafeNullLock<T> where T: ?Sized + Send {}

impl<T> IRQSafeNullLock<T> {
    /// Create an instance.
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

impl<T> super::interface::Mutex for IRQSafeNullLock<T> {
    type Data = T;

    fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R {
        // In a real lock, there would be code encapsulating this line that ensures that this
        // mutable reference will ever only be given out once at a time.
        let data = unsafe { &mut *self.data.get() };

        // TODO: Implement IRQ Safe lock
        irq::exec_with_irq_disabled(|| f(data))
    }
}

