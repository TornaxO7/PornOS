ENTRY(pornos_start)

PHDRS {
    null    PT_NULL    FLAGS(0) ;                   /* Null segment */
    text    PT_LOAD    FLAGS((1 << 0) | (1 << 2)) ; /* Execute + Read */
    rodata  PT_LOAD    FLAGS((1 << 2)) ;            /* Read only */
    data    PT_LOAD    FLAGS((1 << 1) | (1 << 2)) ; /* Write + Read */
}
 
SECTIONS {
    . = 0xffffffff80000000;

    .text : {
        *(.text)
    }

    . += CONSTANT(MAXPAGESIZE);

    .stivale2hdr : {
        KEEP(*(.stivale2hdr))
    }

    .rodata : {
        *(.rodata)
    }

    . += CONSTANT(MAXPAGESIZE);

    .data : {
        *(.data)
    }

    .bss : {
        *(.bss)
    }
}
