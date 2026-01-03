#!/bin/bash
# SPARK EXECUTION SCRIPT.
# ONLY FOR TESTING PURPOSES.
# WILL BE REMOVED ONCE THE CODE IS PRODUCTION READY

EXECUTION_DIR="execution"

# If the directory execution does not exists, then
if [ ! -d "$EXECUTION_DIR" ]; then
    mkdir "$EXECUTION_DIR" # It will create the directory

    # Copy OVMF firmware from your system to the execution dir
    cp /usr/share/edk2-ovmf/OVMF_CODE.fd "$EXECUTION_DIR/" # Those are the correct routes in Gentoo. Yours may defer.
    cp /usr/share/edk2-ovmf/OVMF_VARS.fd "$EXECUTION_DIR/" # Those are the correct routes in Gentoo. Yours may defer.

    # Create the EFI structure
    mkdir -p "$EXECUTION_DIR/esp/efi/boot"
fi
# Copy the final binary from the target directory
cp target/x86_64-unknown-uefi/debug/spark.efi "$EXECUTION_DIR/esp/efi/boot/bootx64.efi"

# Entry the execution directory
cd "$EXECUTION_DIR" || exit 1

# Execute QEMU with KVM enabled 
qemu-system-x86_64 -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp
