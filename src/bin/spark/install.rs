use crate::utils::{confirmation, detect_vfat, dir_operations}; // Modules from the utils crate
use crate::utils::Directories; // Neccesary enums
use crate::auto_detect::{detect_kernels}; // Modules from the auto_detect crate 

// This is the code that spark uses for the installation process of the EFI binary
pub fn install(skip_confirmation: bool, _efi_binary: Option<String>){
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
    let esp = detect_vfat(); // Detect the default installation routes
    if esp == None{ // If it didn't found any compatible route then:
        println!("Haven't found any FAT32 file system, mounted in /boot, /efi or /boot/efi");
        return; // Ends the installation process
    }
    dir_operations(Directories::Create, esp); // This doesn't does anything for the moment. I have only put this line so the rust compiler does not say there is unused code. This is going to be replaced eventually 
    detect_kernels(); // Detect the installed kernels
}
