use crate::errors::IgnixError;
use crate::errors::cmd;

pub fn compatible_esp_partition(disks: Vec<String>) -> Result<String, IgnixError>{
    for disk in disks{
    }
    Err(cmd::Error::NotEFIPartitionFound)?
}

pub fn is_gpt_disk() -> Result<bool, IgnixError>{
    Ok(true)
}
