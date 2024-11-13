use crate::{
    arch::{exception::state::ExceptionState, irq::Interrupt},
    bsp::devicetree,
    driver,
};
use aarch64_cpu::registers::*;
use tock_registers::interfaces::ReadWriteable;

pub struct TimerDriver;

impl driver::interface::DeviceDriver for TimerDriver {
    fn init(&mut self) -> Result<(), &'static str> {
        let timer_compatible = self.compatible();
        if !timer_compatible.contains("armv8-timer") {
            panic!("Compatible Timer (armv8-timer) Not Found");
        }

        Ok(())
    }

    fn compatible(&self) -> &str {
        // "arm,armv8-timer", "arm,armv7-timer"
        core::str::from_utf8(devicetree::get_property("/timer", "compatible").unwrap()).unwrap()
    }

    fn register_from_devicetree_and_enable_irq_handler(&self) {
        let timer_interrupts = devicetree::get_property("/timer", "interrupts").unwrap();

        const SPLIT_SIZE: usize = core::mem::size_of::<u32>();

        // Order: Secure Timer[0:2], NonSecure Timer[3:5], Virtual Timer[6:8], Hypervisor Timer[9:11]
        let chunks: &[[u8; SPLIT_SIZE]] = unsafe { timer_interrupts.as_chunks_unchecked() };
        let timer_irq: Interrupt = Interrupt::from_raw(
            u32::from_be_bytes(chunks[3]),
            u32::from_be_bytes(chunks[4]),
            u32::from_be_bytes(chunks[5]),
            0x00,
            timer_handler,
            "NonSecure Timer",
        );

        timer_irq.register();

        enable_timer_irq(true);

        // Test
        set_timeout_irq_after(CNTFRQ_EL0.get());
    }
}

fn timer_handler(_state: &ExceptionState) -> bool {
    // info!("Timer Event!");
    // Concludes Timer IRQ
    CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::CLEAR);
    // TODO: Create Timer Queue

    // Test
    set_timeout_irq_after(CNTFRQ_EL0.get());

    true
}

// Interrupt Based Timeout
pub fn set_timeout_irq(target: u64) {
    CNTP_CVAL_EL0.set(target);
    CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR);
}

pub fn set_timeout_irq_after(target: u64) {
    CNTP_TVAL_EL0.set(target);
    CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR);
}

fn enable_timer_irq(enable: bool) {
    CNTP_CTL_EL0
        .write(CNTP_CTL_EL0::ENABLE.val(enable as u64) + CNTP_CTL_EL0::IMASK.val(!enable as u64));
}
