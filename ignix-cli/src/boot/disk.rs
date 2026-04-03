use std::fs::read_dir;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use crate::cli::InstallOptions;
use crate::config::{DEVNAME, DEVTYPE, PARTUUID, BLOCK_DEV_ROUTE};
use crate::errors::IgnixError;

/// This function gets the disks and returns the `Vec<String>` containing them depending on the arguments given in the execution.
pub fn get_system_disks(block_route: &str, options: &InstallOptions) 
    -> Result<Vec<String>, IgnixError> {
    
    let mut disks:Vec<String> = Vec::new();
    let disk_devices = read_dir(block_route)?; 
    
    for device in disk_devices{    
        // Unpacks the device, if there is an error it will jump to the next one 
        let Ok(disk) = device else{
            continue;
        };

        // Converts the possible disk into a string, if it fails it will jump to the next one
        let Ok(disk_name) = disk.file_name().into_string() else{
            continue;
        };
        
        if is_valid_block_device(&disk_name, options)?{
            disks.push(disk_name);
        }
    }
    Ok(disks)
}

/// Check if a partition gets a valid block name or not depending on the arguments provided in the moment of the execution.
fn is_valid_block_device(device_name: &str, options: &InstallOptions) -> Result<bool, IgnixError>{
    let route = PathBuf::from(BLOCK_DEV_ROUTE).join(device_name);
    
    /* If the device is a virtual device and the options says to install it in a virutal device
     * then the program will mark it as valid disk*/
    if is_virtual_device(&route)? && !options.allow_virtual {
        return Ok(false);
    }

    if is_removable_device(&route)? && !options.removable_device{
        return Ok(false);
    }

    Ok(true)
}

fn is_virtual_device(device: &Path) -> Result<bool, IgnixError>{
    if device.join("device").exists() {
        return Ok(false);
    }
    Ok(true)
}

fn is_removable_device(device: &Path) -> Result<bool, IgnixError>{
    let content = read_to_string(device.join("removable"))?;
    if content.trim() == "0"{
        return Ok(false)
    }
    Ok(true)
}

pub fn get_esp_partition(device: &str, sysfs_route: &str, partition_guid: &str) 
    -> Result<Option<String>, IgnixError>{
    let subdevice_uevent = get_disk_partition_uevent(sysfs_route, device)?;
    for subdevice in subdevice_uevent{
        let archive = read_to_string(subdevice)?;
        
        let mut devname = None;
        let mut is_partition = false;
        let mut guid_matches = false;
        
        for line in archive.lines(){
            if let Some(value) = line.strip_prefix(DEVNAME) {
                devname = Some(value.to_string())
            } else if let Some(value) = line.strip_prefix(DEVTYPE) && value == "partition"{
                is_partition = true
            } else if let Some(value) = line.strip_prefix(PARTUUID) && value == partition_guid {
                guid_matches = true;
            }
        }
        if is_partition && guid_matches{
            return Ok(devname);
        }
    } 
    Ok(None)
}

fn get_disk_partition_uevent(device_sysfs: &str, device: &str) -> Result<Vec<String>, IgnixError>{
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
