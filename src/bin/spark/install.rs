// This is the code that spark uses for the installation process
use crate::utils::{confirmation, detect_vfat}; // Modules from the utils crate
use crate::auto_detect::{detect_kernels}; // Modules from the auto_detect crate 
pub fn install(skip_confirmation: bool){
    let user_confirmation = if skip_confirmation{ // If the user passed the '-y or --yes' flag: 
        true // Return true directly
    } 
    else{ // If the user did not passed the '-y or --yes' flag: 
        confirmation("install") // Confirm it with user input
    };
    if user_confirmation { // If the user actually confirms
        let _installation_route = detect_vfat(); // Detect the default installation route 
        if _installation_route == None{ // If it didn't found any compatible route then:
            println!("Haven't found any FAT32 file system, mounted in /boot, /efi or /boot/efi");
            return; // Ends the installation process
        }
        detect_kernels(); // Detect the installed kernels
    }
    else{
        println!("The installation process has been aborted."); // Ends the program
    }
}
