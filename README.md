# Spark
# Alpha stage
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
- QEMU installed and configured with an instance of a Linux OS
- QEMU virtual snapshot of the virtual machine before executing the program
- FAT32 file system mounted in either '/boot,/boot/efi,/efi'. If not, the program will not know where to put the installation files and the install function will not work. 
## Installation

Clone the repository and compile:

```bash
git clone https://github.com/Flamitsu/spark
cd spark
cargo build-uefi # For the uefi binary
cargo build-normal # For the general binary
```

If the command
```bash
cargo build-uefi
```
gave you any error, you should execute this command:
```bash
rustup target add x86_64-unknown-uefi
```
Then re-run the cargo build command.

## Execution

For the execution of the program, I strongly recommend trying it inside a virtual machine.
I have the execute.sh script for this porpouse. IT IS NOT MEANT TO BE EXECUTED INSIDE THE HOST MACHINE.
Inside a virtual machine, for example with Debian installed, or another distro, you can try the program.
Clone the repository, execute the script, and it will automatically reboot. Then choose the entry that is named Spark.
If something goes wrong, I recommend to rollback to the previous snapshot, open an issue and describing the problem. Take in mind this is still work in progress and will be.
