BUILD_DIR := "target"
IMAGE_NAME := "pornos"

LIMINE_DIR := BUILD_DIR / "limine"
ISO_ROOT := BUILD_DIR / "iso_root"

# build the kernel
build:
    cargo build
    # RUSTFLAGS="-C relocation-model=static" cargo build --target x86_64-unknown-none

# run the kernel
run: image
    qemu-system-x86_64 -M q35 -m 4G -cdrom {{IMAGE_NAME}}.iso -boot d

# run the kernel but in debug mode with no window and only serial output
debug: image
    qemu-system-x86_64 \
        -M q35 \
        -m 4G \
        -cdrom {{IMAGE_NAME}}.iso \
        -boot d \
        -serial stdio \
        -display none \
        -device isa-debug-exit,iobase=0xf4,iosize=0x04

# "download" limine
limine:
    rm -rf {{LIMINE_DIR}}
    git clone https://github.com/limine-bootloader/limine.git --branch=v8.x-binary --depth=1 {{LIMINE_DIR}}
    make -C {{LIMINE_DIR}}

# create an image file
image: build
    rm -rf {{ISO_ROOT}}
    mkdir -p {{ISO_ROOT}}/boot
    cp -v target/x86_64-unknown-none/debug/pornos {{ISO_ROOT}}/boot/
    mkdir -p {{ISO_ROOT}}/boot/limine
    cp -v conf/limine.conf {{LIMINE_DIR}}/limine-bios.sys {{LIMINE_DIR}}/limine-bios-cd.bin {{LIMINE_DIR}}/limine-uefi-cd.bin {{ISO_ROOT}}/boot/limine/
    mkdir -p {{ISO_ROOT}}/EFI/BOOT
    cp -v {{LIMINE_DIR}}/BOOTX64.EFI {{ISO_ROOT}}/EFI/BOOT/
    cp -v {{LIMINE_DIR}}/BOOTIA32.EFI {{ISO_ROOT}}/EFI/BOOT/
    xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
    	-no-emul-boot -boot-load-size 4 -boot-info-table \
    	--efi-boot boot/limine/limine-uefi-cd.bin \
    	-efi-boot-part --efi-boot-image --protective-msdos-label \
    	{{ISO_ROOT}} -o {{IMAGE_NAME}}.iso
    {{LIMINE_DIR}}/limine bios-install {{IMAGE_NAME}}.iso

# clean up all build artifacts
clean:
    cargo clean

