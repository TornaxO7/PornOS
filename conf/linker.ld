ENTRY(pornos_entry)
OUTPUT_ARCH(i386:x86-64)
OUTPUT_FORMAT(elf64-x86-64)

KERNEL_BASE = 0xffffffff80000000;

SECTIONS {
    . = KERNEL_BASE + SIZEOF_HEADERS;

    .plt                    : { *(.plt .plt.*) }
    .text                   : { *(.text .text.*) }

    . += CONSTANT(MAXPAGESIZE);
    .code_end               : { *(.pornos_code_end) }
    /* ======== CODE END ======== */
    .read_only              : { *(.read_only) }

    . += CONSTANT(MAXPAGESIZE);
    .read_only_end          : { *(.pornos_read_only_end) }
    /* ======== READ ONLY END ======== */

    . = DATA_SEGMENT_RELRO_END(0, .);

    .data                   : { *(.data .data.*) }
    .bss                    : { *(.bss .bss.*) *(COMMON) }

    . = DATA_SEGMENT_END(.);
    .data_end               : { *(.pornos_data_end) }
    /* ======== DATA END ======== */
}
