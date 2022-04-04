OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)
ENTRY(pornos_entry)

PHDRS {
    null    PT_NULL    FLAGS(0) ;                   /* Null segment */
    text    PT_LOAD    FLAGS((1 << 0) | (1 << 2)) ; /* Execute + Read */
    rodata  PT_LOAD    FLAGS((1 << 2)) ;            /* Read only */
    data    PT_LOAD    FLAGS((1 << 1) | (1 << 2)) ; /* Write + Read */
}

PAGE_SIZE = 4096;
 
SECTIONS {
    . = 0xffffffff80000000;

    .text : {
        *(.text .text.*)
    } :text

    . = ALIGN(PAGE_SIZE);

    .stivale2hdr : {
        KEEP(*(.stivale2hdr))
    } :rodata

    .rodata : {
        *(.rodata .rodata.*)
    } :rodata

    . = ALIGN(PAGE_SIZE);

    .porno_stack ALIGN(0x10) : {
        KEEP(*(.porno_stack))
    } :rodata

    . = ALIGN(2 * PAGE_SIZE);

    .data : {
        *(.data .data.*)
    } :data


    .bss : {
        *(COMMON)
        *(.bss .bss.*)
    } :data
}
