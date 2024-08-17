use core::sync::atomic::{AtomicBool, Ordering};
use lock_api::{GuardSend, RawMutex};

pub struct RawSpinlock(AtomicBool);

// reference hermit-sync/src/mutex/spin.rs
unsafe impl RawMutex for RawSpinlock {
    const INIT: Self = Self(AtomicBool::new(false));

    type GuardMarker = GuardSend;

    fn lock(&self) {
        // Note: This isn't the best way of implementing a spinlock, but it
        // suffices for the sake of this example.
        while !self.try_lock() {}
    }

    fn try_lock(&self) -> bool {
        self.0
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    unsafe fn unlock(&self) {
        self.0.store(false, Ordering::Release);
    }

    fn is_locked(&self) -> bool {
        let acquired_lock = self.try_lock();
        if acquired_lock {
            // Safety: The lock has been successfully acquired above.
            unsafe {
                self.unlock();
            }
        }
        !acquired_lock
    }
}

pub type Spinlock<T> = lock_api::Mutex<RawSpinlock, T>;
pub type SpinlockGuard<'a, T> = lock_api::MutexGuard<'a, RawSpinlock, T>;
