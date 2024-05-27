// Adapted from https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/02_runtime_init/src/_arch/aarch64/cpu/boot.s

.equ _core_id_mask, 0xff

.macro ADR_REL register, symbol
  adrp \register, \symbol
  add \register, \register, #:lo12:\symbol
.endm

.section .text._start

_start:
  // Only proceed if the core executes in EL2. Park it otherwise.
  mrs x1, CurrentEL
  cmp x1, #0x8
  b.ne .L_parking_loop


  ADR_REL x0, __bss_start
  ADR_REL x1, __bss_end_exclusive
  cmp x0, x1
  b.ne .L_bss_init_loop

	// Only proceed on the boot core. Park it otherwise.
	mrs	x1, MPIDR_EL1
	and	x1, x1, _core_id_mask
	mov	x2, xzr  // Assume CPU 0 is responsible for booting
	cmp	x1, x2
	b.ne .L_parking_loop

  // Check Timer..
  mrs x1, CNTFRQ_EL0
  cmp x1, xzr
  b.eq .L_parking_loop

.L_bss_init_loop:
	cmp	x0, x1
	b.eq .L_prepare_kernel
	stp	xzr, xzr, [x0], #16
	b	.L_bss_init_loop

  // If execution reaches here, it is the boot core. Now, prepare the jump to Rust code.
.L_prepare_kernel:
	// This loads the physical address of the stack end. For details see
	// https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/16_virtual_mem_part4_higher_half_kernel/src/bsp/raspberrypi/link.ld
  ADR_REL x0, __boot_core_stack_end_exclusive
	mov		sp, x0

  // Setting Jiffies 
  // mrs x2, CNTFRQ_EL0
  // cmp x2, xzr
  // b.eq .L_parking_loop

  // adrp x1, ARCH_TIMER_COUNTER_FREQ 
  // add x1, x1, #:lo12:ARCH_TIMER_COUNTER_FREQ

  // str w2, [x1]

  // mov x1, 0x1000
  // msr CNTFRQ_EL0, x1 

	// Jump to Rust code.
	b	_start_cosmos

	// Infinitely wait for events (aka "park the core").
.L_parking_loop:
  wfe
  b .L_parking_loop

.size	_start, . - _start
.type	_start, function
.global	_start
