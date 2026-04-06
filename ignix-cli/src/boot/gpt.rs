use crate::config::{
    // Signatures and safety measures
    EFI_PART_SIGN, ESP_GUID_BYTES, MAX_BUFFER_SIZE, MAX_GPT_HEADER_SIZE,
    // Header Offsets
    GPT_HDR_SIG_START,
    GPT_HDR_SIG_END, GPT_HDR_SIZE_START, GPT_HDR_SIZE_END, GPT_HDR_CRC_START, GPT_HDR_CRC_END,
    GPT_HDR_PART_LBA_START, GPT_HDR_PART_LBA_END, GPT_HDR_PART_COUNT_START, GPT_HDR_PART_COUNT_END,
    GPT_HDR_PART_SIZE_START, GPT_HDR_PART_SIZE_END, GPT_HDR_PART_CRC_START, GPT_HDR_PART_CRC_END,
    // Entry Offsets
    PART_TYPE_GUID_START, PART_TYPE_GUID_END, PART_UNIQUE_GUID_START, PART_UNIQUE_GUID_END,
};
use crate::errors::{IgnixError, io};
use crate::boot::crc32::calculate_crc32;
use std::io::{SeekFrom, Seek, Read};
use std::fs::File;

pub fn is_disk_efi_signed(buffer: &[u8;MAX_BUFFER_SIZE]) -> bool{
    if buffer[GPT_HDR_SIG_START..GPT_HDR_SIG_END] != EFI_PART_SIGN{
        return false;
    }
    true
}

pub fn validate_crc32_header_checksum(buffer: &[u8;MAX_BUFFER_SIZE], header_size: u32) 
    -> Result<bool, IgnixError>{
    let size = header_size as usize;

    let stored_crc = u32::from_le_bytes(buffer[GPT_HDR_CRC_START..GPT_HDR_CRC_END].try_into()?);
    /* Copies only the header to change the header checksum field to 0, that is how the CRC32
     * checksums says is correct.*/
    let mut header_copy = [0u8; MAX_GPT_HEADER_SIZE];
    header_copy[..size].copy_from_slice(&buffer[..size]);
    header_copy[GPT_HDR_CRC_START..GPT_HDR_CRC_END].fill(0);
    
    let compute_crc = calculate_crc32(&header_copy[..size]);
    
    if compute_crc == stored_crc {
        return Ok(true)
    }

    Ok(false)
}

pub fn validate_crc32_partition_array_checksum(buffer: &[u8;MAX_BUFFER_SIZE], gpt_max_partitions: u32, gpt_entry_size: u32, part_array_start: u64, sector_size: u64) -> Result<bool, IgnixError>{ 
    let array_size = (gpt_max_partitions * gpt_entry_size) as usize;
    let offset = ((part_array_start - 1) * sector_size) as usize;
    
    if offset + sector_size as usize > MAX_BUFFER_SIZE{
        Err(io::Error::InvalidBufferOverflow(MAX_BUFFER_SIZE.to_string()))?
    }

    let part_array_crc = u32::from_le_bytes(buffer[GPT_HDR_PART_CRC_START..GPT_HDR_PART_CRC_END].try_into()?);
    
    let crc32 = calculate_crc32(&buffer[offset..(offset + array_size)]);
    
    if part_array_crc == crc32 {
        return Ok(true);
    }
    Ok(false)
}

pub fn get_esp_guid(buffer: &[u8;MAX_BUFFER_SIZE], gpt_max_partitions: u32, gpt_entry_size: u32, sector_size: u64, part_array_start: u64) -> Result<Option<[u8;16]>, IgnixError>{
    // This -1 here is because the iteration already skips the first LBA0 (MBR PROTECTIVE) sector.
    let offset = sector_size * (part_array_start - 1);
    for partition in 0..gpt_max_partitions{
        let entry_start = offset as usize + (partition as usize * gpt_entry_size as usize);
        let entry_end = entry_start + gpt_entry_size as usize;

        if entry_start > MAX_BUFFER_SIZE || entry_end > MAX_BUFFER_SIZE {
            Err(io::Error::InvalidBufferOverflow(MAX_BUFFER_SIZE.to_string()))?
        }
        
        let gpt_array_header = &buffer[entry_start..entry_end];
        let type_guid = &gpt_array_header[PART_TYPE_GUID_START..PART_TYPE_GUID_END];
        
        if type_guid == ESP_GUID_BYTES{
            let unique_guid: [u8;16] = gpt_array_header[PART_UNIQUE_GUID_START..PART_UNIQUE_GUID_END].try_into()?;
            return Ok(Some(unique_guid))
        }

    }
    Ok(None)
}


pub fn get_gpt_header_size(buffer: &[u8;MAX_BUFFER_SIZE]) -> Result<u32, IgnixError>{
    let header_size = u32::from_le_bytes(buffer[GPT_HDR_SIZE_START..GPT_HDR_SIZE_END].try_into()?);
    if header_size as usize > MAX_GPT_HEADER_SIZE {
        Err(io::Error::InvalidBufferOverflow(MAX_GPT_HEADER_SIZE.to_string()))?
    }
    Ok(header_size)
}

// The offsets in this slices are defined in the GPT specification.
pub fn get_max_gpt_partition(buffer: &[u8;MAX_BUFFER_SIZE]) -> Result<u32, IgnixError>{
    Ok(u32::from_le_bytes(buffer[GPT_HDR_PART_COUNT_START..GPT_HDR_PART_COUNT_END].try_into()?))
}

