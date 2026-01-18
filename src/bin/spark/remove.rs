// This is the script for the uninstall of the spark program
use crate::utils::{Directories, confirmation, detect_vfat, dir_operations};

/*
* Work in progress, this is meant to remove the spark installation if you execute it.
*/

pub fn remove_installation(skip_confirmation: bool){
    let user_confirmation = if skip_confirmation { 
            true 
        } else { 
            confirmation("install") 
        };
    if !user_confirmation{ // If the user says no:
        println!("The removal process has been aborted.");
        return 
    };
    let route = detect_vfat(); // The route where the bootmanager was installed
    dir_operations(Directories::Delete, route);
}
