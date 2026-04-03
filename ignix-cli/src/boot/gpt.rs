use crate::boot::disk;
use crate::config::{BLOCK_DEV_ROUTE, LOGICAL_BLOCK, EFI_PART_SIGN, MAX_BUFFER_SIZE, MAX_GPT_HEADER_SIZE, ESP_GUID_BYTES};
use crate::errors::{IgnixError, io};
use crate::errors::cmd;
use crate::boot::crc32::calculate_crc32;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn compatible_esp_partition(devices: Vec<String>) -> Result<String, IgnixError>{
    for device in devices {
        let disk_sysfs_route = &format!("{}{}",BLOCK_DEV_ROUTE, &device);
        let sector_size = get_disk_sector_size(disk_sysfs_route, LOGICAL_BLOCK)?;
        let disk = File::open(format!("/dev/{}",device))?;
        let buffer = get_gpt_structure(sector_size, &disk)?;
        
        if !is_disk_efi_signed(buffer)?{
            eprintln!("{device} isn't EFI signed. Skipping...");
            continue;
        }
        
        let gpt_header_size: u32 = get_gpt_header_size(buffer)?;

        if !validate_crc32_header_checksum(buffer, gpt_header_size)?{
            eprintln!("{device} is probably corrupt (GPT header). Skipping...");
            continue;
        }

        // The standard says here the max should be 128, however it is dynamic here just in case.
        let gpt_max_partitions: u32 = get_max_gpt_partition(buffer)?;
        let gpt_entry_size: u32 = get_partition_max_size(buffer)?;
        let part_array_start: u64 = get_partition_array_start(buffer)?;
        
        if !validate_crc32_partition_array_checksum(buffer, gpt_max_partitions, gpt_entry_size, part_array_start, sector_size)?{
            eprintln!("{device} is probably corrupt (partition array). Skipping...");
            continue;
        }

        let Some(part_guid) = get_esp_guid(&buffer, 
            gpt_max_partitions, 
            gpt_entry_size, 
            sector_size, 
            part_array_start)? 
        else { 
            eprintln!("Not GUID valid found in {device}"); 
            continue; 
        };
        
        let guid_string = format_partuuid(&part_guid)?;
        
        let Some(partition_name) = disk::get_esp_partition(&device, 
            disk_sysfs_route, 
            &guid_string)? 
        else { 
            eprintln!("Didn't worked the uevent part. {device}"); 
            continue; 
        };

        return Ok(partition_name);
    }

    Err(cmd::Error::NotEFIPartitionFound)?
}


fn is_disk_efi_signed(buffer: [u8;MAX_BUFFER_SIZE]) -> Result<bool, IgnixError>{
    if buffer[0..8] != EFI_PART_SIGN{
        return Ok(false);
    }
    Ok(true)
}

fn validate_crc32_header_checksum(buffer:[u8;MAX_BUFFER_SIZE], header_size: u32) 
    -> Result<bool, IgnixError>{
    let size = header_size as usize;

    let stored_crc = u32::from_le_bytes(buffer[16..20].try_into()?);
    /* Copies only the header to change the header checksum field to 0, that is how the CRC32
     * checksums says is correct.*/
    let mut header_copy = [0u8; MAX_GPT_HEADER_SIZE];
    header_copy[..size].copy_from_slice(&buffer[..size]);
    header_copy[16..20].fill(0);
    
    let compute_crc = calculate_crc32(&header_copy[..size]);
    
    if compute_crc == stored_crc {
        return Ok(true)
    }

    Ok(false)
}

fn validate_crc32_partition_array_checksum(buffer: [u8;MAX_BUFFER_SIZE], gpt_max_partitions: u32, gpt_entry_size: u32, part_array_start: u64, sector_size: u64) -> Result<bool, IgnixError>{ 
    // Checks if it overflows and it is freed instantly after.
    let array_size = (gpt_max_partitions * gpt_entry_size) as usize;
    let offset = ((part_array_start - 1) * sector_size) as usize;
    
    if offset + sector_size as usize > MAX_BUFFER_SIZE{
        Err(io::Error::InvalidBuffer("Partition array buffer overflows.".to_string()))?
    }

    let part_array_crc = u32::from_le_bytes(buffer[88..92].try_into()?);
    
    let crc32 = calculate_crc32(&buffer[offset..(offset + array_size)]);
    
    if part_array_crc == crc32 {
        return Ok(true);
    }
    Ok(false)
}

