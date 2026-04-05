use crate::{cli::InstallOptions, errors::{IgnixError, io}};
use std::{fs::read_to_string, path::{Path, PathBuf}};
use crate::config::MOUNTPOINTS;
/// This function is the one that manages the ESP structure. (delete or create it.)
pub fn create_install_structure(efi_bin:&Path, partition:&str, nvram:bool) -> Result<(), IgnixError>{
    Ok(())
}

pub fn delete_install_structure() -> Result<(), IgnixError>{
    Ok(())
}

/// This function returns the mountpoint provided by compatible_esp_device function.
pub fn get_esp_mountpoint(partition_name: &str) -> Result<Option<PathBuf>, IgnixError>{
    let route = &PathBuf::from(MOUNTPOINTS);
    
    if !route.exists(){
        Err(io::Error::NotFound(route.display().to_string()))?
    }

    let file_content = read_to_string(route)?;
    for line in file_content.lines(){
        if !line.starts_with(partition_name){
            continue;
        }

        if let Some(mountpoint) = line.split_whitespace().nth(1){
            return Ok(Some(PathBuf::from(mountpoint)));
        }
    }
    Ok(None)
}
