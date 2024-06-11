// Exception Vector Table Handlers
// Exception vector in `vector_table.s` will call appropriate handler

use super::state::ExceptionState;
use crate::arch::exception::INTERRUPTS;
use aarch64_cpu::registers::*;
use arm_gic::gicv3::GicV3;
use log::info;

/* Current EL with SP0 */
// Exception is taken from EL1 while stack pointer was shared with EL0.
// This happens when `SPSel` register holds the value 0
#[no_mangle]
extern "C" fn handle_el1t_sync(state: &ExceptionState) -> *mut usize {
    panic!("handle_el1t_sync Called!");
}

#[no_mangle]
extern "C" fn handle_el1t_irq(state: &ExceptionState) -> *mut usize {
    panic!("handle_el1t_irq Called!");
}

#[no_mangle]
extern "C" fn handle_el1t_fiq(state: &ExceptionState) -> *mut usize {
    panic!("handle_el1t_fiq Called!");
}

#[no_mangle]
extern "C" fn handle_el1t_err(state: &ExceptionState) -> *mut usize {
    panic!("handle_el1t_err Called!");
}

/* Current EL with SPx */
// Exception is taken from EL1 at the time when dedicated stack pointer was allocated for EL1.
// This means that `SPSel` holds the value 1 and this is the mode that we are currently using
#[no_mangle]
extern "C" fn handle_el1h_sync(state: &mut ExceptionState) -> *mut usize {
    println!("\n*** HANDLE_EL1H_SYNC ExceptionState ***");
    println!("FAR_EL1: {:#018x}", FAR_EL1.get());
    println!("{}", state);

    // // Surviving from test_segfault
    // if FAR_EL1.get() == 8 * 1024 * 1024 * 1024 {
    //     FAR_EL1.set(0);
    //     println!("Resetting FAR_EL1 as {:#018x}", FAR_EL1.get());
    //     println!("reviving from test_segfault\n");
    //     state.elr_el1 += 4;
    //     return core::ptr::null_mut();
    // }

    panic!("handle_el1h_sync Called!");
}

fn handle_interrupt(state: ExceptionState) -> *mut usize {
    if let Some(irqid) = GicV3::get_and_acknowledge_interrupt() {
        let id: u32 = irqid.into();
        let irq = INTERRUPTS.lock()[id as usize].unwrap();
        irq.handle_irq(state);

        // dbg!("Received IRQ name: {} ({:?})", irq.get_name(), irqid);
        GicV3::end_interrupt(irqid);
    }
    core::ptr::null_mut()
}

#[no_mangle]
extern "C" fn handle_el1h_irq(state: ExceptionState) -> *mut usize {
    handle_interrupt(state)
}

#[no_mangle]
extern "C" fn handle_el1h_fiq(state: ExceptionState) -> *mut usize {
    handle_interrupt(state)
}

#[no_mangle]
extern "C" fn handle_el1h_err(state: &ExceptionState) -> *mut usize {
    println!("{}", state);

    // state.elr_el1 += 4;
    // return core::ptr::null_mut();
    panic!("handle_el1h_err Called!");
}

/* Lower EL using AArch64 */
// Exception is taken from EL0 while running in 64-bit mode
#[no_mangle]
extern "C" fn handle_el0_sync64(state: &ExceptionState) -> *mut usize {
    println!("{}", state);

    // state.elr_el1 += 4;
    // return core::ptr::null_mut();
    panic!("handle_el0_sync64 Called!");
}

#[no_mangle]
extern "C" fn handle_el0_irq64(state: &ExceptionState) -> *mut usize {
    println!("{}", state);
    // state.elr_el1 += 4;
    // return core::ptr::null_mut();
    panic!("handle_el0_irq64 Called!");
}

#[no_mangle]
extern "C" fn handle_el0_fiq64(state: &ExceptionState) -> *mut usize {
    println!("{}", state);
    // state.elr_el1 += 4;
    // return core::ptr::null_mut();
    panic!("handle_el0_fiq64 Called!");
}

#[no_mangle]
extern "C" fn handle_el0_err64(state: &ExceptionState) -> *mut usize {
    println!("{}", state);
    // state.elr_el1 += 4;
    // return core::ptr::null_mut();
    panic!("handle_el0_err64 Called!");
}

/* Lower EL using AArch32 */
#[no_mangle]
extern "C" fn handle_el0_sync32(state: &ExceptionState) -> *mut usize {
    println!("{}", state);
    // state.elr_el1 += 4;
    // return core::ptr::null_mut();
    panic!("handle_el0_sync32 Called!");
}

#[no_mangle]
extern "C" fn handle_el0_irq32(state: &ExceptionState) -> *mut usize {
    println!("{}", state);
    // state.elr_el1 += 4;
    // return core::ptr::null_mut();
    panic!("handle_el0_irq32 Called!");
}

#[no_mangle]
extern "C" fn handle_el0_fiq32(state: &ExceptionState) -> *mut usize {
    println!("{}", state);
    // state.elr_el1 += 4;
    // return core::ptr::null_mut();
    panic!("handle_el0_fiq32 Called!");
}

#[no_mangle]
extern "C" fn handle_el0_err32(state: &ExceptionState) -> *mut usize {
    println!("{}", state);
    // state.elr_el1 += 4;
    // return core::ptr::null_mut();
    panic!("handle_el0_err32 Called!");
}
