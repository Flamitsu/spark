use std::fs;
use std::process::exit;
/// Read the GUID LE bytes to see if it is ESP worth it 
pub fn _read_guid_bytes(){
    print!("This should do something")
}

/// Detect the devices of the current running system and returns them into a Vec<String>
pub fn _detect_devices() -> Vec<String>{
    let route:&str = "/sys/block/";
    let mut disks = Vec::new();
    let entries = if let Ok(operation_result) = fs::read_dir(&route){
        operation_result
    } else{
        eprintln!("Error. Can not read the {} route. Check if you have enough privilegies.", route);
        exit(2);
    };
    for entry in entries{
        if let Ok(entry) = entry{
            let file_name = entry.file_name().into_string().unwrap();
            /* If there is any sd device or nvme device and it is not a partition, the program will
             add the /dev/{device} to the 'disks' vector.*/
            if file_name.starts_with("sd") && file_name.len() == 3 
            || file_name.starts_with("nvme") && !file_name.contains("p"){
                disks.push(format!("/dev/{}",file_name))
            } 
        }
    }
    return disks;
}
