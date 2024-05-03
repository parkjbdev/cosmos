#![allow(dead_code)]
use crate::arch::state::ExceptionState;
use aarch64_cpu::registers::*;

/* Current EL with SP0 */
// Exception is taken from EL1 while stack pointer was shared with EL0.
// This happens when `SPSel` register holds the value 0
#[no_mangle]
extern "C" fn handle_el1t_sync(state: &ExceptionState) -> *mut usize {
    println!("{}", state);
    panic!("handle_el1t_sync Called!");

    core::ptr::null_mut()
}
#[no_mangle]
extern "C" fn handle_el1t_irq(state: &ExceptionState) -> *mut usize {
    println!("{}", state);
    panic!("handle_el1t_irq Called!");

    core::ptr::null_mut()
}
#[no_mangle]
extern "C" fn handle_el1t_fiq(state: &ExceptionState) -> *mut usize {
    println!("{}", state);
    panic!("handle_el1t_fiq Called!");

    core::ptr::null_mut()
}

#[no_mangle]
extern "C" fn handle_el1t_err(state: &ExceptionState) -> *mut usize {
    println!("{}", state);
    panic!("handle_el1t_err Called!");

    core::ptr::null_mut()
}

/* Current EL with SPx */
// Exception is taken from EL1 at the time when dedicated stack pointer was allocated for EL1.
// This means that `SPSel` holds the value 1 and this is the mode that we are currently using
#[no_mangle]
extern "C" fn handle_el1h_sync(state: &mut ExceptionState) -> *mut usize {
    println!();
    println!("*** HANDLE_EL1H_SYNC ExceptionState ***");
    println!("{}", state);

    if FAR_EL1.get() == 8 * 1024 * 1024 * 1024 {
        println!("reviving from test_segfault\n");
        state.elr_el1 += 4;
        return core::ptr::null_mut();
    }

    panic!("handle_el1h_sync Called!");
}

#[no_mangle]
extern "C" fn handle_el1h_irq(state: &mut ExceptionState) -> *mut usize {
    println!();
    println!("*** HANDLE_EL1H_IRQ ExceptionState ***");
    println!("{}", state);

    // state.elr_el1 += 4;
    // return core::ptr::null_mut();
    panic!("handle_el1h_irq Called!");
}

#[no_mangle]
extern "C" fn handle_el1h_fiq(state: &ExceptionState) -> *mut usize {
    println!("{}", state);

    // state.elr_el1 += 4;
    // return core::ptr::null_mut();
    panic!("handle_el1h_fiq Called!");
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
