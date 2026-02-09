#!/bin/bash
set -euo pipefail
# This adds the needed targets for the compilation
rustup target add x86_64-unknown-uefi && rustup target add x86_64-unknown-linux-gnu && echo "Rust targets were added correctly."
cargo build-normal && cargo build-uefi;
# Changes to the current binary directory
cd target/x86_64-unknown-linux-gnu/debug/
doas ./spark install --efi-bin=../../x86_64-unknown-uefi/debug/sparkx64.efi || sudo ./spark install --efi-bin=../../x86_64-unknown-uefi/debug/sparkx64;
echo "To try the spark program you should reboot your virtual machine.";
