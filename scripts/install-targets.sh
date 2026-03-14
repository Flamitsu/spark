#!/usr/bin/bash 
rustup target add x86_64-unknown-uefi || { echo "Error installing x86_64-unknown-uefi target. You may need try installing rustup first with this command: \n 'curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh' \nprovided by the official Rust team."; exit 1; }
