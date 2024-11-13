pub mod irq_type;

use self::irq_type::InterruptType;
use super::state::ExceptionState;
use super::Handler;
use crate::sync::spinlock::{RawSpinlock, Spinlock};
use aarch64_cpu::asm;
use aarch64_cpu::registers::*;
use arm_gic::gicv3::{GicV3, IntId, SgiTarget, Trigger};
use core::fmt::Display;
use generic_once_cell::OnceCell;
use log::info;
use tock_registers::interfaces::ReadWriteable;

const MAX_INTERRUPTS: usize = 1024;
pub static INTERRUPTS: Spinlock<[Option<Interrupt>; MAX_INTERRUPTS]> =
    Spinlock::new([None; MAX_INTERRUPTS]);

pub(crate) static mut GIC: OnceCell<RawSpinlock, GicV3> = OnceCell::new();
// pub(crate) static GIC: Mutex<Option<GicV3>> = Mutex::new(None);

pub fn exec_with_irq_disabled<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let daif = DAIF.get();
    let ret = f();
    DAIF.set(daif);
    ret
}

const SGI_START: u32 = 0;
const SGI_END: u32 = 15;

const PPI_START: u32 = 16;
const PPI_END: u32 = 31;

const SPI_START: u32 = 32;
const SPI_END: u32 = 1020;

#[derive(Clone, Copy)]
pub struct Interrupt {
    id: u32,
    trigger: Trigger,
    prio: u8,
    name: &'static str,
    handler: Handler,
}

impl Interrupt {
    pub fn from_raw(
        irq_type: u32,
        id: u32,
        trigger: u32,
        prio: u8,
        handler: Handler,
        name: &'static str,
    ) -> Self {
        let id = u32::from(match irq_type {
            0 => IntId::spi(id),
            1 => IntId::ppi(id),
            _ => IntId::sgi(id),
        });

        Interrupt::new(id, trigger, prio, handler, name)
    }

    pub fn new(id: u32, trigger: u32, prio: u8, handler: Handler, name: &'static str) -> Self {
        let trigger = match trigger {
            4 | 8 => Trigger::Level,
            2 | 1 => Trigger::Edge,
            // SGI is always edge triggered
            _ => Trigger::Edge,
            // _ => panic!("Invalid interrupt trigger type!"),
        };

        assert!(id >= SGI_START && id <= SPI_END);

        let ret = Self {
            id,
            trigger,
            prio,
            name,
            handler,
        };

        if INTERRUPTS.lock()[id as usize].is_some() {
            panic!("IRQ #{} already registered", id);
        } else {
            INTERRUPTS.lock()[id as usize] = Some(ret);
        }

        asm::barrier::dsb(asm::barrier::NSH);
        asm::barrier::isb(asm::barrier::SY);

        ret
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_type(&self) -> &InterruptType {
        match self.id {
            SGI_START..=SGI_END => &InterruptType::SGI,
            PPI_START..=PPI_END => &InterruptType::PPI,
            SPI_START..=SPI_END => &InterruptType::SPI,
            _ => panic!("Invalid interrupt ID"),
        }
    }

    fn get_id(&self) -> IntId {
        match self.get_type() {
            InterruptType::SGI => IntId::sgi(self.id),
            InterruptType::PPI => IntId::ppi(self.id - PPI_START),
            InterruptType::SPI => IntId::spi(self.id - SPI_START),
        }
    }

    pub fn handle_irq(&self, state: ExceptionState) {
        (self.handler)(&state);
    }

    pub fn register(&self) -> &Self {
        // let mut gic_binding = GIC.lock();
        // let gic = gic_binding.as_mut().expect("GIC is not initialized");
        let gic = unsafe { GIC.get_mut().unwrap() };
        let intid = self.get_id();
        println!("Registering interrupt: {}", self);
        gic.set_interrupt_priority(intid, self.prio);
        gic.set_trigger(intid, self.trigger);
        gic.enable_interrupt(intid, true);

        asm::barrier::dsb(asm::barrier::NSH);
        asm::barrier::isb(asm::barrier::SY);

        self
    }

    pub fn enable_irq(&self, enable: bool) -> &Self {
        // let mut gic_binding = GIC.lock();
        // let gic = gic_binding.as_mut().expect("GIC is not initialized");
        let gic = unsafe { GIC.get_mut().unwrap() };
        gic.enable_interrupt(self.get_id(), enable);
        self
    }

    pub fn remove(&self) {
        INTERRUPTS.lock()[self.id as usize] = None;
    }
}

impl Display for Interrupt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "Interrupt: {}#{} {}",
            self.get_type(),
            self.id,
            self.get_name(),
        )
    }
}

impl core::fmt::Debug for Interrupt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "      {: <3}. {}",
            u32::from(self.get_id()),
            self.get_name()
        )
    }
}

pub fn print_interrupts() {
    let interrupts = INTERRUPTS.lock();
    for interrupt in interrupts.iter() {
        if !interrupt.is_none() {
            info!("{:?}", interrupt.unwrap());
        }
    }
}

pub fn send_sgi(id: u32) {
    GicV3::send_sgi(
        IntId::sgi(id),
        SgiTarget::List {
            affinity3: 0,
            affinity2: 0,
            affinity1: 0,
            target_list: 0b1,
        },
    );
}

pub fn irq_enable() {
    DAIF.modify(DAIF::I::Unmasked);
}

pub fn irq_disable() {
    DAIF.modify(DAIF::I::Masked);
}

pub fn fiq_enable() {
    DAIF.modify(DAIF::F::Unmasked);
}

pub fn fiq_disable() {
    DAIF.modify(DAIF::F::Masked);
}
