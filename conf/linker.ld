ENTRY(pornos_entry)
OUTPUT_ARCH(i386:x86-64)
OUTPUT_FORMAT(elf64-x86-64)

KERNEL_BASE = 0xffffffff80000000;

SECTIONS {
    . = KERNEL_BASE + SIZEOF_HEADERS;

    .plt                    : { *(.plt .plt.*) }
    .text                   : { *(.text .text.*) }

    . = ALIGN(CONSTANT(MAXPAGESIZE));
    .code_end               : { *(.pornos_code_end) }
    /* ======== CODE END ======== */
    .read_only              : { *(.read_only) *(.rodata .rodata.*) }

    .debug_abbrev           : { *(.debug_abbrev) }
    .debug_info             : { *(.debug_info) }
    .debug_aranges          : { *(.debug_aranges) }
    .debug_ranges           : { *(.debug_ranges) }
    .debug_str              : { *(.debug_str) }
    .debug_pubnames         : { *(.debug_pubnames) }
    .debug_pubtypes         : { *(.debug_pubtypes) }
    .debug_frame            : { *(.debug_frame) }
    .debug_line             : { *(.debug_line) }
    .debug_loc              : { *(.debug_loc) }
    .comment                : { *(.comment) }
    .symtab                 : { *(.symtab) }
    .shstrtab               : { *(.shstrtab) }
    .strtab                 : { *(.strtab) }

    . = ALIGN(CONSTANT(MAXPAGESIZE));
    .read_only_end          : { *(.pornos_read_only_end) }

    .data                   : { *(.data .data.*) }
    .bss                    : { *(.bss .bss.*) *(COMMON) }
    .got                    : { *(.got .got.*) }
    .plt                    : { *(.plt .plt.*) }

    . = DATA_SEGMENT_END(.);
    .data_end               : { *(.pornos_data_end) }
    /* ======== DATA END ======== */
}
