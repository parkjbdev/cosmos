// Adapted from https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/02_runtime_init/src/_arch/aarch64/cpu/boot.s

.equ _core_id_mask, 0xff

.macro ADR_REL register, symbol
  adrp \register, \symbol
  add \register, \register, #:lo12:\symbol
.endm

.section .text._start

// https://www.kernel.org/doc/html/latest/arch/arm64/booting.html
_start:
  b _primary_entry      // code0
  .word 0               // code1
  .quad 0x100000        // text_offset: 1MB offset (device tree gap in linker script)
  .quad 0x100000000     // image_size: Effective Image size, little endian
  .quad 0               // flags: kernel flags, little endian
  .quad 0               // res2
  .quad 0               // res3
  .quad 0               // res4
  .ascii "ARM\x64"      // Magic number, little endian, "ARM\x64" (=0x644d5241)
  .word 0               // res5

_primary_entry:
  // save x0 (dtb pointer) before clobbering it
  mov x19, x0

  // Only proceed if the core executes in EL2. Park it otherwise.
  mrs x1, CurrentEL
  cmp x1, #0x8
  b.ne .L_parking_loop

  ADR_REL x0, __bss_start_
  ADR_REL x1, __bss_end_
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
  ADR_REL x0, __boot_core_stack_end_
	mov		sp, x0

  mov x1, x0          // x1 = stack_end (2nd arg to _start_cosmos)
  mov x0, x19         // x0 = dtb_addr  (1st arg to _start_cosmos)
	// Jump to Rust code.
	bl	_start_cosmos

	// Infinitely wait for events (aka "park the core").
.L_parking_loop:
  wfe
  b .L_parking_loop

.size	_start, . - _start
.type	_start, function
.global	_start
