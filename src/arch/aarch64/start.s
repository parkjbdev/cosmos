.extern do_irq
.extern get_last_sp

.macro trap_entry spsel
    stp x29, x30, [sp, #-16]!
    stp x27, x28, [sp, #-16]!
    stp x25, x26, [sp, #-16]!
    stp x23, x24, [sp, #-16]!
    stp x21, x22, [sp, #-16]!
    stp x19, x20, [sp, #-16]!
    stp x17, x18, [sp, #-16]!
    stp x15, x16, [sp, #-16]!
    stp x13, x14, [sp, #-16]!
    stp x11, x12, [sp, #-16]!
    stp x9, x10, [sp, #-16]!
    stp x7, x8, [sp, #-16]!
    stp x5, x6, [sp, #-16]!
    stp x3, x4, [sp, #-16]!
    stp x1, x2, [sp, #-16]!

    mrs x22, tpidr_el0
    stp x22, x0, [sp, #-16]!

    mrs x23, sp_el0
    mrs x22, spsr_el1
    stp x22, x23, [sp, #-16]!

    mrs x23, elr_el1
    mov x22, #\spsel
    stp x22, x23, [sp, #-16]!
.endm

.macro trap_exit
    ldp x22, x23, [sp], #16
    msr elr_el1, x23

    ldp x22, x23, [sp], #16
    msr spsr_el1, x22
    msr sp_el0, x23

    ldp x22, x0, [sp], #16
    msr tpidr_el0, x22

    ldp x1, x2, [sp], #16
    ldp x3, x4, [sp], #16
    ldp x5, x6, [sp], #16
    ldp x7, x8, [sp], #16
    ldp x9, x10, [sp], #16
    ldp x11, x12, [sp], #16
    ldp x13, x14, [sp], #16
    ldp x15, x16, [sp], #16
    ldp x17, x18, [sp], #16
    ldp x19, x20, [sp], #16
    ldp x21, x22, [sp], #16
    ldp x23, x24, [sp], #16
    ldp x25, x26, [sp], #16
    ldp x27, x28, [sp], #16
    ldp x29, x30, [sp], #16
.endm


.align 6
el1_irq:
    trap_entry 1
    mov x0, sp
    bl do_irq
    cmp x0, 0
    b.eq 1f

    mov x1, sp
    str x1, [x0]
    // bl get_last_sp
    mov sp, x0
1:
    trap_exit
    eret
    dsb nsh
    isb
.type el1_irq, @function
.size el1_irq, .-el1_irq
