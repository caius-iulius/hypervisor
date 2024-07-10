ENTRY(_start)
SECTIONS
{
    . = 0x40080000;
    .text.boot : { *(.text.boot) }
    .text : { *(.text) }
    .data : { *(.data) }
    .rodata : { *(.rodata) }
    .bss (NOLOAD) : ALIGN(16) {
        __bss_start = .;
        *(.bss);
        . = ALIGN(16);
        __bss_end_exclusive = .;
    }

    . = ALIGN(8);
    . = . + 0x4000;
    LD_STACK_PTR = .;
}
