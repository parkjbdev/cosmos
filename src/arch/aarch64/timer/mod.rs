use aarch64_cpu::{asm::barrier, registers::*};
use core::time::Duration;
use log::info;

pub fn print_timer_status() {
    info!("      CNTP_CTL_EL0: {:#06x}", CNTP_CTL_EL0.get());
    info!(
        "      ISTATUS: {:?}",
        CNTP_CTL_EL0.read(CNTP_CTL_EL0::ISTATUS)
    );
    info!("      IMASK: {:?}", CNTP_CTL_EL0.read(CNTP_CTL_EL0::IMASK));
    info!(
        "      ENABLE: {:?}",
        CNTP_CTL_EL0.read(CNTP_CTL_EL0::ENABLE)
    );

    info!("      CNTPCT_EL0: {:?}", CNTPCT_EL0.get());
    info!("      CNTP_TVAL_EL0: {:?}", CNTP_TVAL_EL0.get());
    info!("      CNTP_CVAL_EL0: {:?}", CNTP_CVAL_EL0.get());
}

struct CounterTimerValue(u64);

impl From<CounterTimerValue> for Duration {
    fn from(value: CounterTimerValue) -> Self {
        let cntfrq = CNTFRQ_EL0.get();

        let secs = value.0 / cntfrq;
        let nanos = (value.0 % cntfrq * 1_000_000_000 / cntfrq) as u32;

        Duration::new(secs, nanos)
    }
}

pub fn resolution() -> Duration {
    Duration::from(CounterTimerValue(1))
}

// #[no_mangle]
// static ARCH_TIMER_COUNTER_FREQ: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(10_0000_0000) };
// static ARCH_TIMER_COUNTER_FREQ: u32 = 123;

#[inline(always)]
pub fn uptime() -> Duration {
    barrier::isb(barrier::SY);
    uptime_unsafe()
}

fn uptime_unsafe() -> Duration {
    let cnt = CNTPCT_EL0.get();
    let freq = CNTFRQ_EL0.get();

    let secs = cnt / freq;
    let nanos = (cnt % freq * 1_000_000_000 / freq) as u32;

    let duration = Duration::new(secs, nanos);
    duration
}

pub fn spin_for_ns(ns: u64) {
    barrier::isb(barrier::SY);
    let end = uptime_unsafe() + Duration::from_nanos(ns);
    while uptime_unsafe() < end {}
}

pub fn spin_for(sec: u64) {
    spin_for_ns(sec * 1_000_000_000)
}

pub fn spin_for_ms(ms: u64) {
    spin_for_ns(ms * 1_000_000)
}
