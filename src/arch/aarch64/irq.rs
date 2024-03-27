use arm_gic::gicv3::{GicV3, IntId, SgiTarget, Trigger};
use core::arch::asm;
use log::{debug, info};

use generic_once_cell::OnceCell;

use crate::{init::dtb, sync::spin::RawSpinlock};

const MAX_HANDLERS: usize = 1024;
type Handler = fn() -> bool;
// type Handler = fn(state: &State) -> bool;
static mut IRQ_NAMES: [Option<&'static str>; MAX_HANDLERS] = [None; MAX_HANDLERS];
static mut IRQ_HANDLERS: [Option<Handler>; MAX_HANDLERS] = [None; MAX_HANDLERS];

const RAM_START: u64 = 0x4000_0000;
const DEVICE_TREE: u64 = RAM_START;

pub(crate) static mut GIC: OnceCell<RawSpinlock, GicV3> = OnceCell::new();
static mut TIMER_INTERRUPT: u32 = 0;
const RESCHED_SGI: u32 = 1;

fn parse_irqcells(irqcells: &[u8]) -> (u32, u32, u32) {
    const SPLIT_SIZE: usize = core::mem::size_of::<u32>();

    let (irqtype, irqcells) = irqcells.split_at(SPLIT_SIZE);
    let (irq, irqcells) = irqcells.split_at(SPLIT_SIZE);
    let (irqflags, _) = irqcells.split_at(SPLIT_SIZE);

    (
        u32::from_be_bytes(irqtype.try_into().unwrap()),
        u32::from_be_bytes(irq.try_into().unwrap()),
        u32::from_be_bytes(irqflags.try_into().unwrap()),
    )
}

fn timer_handler() -> bool {
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

pub fn init() {
    info!("Initializing GIC (Generic Interrupt Controller)");

    // TODO: Load DTB (Device Tree Blob)
    let device_tree = dtb::get();

    // TODO: parse GICD & GICC from the dtb /intc reg
    const GICD_BASE_ADDRESS: *mut u64 = 0x800_0000 as _;
    const GICC_BASE_ADDRESS: *mut u64 = 0x80A_0000 as _;

    // TODO: allocate address on virtual memory

    // Gic Initialization
    GicV3::set_priority_mask(0xff);
    let mut gic = unsafe { GicV3::new(GICD_BASE_ADDRESS, GICC_BASE_ADDRESS) };
    gic.setup();

    // TODO: Timer Interrupt
    info!("finding for timer interrupt from device_tree");

    let prop_compat_timer = device_tree.get_property("/timer", "compatible").unwrap();

    if !core::str::from_utf8(prop_compat_timer)
        .unwrap()
        .contains("arm,armv8-timer")
    {
        info!("Timer not found");
        return;
    }

    info!("Timer found");
    let prop_irq_timer = device_tree.get_property("/timer", "interrupts").unwrap();
    const SPLIT_SIZE: usize = core::mem::size_of::<u32>();
    /* Secure Physical Timer */
    let (st_irqtype, st_irq, st_irqflags) = parse_irqcells(prop_irq_timer);
    let (_, prop_irq_timer) = prop_irq_timer.split_at(SPLIT_SIZE * 3);
    info!(
        "Secure Timer interrupt: {}, type {}, flags {}",
        st_irq, st_irqtype, st_irqflags
    );

    // TODO: register secure timer

    /* Non-secure Physical Timer */
    let (nst_irqtype, nst_irq, nst_irqflags) = parse_irqcells(prop_irq_timer);
    let (_, prop_irq_timer) = prop_irq_timer.split_at(SPLIT_SIZE * 3);

    info!(
        "Timer interrupt: {}, type {}, flags {}",
        nst_irq, nst_irqtype, nst_irqflags
    );

    /* Virtual Timer */
    let (vt_irqtype, vt_irq, vt_irqflags) = parse_irqcells(prop_irq_timer);
    let (_, prop_irq_timer) = prop_irq_timer.split_at(SPLIT_SIZE * 3);
    info!(
        "Virtual Timer interrupt: {}, type {}, flags {}",
        vt_irq, vt_irqtype, vt_irqflags
    );

    // TODO: register virtual timer

    /* Hypervisor Timer */
    let (ht_irqtype, ht_irq, ht_irqflags) = parse_irqcells(prop_irq_timer);
    let (_, prop_irq_timer) = prop_irq_timer.split_at(SPLIT_SIZE * 3);
    info!(
        "Hypervisor Timer interrupt: {}, type {}, flags {}",
        ht_irq, ht_irqtype, ht_irqflags
    );
    // TODO: register hypervisor timer

    // Register Non-Secure Timer
    unsafe {
        TIMER_INTERRUPT = nst_irq;
    }
    let ns_timer_irq = irq_idx(nst_irqtype, nst_irq);
    gic.set_interrupt_priority(ns_timer_irq, 0x00);
    unsafe {
        IRQ_NAMES[u32::from(ns_timer_irq) as usize] = Some("Timer");
        IRQ_HANDLERS[u32::from(ns_timer_irq) as usize] = Some(timer_handler);
    }
    gic.set_trigger(
        ns_timer_irq,
        if nst_irqflags == 4 || nst_irqflags == 8 {
            Trigger::Level
        } else if nst_irqflags == 2 || nst_irqflags == 1 {
            Trigger::Edge
        } else {
            panic!("Invalid irqflag");
        },
    );
    gic.enable_interrupt(ns_timer_irq, true);

    info!("Timer Interrupt Enabled");
    unsafe {
        info!(
            "name: {}",
            IRQ_NAMES[u32::from(ns_timer_irq) as usize].unwrap()
        );
    }

    // Scheduler Interrupt
    let resched_sgi = IntId::sgi(RESCHED_SGI);
    gic.set_interrupt_priority(resched_sgi, 0x00);
    gic.enable_interrupt(resched_sgi, true);
    unsafe {
        IRQ_NAMES[u32::from(resched_sgi) as usize] = Some("Scheduler");
    }

    unsafe {
        GIC.set(gic).unwrap();
    }
}

// ARM DAIF Documentation
// https://developer.arm.com/documentation/ddi0601/2023-12/AArch64-Registers/DAIF--Interrupt-Mask-Bits?lang=en#fieldset_0-63_10

#[inline]
pub fn disable() {
    unsafe {
        asm!("msr DAIFSet, #0xf", options(nomem, nostack));
    }
}

#[inline]
pub fn enable() {
    unsafe {
        asm!("msr DAIFClr, #0xf", options(nomem, nostack));
    }
}

#[inline]
pub fn halt() {
    unsafe {
        asm!("wfi");
    }
}

#[inline]
pub fn safe_halt() {
    unsafe {
        asm!("sti;hlt");
    }
}

fn irq_idx(irq_type: u32, irq_number: u32) -> IntId {
    let irq_id = if irq_type == 0 {
        // Shared Peripheral Interrupt
        IntId::spi(irq_number)
    } else if irq_type == 1 {
        // Private Peripheral Interrupt
        IntId::ppi(irq_number)
    } else {
        // Software Generated Interrupt
        IntId::sgi(irq_number)
    };
    irq_id
}

fn do_irq() {

}

fn test_irq() {
    // Testing Interrupt
    // Configure an SGI(Software Generated Interrupt) and then send it to ourself.
    let sgi_id = IntId::sgi(3);

    let gic = unsafe { GIC.get_mut().unwrap() };
    gic.set_interrupt_priority(sgi_id, 0x80);
    gic.enable_interrupt(sgi_id, true);
    enable();
    GicV3::send_sgi(
        sgi_id,
        SgiTarget::List {
            affinity3: 0,
            affinity2: 0,
            affinity1: 0,
            target_list: 0b1,
        },
    );
}
