use crate::boot::disk;
use crate::cli::{InstallOptions, ask_user_confirmation};
use crate::config::LOGICAL_BLOCK;
use crate::IgnixError;
/// This function should install the ignixx64.efi binary in the current ESP partition.
pub fn install_ignix(options: InstallOptions) -> Result<(), IgnixError>{
    // If the user did not used the flags '-y' or '--yes' in the execution, ask confirmation prompt
    if !options.force{
        ask_user_confirmation("install")?;
    }
    
    let _esp_target = if let Some(route) = &options.install_route{
        route.to_string_lossy().to_string()
    } else {
        let disks = disk::get_system_disks(LOGICAL_BLOCK, &options)?;
        disk::compatible_esp_partition(disks)?
    };
 
    Ok(())
}
