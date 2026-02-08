use std::fs::{self,create_dir_all,remove_dir_all,copy};
use crate::utils::exists;
use crate::boot::guid::get_esp_partition;
use std::process::exit;
/// Dir operations such as deleting or creating directories
pub enum Operations{
    Create,
    Delete
}

/// This function detects the mount point of the ESP partition.
fn esp_mountpoint() -> Option<String>{ // It returns the string with the final installation route of the ESP 
    let mounts = fs::read_to_string("/proc/self/mounts") // Opens the /proc/self/mounts file 
        .expect("Could not read '/proc/self/mounts'"); // If it can't find the mounts file the program says this 
    let esp_partition = match get_esp_partition(){
        Some(partition) => partition,
        None => {
            eprintln!("There is no ESP in the system.");
            exit(3);
        }
    };
    for device in mounts.lines(){
        let mount: Vec<&str> = device.split_whitespace().collect();
        if mount.len() >= 3{
            let device = mount[0];
            let mount_route = mount[1];
            let file_system = mount[2];
            if (&device == &esp_partition) && (file_system == "VFAT"){
                return Some(mount_route.to_string())
            }
        }
    }
    eprintln!("The program found the ESP partition {} but can't find the mount point. Try to mount the ESP partition first.", esp_partition);
    return None; 
}

/// This function is a confirmation for important operations such as delete or create directories in the file system.
pub fn dir_operations(operations: Operations,route: Option<String>){
    // Those are the directories that spark needs to work properly. 
    let dir_array: [&str;3] = ["/EFI/BOOT", "/EFI/spark", "/loader/entries"]; 
    /*
     * This is important, parse the mount routes and detects which route of the system is assigned
     * for the ESP to be installed
    */ 
    let esp = esp_mountpoint();
    // This code is needed to convert the Option<String> value to a String type value. 
    let esp = match esp{ 
        Some(esp) => esp,
        None => {
            return;
        }
    };
    // This for is needed so the program can iterate the array of the routes and create them later.
    for dir in dir_array{
        let full_route = format!("{}{}",esp,dir);
        match operations {
            Operations::Create => {
                if let Err(error) = create_dir_all(&full_route){
                    eprintln!("Error creating {}: {}",full_route,error);
                };
                if dir == "/EFI/BOOT" || dir == "/EFI/spark"{
                    // 
                    if let Some(source_efi) = route.as_ref(){
                        /*
                         * The final file name depends on if the directory of destination is
                         * /efi/boot or /efi/spark . 
                         */
                        let file_name = if dir == "/EFI/BOOT"{
                            "BOOTX64.efi"
                        } 
                        else {"sparkx64.efi"};
                        let destination = format!("{}/{}",full_route,file_name);
                        if let Err(error) = copy(&source_efi, &destination){
                            eprintln!("Error when trying to copy the binary {} to {}: {}",source_efi,destination,error);
                        } else{
                            println!("EFI binary copied correctly to: {}",destination)
                        }
                    }
                }
            },
            // To delete directories, first is needed to know if the directory actually exists or not
            Operations::Delete => {
                if exists(Some(&full_route)){
                    if let Err(error) = remove_dir_all(&full_route){
                        eprintln!("Error removing {}: {}",full_route,error);
                    }
                }
            }
        }
    }
}
