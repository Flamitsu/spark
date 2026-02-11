use std::fs;
use std::process::exit;
pub fn check_spark_nvram_variable() -> bool{
    let route: &str = "/sys/firmware/efi/efivars";
    let _nvram_dir = if let Ok(directories) = fs::read_dir(&route){        
        directories
    } else{
        eprintln!("Error. Can not open {} . Check if you have enough privileges or if it exists.",route);
        exit(4);
    };
    true;
    todo!("WIP function. Read the nvram variables.")
}
