// This archive is used to know which device is the correct for the installation of the ESP
use crate::errors::SparkError;
use std::fs::read_to_string;

#[allow(unused)]
/// Returns the actual ESP device, so the esp.rs module can discover its path.
pub fn compatible_esp_device() -> Result<(), SparkError>{
    Ok(())
}
#[allow(unused)]
/// Returns a list of the disks in the system (block ones).
pub fn get_disks() -> Result<(), SparkError>{
    let mut devices: Vec<String>;
    let possible_devices = read_to_string("/sys/block/")?; 
    for device in possible_devices.lines(){
    
    }
    Ok(())
}
#[allow(unused)]
/// Checks if the provided name is valid or not. (Example: sda, nvme... Bad example: dm-0) 
pub fn check_valid_block_name() -> Result<bool, SparkError>{   
    
    Ok(true)
}
#[allow(unused)]
/// Get disk logical sector size (Example: 512, 4096...)
pub fn get_disk_logical_sector_size() -> Result<(), SparkError>{
    Ok(())
}
#[allow(unused)]
/// This function returns the number of partitions of an specific device.
pub fn get_number_partitions() -> Result<(), SparkError>{
    Ok(())
}
#[allow(unused)]
/// Checks and compare the partition type GUID
pub fn check_uefi_guid() -> Result<(), SparkError>{
    Ok(())
}
