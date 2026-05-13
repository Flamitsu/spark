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
use crate::{boot::{disk::DiskScanner, esp}, cli::{args::InstallOptions, validate::ask_user_confirmation}, errors::IgnixError};
pub fn install_ignix(options: InstallOptions) -> Result<(), IgnixError>{
    if !options.force{
        ask_user_confirmation("install")?;
    }
    let scanner = DiskScanner::new(options.allow_virtual, options.removable_device);
    let esp = scanner.find_compatible_esp()?;
    esp::create_ignix_structure(&esp, &options.efi_bin, options.no_nvram, options.force)?;
    Ok(())
}
