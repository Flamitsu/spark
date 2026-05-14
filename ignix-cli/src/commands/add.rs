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
use crate::errors::IgnixError;
use crate::cli::args::AddOptions;
use crate::boot::disk::{DiskScanner};
use std::fs::{self, File};

pub fn add_entry(options: AddOptions) -> Result<(), IgnixError> {
    let disk = DiskScanner::new(false, false);
    let esp = disk.find_compatible_esp()?;
    let entries_route = esp.mountpoint.join("loader/entries");

    if !entries_route.exists() {
        fs::create_dir_all(&entries_route)?;
    }

    let file_name = format!("{}-{}.conf", options.machine_id, options.kernel_version);
    let file_path = entries_route.join(&file_name);
    let tmp_path = entries_route.join(format!("{}.tmp", file_name));
    let mut file_content = format!(
        "title       {}\n\
         version     {}\n\
         machine-id  {}\n\
         sort-key    {}\n\
         options     {}\n\
         linux       {}\n",
        options.title.trim_matches('\''), 
        options.kernel_version,
        options.machine_id,
        options.sort_key.trim_matches('\''),
        options.options,
        options.linux
    );

    for initrd in options.initrd {
        file_content.push_str(&format!("initrd      {}\n", initrd));
    }

    fs::write(&tmp_path, &file_content)?;
    fs::rename(&tmp_path, &file_path)?;

    let dir = File::open(&entries_route)?;
    dir.sync_all()?;

    println!("Entry created at: {:?}", file_path);
    Ok(())
}
