use crate::boot::disk;
use crate::config::{BLOCK_DEV_ROUTE, LOGICAL_BLOCK, EFI_PART_SIGN, MAX_BUFFER_SIZE, MAX_HEADER_SIZE};
use crate::errors::{IgnixError, io};
use crate::errors::cmd;
use crate::boot::crc32::calculate_crc32;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn compatible_esp_partition(devices: Vec<String>) -> Result<String, IgnixError>{

    for device in devices {
        
        let sector_size = get_disk_sector_size(&device, BLOCK_DEV_ROUTE, LOGICAL_BLOCK)?;
        let disk = File::open(format!("/dev/{}",device))?; 
        let buffer = get_gpt_structure(sector_size, &disk)?;
        
        if !is_efi_signed(buffer)?{
            continue;
        }
        
        let gpt_header_size: u32 = get_gpt_header_size(buffer)?;

        if !validate_crc32_header_checksum(buffer, gpt_header_size)?{
            continue;
        }

        // The standard says here the max should be 128, however it is dynamic here just in case.
        let gpt_max_partitions: u32 = get_max_gpt_partition(buffer)?;
        let gpt_max_partition_size: u32 = get_partition_max_size(buffer)?;
        let part_array_start: u64 = get_partition_array_start(buffer)?;

    }
    
    Err(cmd::Error::NotEFIPartitionFound)?
}


fn is_efi_signed(buffer: [u8;MAX_BUFFER_SIZE]) -> Result<bool, IgnixError>{
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
    let mut header_copy = [0u8; MAX_HEADER_SIZE];
    header_copy[..size].copy_from_slice(&buffer[..size]);
    header_copy[16..20].fill(0);
    
    let compute_crc = calculate_crc32(&header_copy[..size]);
    
    if compute_crc == stored_crc {
        return Ok(true)
    }

    Ok(false)
}

fn validate_crc32_partition_array_checksum(buffer: [u8;MAX_BUFFER_SIZE], header_size: u32) 
-> Result<bool, IgnixError> {
    let part_array_crc = u32::from_le_bytes(buffer[88..92].try_into()?);
    calculate_crc32(&buffer[header_size as usize..MAX_BUFFER_SIZE]);
    Ok(true)
}


fn get_gpt_header_size(buffer: [u8;MAX_BUFFER_SIZE]) -> Result<u32, IgnixError>{
    let header_size = u32::from_le_bytes(buffer[12..16].try_into()?);
 
    if header_size as usize >= MAX_HEADER_SIZE {
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

fn get_disk_sector_size(disk: &str, block_route: &str, lba_size_route: &str)
    -> Result<u64, IgnixError>{
    // Example: /sys/class/block/nvme0n1/queue/logical_block_size
    let sector_size_path = &format!(r"{}{}{}",block_route,disk,lba_size_route);
    // Reads the value into a string and parses it into an unsigned int value.
    let value: u64 = std::fs::read_to_string(sector_size_path)?.trim().parse()?;
    
    Ok(value)
}
