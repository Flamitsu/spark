use crate::utils::exists;
use std::io::{Write, stdin,stdout};
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
                if exists(Some(&route)){ 
                    return Some(route);  
                }
            }
        }
    }
    // If the route given as an argument didn't find anything useful, tries the default route. 
    let default_route = String::from("/usr/bin/lib/spark/sparkx64.efi"); 
    if exists(Some(&default_route)){
        return Some(default_route)
    }
    // If the EFI bin parameter is not correct, and the default route is not correct, then the program fails to execute.  
    return None;
}

/// create directories in the file system. 
pub fn confirmation(context: &str) -> bool{
    println!("Type 'YES' to {} spark, or 'NO' to cancel: ", context);
    // Cleans the stdout buffer. 
    stdout().flush().ok();
    // Creates a new string where the input of the user is going to be storaged.
    let mut decision = String::new();
    // Read the input and storage it to the 'decision' variable. 
    stdin().read_line(&mut decision).ok(); 

    match decision.trim() { // Match the options
        "YES" => return true, 
        "NO" => return false,
        // If the user uses the argument wrong, the program assumes 'NO'.
        _ => {
            eprintln!("The program did not understood the input. Assuming 'NO'.");
            return false
        }
    }
}
