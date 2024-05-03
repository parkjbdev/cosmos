use core::{
    arch::asm,
    num::{NonZeroU32, NonZeroU64},
    time::Duration,
};

use aarch64_cpu::{asm::barrier, registers::*};
use arm_gic::gicv3::{GicV3, IntId};
use log::{error, info};

use crate::arch::{dtb::get_dtb, state::ExceptionState};

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

pub fn init_timer(gic: &GicV3) {
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
    let _timer_secure = Timer {
        irq_type: u32::from_be_bytes(chunks[0]),
        irq_num: u32::from_be_bytes(chunks[1]),
        irq_flag: u32::from_be_bytes(chunks[2]),
    };
    let _timer_nonsecure = Timer {
        irq_type: u32::from_be_bytes(chunks[3]),
        irq_num: u32::from_be_bytes(chunks[4]),
        irq_flag: u32::from_be_bytes(chunks[5]),
    };
    let _timer_virtual = Timer {
        irq_type: u32::from_be_bytes(chunks[6]),
        irq_num: u32::from_be_bytes(chunks[7]),
        irq_flag: u32::from_be_bytes(chunks[8]),
    };
    let _timer_hypervisor = Timer {
        irq_type: u32::from_be_bytes(chunks[9]),
        irq_num: u32::from_be_bytes(chunks[10]),
        irq_flag: u32::from_be_bytes(chunks[11]),
    };
}

pub struct Timer {
    irq_type: u32,
    irq_num: u32,
    irq_flag: u32,
}

impl Timer {
    fn to_intid(self) -> IntId {
        if self.irq_type == 0 {
            IntId::spi(self.irq_num)
        } else if self.irq_type == 1 {
            IntId::ppi(self.irq_num)
        } else {
            IntId::sgi(self.irq_num)
        }
    }
    fn set_prio(self, prio: u8, gic: &GicV3) {
        // gic.set_interrupt_priority(self.to_intid(), prio);
    }
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

pub fn get_jiffies() -> u64 {
    barrier::isb(barrier::SY);
    get_jiffies_unsafe()
}

pub fn nsleep(ns: u64) {
    barrier::isb(barrier::SY);
    let end = uptime_unsafe() + Duration::from_nanos(ns);
    while uptime_unsafe() < end {}
}

pub fn sleep(sec: u64) {
    nsleep(sec * 1_000_000_000)
}

pub fn msleep(ms: u64) {
    nsleep(ms * 1_000_000)
}
