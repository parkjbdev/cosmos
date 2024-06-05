use super::exception::state::ExceptionState;
use crate::{
    arch::{dtb::get_dtb, exception::irq::Interrupt},
    sync::spin::RawSpinlock,
};
use aarch64_cpu::{asm::barrier, registers::*};
use core::{ops::Index, time::Duration};
use generic_once_cell::OnceCell;
use log::info;
use tock_registers::interfaces::ReadWriteable;

pub fn init() {
    let dtb = get_dtb();
    let timer_compatible =
        core::str::from_utf8(dtb.get_property("/timer", "compatible").unwrap()).unwrap();
    if !timer_compatible.contains("armv8-timer") {
        panic!("Compatible Timer (armv8-timer) Not Found");
    }
    info!("Timer Compatible: {}", timer_compatible);
    let timer_interrupts = dtb.get_property("/timer", "interrupts").unwrap();
    const SPLIT_SIZE: usize = core::mem::size_of::<u32>();

    // Order: Secure Timer[0:2], NonSecure Timer[3:5], Virtual Timer[6:8], Hypervisor Timer[9:11]
    let chunks: &[[u8; SPLIT_SIZE]] = unsafe { timer_interrupts.as_chunks_unchecked() };
    let timer_irq: Interrupt = Interrupt::from_raw(
        u32::from_be_bytes(chunks[3]),
        u32::from_be_bytes(chunks[4]),
        u32::from_be_bytes(chunks[5]),
        0x00,
        Some(timer_handler),
        Some("NonSecure Timer"),
    );

    info!("Registering Timer.. ");
    timer_irq.register();

    set_timeout_irq_after(CNTFRQ_EL0.get());
    enable_timer_irq(true);
}

fn enable_timer_irq(enable: bool) {
    CNTP_CTL_EL0
        .write(CNTP_CTL_EL0::ENABLE.val(enable as u64) + CNTP_CTL_EL0::IMASK.val(!enable as u64));
}

fn timer_handler(_state: &ExceptionState) -> bool {
    CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::CLEAR); // Concludes Timer IRQ
    set_timeout_irq_after(CNTFRQ_EL0.get());

    true
}

// Interrupt Based Timeout
fn set_timeout_irq(target: u64) {
    CNTP_CVAL_EL0.set(target);
    CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR);
}

fn set_timeout_irq_after(target: u64) {
    CNTP_TVAL_EL0.set(target);
    CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR);
}

#[allow(dead_code)]
pub fn conclude_timeout_irq() {
    CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::CLEAR);
}

pub fn print_timer_reg() {
    info!("CNTP_CTL_EL0: {:#06x}", CNTP_CTL_EL0.get());
    info!("  ISTATUS: {:?}", CNTP_CTL_EL0.read(CNTP_CTL_EL0::ISTATUS));
    info!("  IMASK: {:?}", CNTP_CTL_EL0.read(CNTP_CTL_EL0::IMASK));
    info!("  ENABLE: {:?}", CNTP_CTL_EL0.read(CNTP_CTL_EL0::ENABLE));

    info!("CNTPCT_EL0: {:?}", CNTPCT_EL0.get());
    info!("CNTP_TVAL_EL0: {:?}", CNTP_TVAL_EL0.get());
    info!("CNTP_CVAL_EL0: {:?}", CNTP_CVAL_EL0.get());
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
