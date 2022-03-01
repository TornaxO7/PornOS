#!/bin/bash

cargo build
cp target/target/debug/pornos boot/
grub-mkrescue -o pornos.iso boot/pornos
