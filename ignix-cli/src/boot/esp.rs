// This file prepare and configure the EFI System Partition detecting the current path of it.
use crate::errors::IgnixError;
use std::path::Path;
/// This enum is used for dir operations. Such as create or delete them
#[allow(unused)]
pub enum Operations{
    Create,
    Delete
}

#[allow(unused)]
/// This function is the one that manages the ESP structure. (delete or create it.)
pub fn manage_esp_structure(operation: Operations, partition: &str, efi_binary_route: &Path) 
    -> Result<(),IgnixError>{
    
    Ok(())
}

/// This function returns the mountpoint provided by compatible_esp_device function.
#[allow(unused)]
pub fn get_esp_mountpoint() -> Result<(), IgnixError>{
    Ok(())
}
