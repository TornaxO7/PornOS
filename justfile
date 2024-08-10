LIMINE_GIT_URL := "https://github.com/limine-bootloader/limine.git"

BUILD_DIR := "target"

IMAGE_NAME := "pornos"

limine:
    rm -rf {{BUILD_DIR}}/limine
    git clone {{LIMINE_GIT_URL}} --depth=1 --branch v8.x-binary target/limine
    make -C target/limine

build:
    # build the kernel and move it to the iso directory
    cargo build --target ./x86_64.json
    mkdir -p {{BUILD_DIR}}/iso_root
    cp {{BUILD_DIR}}/x86_64/debug/{{IMAGE_NAME}} conf/limine.conf target/limine/limine{-bios.sys,-bios-cd.bin,-uefi-cd.bin} {{BUILD_DIR}}/iso_root

    xorriso -as mkisofs                                             \
        -b limine-bios-cd.bin                                       \
        -no-emul-boot -boot-load-size 4 -boot-info-table            \
        --efi-boot limine-uefi-cd.bin                               \
        -efi-boot-part --efi-boot-image --protective-msdos-label    \
        {{BUILD_DIR}}/iso_root -o {{IMAGE_NAME}}.iso

    {{BUILD_DIR}}/limine/limine bios-install {{IMAGE_NAME}}.iso

run-debug: build
    qemu-system-x86_64 -machine q35 -cpu qemu64 -M smm=off -D target/log.txt -d int,guest_errors {{IMAGE_NAME}}.iso

clean:
    cargo clean
