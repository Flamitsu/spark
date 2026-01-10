// This is the script for the uninstall of the spark program
use crate::utils::{confirmation,detect_vfat};
pub fn remove_installation(skip_confirmation: bool){
    let confirmation = if skip_confirmation{ true } else{ confirmation("install") };
    if confirmation{
        println!("Removing spark installation..."); // Placeholder
        detect_vfat();
    }
    else{
        println!("The removal process has been aborted.");
    }
}
