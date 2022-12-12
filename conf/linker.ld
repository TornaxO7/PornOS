ENTRY(pornos_entry)
OUTPUT_ARCH(i386:x86-64)
OUTPUT_FORMAT(elf64-x86-64)

KERNEL_BASE = 0xffffffff80000000;

SECTIONS {
    . = KERNEL_BASE + SIZEOF_HEADERS;

    .plt                    : { *(.plt .plt.*) }
    .text                   : { *(.text .text.*) }

    /* ======== C START ======== */
    . += CONSTANT(MAXPAGESIZE);
    .code_end               : { *(.pornos_code_end) }

    .hash                   : { *(.hash) }
    .gnu.hash               : { *(.gnu.hash) }
    .dynsym                 : { *(.dynsym) }
    .dynstr                 : { *(.dynstr) }
    .rela                   : { *(.rela*) }
    .rodata                 : { *(.rodata .rodata.*) }
    .note.gnu.build-id      : { *(.note.gnu.build-id) }
    .eh_frame_hdr           : {
        PROVIDE(__eh_frame_hdr = .);
        KEEP(*(.eh_frame_hdr))
        PROVIDE(__eh_frame_hdr_end = .);
    }
    .eh_frame               : {
        PROVIDE(__eh_frame = .);
        KEEP(*(.eh_frame))
        PROVIDE(__eh_frame_end = .);
    }
    .gcc_except_table       : { KEEP(*(.gcc_except_table .gcc_except_table.*)) }

    .data.rel.ro            : { *(.data.rel.ro .data.rel.ro.*) }
    .dynamic                : { *(.dynamic) }

    .comment                : { *(.comment) }
    .debug                  : { *(.debug) }
    .debug_abbrev           : { *(.debug_abbrev) }
    .debug_aranges          : { *(.debug_aranges) }
    .debug_frame            : { *(.debug_frame) }
    .debug_funcnames        : { *(.debug_funcnames) }
    .debug_info             : { *(.debug_info .gnu.linkonce.wi.*) }
    .debug_line             : { *(.debug_line) }
    .debug_loc              : { *(.debug_loc) }
    .debug_macinfo          : { *(.debug_macinfo) }
    .debug_pubnames         : { *(.debug_pubnames) }
    .debug_pubtypes         : { *(.debug_pubtypes) }
    .debug_ranges           : { *(.debug_ranges) }
    .debug_sfnames          : { *(.debug_sfnames) }
    .debug_srcinfo          : { *(.debug_srcinfo) }
    .debug_str              : { *(.debug_str) }
    .debug_typenames        : { *(.debug_typenames) }
    .debug_varnames         : { *(.debug_varnames) }
    .debug_weaknames        : { *(.debug_weaknames) }
    .line                   : { *(.line) }
    .shstrtab               : { *(.shstrtab) }
    .strtab                 : { *(.strtab) }
    .symtab                 : { *(.symtab) }

    . += CONSTANT(MAXPAGESIZE);
    .read_only_end          : { *(.pornos_read_only_end) }

    . = DATA_SEGMENT_RELRO_END(0, .);

    .got                    : { *(.got .got.*) }
    .got.plt                : { *(.got.plt .got.plt.*) }
    .data                   : { *(.data .data.*) }
    .bss                    : { *(.bss .bss.*) *(COMMON) }

    .tdata                  : { *(.tdata .tdata.*) }
    .tbss                   : { *(.tbss .tbss.*) }

    . = DATA_SEGMENT_END(.);

    .data_end               : { *(.pornos_data_end) }
}
