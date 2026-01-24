// This is the script for the uninstall of the spark program
use crate::cli::confirmation;
use crate::boot::esp::{Operations, dir_operations};
/*
* Work in progress, this is meant to remove the spark installation if you execute it.
*/
pub fn remove_installation(skip_confirmation: bool, efi_bin: Option<String>){
    let user_confirmation = if skip_confirmation { 
            true 
        } else { 
            confirmation("install") 
        };
    if !user_confirmation{ // If the user says no:
        println!("The removal process has been aborted.");
        return 
    };
    dir_operations(Operations::Delete, efi_bin);
}
