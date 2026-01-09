use crate::utils::{confirmation, detect_vfat}; // Modules from the utils crate
use crate::auto_detect::{detect_kernels}; // Modules from the auto_detect crate 
pub fn install(){
    let confirm = confirmation("install"); // Confirmation needed
    if confirm == true{ // If the user actually confirms
        let _installation_route = detect_vfat(); // Detect the default installation route 
        if _installation_route == None{
            println!("Haven't found any FAT32 file system, mounted in /boot, /efi or /boot/efi");
            return;
        }
        detect_kernels(); // Detect the installed kernels
    }
    else{
        println!("The installation process has been aborted."); // Ends the program
    }
}
