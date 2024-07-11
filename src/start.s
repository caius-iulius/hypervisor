.globl _start
.extern LD_STACK_PTR

.section ".text.boot"

_start:
	// Initialize DRAM.
	ldr	x0, __bss_start
	ldr x1, __bss_end_exclusive

// Initialize bss section to zero
.L_init_bss:
	cmp	x0, x1
	b.eq	.L_init_bss_end
	stp	xzr, xzr, [x0], #16
	b	.L_init_bss
.L_init_bss_end:

    // Load stack pointer
    ldr     x30, =LD_STACK_PTR
    mov     sp, x30

    // Rust entry
    bl      main

.equ PSCI_SYSTEM_OFF, 0x84000008
.globl system_off
system_off:
    bl system_off_hook
    ldr     x0, =PSCI_SYSTEM_OFF
    smc     #0
