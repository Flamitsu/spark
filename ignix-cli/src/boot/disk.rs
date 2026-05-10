use std::path::PathBuf;

use crate::boot::sysfs;

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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ignix.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::config::Routes;
pub struct BlockDevice{
    pub name: String,
    pub dev_path: PathBuf,
    pub sysfs_path: PathBuf
}

impl BlockDevice {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(),
        dev_path: PathBuf::from("/dev/").join(name),
        sysfs_path: PathBuf::from(Routes::BLOCK_DEV_ROUTE).join(name)}
    }
}