fn get_esp_guid(buffer: &[u8;MAX_BUFFER_SIZE], gpt_max_partitions: u32, gpt_entry_size: u32, sector_size: u64, part_array_start: u64) -> Result<Option<[u8;16]>, IgnixError>{
    // This -1 here is because the iteration already skips the first LBA0 (MBR PROTECTIVE) sector.
    let offset = sector_size * (part_array_start - 1);
    for partition in 0..gpt_max_partitions{
        let entry_start = offset as usize + (partition as usize * gpt_entry_size as usize);
        let entry_end = entry_start + gpt_entry_size as usize;
        
        if entry_start > MAX_BUFFER_SIZE{
            Err(io::Error::InvalidBuffer("Invalid buffer while parsing the partition entries. (too long.)".to_string()))?
        }
        
        let gpt_array_header = &buffer[entry_start..entry_end];
        let type_guid = &gpt_array_header[0..16];
        
        if type_guid == ESP_GUID_BYTES{
            let unique_guid: [u8;16] = gpt_array_header[16..32].try_into()?;
            return Ok(Some(unique_guid))
        }

    }
    Ok(None)
}

fn format_partuuid(guid: &[u8;16]) -> Result<String, IgnixError>{
    let data1 = u32::from_le_bytes(guid[0..4].try_into()?);
    let data2 = u16::from_le_bytes(guid[4..6].try_into()?);
    let data3 = u16::from_le_bytes(guid[6..8].try_into()?);
    Ok(
        // If the field isn't big enough the format says to add padding. x is for small letters.
        format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        data1, data2, data3,
        guid[8], guid[9],
        guid[10], guid[11], guid[12], guid[13], guid[14], guid[15]
        )
    )
}

fn get_gpt_header_size(buffer: [u8;MAX_BUFFER_SIZE]) -> Result<u32, IgnixError>{
    let header_size = u32::from_le_bytes(buffer[12..16].try_into()?);
    if header_size as usize > MAX_GPT_HEADER_SIZE {
        return Err(io::Error::InvalidBuffer("Header size overflows.".to_string()))?
    }
    Ok(header_size)
}

fn get_max_gpt_partition(buffer: [u8;MAX_BUFFER_SIZE]) -> Result<u32, IgnixError>{
    Ok(u32::from_le_bytes(buffer[80..84].try_into()?))
}

fn get_partition_array_start(buffer: [u8;MAX_BUFFER_SIZE]) -> Result<u64, IgnixError>{
    Ok(u64::from_le_bytes(buffer[72..80].try_into()?))
}

fn get_partition_max_size(buffer: [u8;MAX_BUFFER_SIZE]) -> Result<u32, IgnixError>{
    Ok(u32::from_le_bytes(buffer[84..88].try_into()?))
}

fn get_gpt_structure(lba_size: u64, mut disk: &File) -> Result<[u8;MAX_BUFFER_SIZE], IgnixError>{
    let mut buffer = [0u8;MAX_BUFFER_SIZE];
    
    disk.seek(SeekFrom::Start(lba_size))?;
    disk.read_exact(&mut buffer)?;

    Ok(buffer)
}

/*
 * If you're wondering why this is a string and not a 'Path' as it should be,
 * it's because Rust is being bitchy about that it doesn't know a 'Path' size in
 * compilation time.
 * */
fn get_disk_sector_size(disk: &str, lba_size_route: &str)
    -> Result<u64, IgnixError>{
    // Example: /sys/class/block/nvme0n1/queue/logical_block_size
    let sector_size_path = format!(r"{}{}",disk,lba_size_route);
    // Reads the value into a string and parses it into an unsigned int value.
    let value: u64 = std::fs::read_to_string(sector_size_path)?.trim().parse()?;
    
    Ok(value)
}
