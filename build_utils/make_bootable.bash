#!/bin/bash

if [[ $# -ge 1 ]]; then
    cp $1 isodir/pornos
else
    cp target/target/debug/pornos isodir/
fi

xorriso -as mkisofs -b limine-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        --efi-boot limine-eltorito-efi.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        isodir -o pornos.iso

qemu-system-x86_64 -m 2G -cdrom pornos.iso -device isa-debug-exit,iobase=0xf4,iosize=0x04

if [[ $? == 33 ]] || [[ $? == 0 ]] || [[ $? == 1 ]]; then
    exit 0
else
    exit $?
fi

# if [[ $1 == test ]]; then
#     qemu-system-x86_64 -m 2G -cdrom pornos.iso -device isa-debug-exit,iobase=0xf4,iosize=0x04
# else
#     qemu-system-x86_64 -m 2G -cdrom pornos.iso
# fi
#
