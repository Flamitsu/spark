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
use crate::cli::args::RemoveOptions;
use crate::errors::IgnixError;
use crate::cli::validate::ask_user_confirmation;
use crate::boot::{esp, disk};
pub fn remove_ignix(options: RemoveOptions) -> Result<(), IgnixError> {
    if !options.force {
        ask_user_confirmation("uninstall")?;
    }

    let scanner = disk::DiskScanner::new(true, true); 
    let esp_target = scanner.find_compatible_esp()?;

    esp::delete_ignix_structure(&esp_target)?;
    
    Ok(())
}
