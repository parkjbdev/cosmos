.extern handle_el1t_sync
.extern handle_el1t_irq
.extern handle_el1t_fiq
.extern handle_el1t_err
.extern handle_el1h_sync
.extern handle_el1h_irq
.extern handle_el1h_fiq
.extern handle_el1h_err
.extern handle_el0_sync64
.extern handle_el0_irq64
.extern handle_el0_fiq64
.extern handle_el0_err64
.extern handle_el0_sync32
.extern handle_el0_irq32
.extern handle_el0_fiq32
.extern handle_el0_err32

.macro call_with_context handler
__vector_\handler:
  b store_context
  mov x0, sp
  bl \handler
  b restore_context
  eret
  dsb nsh
  isb
.size __vector_\handler, . - __vector_\handler
.type __vector_\handler, function
.endm

.section .text
.align  11
.global vector_table
vector_table:
  // https://developer.arm.com/documentation/100933/0100/AArch64-exception-vector-table
  /* Current EL with SP0 */
  // Exception is taken from EL1 while stack pointer was shared with EL0.
  // This happens when `SPSel` register holds the value 0
  .org 0x000
    call_with_context handle_el1t_sync
  .org 0x080
    call_with_context handle_el1t_irq
  .org 0x100
    call_with_context handle_el1t_fiq
  .org 0x180
    call_with_context handle_el1t_err
  
  /* Current EL with SPx */
  // Exception is taken from EL1 at the time when dedicated stack pointer was allocated for EL1.
  // This means that `SPSel` holds the value 1 and this is the mode that we are currently using
  .org 0x200
    call_with_context handle_el1h_sync
  .org 0x280
    call_with_context handle_el1h_irq
  .org 0x300
    call_with_context handle_el1h_fiq
  .org 0x380
    call_with_context handle_el1h_err
  
  /* Lower EL using AArch64 */
  // Exception is taken from EL0 while running in 64-bit mode
  .org 0x400
    call_with_context handle_el0_sync64
  .org 0x480
    call_with_context handle_el0_irq64
  .org 0x500
    call_with_context handle_el0_fiq64
  .org 0x580
    call_with_context handle_el0_err64
  /* Lower EL using AArch32 */
  .org 0x600
    call_with_context handle_el0_sync32
  .org 0x680
    call_with_context handle_el0_irq32
  .org 0x700
    call_with_context handle_el0_fiq32
  .org 0x780
    call_with_context handle_el0_err32
  .org 0x800


store_context:
  sub sp, sp, #16 * 17

  stp x0, x1, [sp, #16 * 0]
  stp x2, x3, [sp, #16 * 1]
  stp x4, x5, [sp, #16 * 2]
  stp x6, x7, [sp, #16 * 3]
  stp x8, x9, [sp, #16 * 4]
  stp x10, x11, [sp, #16 * 5]
  stp x12, x13, [sp, #16 * 6]
  stp x14, x15, [sp, #16 * 7]
  stp x16, x17, [sp, #16 * 8]
  stp x18, x19, [sp, #16 * 9]
  stp x20, x21, [sp, #16 * 10]
  stp x22, x23, [sp, #16 * 11]
  stp x24, x25, [sp, #16 * 12]
  stp x26, x27, [sp, #16 * 13]
  stp x28, x29, [sp, #16 * 14]

  mrs x22, elr_el1
  mrs x23, spsr_el1

  stp x30, x22, [sp, #16 * 15]
  str x23, [sp, #16 * 16]
.size store_context, . - store_context
.type store_context, function


restore_context:
  ldr x23, [sp, #16 * 16]
  ldp x30, x22, [sp, #16 * 15]

  msr elr_el1, x22
  msr spsr_el1, x23

  ldp x0, x1, [sp, #16 * 0]
  ldp x2, x3, [sp, #16 * 1]
  ldp x4, x5, [sp, #16 * 2]
  ldp x6, x7, [sp, #16 * 3]
  ldp x8, x9, [sp, #16 * 4]
  ldp x10, x11, [sp, #16 * 5]
  ldp x12, x13, [sp, #16 * 6]
  ldp x14, x15, [sp, #16 * 7]
  ldp x16, x17, [sp, #16 * 8]
  ldp x18, x19, [sp, #16 * 9]
  ldp x20, x21, [sp, #16 * 10]
  ldp x22, x23, [sp, #16 * 11]
  ldp x24, x25, [sp, #16 * 12]
  ldp x26, x27, [sp, #16 * 13]
  ldp x28, x29, [sp, #16 * 14]

  add sp, sp, #16 * 17
.size restore_context, . - restore_context
.type restore_context, function

