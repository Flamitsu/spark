use std::fs;
pub fn count_entries() -> u16{
    // The standard boot entry in the NVRAM's variable starts with "Boot"
    let boot_str:&str = "Boot0";
    let nvram_directory:&str = "/sys/firmware/efi/efivars";
    // If the operation is successfull, the variable will have all the contents of that dir. 
    let entries = if let Ok(operation_result) = fs::read_dir(&nvram_directory){
        operation_result
    } else{
        eprintln!("Error while reading the NVRAM variables. Check if you have enough privilegies.");
        return 0;
    };
    /*
    * If you wonder why this is right here, hardcoded, it is because this is the standard UUID that 
    * is marked for the blob in the UEFI indication to search for a boot .efi file.
    */
    let uefi_uid:&str = "-8be4df61-93ca-11d2-aa0d-00e098032b8c"; 
    // Storages the number of BootXXXX entries in the NVRAM.
    let mut counting:u16 = 0;
    for entry in entries{
        if let Ok(entry) = entry{
            // This catches the name of the file in a non UTF-8 string.
            let file_name = entry.file_name();
            // This converts the name of the file to UTF-8 string.
            let file_name = file_name.to_string_lossy();
            if file_name.starts_with(&boot_str) && file_name.contains(&uefi_uid){
                counting = counting + 1;
            }
        }
    }
    return counting;
}
