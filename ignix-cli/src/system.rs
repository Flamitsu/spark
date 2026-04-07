use std::io::Read;
use crate::errors::IgnixError;
// This is a new way that I will implement to integrate unitary tests to functions that implies
// reading a file. 
fn generate_seed<R: Read>(mut source: R) -> Result<[u8;32], IgnixError>{
    let mut buffer = [0u8; 32];
    source.read_exact(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_generate_seed(){
        let fake_entrophy: &[u8] = &[0xFF; 32];
        let seed = generate_seed(fake_entrophy).expect("Function returned an error instead of the buffer.");
        assert_eq!(fake_entrophy, seed)
    }
    #[test]
    fn test_generate_seed_invalid(){
        let fake_entrophy: &[u8] = &[0xFF; 10];
        let seed = generate_seed(fake_entrophy);
        assert!(seed.is_err(), "The test should have failed. what the fuck")
    }
}
