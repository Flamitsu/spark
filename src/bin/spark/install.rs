use crate::utils::{confirmation, dir_operations}; // Modules from the utils crate
use crate::utils::Directories; // Neccesary enums
use crate::auto_detect::{detect_kernels}; // Modules from the auto_detect crate 

// This is the code that spark uses for the installation process of the EFI binary
pub fn install(skip_confirmation: bool, efi_binary: Option<String>){
    let user_confirmation = if skip_confirmation{ // If the user passed the '-y or --yes' flag: 
        true // Return true directly
    } 
    else{ // If the user did not passed the '-y or --yes' flag: 
        confirmation("install") // Confirm it with user input
    };
    if !user_confirmation{ // If the user does not confirm, then the code flow stops here 
        println!("The installation process has been aborted.");
        return
    }
    dir_operations(Directories::Create, efi_binary); // Create the directory structure 
    detect_kernels(); // Detect the installed kernels
}
