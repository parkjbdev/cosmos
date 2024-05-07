use core::{arch::asm, time::Duration};

use aarch64_cpu::{asm::barrier, registers::*};
use log::info;

use crate::arch::{
    dtb::get_dtb,
    exception::{Interrupt, RawInterrupt},
    state::ExceptionState,
};

pub fn init_timer() {
    let dtb = get_dtb();
    let timer_compatible =
        core::str::from_utf8(dtb.get_property("/timer", "compatible").unwrap()).unwrap();
    if !timer_compatible.contains("armv8-timer") {
        panic!("Compatible Timer (armv8-timer) Not Found");
    }
    info!("Timer Compatible: {}", timer_compatible);
    // armv8-timer found..
    // parse timer interrupts
    let timer_interrupts = dtb.get_property("/timer", "interrupts").unwrap();
    const SPLIT_SIZE: usize = core::mem::size_of::<u32>();

    let chunks: &[[u8; SPLIT_SIZE]] = unsafe { timer_interrupts.as_chunks_unchecked() };
    let _timer_secure: Interrupt = RawInterrupt {
        irq_type: u32::from_be_bytes(chunks[0]),
        id: u32::from_be_bytes(chunks[1]),
        trigger: u32::from_be_bytes(chunks[2]),
        prio: 0x00,
    }
    .into();
    let _timer_nonsecure: Interrupt = RawInterrupt {
        irq_type: u32::from_be_bytes(chunks[3]),
        id: u32::from_be_bytes(chunks[4]),
        trigger: u32::from_be_bytes(chunks[5]),
        prio: 0x00,
    }
    .into();
    let _timer_virtual: Interrupt = RawInterrupt {
        irq_type: u32::from_be_bytes(chunks[6]),
        id: u32::from_be_bytes(chunks[7]),
        trigger: u32::from_be_bytes(chunks[8]),
        prio: 0x00,
    }
    .into();
    let _timer_hypervisor: Interrupt = RawInterrupt {
        irq_type: u32::from_be_bytes(chunks[9]),
        id: u32::from_be_bytes(chunks[10]),
        trigger: u32::from_be_bytes(chunks[11]),
        prio: 0x00,
    }
    .into();

    _timer_nonsecure.register_gic();
    _timer_nonsecure.enable();
    // _timer_nonsecure.disable();
}

fn timer_handler(_state: &ExceptionState) -> bool {
    info!("Handle Timer Interrupt");

    // TODO: Handle timer
    unsafe {
        asm!(
            "msr cntp_cval_el0, xzr",
            "msr cntp_ctl_el0, xzr",
            options(nostack, nomem)
        );
    }
    true
}

struct JiffyValue(u64);

impl From<JiffyValue> for Duration {
    fn from(value: JiffyValue) -> Self {
        let cntfrq = CNTFRQ_EL0.get();

        let secs = value.0 / cntfrq;
        let nanos = (value.0 % cntfrq * 1_000_000_000 / cntfrq) as u32;

        Duration::new(secs, nanos)
    }
}

pub fn resolution() -> Duration {
    Duration::from(JiffyValue(1))
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
    let jiffies = get_jiffies_unsafe();

    let cntfrq = CNTFRQ_EL0.get();

    let secs = jiffies / cntfrq;
    let nanos = (jiffies % cntfrq * 1_000_000_000 / cntfrq) as u32;

    let duration = Duration::new(secs, nanos);
    duration
}

fn get_jiffies_unsafe() -> u64 {
    CNTPCT_EL0.get()
}

#[allow(dead_code)]
pub fn get_jiffies() -> u64 {
    barrier::isb(barrier::SY);
    get_jiffies_unsafe()
}

#[allow(dead_code)]
pub fn nsleep(ns: u64) {
    barrier::isb(barrier::SY);
    let end = uptime_unsafe() + Duration::from_nanos(ns);
    while uptime_unsafe() < end {}
}

#[allow(dead_code)]
pub fn sleep(sec: u64) {
    nsleep(sec * 1_000_000_000)
}

#[allow(dead_code)]
pub fn msleep(ms: u64) {
    nsleep(ms * 1_000_000)
}