pub fn get_partition_array_start(buffer: &[u8;MAX_BUFFER_SIZE]) -> Result<u64, IgnixError>{
    Ok(u64::from_le_bytes(buffer[GPT_HDR_PART_LBA_START..GPT_HDR_PART_LBA_END].try_into()?))
}

pub fn get_partition_max_size(buffer: &[u8;MAX_BUFFER_SIZE]) -> Result<u32, IgnixError>{
    Ok(u32::from_le_bytes(buffer[GPT_HDR_PART_SIZE_START..GPT_HDR_PART_SIZE_END].try_into()?))
}

pub fn format_partuuid(guid: &[u8;16]) -> Result<String, IgnixError>{
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

pub fn get_gpt_structure(lba_size: u64, mut disk: &File) -> Result<[u8;MAX_BUFFER_SIZE], IgnixError>{
    let mut buffer = [0u8;MAX_BUFFER_SIZE];
    
    disk.seek(SeekFrom::Start(lba_size))?;
    disk.read_exact(&mut buffer)?;

    Ok(buffer)
}

#[cfg(test)]
enum FillMode{
    Sequential,
    Random,
}

#[cfg(test)]
fn fill_test_data(buffer: &mut [u8;MAX_BUFFER_SIZE], start: usize, end: usize, mode: FillMode){
    match mode{
        FillMode::Sequential => {
            for index in start..end{
                buffer[index] = (index - start) as u8;
            }
        },
        FillMode::Random => {
            for index in start..end{
                // multiply a prime number and applies an XOR operation.
                buffer[index] = ((index * 13) ^ 0xAA) as u8;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::{MAX_GPT_PARTITION_ENTRY_SIZE, MAX_GPT_PARTITIONS, MAX_LBA_SECTOR_SIZE};
    #[test]
    fn test_is_disk_efi_signed(){
        let mut dummy_buffer: [u8;MAX_BUFFER_SIZE] = [0u8;MAX_BUFFER_SIZE];
        dummy_buffer[0..8].copy_from_slice(&EFI_PART_SIGN);
        assert_eq!(is_disk_efi_signed(&dummy_buffer), true);
    }
    #[test]
    fn test_disk_is_not_efi_signed() {
        let mut dummy_buffer: [u8;MAX_BUFFER_SIZE] = [0u8;MAX_BUFFER_SIZE];
        fill_test_data(&mut dummy_buffer, 0, 8, FillMode::Random);
        assert_eq!(is_disk_efi_signed(&dummy_buffer), false);
    }

    #[test]
    fn test_validate_crc32_header_checksum(){
        let mut dummy_buffer: [u8;MAX_BUFFER_SIZE] = [0u8;MAX_BUFFER_SIZE];
        fill_test_data(&mut dummy_buffer, 0, MAX_GPT_HEADER_SIZE, FillMode::Sequential);
        let header_size = MAX_GPT_HEADER_SIZE as usize;
        dummy_buffer[16..20].fill(0);
        let compute_crc = calculate_crc32(&dummy_buffer[..header_size]);
        dummy_buffer[16..20].copy_from_slice(&compute_crc.to_le_bytes());
        assert_eq!(validate_crc32_header_checksum(&dummy_buffer, MAX_GPT_HEADER_SIZE as u32).expect("Function returned error instead of true."), true, "Function returned false instead of true.");
    }
    #[test]
    fn test_validate_crc32_header_checksum_invalid() {
        let mut dummy_buffer: [u8;MAX_BUFFER_SIZE] = [0u8;MAX_BUFFER_SIZE];
        fill_test_data(&mut dummy_buffer, 0, MAX_BUFFER_SIZE, FillMode::Random);
        assert_eq!(validate_crc32_header_checksum(&dummy_buffer, MAX_GPT_HEADER_SIZE as u32).expect("Function returned error instead of false."), false, "Function returned true instead of false.")
    }

    #[test]
    fn test_validate_crc32_partition_array_checksum_valid() {
        let mut dummy_buffer: [u8; MAX_BUFFER_SIZE] = [0u8; MAX_BUFFER_SIZE];
        fill_test_data(&mut dummy_buffer, 0, MAX_BUFFER_SIZE, FillMode::Sequential);
        let expected_crc = calculate_crc32(&dummy_buffer[MAX_LBA_SECTOR_SIZE..MAX_BUFFER_SIZE]);
        dummy_buffer[88..92].copy_from_slice(&expected_crc.to_le_bytes());
        assert_eq!(validate_crc32_partition_array_checksum(&dummy_buffer, MAX_GPT_PARTITIONS as u32, 
                MAX_GPT_PARTITION_ENTRY_SIZE as u32, 
                2, 
                MAX_LBA_SECTOR_SIZE as u64)
            .expect("Function returned error instead of true."), true, 
            "Function returned false instead of true."
            );
    }
    #[test]
    fn test_validate_crc32_partition_array_checksum_invalid(){
        let mut dummy_buffer: [u8;MAX_BUFFER_SIZE] = [0u8; MAX_BUFFER_SIZE];
        fill_test_data(&mut dummy_buffer, MAX_LBA_SECTOR_SIZE, MAX_BUFFER_SIZE, FillMode::Random);
        assert_eq!(validate_crc32_partition_array_checksum(&dummy_buffer, MAX_GPT_PARTITIONS as u32,
                MAX_GPT_PARTITION_ENTRY_SIZE as u32, 
                2, 
                MAX_LBA_SECTOR_SIZE as u64)
            .expect("Function returned error instead of false."), false, 
            "Function returned true instead of false."
            )
    }
}
