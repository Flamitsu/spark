/*
 * Copyright (C) 2026 Flamitsu
 *
 * This file is part of Ignix.
 *
 * Ignix is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * Ignix is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ignix.  If not, see <https://www.gnu.org/licenses/>.
 */
pub fn show_help() {
    const HELP_TEXT: &str = r#"
USAGE:
    ignix [COMMAND] [OPTIONS]

COMMANDS:
    add         adds a new entry into the ESP directory.
    install     Installs ignix binary into the EFI System Partition
    uninstall   Removes ignix binary and its configuration from the ESP
    help        Prints this help information

OPTIONS:
    --force                 Skip all interactive confirmation prompts. Only use if you know what you're doing.
    --efi-bin=[PATH]        Manual path to the EFI binary (default: auto-detect)
    --no-nvram              Skips all the logic to write a NVRAM variable.
    --allow-virtual         Allows to install the .efi bin in a virtual device.
    --removable             Allows to install the .efi bin in a removable device. 
"#;
    println!("{}", HELP_TEXT.trim());
}
