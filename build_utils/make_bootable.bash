#!/bin/bash

if [[ $# -ge 1 ]]; then
    notify-send "Using given path"
    cp $1 isodir/pornos
else
    cp target/target/debug/pornos isodir/
    notify-send "Using default path"
fi

xorriso -as mkisofs -b limine-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        --efi-boot limine-eltorito-efi.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        isodir -o pornos.iso

qemu-system-x86_64 -m 2G -cdrom pornos.iso
