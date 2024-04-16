#![allow(dead_code)]
// use log::println;
use crate::arch::state::State;

/* Current EL with SP0 */
// Exception is taken from EL1 while stack pointer was shared with EL0.
// This happens when `SPSel` register holds the value 0
#[no_mangle]
extern "C" fn handle_el1t_sync(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
#[no_mangle]
extern "C" fn handle_el1t_irq(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
#[no_mangle]
extern "C" fn handle_el1t_fiq(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
#[no_mangle]
extern "C" fn handle_el1t_err(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
/* Current EL with SPx */
// Exception is taken from EL1 at the time when dedicated stack pointer was allocated for EL1.
// This means that `SPSel` holds the value 1 and this is the mode that we are currently using
#[no_mangle]
extern "C" fn handle_el1h_sync(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
#[no_mangle]
extern "C" fn handle_el1h_irq(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
#[no_mangle]
extern "C" fn handle_el1h_fiq(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
#[no_mangle]
extern "C" fn handle_el1h_err(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
/* Lower EL using AArch64 */
// Exception is taken from EL0 while running in 64-bit mode
#[no_mangle]
extern "C" fn handle_el0_sync64(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
#[no_mangle]
extern "C" fn handle_el0_irq64(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
#[no_mangle]
extern "C" fn handle_el0_fiq64(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
#[no_mangle]
extern "C" fn handle_el0_err64(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
/* Lower EL using AArch32 */
#[no_mangle]
extern "C" fn handle_el0_sync32(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
#[no_mangle]
extern "C" fn handle_el0_irq32(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
#[no_mangle]
extern "C" fn handle_el0_fiq32(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
#[no_mangle]
extern "C" fn handle_el0_err32(_state: &State) -> *mut usize {
    println!("Handler Called!");
    core::ptr::null_mut()
}
