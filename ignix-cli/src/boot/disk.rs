use std::fs::read_dir;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use crate::cli::InstallOptions;
use crate::config::BLOCK_DEV_ROUTE;
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
