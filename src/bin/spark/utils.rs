use std::io::{stdin, Write, stdout}; // Import the input output standard library 
use std::fs::{self, create_dir_all, remove_dir_all, copy};
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
    let flag = "--efi-bin="; 
    // Iterate the argument so it can check if the flag is inside the argument or not. 
    for arg in args{ 
        if arg.starts_with(flag){ 
            // Slices the argument after the flag and catchs the route as an argument 
            let route = String::from(&arg[flag.len()..]);
            if route.ends_with(".efi"){
                // If the archive the user is trying to give as an argument exists, it returns it. 
                if it_exists(Some(&route)){ 
                    return Some(route);  
                }
            }
        }
    }
    // If the route given as an argument didn't find anything useful, tries the default route. 
    let default_route = String::from("/usr/bin/lib/spark/sparkx64.efi"); 
    if it_exists(Some(&default_route)){
        return Some(default_route)
    }
    // If the EFI bin parameter is not correct, and the default route is not correct, then the program fails to execute.  
    return None;
}

/// Shows if an archive exists or not, returns a boolean.
pub fn it_exists(route: Option<&str>) -> bool{
    // If the archive exists, it returns true, if not, it returns false. 
    match route{
        Some(path) => {
            Path::new(&path).exists()
        },
        None => false 
    }
}

/// Dir operations such as deleting or creating directories
pub fn dir_operations(operations: Directories,route: Option<String>){
    // Those are the directories that spark needs to work properly. 
    let dir_array: [&str;3] = ["/EFI/BOOT", "/EFI/spark", "/loader/entries"]; 
    /*
     * This is important, parse the mount routes and detects which route of the system is assigned
     * for the ESP to be installed
    */ 
    let esp = detect_vfat();
    // This code is needed to convert the Option<String> value to a String type value. 
    let esp = match esp{ 
        Some(esp) => esp,
        None => {
            eprintln!("Haven't found any FAT32 file system, mounted on /boot, /boot/efi or /efi.");
            return;
        }
    };
    // This for is needed so the program can iterate the array of the routes and create them later.
    for dir in dir_array{
        let full_route = format!("{}{}",esp,dir);
        match operations {
            Directories::Create => {
                if let Err(error) = create_dir_all(&full_route){
                    eprintln!("Error creating {}: {}",full_route,error);
                };
                if dir == "/EFI/BOOT" || dir == "/EFI/spark"{
                    // 
                    if let Some(source_efi) = route.as_ref(){
                        /*
                         * The final file name depends on if the directory of destination is
                         * /efi/boot or /efi/spark . 
                         */
                        let file_name = if dir == "/EFI/BOOT"
                        {"BOOTX64.efi"} else 
                        {"sparkx64.efi"};
                        let destination = format!("{}/{}",full_route,file_name);
                        if let Err(error) = copy(&source_efi, &destination){
                            eprintln!("Error when trying to copy the binary {} to {}: {}",source_efi,destination,error);
                        } else{
                            println!("EFI binary copied correctly to: {}",destination)
                        }
                    }
                }
            },
            // To delete directories, first is needed to know if the directory actually exists or not
            Directories::Delete => {
                if it_exists(Some(&full_route)){
                    if let Err(error) = remove_dir_all(&full_route){
                        eprintln!("Error removing {}: {}",full_route,error);
                    }
                }
            }
        }
    }
}


/// Help message that will show up when spark is used with a wrong argument. 
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
