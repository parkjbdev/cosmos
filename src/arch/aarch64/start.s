// Adapted from https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/02_runtime_init/src/_arch/aarch64/cpu/boot.s

.equ _core_id_mask, 0xff

.section .text._start

_start:
	// Only proceed on the boot core. Park it otherwise.
	mrs	x1, mpidr_el1
	and	x1, x1, _core_id_mask
	mov	x2, xzr  // Assume CPU 0 is responsible for booting
	cmp	x1, x2
	b.ne	1f

	// If execution reaches here, it is the boot core. Now, prepare the jump to Rust code.

	// This loads the physical address of the stack end. For details see
	// https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/16_virtual_mem_part4_higher_half_kernel/src/bsp/raspberrypi/link.ld
	adrp	x4, __boot_core_stack_end_exclusive
	add		x4, x4, #:lo12:__boot_core_stack_end_exclusive
	mov		sp, x4
	
	// Jump to Rust code.
	b	_start_cosmos

	// Infinitely wait for events (aka "park the core").
1:	wfe
	b	1b

.size	_start, . - _start
.type	_start, function
.global	_start

.section .bss

.global l0_pgtable
.global l1_pgtable
.global l2_pgtable
.global l2k_pgtable
.global l3_pgtable
.global L0mib_pgtable

.align 12
l0_pgtable:
    .space 512*8, 0
l1_pgtable:
    .space 512*8, 0
l2_pgtable:
    .space 512*8, 0
l2k_pgtable:
    .space 512*8, 0
l3_pgtable:
    .space 512*8, 0
L0mib_pgtable:
    .space 512*8, 0
L2mib_pgtable:
    .space 512*8, 0
L4mib_pgtable:
    .space 512*8, 0
L6mib_pgtable:
    .space 512*8, 0
L8mib_pgtable:
    .space 512*8, 0
L10mib_pgtable:
    .space 512*8, 0
L12mib_pgtable:
    .space 512*8, 0
L14mib_pgtable:
    .space 512*8, 0
L16mib_pgtable:
    .space 512*8, 0
L18mib_pgtable:
    .space 512*8, 0

// .extern do_irq
// .extern get_last_sp
// 
// .macro trap_entry spsel
//     stp x29, x30, [sp, #-16]!
//     stp x27, x28, [sp, #-16]!
//     stp x25, x26, [sp, #-16]!
//     stp x23, x24, [sp, #-16]!
//     stp x21, x22, [sp, #-16]!
//     stp x19, x20, [sp, #-16]!
//     stp x17, x18, [sp, #-16]!
//     stp x15, x16, [sp, #-16]!
//     stp x13, x14, [sp, #-16]!
//     stp x11, x12, [sp, #-16]!
//     stp x9, x10, [sp, #-16]!
//     stp x7, x8, [sp, #-16]!
//     stp x5, x6, [sp, #-16]!
//     stp x3, x4, [sp, #-16]!
//     stp x1, x2, [sp, #-16]!
// 
//     mrs x22, tpidr_el0
//     stp x22, x0, [sp, #-16]!
// 
//     mrs x23, sp_el0
//     mrs x22, spsr_el1
//     stp x22, x23, [sp, #-16]!
// 
//     mrs x23, elr_el1
//     mov x22, #\spsel
//     stp x22, x23, [sp, #-16]!
// .endm
// 
// .macro trap_exit
//     ldp x22, x23, [sp], #16
//     msr elr_el1, x23
// 
//     ldp x22, x23, [sp], #16
//     msr spsr_el1, x22
//     msr sp_el0, x23
// 
//     ldp x22, x0, [sp], #16
//     msr tpidr_el0, x22
// 
//     ldp x1, x2, [sp], #16
//     ldp x3, x4, [sp], #16
//     ldp x5, x6, [sp], #16
//     ldp x7, x8, [sp], #16
//     ldp x9, x10, [sp], #16
//     ldp x11, x12, [sp], #16
//     ldp x13, x14, [sp], #16
//     ldp x15, x16, [sp], #16
//     ldp x17, x18, [sp], #16
//     ldp x19, x20, [sp], #16
//     ldp x21, x22, [sp], #16
//     ldp x23, x24, [sp], #16
//     ldp x25, x26, [sp], #16
//     ldp x27, x28, [sp], #16
//     ldp x29, x30, [sp], #16
// .endm
// 
// 
// .align 6
// el1_irq:
//     trap_entry 1
//     mov x0, sp
//     bl do_irq
//     cmp x0, 0
//     b.eq 1f
// 
//     mov x1, sp
//     str x1, [x0]
//     // bl get_last_sp
//     mov sp, x0
// 1:
//     trap_exit
//     eret
//     dsb nsh
//     isb
// .type el1_irq, @function
// .size el1_irq, .-el1_irq
