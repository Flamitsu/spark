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
use crate::boot::disk::EspPartition;
use crate::cli::validate::ask_user_confirmation;
use crate::config::{EspStructure, Routes};
use crate::errors::{IgnixError, io};
use std::fs::read_to_string;
use std::{fs, path::Path, path::PathBuf};

pub fn create_ignix_structure(esp: &EspPartition, efi_bin: &Path, no_nvram: bool, force: bool)
    -> Result<(), IgnixError> {
    let route = &esp.mountpoint;

    let efi_fallback = route.join("EFI/BOOT/BOOTX64.efi");
    
    if efi_fallback.exists() && no_nvram && !force {
        ask_user_confirmation("remove the BOOTX64.efi old binary to replace it with ignix one")?;
    }

    for dir in EspStructure::ESP_DIRECTORIES {
        let dir_route = route.join(dir);
        if !dir_route.exists() {
            fs::create_dir_all(&dir_route)?;
        }
        
        if dir.ends_with("ignix/") {
            fs::copy(efi_bin, dir_route.join("ignixx64.efi"))?;
        }
        
        if dir.ends_with("BOOT/") {
            fs::copy(efi_bin, &efi_fallback)?;
        }
    }

    Ok(())
}

pub fn delete_ignix_structure(esp: &EspPartition) -> Result<(), IgnixError> {
    let route = &esp.mountpoint;
    
    let bootloader_home = route.join("EFI/ignix");
    if bootloader_home.exists() {
        fs::remove_dir_all(bootloader_home)?;
    }

    let config_home = route.join("loader/ignix.conf");
    if config_home.exists() {
        fs::remove_file(config_home)?;
    }
    Ok(())
}

pub fn get_esp_mountpoint(partition_name: &str) -> Result<Option<PathBuf>, IgnixError>{
    let route = Path::new(Routes::MOUNTPOINTS);
    
    if !route.exists(){
        Err(io::Error::NotFound(route.display().to_string()))?
    }

    let file_content = read_to_string(route)?;
    let expected_dev_string = format!("/dev/{}", partition_name);

    for line in file_content.lines(){
        let mut parts = line.split_whitespace();
        
        let device = parts.next();
        let mountpoint = parts.next();
        if let (Some(dev), Some(mnt)) = (device, mountpoint) 
            && dev == expected_dev_string{
                return Ok(Some(PathBuf::from(mnt)));
        } 
    }
    Ok(None)
}
