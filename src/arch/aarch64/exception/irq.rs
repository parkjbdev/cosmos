use super::{Handler, IRQ_HANDLERS, IRQ_NAMES};
use crate::arch::exception::GIC;
use arm_gic::gicv3::{GicV3, IntId, SgiTarget, Trigger};
use log::info;
use core::{arch::asm, fmt::Display};

const SGI_START: u32 = 0;
const SGI_END: u32 = 15;

const PPI_START: u32 = 16;
const PPI_END: u32 = 31;

const SPI_START: u32 = 32;
const SPI_END: u32 = 1020;

#[derive(Eq, PartialEq)]
pub enum InterruptType {
    PPI,
    SPI,
    SGI,
}

impl InterruptType {}

impl Display for InterruptType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InterruptType::PPI => write!(f, "PPI"),
            InterruptType::SPI => write!(f, "SPI"),
            InterruptType::SGI => write!(f, "SGI"),
        }
    }
}

pub struct Interrupt {
    id: u32,
    trigger: Trigger,
    prio: u8,
}

impl Interrupt {
    pub fn from_raw(
        irq_type: u32,
        id: u32,
        trigger: u32,
        prio: u8,
        handler: Option<Handler>,
        name: Option<&'static str>,
    ) -> Self {
        let id = u32::from(match irq_type {
            0 => IntId::spi(id),
            1 => IntId::ppi(id),
            _ => IntId::sgi(id),
        });

        Interrupt::new(id, trigger, prio, handler, name)
    }

    pub fn new(
        id: u32,
        trigger: u32,
        prio: u8,
        handler: Option<Handler>,
        name: Option<&'static str>,
    ) -> Self {
        let trigger = match trigger {
            4 | 8 => Trigger::Level,
            2 | 1 => Trigger::Edge,
            _ => Trigger::Edge, // SGI is always edge triggered
                                // _ => panic!("Invalid interrupt trigger type!"),
        };

        assert!(id >= SGI_START && id <= SPI_END);

        let ret = Self {
            id,
            trigger,
            prio,
        };

        if unsafe { IRQ_NAMES[id as usize] } != None {
            panic!("IRQ #{} already registered", id);
        }

        unsafe {
            IRQ_NAMES[id as usize] = match name {
                Some(name) => Some(name),
                None => Some("Unnamed IRQ"),
            };
            IRQ_HANDLERS[id as usize] = handler;
            asm!("dsb nsh", "isb", options(nostack, nomem, preserves_flags));
        }

        info!("New IRQ: {} ({})", name.unwrap(), id);

        ret
    }

    pub fn get_name(&self) -> &'static str {
        unsafe {
            match IRQ_NAMES[self.id as usize] {
                Some(name) => name,
                None => panic!("No name found for IRQ #{}", self.id),
            }
        }
    }

    pub fn get_type(&self) -> &InterruptType {
        match self.id {
            SGI_START..=SGI_END => &InterruptType::SGI,
            PPI_START..=PPI_END => &InterruptType::PPI,
            SPI_START..=SPI_END => &InterruptType::SPI,
            _ => panic!("Invalid interrupt ID"),
        }
    }

    pub fn get_handler(&self) -> Handler {
        unsafe { IRQ_HANDLERS[self.id as usize].unwrap() }
    }

    fn get_id(&self) -> IntId {
        match self.get_type() {
            InterruptType::SGI => IntId::sgi(self.id),
            InterruptType::PPI => IntId::ppi(self.id - PPI_START),
            InterruptType::SPI => IntId::spi(self.id - SPI_START),
        }
    }

    pub fn register(&self) -> &Self {
        let gic = unsafe { GIC.get_mut().unwrap() };
        let intid = self.get_id();
        gic.set_interrupt_priority(intid, self.prio);
        gic.set_trigger(intid, self.trigger);
        gic.enable_interrupt(intid, true);

        unsafe {
            asm!("dsb nsh", "isb", options(nostack, nomem, preserves_flags));
        }

        self
    }

    pub fn enable(&self) -> &Self{
        let gic = unsafe { GIC.get_mut().unwrap() };
        gic.enable_interrupt(self.get_id(), true);
        self
    }

    pub fn disable(&self) -> &Self {
        let gic = unsafe { GIC.get_mut().unwrap() };
        gic.enable_interrupt(self.get_id(), false);
        self
    }

    pub fn remove(&self) {
        unsafe {
            IRQ_NAMES[self.id as usize] = None;
            IRQ_HANDLERS[self.id as usize] = None;
        }
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
