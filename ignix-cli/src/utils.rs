use std::io::Read;
use crate::errors::IgnixError;
// entropy_source needs to be mutable, because if you read something you are "modifying" it.
// In theory it is just modifying the cursor position.
pub fn get_random<SOURCE: Read>(mut entropy_source: SOURCE, buffer: &mut [u8]) 
-> Result<(), IgnixError>{
    entropy_source.read_exact(buffer)?;
    Ok(())
}
