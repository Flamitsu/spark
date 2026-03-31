use crate::config::{BLOCK_DEV_ROUTE, LOGICAL_BLOCK};
use crate::errors::{IgnixError, io};
use crate::errors::cmd;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
// EFI SIGNATURE: in raw bytes: 'EFI PART' is 0x45, 0x46, 0x49, 0x20, 0x50, 0x41, 0x52, 0x54 .
const EFI_PART_SIGN: [u8;8] = *b"EFI PART";
const MAX_HEADER_SIZE: usize = 512;

pub fn compatible_esp_partition(devices: Vec<String>) -> Result<String, IgnixError>{

    for device in devices {
        
        let sector_size = get_disk_sector_size(&device, BLOCK_DEV_ROUTE, LOGICAL_BLOCK)?;
        let disk = File::open(format!("/dev/{}",device))?; 
        let buffer = get_gpt_disk_header(sector_size, disk)?;
        
        if !is_efi_signed(buffer)?{
            continue;
        }
        
        let gpt_header_size: usize = get_gpt_header_size(buffer)?;

        if !validate_crc32_checksum(gpt_header_size, buffer)?{
            eprintln!("An error occured while trying to calculate the CRC32 for {} disk. Maybe the disk is corrupt or it is not a valid GPT device.", device);
            continue;
        }

    }
    
    Err(cmd::Error::NotEFIPartitionFound)?
}

fn get_disk_sector_size(disk: &str, block_route: &str, lba_size_route: &str)
    -> Result<u64, IgnixError>{
    // Example: /sys/class/block/nvme0n1/queue/logical_block_size
    let sector_size_path = &format!(r"{}{}{}",block_route,disk,lba_size_route);
    // Reads the value into a string and parses it into an unsigned int value.
    let value: u64 = std::fs::read_to_string(sector_size_path)?.trim().parse()?;
    
    Ok(value)
}

fn get_gpt_disk_header(lba_size: u64, mut disk: File) -> Result<[u8;MAX_HEADER_SIZE], IgnixError>{
    let mut buffer = [0u8; MAX_HEADER_SIZE];
    
    disk.seek(SeekFrom::Start(lba_size))?;
    disk.read_exact(&mut buffer)?;

    Ok(buffer)
}

fn is_efi_signed(buffer: [u8;MAX_HEADER_SIZE]) -> Result<bool, IgnixError>{
    // Checks if the EFI PART signature located in bytes 1 to 8 in the LBA1 matches 
    if buffer[0..8] != EFI_PART_SIGN{
        return Ok(false);
    }

    Ok(true)
}

fn get_gpt_header_size(buffer: [u8;MAX_HEADER_SIZE]) -> Result<usize, IgnixError>{
    let bytes = buffer[12..16].try_into()?;
    let header_size = usize::from_le_bytes(bytes);
    
    if header_size >= MAX_HEADER_SIZE{
        return Err(io::Error::InvalidBuffer("Header size exceeds the maximum allowed. (512)".to_string()))?
    }

    Ok(header_size)
}

/// Validates the GPT header CRC32 checksum to detect disk corruption. It uses the standard IEEE 802.3 polynomial (0xEDB88320) via bitwise operations.
fn validate_crc32_checksum(header_size: usize, buffer:[u8;MAX_HEADER_SIZE])
    -> Result<bool, IgnixError> {
    // Extract the stored CRC32 from the header (16 to 19 bytes.)
    let original_data: [u8;4] = buffer[16..20].try_into()?;
    let original_crc = u32::from_le_bytes(original_data);
    
    let mut crc: u32 = 0xFFFFFFFF;
    
    for index in 0..header_size {
        /* According to the GPT specification, the CRC field needs to be 0 during the calculation
        process.*/
        let byte = 
            if index >= 16 && index < 20{
                0u8
            } else {
                buffer[index]
            };
        // XOR the current byte 
        crc ^= byte as u32;
        // Process each bit (LSB first) for the current byte.
        for _ in 0..8{
            // If the LSB is 1, shift and XOR to the polynomial number. 
            if crc & 1 != 0{
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                // If LSB is 0, only shift right 
                crc >>= 1;
            }
        }

    }
    // The comparation must be with the inverted (bitwise not) to match correctly with the original crc one.
    if !crc == original_crc{
        return Ok(true)
    }

    Ok(false)
}
