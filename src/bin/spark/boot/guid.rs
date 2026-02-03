use std::fs::{self, File};
use std::io::{Seek, SeekFrom, Read};
use std::process::exit;
/// Read the GUID little endian (LE) and big endian (BE) bytes to see if the device contains an ESP partition 
pub fn esp_guid_device() -> Option<String>{
    let disks = detect_devices();
    /* If you wonder why those integers are u64 and not u16 and u8, because I can't. seek method do 
    * not support it.*/ 
    // The sector size in bytes 
    let sector_size:u64 = 512;
    // The Logical Block Addresing 2 is where the GUID needed is located.
    let lba_index:u64 = 2;
    // The offset byte is where the disk is started to get read
    let offset:u64 = sector_size * lba_index;
    let esp_guid_bytes: [u8;16]= [
        0x28, 0x73, 0x2A, 0xC1, // (LE) DATASET1 -> 28 73 2A C1 -> C12A7328 
        0x1F, 0xF8, // (LE) DATASET2 -> 1F F8 -> 1FF8
        0xD2, 0x11, // (LE) DATASET3 -> 11 D2 -> D211
        0xBA, 0x4B, // (LE) DATASET4 -> BA 4B -> BA4B
        0x00, 0xA0, 0xC9, 0x3E, 0xC9, 0x3B // (BE) DATASET5 -> 00 A0 C9 3E C9 3B -> 00A0C93EC93B  
        /* 
         * In the end, all this values form this last GUID in LE:
         * LE AND BE (GUID RAW): 28 73 2A C1 1F F8 11 D2 BA 4B 00 A0 C9 3E C9 3B 
         * String (GUID LE AND BE to String): C12A7328-1FF8-D211-BA4B-00A0C93EC93B
        */
    ];
    for disk_path in disks{
        // If there is an error with a disk, the program will change this to true. 
        let mut disk = if let Ok(open_disk) = File::open(&disk_path){
            open_disk
        } else{
            eprintln!("Error. Can not open the disk {} check if you have enough privileges.",disk_path);
        continue;
        };
        // Moves the cursor to an specific offset.
        if let Err(error) = disk.seek(SeekFrom::Start(offset)){
            eprintln!("Error {error}. Can not move the disk pointer to the LBA2 sector in the disk.");
            continue;
        }
        // Creates a buffer to read the GUID 
        let mut buffer = [0u8;16];
        println!("{}",disk_path);
        // Reads the buffer and if there is an error it says it to the user.
        if let Err(error) = disk.read_exact(&mut buffer){
            eprintln!("Can not read bytes from {} : {}",disk_path,error);
            continue;
        }
        else {
            // If the disk has a ESP partition, then, the disk is returned
            if buffer == esp_guid_bytes{
                return Some(disk_path);
            } 
            continue;
        };
    }
    return None;
}
// This function should work for getting the block size in the disk to know if LBA2 is in 1024(512)
// or in another block size like 4096. 
pub fn _get_block_size() -> u16{
    1
}


/// Detect the devices of the current running system and returns them into a Vec<String>
pub fn detect_devices() -> Vec<String>{
    let route:&str = "/sys/block/";
    let mut disks:Vec<String> = Vec::new();
    let disk_devices = if let Ok(list_devices) = fs::read_dir(&route){
        list_devices
    } else{
        eprintln!("Error. Can not read the {} route. Check if you have enough privileges.", route);
        exit(2);
    };
    for disk in disk_devices{
        if let Ok(disk) = disk{
            if let Ok(disk_name) = disk.file_name().into_string(){
            /* If there is any sd device or nvme device and it is not a partition, the program will
             add the /dev/{device} to the 'disks' vector.*/
                if (disk_name.starts_with("sd") && disk_name.len() > 3) || 
                (disk_name.starts_with("nvme") && !disk_name.contains("p")){
                    disks.push(format!("/dev/{}",disk_name))
                }
            } else{
                eprintln!("Error. Can not iterate correctly through a disk inside '{}'.",route);
                continue;
            }
        }
    }
    return disks;
}
