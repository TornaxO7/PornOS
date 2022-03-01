#!/bin/bash

mkdir -p isodir/boot/grub
cp target/target/debug/pornos isodir/boot/pornos
grub-mkrescue -o pornos.iso isodir/
