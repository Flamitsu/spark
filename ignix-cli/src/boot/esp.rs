use crate::{cli::{InstallOptions, ask_user_confirmation}, errors::{IgnixError, io, nvram}};
use std::{fs::{self, read_to_string}, path::{Path, PathBuf}};
use crate::config::{MOUNTPOINTS, ESP_DIR};
/// This function is the one that manages the ESP structure. (delete or create it.)
pub fn create_ignix_structure(partition:&str, efi_bin:&Path, no_nvram:bool, force: bool)
    -> Result<(),IgnixError>{
    
    let Some(route) = get_esp_mountpoint(partition)? else { 
        Err(io::Error::NotFound(partition.into()))? 
    };

    let efi_fallback = route.join("EFI/BOOT/BOOTX64.efi");
    if (efi_fallback.exists() && no_nvram && !force){
        ask_user_confirmation("remove the BOOTX64.efi old binary to replace it with ignix one")?;
    }

    for dir in ESP_DIR{
        let dir_route = route.join(dir);
        if !dir_route.exists(){
            fs::create_dir_all(&dir_route);
        }
        if dir.ends_with("ignix/"){
            fs::copy(efi_bin, &dir_route.join("ignixx64.efi"))?;
        }
        if dir.ends_with("BOOT/"){
            fs::copy(efi_bin, &efi_fallback)?;
        }
    }

    Ok(())
}

pub fn delete_ignix_structure(partition: &str) -> Result<(), IgnixError>{
    let Some(route) = get_esp_mountpoint(partition)? else{
        Err(io::Error::NotFound(partition.into()))?
    };
    
    let bootloader_home = route.join("EFI/ignix");
    if bootloader_home.exists(){
        fs::remove_dir_all(bootloader_home)?;
    }

    let config_home = route.join("loader/ignix.conf");
    if config_home.exists(){
        fs::remove_file(config_home)?
    }
    Ok(())
}

/// This function returns the mountpoint provided by compatible_esp_device function.
pub fn get_esp_mountpoint(partition_name: &str) -> Result<Option<PathBuf>, IgnixError>{
    let route = Path::new(MOUNTPOINTS);
    
    if !route.exists(){
        Err(io::Error::NotFound(route.display().to_string()))?
    }

    let file_content = read_to_string(route)?;
    let expected_dev_string = format!("/dev/{}", partition_name);

    for line in file_content.lines(){
        let mut parts = line.split_whitespace();
        
        let device = parts.next();
        let mountpoint = parts.next();
        if let (Some(dev), Some(mnt)) = (device, mountpoint) 
            && dev == expected_dev_string{
                return Ok(Some(PathBuf::from(mnt)));
        } 
    }
    Ok(None)
}
