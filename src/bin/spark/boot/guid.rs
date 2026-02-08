use std::fs::{self, File};
use std::io::{Seek, SeekFrom, Read};
use std::process::exit;
/// Read the GUID little endian (LE) and big endian (BE) bytes to see if the device contains an ESP partition 
pub fn get_esp_partition() -> Option<String>{
    // The disks aviables inside the system. 
    let disks = detect_devices();
    /*
    * All this values from the last GUID in LE to the first:
    * LE and BE (GUID RAW): 28 73 2A C1 1F F8 11 D2 BA 4B 00 A0 C9 3E C9 3B 
    * String (GUID LE and BE to String): C12A7328-1FF8-D211-BA4B-00A0C93EC93B 
    */
    const ESP_GUID_BYTES: [u8;16]= [
        0x28, 0x73, 0x2A, 0xC1, // (LE) DATASET1 -> 28 73 2A C1 -> C12A7328 
        0x1F, 0xF8, // (LE) DATASET2 -> 1F F8 -> 1FF8
        0xD2, 0x11, // (LE) DATASET3 -> 11 D2 -> D211
        0xBA, 0x4B, // (LE) DATASET4 -> BA 4B -> BA4B
        0x00, 0xA0, 0xC9, 0x3E, 0xC9, 0x3B // (BE) DATASET5 -> 00 A0 C9 3E C9 3B -> 00A0C93EC93B  
    ];
    for disk_path in disks{
        // If there is an error with a disk, the program will change this to true.
        let mut disk = match File::open(&disk_path){
            Ok(disk_file) => disk_file,
            Err(error) => {
                eprintln!("Error opening {} : {}",disk_path,error);
                continue;
            }
        };
        // Moves the cursor to the LBA2 sector inside the disk.
        if let Err(error) = disk.seek(SeekFrom::Start(1024)){
            eprintln!("Error. Can not move the disk pointer to the LBA2 sector to read the partitions. {}",error);
            continue;
        };

        // Reads all the possible 128 entries possible in the GPT partitions.
        for partition_number in 1..=128{
            // Creates a buffer to read the GUID
            let mut buffer = [0u8;16];
            // Reads the buffer and if there is an error skips to the next disk.
            if let Err(error) = disk.read_exact(&mut buffer){
                eprintln!("Can not read bytes from {} : {}", disk_path,error);
                break;
            };
            // If the disk has a ESP partition, then, the disk is returned.
            if buffer == ESP_GUID_BYTES{
                if disk_path.contains("sd"){
                    return Some(format!("{}{}",disk_path,partition_number));
                } else{
                    return Some(format!("{}p{}",disk_path,partition_number))
                }
            }
            
            /*
            * When the buffer is 000000... then it skips to the next disk
            * because it should mean it is the end of the partition table.
            */
            if buffer == [0u8;16]{
                break;
            };
            // Moves the cursor 112 bytes ahead. If there is an error, skips to the next disk.
            if let Err(error) = disk.seek(SeekFrom::Current(112)){
                eprintln!("Error. Can not move the disk pointer inside the LBA2 sector. {}",error);
                break;
            }
        }
    };
    return None;
}

// Function still WIP 
fn _entries_start_lba(){
    todo!("This function should get the LBA2 starting position instead of assume 1024 byte offset");
}

/// Function to see if the argument is a disk or not. 
fn is_block_device (name: &str) -> bool{
    // If the disk is an nvme, it checks that the name does not contain a 'p':
    if name.starts_with("nvme") && !name.contains("p"){
        return true;
    };
    // If the name is longer than 3, starts with 'vd' or 'sd' and does not end in a number:
    if (name.starts_with("sd") || name.starts_with("vd")) && name.len() >= 3{
        if let Some(last_char) = name.chars().last(){
            if !last_char.is_numeric(){
                return true;
            }
        }
    }
    // If anything before didn't worked, then probably it's not a block device.
    return false;
}

/// Detect the devices of the current running system and returns them into a Vec<String>
fn detect_devices() -> Vec<String>{
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
                if is_block_device(&disk_name){    
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
