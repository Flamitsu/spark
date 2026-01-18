use std::io::{stdin, Write, stdout}; // Import the input output standard library 
use std::fs::{self, create_dir};
use std::path::Path;

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

/* 
* Function to skip confirmation 
* Arguments to skip confirmation on execution are: "-y and --yes"
*/
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
fn detect_vfat() -> Option<String>{ // It returns the string 
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
pub fn get_efi_bin_path(args: &[String]) -> Option<String>{
    let long_flag = "--efi-bin="; // Define the long_flag
    for arg in args{ // Iterate the argument 
        if arg.starts_with(long_flag){ // Checks if the argument starts with long_flag 
            let route = &arg[long_flag.len()..]; // Slices the argument and catchs the route
            if route.ends_with(".efi"){ // check if the archive ends in .efi
                if it_exists(Some(route.to_string())){ // Checks if the route exists
                    return Some(route.to_string()); // Returns the actual value
                }
            }
        }
    }
    return None; // If the efi bin path is not correct it returns none
}

// Shows if an archive exists or not.
pub fn it_exists(route: Option<String>) -> bool{
    match route{
        Some(path) => {
            Path::new(&path).exists() // If the archive exists, it returns true.
        },
        None => false // If the archive does not exists, it returns false 
    }
}

// Dir operations such as deleting or creating directories
pub fn dir_operations(operations: Directories,_route: Option<String>){
    let esp = detect_vfat(); // The installation/uninstall route  
    if esp == None{ // If it didn't found any compatible route then:
        println!("Haven't found any FAT32 file system, mounted in /boot, /efi or /boot/efi");
        return ; // Ends the installation process
    }
    let dir_array: [&str;5] = [
        "/EFI", "/EFI/BOOT", "/EFI/spark", "/loader", "/loader/entries"];
    match operations{
        Directories::Create => { // This option will create the structure in the ESP route 
            for dir in 1..dir_array.len() {
                if it_exists(Some(dir_array[dir].to_string())){
                    print!("XD");
                }
            }  
        },
        Directories::Delete => { // This option will delete the structure in the ESP route
            println!("Delete operation.") // Placeholder
        }
    }
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
