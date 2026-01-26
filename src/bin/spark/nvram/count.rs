use std::fs;
pub fn count_entries() -> i16{
    // The standard boot entry in the NVRAM's variable starts with "Boot"
    let _boot_str:&str = "Boot";
    /*
    * If you wonder why this is right here, hardcoded, it is because this is the standard UUID that 
    * is marked for the blob in the UEFI indication to search for a boot .efi file.
    */
    let _uefi_uid:&str = "-8be4df61-93ca-11d2-aa0d-00e098032b8c";
    // This is where the nvram variables are currently stored 
    let nvram_directory:&str = "/sys/firmware/efi/efivars/";
    let entries = fs::read_dir(&nvram_directory);
    let mut counting_stars = 0;
    for entry in entries{
        let mut counting_stars = counting_stars + 1;
        println!();
    }
    1
}
