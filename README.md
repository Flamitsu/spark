# Spark

Spark is a minimalist bootmanager written in Rust. Its goal is to be lightweight, secure, and stable bootmanager. 

## Features

- Minimalist and fast
- Written in Rust
- Easy to maintain and update
- Designed to evolve into a rEFInd-like boot manager, but more stable

## Requirements

- UEFI (Not planning in the future supporting BIOS, this can change tho.)
- Rust installed
- x86_64 architecture
- QEMU installed and configured
- OVFM tools

## Installation

Clone the repository and compile:

```bash
git clone https://github.com/Flamitsu/spark
cd spark
cargo build --release --target=x86_64-unknown-uefi
```

If the command
```bash
cargo build --release --target=x86_64-unknown-uefi
```
gave you any error, you should execute this command:
```bash
rustup target add x86_64-unknown-uefi
```
Then re-run the cargo build command.

## Execution
```bash
./execute.sh
```
That should execute the final compiled binary with QEMU.
You should execute the command "./execute" if you have rebuilt the package. It does the work for you and replace the old binary in the directory. 
