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
use crate::{boot::disk::DiskScanner, errors::IgnixError};
/*
 * I made this file because i don't trust install hooks that depends on lsblk or shit.
 * I'm sorry, if you don't like it, it's just a hack to complay with the hook install
 * process shit.
*/
pub fn locate_esp() -> Result<String, IgnixError>{
    let disk = DiskScanner::new(false, false);
    Ok(disk.find_compatible_esp()?.mountpoint.display().to_string())
}
