SECTIONS {
    beginning : {
        boot.o(code_section)
        . = 510;
        boot.o(boot_magic_section)
    } = 0
    .bss : {
        *(COMMON)
    }
}
