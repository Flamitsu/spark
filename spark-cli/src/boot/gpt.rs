// This archive is used to know which device is the correct for the installation of the ESP
use crate::errors::SparkError;
use std::fs::read_dir;

/// Returns the actual ESP device, so the esp.rs module can discover its path.
pub fn compatible_esp_device() -> Result<String, SparkError>{
    const BLOCK_DEV_ROUTE: &str = "/sys/block/";
    const LOGICAL_BLOCK: &str = "/queue/logical_block_size";
    let aviable_disks = get_disks(BLOCK_DEV_ROUTE)?;
    
    for disk in aviable_disks{
        let dev_disk: String = format!("/dev/{}",disk);
        let block_size: u16 = get_disk_logical_sector_size(&disk,BLOCK_DEV_ROUTE,LOGICAL_BLOCK)?;
        let _number_partitions: u8 = get_number_partitions(block_size,&dev_disk)?;
        println!("{} LBA Size: {}",disk,block_size);
    }

    Ok(String::new())
}

/// Returns a list of the disks in the system (block ones).
fn get_disks(block_route: &str) -> Result<Vec<String>, SparkError>{
    // Creates a new empty vec where the found disk devices are going to be storaged
    let mut disks:Vec<String> = Vec::new();
    // Creates an empty vec
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
    logical_block_route: &str) -> Result<u16, SparkError>{
    // The route where the logical block sector size is storaged in
    let complete_route = format!("{}{}{}",block_route,disk_block,logical_block_route);
    // Gets the LBA sector size value and converts it to a string
    let value : u16 = std::fs::read_to_string(complete_route)?.trim().parse()?;
    Ok(value)
}

#[allow(unused)]
/// Gets the proper bytes from the disk to know if it is a good GPT and have LBA0 MBR protective 
fn try_gpt_sectors(logical_block_size: u16) -> bool{
    false
}

#[allow(unused)]
/// This function returns the number of partitions of an specific device.
fn get_number_partitions(logical_block_size: u16, disk: &str) -> Result<u8, SparkError>{
    Ok(1)
}

#[allow(unused)]
/// Checks and compare the partition type GUID
fn check_uefi_guid() -> Result<(), SparkError>{
    Ok(())
}
