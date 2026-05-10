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
use std::fs::{read_to_string, read_dir};
use std::path::Path;
use crate::config::DevLinuxTags;
use crate::IgnixError;

pub fn get_disk_partition_uevent(device_sysfs: &Path, device: &str) -> Result<Vec<String>, IgnixError>{
    let mut uevent_paths = Vec::new();
    let entries = read_dir(device_sysfs)?;
    
    for entry in entries{
        // Needs to unpack the entry because they can cause an error of lack of permission etc.
        let Ok(entry) = entry else {continue;};
        // Converts the entry from Path into a PathBuf.
        let path = entry.path();
        let Some(file_name_os) = path.file_name() else {continue;};
        let Some(file_name) = file_name_os.to_str() else {continue;};
        
        if !file_name.starts_with(device) || !path.is_dir(){
            continue;
        }
        
        let uevent_file = path.join("uevent");
        if !uevent_file.exists() {
            continue;
        }

        if let Some(subdevice) = uevent_file.to_str(){
            uevent_paths.push(subdevice.to_string());
        }
    }
    Ok(uevent_paths)
}

pub fn get_esp_partition(device: &str, sysfs_route: &Path, partition_guid: &str) 
    -> Result<Option<String>, IgnixError>{
    let subdevice_uevent = get_disk_partition_uevent(sysfs_route, device)?;
    for subdevice in subdevice_uevent{
        let archive = read_to_string(subdevice)?;
        
        let mut devname = None;
        let mut is_partition = false;
        let mut guid_matches = false;
        
        for line in archive.lines(){
            if let Some(value) = line.strip_prefix(DevLinuxTags::DEVNAME) {
                devname = Some(value.to_string())
            } else if let Some(value) = line.strip_prefix(DevLinuxTags::DEVTYPE) && value == "partition"{
                is_partition = true
            } else if let Some(value) = line.strip_prefix(DevLinuxTags::PARTUUID) && value == partition_guid {
                guid_matches = true;
            }
        }
        if is_partition && guid_matches{
            return Ok(devname);
        }
    } 
    Ok(None)
}

pub fn get_disk_sector_size(disk: &Path, lba_size_route: &str) -> Result<u64, IgnixError>{
    let sector_size_path = disk.join(lba_size_route);
    let value: u64 = std::fs::read_to_string(sector_size_path)?.trim().parse()?; 
    Ok(value)
}

pub fn is_virtual_device(device: &Path) -> Result<bool, IgnixError>{
    if device.join("device").exists() {
        return Ok(false);
    }
    Ok(true)
}

pub fn is_removable_device(device: &Path) -> Result<bool, IgnixError>{
    let content = read_to_string(device.join("removable"))?;
    if content.trim() == "0"{
        return Ok(false)
    }
    Ok(true)
}
