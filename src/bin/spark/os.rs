use std::fs;
use crate::utils::exists;
pub enum GetDistroInfo{
    Id,
    _Name
}

/// Gets the attribute needed of the distribution that is running the program
pub fn get_distro_information(attribute: GetDistroInfo) -> String {
    // Opens the file 
    let os_config_file = if let Ok(file) = fs::read_to_string(get_route_os_release_file()){
        file
    } else{
        eprintln!("Error, unable to determine the distribution on which the programe is running. Proceeding with 'unknown'.");
        return "unknown".to_string();
    };
    // Match the keys to know which attribute is needed  
    let match_key = match attribute{
        GetDistroInfo::_Name => {"NAME="},
        GetDistroInfo::Id => {"ID="}
    };
    for line in os_config_file.lines(){
        if line.starts_with("#"){
            continue;
        }
        // Returns the slice of the string 
        if line.starts_with(match_key){
            return line[match_key.len()..].to_string();
        }
    }
    // If nothing matches, the script returns unknown.
    eprintln!("Error, unable to determine the distribution on which the program is running. Proceeding with 'unknown'.");
    return "unknown".to_string();
}

/// This function returns the possible route of the os-release file. 
fn get_route_os_release_file() -> String {
    // Declares the route of the os-release file
    let mut path = "/etc/os-release";
    // Checks if the file exists or not 
    if exists(Some(&path)) {
        return String::from(path);
    }
    // If it does not find anything, tries to send this route.
    path = "/usr/lib/os-release";
    return String::from(path);
}
