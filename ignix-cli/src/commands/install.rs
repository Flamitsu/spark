use crate::boot::{disk, esp};
use crate::cli::{InstallOptions, ask_user_confirmation};
use crate::config::BLOCK_DEV_ROUTE;
use crate::IgnixError;
/// This function should install the ignixx64.efi binary in the current ESP partition.
pub fn install_ignix(options: InstallOptions) -> Result<(), IgnixError>{
    // If the user did not used the flags '-y' or '--yes' in the execution, ask confirmation prompt
    if !options.force{
        ask_user_confirmation("install")?;
    }
    
    let esp_target = if let Some(route) = &options.install_route{
        route.to_string_lossy().to_string()
    } else {
        let disks = disk::get_system_disks(BLOCK_DEV_ROUTE, options.allow_virtual, options.removable_device)?;
        disk::compatible_esp_partition(disks)?
    };
    esp::create_ignix_structure(&esp_target, &options.efi_bin, options.no_nvram, options.force)?;
    Ok(())
}
