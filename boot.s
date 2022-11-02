.section code_section
code:
    mov $0x0e, %ah

    mov 'H', %al
    int $0x10

    mov 'e', %al
    int $0x10

    mov 'l', %al
    int $0x10

    mov 'l', %al
    int $0x10

    mov 'o', %al
    int $0x10

yeet:
    jmp yeet

.section boot_magic_section
boot_magic:
    .word 0xaa55
