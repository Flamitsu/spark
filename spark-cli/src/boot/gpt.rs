// This archive is used to know which device is the correct for the installation of the ESP
use crate::errors::SparkError;
use std::fs;
use std::fs::File;
use std::io::{Seek,SeekFrom,Read};
/// Returns the actual ESP device, so the esp.rs module can discover its path.
pub fn compatible_esp_device() -> Result<String, SparkError>{
    const BLOCK_DEV_ROUTE: &str = "/sys/block/";
    const LOGICAL_BLOCK: &str = "/queue/logical_block_size";
    let aviable_disks = get_disks(BLOCK_DEV_ROUTE)?;

    // Iterates the aviable disks in the system (plugged in)
    for disk in aviable_disks{
        let dev_route_disk: String = format!("/dev/{}",disk);
        let block_size: u64 = get_disk_logical_sector_size(&disk,BLOCK_DEV_ROUTE,LOGICAL_BLOCK)?;
        let open_disk = File::open(&dev_route_disk)?;
        // If the disk is not a GPT valid disk, it will jump to the next one 
        if !try_gpt_sectors(block_size, open_disk)?{
            eprintln!("Warning: the disk {} may not be a valid GPT. Skipping...",dev_route_disk);
            continue
        }

        let _number_partitions: u8 = get_number_partitions(block_size,&dev_route_disk)?;
        println!("{} LBA Size: {}",disk,block_size);
    }
    
    Ok(String::new())
}

/// Returns a list of the disks in the system (block ones).
fn get_disks(block_route: &str) -> Result<Vec<String>, SparkError>{
    // Creates a new empty vec where the found disk devices are going to be storaged
    let mut disks:Vec<String> = Vec::new();
    // Creates an empty vec
    let disk_devices = fs::read_dir(block_route)?; 
    for device in disk_devices{
        
        // Unpacks the device, if there is an error it will jump to the next one 
        let Ok(disk) = device else{
            continue;
        };

        // Converts the possible disk into a string, if it fails it will jump to the next one
        let Ok(disk_name) = disk.file_name().into_string() else{
            continue;
        };
        
        // Checks if it is a valid block device, like for example 'nvmeXnY', not partitions.
        if check_valid_block_name(&disk_name){
            disks.push(disk_name);
        }
    }
    Ok(disks)
}

/// Checks if the provided name is valid or not. (Example: sda, nvme... Bad example: dm-0,sda1...) 
fn check_valid_block_name(device_name: &str) -> bool {
    // If the device is a block device and not a partition then return true 
    if device_name.starts_with("nvme") && !device_name.contains("p"){
        return true
    }
    
    // If the device names start with vd or sd, and its not a partition then it will return true
    if (device_name.starts_with("vd") || device_name.starts_with("sd")) && 
        !device_name.ends_with(|c: char| c.is_numeric()) {
        return true
    }
    
    false
}

/// Get disk logical sector size (Example: 512, 4096...)
fn get_disk_logical_sector_size(disk_block: &str, block_route: &str, 
    logical_block_route: &str) -> Result<u64, SparkError>{
    
    // The route where the logical block sector size is storaged in
    let complete_route = format!("{}{}{}",block_route,disk_block,logical_block_route);
    // Gets the LBA sector size value and converts it to a string
    let value : u64 = std::fs::read_to_string(complete_route)?.trim().parse()?;
    
    Ok(value)
}

/// Gets the proper bytes from the disk to know if it is a good GPT and have LBA1 sign "EFI PART" 
fn try_gpt_sectors(logical_block_size: u64, mut open_disk: File) -> Result<bool, SparkError>{
    
    // The EFI part signature. "EFI PART" in ascii, 
    const EFI_PART_SIGN: [u8;8] = [0x45, 0x46, 0x49, 0x20, 0x50, 0x41, 0x52, 0x54];
    let mut buffer = [0u8;8];
    
    // Moves the pointer to the logical_block_size value (byte 512... 4096...)
    open_disk.seek(SeekFrom::Start(logical_block_size))?;
    // Reads the exact buffer and storages it 
    open_disk.read_exact(&mut buffer)?;
    
    // If the buffer matches with the EFI_PART_SIGN then it is a valid GPT partition.
    if buffer == EFI_PART_SIGN{
        return Ok(true)
    }
    Ok(false)
}

#[allow(unused)]
/// This function returns the number of partitions of an specific device.
fn get_number_partitions(logical_block_size: u64, disk: &str) -> Result<u8, SparkError>{
    Ok(1)
}

#[allow(unused)]
/// Checks and compare the partition type GUID
fn check_uefi_guid() -> Result<(), SparkError>{
    Ok(())
}
