// This file prepare and configure the EFI System Partition detecting the current path of it.
use crate::errors::SparkError;
use std::path::Path;
/// This enum is used for dir operations. Such as create or delete them
pub enum Operations{
    Create,
    Delete
}

/// This function is the one that manages the ESP structure. (delete or create it.)
pub fn manage_esp_structure(operation: Operations, efi_binary_route: &Path) -> Result<(), SparkError>{ 
    Ok(())
}
/// This function returns the mountpoint provided by compatible_esp_device function.
pub fn _search_current_mountpoint(){

}
