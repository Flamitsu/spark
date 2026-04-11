use crate::config::{
    EFI_PART_SIGN, ESP_GUID_BYTES, GPT_HDR_CRC, GPT_HDR_MAX_SIZE, GPT_HDR_PART_COUNT, GPT_HDR_PART_CRC, GPT_HDR_PART_LBA, GPT_HDR_PART_SIZE, GPT_HDR_SIG, GPT_HDR_SIZE, PART_TYPE_GUID, PART_UNIQUE_GUID
};
use crate::errors::{IgnixError, io};
use crate::boot::crc32::calculate_crc32;
use std::io::{SeekFrom, Seek, Read};

pub fn is_disk_efi_signed(buffer: &[u8]) -> bool{
    buffer[GPT_HDR_SIG] == EFI_PART_SIGN
}

pub fn validate_crc32_header_checksum(buffer: &[u8],header_size: u32) -> Result<bool, IgnixError>{
    
    let size = header_size as usize;
    // Clone function is needed because the Range<usize> parameter is used 2 times.
    let stored_crc = u32::from_le_bytes(buffer[GPT_HDR_CRC.clone()].try_into()?);
    
    /* Copies only the header to change the header checksum field to 0, that is how the CRC32
     * checksums says is correct.*/
    let mut header_copy = [0u8; GPT_HDR_MAX_SIZE];
    header_copy[..size].copy_from_slice(&buffer[..size]);
    header_copy[GPT_HDR_CRC].fill(0);
    
    let compute_crc = calculate_crc32(&header_copy[..size]);
    
    if compute_crc == stored_crc {
        return Ok(true)
    }

    Ok(false)
}

pub fn validate_crc32_partition_array_checksum<const SIZE: usize>(
    buffer: &[u8;SIZE],
    gpt_max_partitions: u32,
    gpt_entry_size: u32,
    part_array_start: u64,
    sector_size: u64) -> Result<bool, IgnixError>{ 
    
    let array_size = (gpt_max_partitions * gpt_entry_size) as usize;
    let offset = ((part_array_start - 1) * sector_size) as usize;
    
    if offset + sector_size as usize > SIZE{
        Err(io::Error::InvalidBufferOverflow(SIZE.to_string()))?
    }

    let part_array_crc = u32::from_le_bytes(buffer[GPT_HDR_PART_CRC].try_into()?);
    
    let crc32 = calculate_crc32(&buffer[offset..(offset + array_size)]);
    
    if part_array_crc == crc32 {
        return Ok(true);
    }
    Ok(false)
}

pub fn get_esp_guid<const MAX_BUFFER_SIZE: usize>(
    buffer: &[u8;MAX_BUFFER_SIZE], 
    gpt_max_partitions: u32,
    gpt_entry_size: u32,
    sector_size: u64,
    part_array_start: u64,
    ) -> Result<Option<[u8;16]>, IgnixError>{
    
    // This -1 here is because the iteration already skips the first LBA0 (MBR PROTECTIVE) sector.
    let offset = sector_size * (part_array_start - 1);
    
    for partition in 0..gpt_max_partitions{
        let entry_start = offset as usize + (partition as usize * gpt_entry_size as usize);
        let entry_end = entry_start + gpt_entry_size as usize;

        if entry_start > MAX_BUFFER_SIZE || entry_end > MAX_BUFFER_SIZE {
            Err(io::Error::InvalidBufferOverflow(MAX_BUFFER_SIZE.to_string()))?
        }
        
        let gpt_array_header = &buffer[entry_start..entry_end];
        // The part_type_guid needs to be cloned because is inside a for loop.
        let type_guid = &gpt_array_header[PART_TYPE_GUID.clone()];
        
        if type_guid == ESP_GUID_BYTES{
            let unique_guid: [u8;16] = gpt_array_header[PART_UNIQUE_GUID].try_into()?;
            return Ok(Some(unique_guid))
        }

    }
    Ok(None)
}


