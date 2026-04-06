use std::path::PathBuf;
use std::fs::{read_dir,File};
use crate::config::{BLOCK_DEV_ROUTE, LOGICAL_BLOCK_SIZE};
use crate::errors::IgnixError;
use crate::boot::{gpt, sysfs};
use crate::errors::cmd;

/// This function gets the disks and returns the `Vec<String>` containing them depending on the arguments given in the execution.
pub fn get_system_disks(block_route: &str, allow_virtual: bool, allow_removable: bool) 
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
        
        if is_valid_block_device(&disk_name, allow_removable, allow_virtual)?{
            disks.push(disk_name);
        }
    }
    Ok(disks)
}

pub fn compatible_esp_partition(devices: Vec<String>) -> Result<String, IgnixError>{
    for device in devices {
        let disk_sysfs_route = PathBuf::from(BLOCK_DEV_ROUTE).join(&device);
        
        let sector_size = sysfs::get_disk_sector_size(&disk_sysfs_route, LOGICAL_BLOCK_SIZE)?;
        let disk = File::open(PathBuf::from("/dev/").join(&device))?;
        let buffer = gpt::get_gpt_structure(sector_size, &disk)?;
        
        if !gpt::is_disk_efi_signed(&buffer){
            eprintln!("{device} isn't EFI signed. Skipping...");
            continue;
        }
        
        let gpt_header_size: u32 = gpt::get_gpt_header_size(&buffer)?;

        if !gpt::validate_crc32_header_checksum(&buffer, gpt_header_size)?{
            eprintln!("{device} is probably corrupt (GPT header). Skipping...");
            continue;
        }

        // The standard says here the max should be 128, however it is dynamic here just in case.
        let gpt_max_partitions: u32 = gpt::get_max_gpt_partition(&buffer)?;
        let gpt_entry_size: u32 = gpt::get_partition_max_size(&buffer)?;
        let part_array_start: u64 = gpt::get_partition_array_start(&buffer)?;
        
        if !gpt::validate_crc32_partition_array_checksum(&buffer, gpt_max_partitions, gpt_entry_size, part_array_start, sector_size)?{
            eprintln!("{device} is probably corrupt (partition array). Skipping...");
            continue;
        }

        let Some(part_guid) = gpt::get_esp_guid(&buffer, 
            gpt_max_partitions, 
            gpt_entry_size, 
            sector_size, 
            part_array_start)? 
        else { 
            eprintln!("Not GUID valid found in {device}"); 
            continue; 
        };
        // This conversion is needed because the kernel exposes it as a string ordered.
        let guid_string = gpt::format_partuuid(&part_guid)?;
        
        let Some(partition_name) = sysfs::get_esp_partition(&device, 
            &disk_sysfs_route, 
            &guid_string)? 
        else { 
            eprintln!("Didn't found {device} with {guid_string} in the sysfs interface."); 
            continue; 
        };
        return Ok(partition_name);
    }

    Err(cmd::Error::NotEFIPartitionFound)?
}

/// Check if a partition gets a valid block name or not depending on the arguments provided in the moment of the execution.
fn is_valid_block_device(device_name: &str, allow_removable: bool, allow_virtual: bool) 
    -> Result<bool, IgnixError>{
    let route = PathBuf::from(BLOCK_DEV_ROUTE).join(device_name);
    
    /* If the device is a virtual device and the options says to install it in a virutal device
     * then the program will mark it as valid disk*/
    if sysfs::is_virtual_device(&route)? && !allow_virtual {
        return Ok(false);
    }

    if sysfs::is_removable_device(&route)? && !allow_removable{
        return Ok(false);
    }

    Ok(true)
}
