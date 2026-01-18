use std::io::{stdin, Write, stdout}; // Import the input output standard library 
use std::{fs};

pub enum Directories{
    Create,
    Delete
}

// This is the archive where is going to be stored common code between the modules
pub fn confirmation(context: &str) -> bool{ // This function needs an string and returns a bool
    println!("Type 'YES' to {} spark, or 'NO' to cancel: ", context); // Prints the user what they need to input
    stdout().flush().unwrap();
    let mut decision = String::new(); // Creates a new string 
    stdin().read_line(&mut decision).unwrap(); // Read the input 
    match decision.trim() { // Match the options
        "YES" => return true, // If the user said yes all caps, it returns true 
        "NO" => return false, // If the user said no all caps, it returns false
        _ => {
            println!("The program did not understood the input. Assuming 'NO'."); // If the user said something that is not 'NO' or 'YES' it returns false
            return false
        }
    }
}

// Function to skip confirmation 
// Arguments to skip confirmation on execution are: "-y and --yes"
pub fn skip_confirmation(args: &[String]) -> bool{ // Function returns a bool 
    let short_flag = "-y"; // Flag that can be used to skip confirmation 
    let long_flag = "--yes"; // Flag that can be used to skip confirmation 
    for arg in args{ // Iterate arg 
        if arg == short_flag || arg == long_flag{ // If the argument matchs, return true
            return true; 
        }
    }
    return false // If the argument does not match, return false, so it does not skip. 
}

/* 
* This function detect vfat partitions and the following mounted partitions:
* /boot 
* /boot/efi
* /efi 
*/
pub fn detect_vfat() -> Option<String>{ // It returns the string 
    let mounts = fs::read_to_string("/proc/self/mounts") // Opens the /proc/self/mounts file 
        .expect("Could not read '/proc/self/mounts'"); // If it can't find the mounts file the program says this

    for line in mounts.lines() { // Iterate all the lines of the mounts aviables
        let parts: Vec<&str> = line.split_whitespace().collect(); // declare a vector, every object is separated by a tab or a whitespace, and collect converts it into a vector
        if parts.len() >= 3 { // If the vector have more length than 3, then 
            let mount_point = parts[1]; // The second slice is the mount point string 
            let file_system = parts[2]; // The third slice is the file system type
            if (file_system == "vfat") && // If the file_system is vfat and... 
            mount_point == "/boot" || // Any of these routes are the mount_point, it returns the mount_point
            mount_point == "/boot/efi" || 
            mount_point == "/efi"{ // To date, there are three types of routes where you can install a bootloader and mount a FAT32 partition.
                return Some(mount_point.to_string());
            };
        };
    };
    return None; // If it does not detect any vfat partition that is mounted on either those mount_points, then it is going to return None. 
}

// This function gets the argument of installation and checks if it matches the argument flags
pub fn get_efi_bin_path(_args: &[String]) -> Option<String>{
    let _long_flag = "--efi-bin=";
    let mut _command = String::new();
    return None;
}

// Help message that will show up when spark is used wrong
pub fn show_help() {
    print!("Usage: spark [COMMAND] [OPTIONS] 
    
    Commands: 
        install     Installs spark binary into the boot partition.
        remove      Remove the spark installation from the boot partition 
        update      Detects and adds new kernel entries.
        clean       Clean old system entries.
        help        Shows this dialog. 

    Options: 
        -y, --yes   Skip confirmation prompts. 
        --efi-bin   [PATH] Specify the route of the EFI binary (use with 'install')\n"
    );
}

// Shows if an archive exists or not.
pub fn _it_exists(_route: Option<String>) -> bool{
    return true
}
// Dir operations such as deleting or creating directories
pub fn dir_operations(operations: Directories,_route: Option<String>){
    match operations{
        Directories::Create => { // 
            // Placeholder
            println!("Create operation.")
        },
        Directories::Delete => { // The delete operation of the directories enum struct
            //Placeholder
            println!("Delete operation.")
        }
    }
}
