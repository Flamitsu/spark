// This file is for parsing the conf file 
use uefi::println;
// use uefi::prelude::*;
// use uefi::CString16;
// use uefi::fs::FileSystem;
/// Using the route of the spark configuration file. Needed to parse it and configure the
/// bootmanager globally
const SPARK_FILE_ROUTE:&str = r"\spark.conf"; // Using the raw parameter for the string and declaring the route for the conf file


/// This is a placeholder for the spark_config function, it should iterate the config file for the
/// bootmanager and deliver it to the main function
pub fn load_config(){
    println!("Parsing the default config file at {}...", SPARK_FILE_ROUTE); // Placeholder
}
/// This function is meant to parse the boot entries inside the boot installation, at this moment
/// it is still a placeholder, but should also deliver it as a return.
pub fn boot_entries(){
    println!("Parsing the config files for boot entries and saving the routes..."); // Placeholder
}
