use crate::IgnixError;
use crate::cli::ask_user_confirmation;
use crate::boot::esp;
use crate::cli::RemoveOptions;
use crate::config::BLOCK_DEV_ROUTE;
use crate::boot::disk;
/// This function should remove the ignix installation in the current ESP partition.
pub fn remove_ignix(options: RemoveOptions)->Result<(),IgnixError>{
    
    if !options.force{
        ask_user_confirmation("remove")?;
    }

    let disks = disk::get_system_disks(BLOCK_DEV_ROUTE, true, true)?; 
    let esp_target = disk::compatible_esp_partition(disks)?;
    esp::delete_ignix_structure(&esp_target)?;
    
    Ok(())
}
