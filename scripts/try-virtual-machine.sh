#!/usr/bin/bash
set -euo pipefail
echo "This script is only meant to be executed inside a virtual machine. BE CAREFUL."
read -p "Press enter to continue the execution..."
git pull || { echo "Git pull failed. You may need to install git before executing this program."; exit 1; }
cd ..
cargo cli && cargo loader;
cd target/debug/
doas ./spark install --efi-bin=../x86_64-unknown-uefi/debug/sparkx64.efi || sudo ./spark install --efi-bin=../x86_64-unknown-uefi/debug/sparkx64.efi
reboot || doas reboot