pub fn get_gpt_header_size(buffer: &[u8]) -> Result<u32, IgnixError>{
    let header_size = u32::from_le_bytes(buffer[GPT_HDR_SIZE].try_into()?);
    if header_size as usize > GPT_HDR_MAX_SIZE {
        Err(io::Error::InvalidBufferOverflow(GPT_HDR_MAX_SIZE.to_string()))?
    }
    Ok(header_size)
}

// The offsets in this slices are defined in the GPT specification.
pub fn get_max_gpt_partition(buffer: &[u8]) -> Result<u32, IgnixError>{
    Ok(u32::from_le_bytes(buffer[GPT_HDR_PART_COUNT].try_into()?))
}

pub fn get_partition_array_start(buffer: &[u8]) -> Result<u64, IgnixError>{
    Ok(u64::from_le_bytes(buffer[GPT_HDR_PART_LBA].try_into()?))
}

pub fn get_partition_max_size(buffer: &[u8]) -> Result<u32, IgnixError>{
    Ok(u32::from_le_bytes(buffer[GPT_HDR_PART_SIZE].try_into()?))
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

pub fn get_gpt_structure<DISK: Read + Seek>(
    lba_size: u64, 
    mut disk: DISK, 
    buffer: &mut [u8]) -> Result<(), IgnixError>{
    
    disk.seek(SeekFrom::Start(lba_size))?;
    disk.read_exact(buffer)?;
    Ok(())
}

#[cfg(test)]
enum FillMode{
    Sequential,
    Random,
}
#[cfg(test)]
fn fill_test_data(buffer: &mut [u8], start: usize, end: usize, mode: FillMode){
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
    use crate::config::{MAX_GPT_PARTITION_ENTRY_SIZE, MAX_GPT_PARTITIONS, MAX_LBA_SECTOR_SIZE, MAX_BUFFER_SIZE, GPT_HDR_MAX_SIZE};
    
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
        fill_test_data(&mut dummy_buffer, 0, GPT_HDR_MAX_SIZE, FillMode::Sequential);
        let header_size = GPT_HDR_MAX_SIZE as usize;
        dummy_buffer[16..20].fill(0);
        let compute_crc = calculate_crc32(&dummy_buffer[..header_size]);
        dummy_buffer[16..20].copy_from_slice(&compute_crc.to_le_bytes());
        
        assert_eq!(
            validate_crc32_header_checksum(&dummy_buffer, GPT_HDR_MAX_SIZE as u32)
            .expect("Function returned error instead of true."), 
            true, 
            "Function returned false instead of true.");
    }

    #[test]
    fn test_validate_crc32_header_checksum_invalid() {
        let mut dummy_buffer: [u8;MAX_BUFFER_SIZE] = [0u8;MAX_BUFFER_SIZE];
        fill_test_data(&mut dummy_buffer, 0, MAX_BUFFER_SIZE, FillMode::Random);
        assert_eq!(
            validate_crc32_header_checksum(
                &dummy_buffer, GPT_HDR_MAX_SIZE as u32)
            .expect("Function returned error instead of false."), 
            false, 
            "Function returned true instead of false.")
    }

    #[test]
    fn test_validate_crc32_partition_array_checksum_valid() {
        let mut dummy_buffer: [u8; MAX_BUFFER_SIZE] = [0u8; MAX_BUFFER_SIZE];
        fill_test_data(&mut dummy_buffer, 0, MAX_BUFFER_SIZE, FillMode::Sequential);
        let expected_crc = calculate_crc32(&dummy_buffer[MAX_LBA_SECTOR_SIZE..MAX_BUFFER_SIZE]);
        dummy_buffer[88..92].copy_from_slice(&expected_crc.to_le_bytes());
        assert_eq!(
            validate_crc32_partition_array_checksum(
                &dummy_buffer, 
                MAX_GPT_PARTITIONS as u32, 
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
        assert_eq!(
            validate_crc32_partition_array_checksum(
                &dummy_buffer, 
                MAX_GPT_PARTITIONS as u32,
                MAX_GPT_PARTITION_ENTRY_SIZE as u32, 
                2, 
                MAX_LBA_SECTOR_SIZE as u64)
            .expect("Function returned error instead of false."), false, 
            "Function returned true instead of false."
            )
    }
}
