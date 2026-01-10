// This is the script for the uninstall of the spark program
use crate::utils::{confirmation,detect_vfat};

/*
* Work in progress, this is meant to remove the spark installation if you execute it.
*/

pub fn remove_installation(skip_confirmation: bool){
    let confirmation = if skip_confirmation{ true } else{ confirmation("install") };
    if confirmation{
        println!("Removing spark installation..."); // Placeholder
        let _installation_route = detect_vfat(); // The route where the bootmanager was installed
    }
    else{
        println!("The removal process has been aborted.");
    }
}
