use std::io::{stdin, Write, stdout}; // Import the input output standard library 
use std::fs::{self, create_dir_all, remove_dir_all};
use std::path::Path;
/// The enum structure 'Directories' is made to make directories operations such as create or delete directories 
pub enum Directories{ 
    Create, 
    Delete
}

// This function is a confirmation for important operations such as delete or create directories in the file system.
/// The confirmation function is to ask user input for important operations, such as delete or
/// create directories in the file system. 
pub fn confirmation(context: &str) -> bool{
    println!("Type 'YES' to {} spark, or 'NO' to cancel: ", context);
    // Cleans the stdout buffer. 
    stdout().flush().unwrap();
    // Creates a new string where the input of the user is going to be storaged.
    let mut decision = String::new();
    // Read the input and storage it to the 'decision' variable. 
    stdin().read_line(&mut decision).unwrap(); 

    match decision.trim() { // Match the options
        "YES" => return true, 
        "NO" => return false,
        _ => {
            eprintln!("The program did not understood the input. Assuming 'NO'."); // If the user said something that is not 'NO' or 'YES' it returns false
            return false
        }
    }
}

/// Function to skip user information. Needs a '-y' and '--yes' parameter when executed to skip the confirmation
pub fn skip_confirmation(args: &[String]) -> bool{ 
    // Those are the short and the long flag to skip the confirmation process. 
    let short_flag = "-y"; 
    let long_flag = "--yes";
    // This for iterates the argument and if the arguments match, then it returns true (skip confirmation)
    for arg in args{ 
        if arg == short_flag || arg == long_flag{
            return true; 
        }
    }
    // If the flags are not in the execution argument, then it returns false (do not skip user confirmation)
    return false 
}

/// This function detect the vfat partitions in the following mount points: '/boot,/boot/efi,/efi'.
fn detect_vfat() -> Option<String>{ // It returns the string with the final installation route of the ESP 
    let mounts = fs::read_to_string("/proc/self/mounts") // Opens the /proc/self/mounts file 
        .expect("Could not read '/proc/self/mounts'"); // If it can't find the mounts file the program says this
    /* 
     * Runs and compares every line of the mounts variable route and see 
     * if it matches the following requirements:
     * Mountpoints either in /boot,/boot/efi or /efi.
     * FAT32. Filesystem of the mountpoint. 
    */
    for line in mounts.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // This comparation tries to chop off wrong-formatted lines
        if parts.len() >= 3 { 
            let mount_point = parts[1]; 
            let file_system = parts[2];
            if (file_system == "vfat") && 
            mount_point == "/boot" ||
            mount_point == "/boot/efi" ||
            mount_point == "/efi"{
                return Some(mount_point.to_string());
            };
        };
    };
    // If the neither one of the lines matches the requirements, the program returns None. 
    return None; 
}

/// This function gets an specific flag in the binary execution, and gets sliced to get the .efi binary   
pub fn get_efi_bin_path(args: &[String]) -> Option<String>{
    // Define the valid flag in the execution of the binary 
    let flag = "--efi-bin="; // Define the long_flag
    for arg in args{ // Iterate the argument 
        if arg.starts_with(flag){ // Checks if the argument starts with long_flag 
            let route = String::from(&arg[flag.len()..]); // Slices the argument and catchs the route
            if route.ends_with(".efi"){ // check if the archive ends in .efi
                if it_exists(Some(&route)){ // Checks if the route exists
                    return Some(route); // Returns the actual value 
                }
            }
        }
    }
    let default_route = String::from("/usr/bin/lib/spark/sparkx64.efi"); 
    // This condition tries to execute the default installation route if the previous one failed. 
    if it_exists(Some(&default_route)){
        return Some(default_route)
    }
    // If the EFI bin parameter is not correct, and the default route is not correct, then the program fails to execute.  
    return None;
}

// Shows if an archive exists or not.
pub fn it_exists(route: Option<&str>) -> bool{
    match route{
        Some(path) => {
            Path::new(&path).exists() // If the archive exists, it returns true, else, false.
        },
        None => false // If the archive does not exists, it returns false 
    } 
}

// Dir operations such as deleting or creating directories
pub fn dir_operations(operations: Directories,_route: Option<String>){
    let dir_array: [&str;3] = ["/EFI/BOOT", "/EFI/spark", "/loader/entries"]; 
    let esp = detect_vfat(); // The installation/uninstall routes
    let esp = match esp{ 
        Some(esp) => esp, // Converts the Option<String> type to String type. 
        None => { // If it didn't found any compatible route then:
            eprintln!("Haven't found any FAT32 file system, mounted on /boot, /boot/efi or /efi.");
            return; // Ends the operations here.
        }
    };
    for dir in 0..dir_array.len(){
        let full_route = format!("{}{}",esp,dir_array[dir]);
        if !it_exists(Some(&full_route)){ // If the directory does not exists 
            match operations{
                Directories::Create => { // And the operation is create 
                    match create_dir_all(full_route){ // Create the directories 
                        Ok(_) => continue, // Continue with the execution 
                        Err(error) => eprintln!("Error: {}",error) // If there is an error, print it. 
                    }
                },
                Directories::Delete => { // And the operation is delete 
                    continue // Continue with the iteration. 
                }
            }
        } else { // If the directory exists
            match operations{
                Directories::Delete => { // And the operation is delete 
                    match remove_dir_all(full_route){ // Remove the directory 
                        Ok(_) => {}, // If the operation is successfull, continue 
                        Err(error) => {eprintln!("Error: {}", error)} // If there is an error, print it 
                    }
                }
                Directories::Create => { // And the operation is create 
                    continue // Continue with the iteratio 
                }
            }
        }
    }
}


// Help message that will show up when spark is used with a wrong argument. 
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
