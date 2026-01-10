// This file is for parsing the conf file 
use uefi::println;
// use uefi::prelude::*;
// use uefi::CString16;
// use uefi::fs::FileSystem;
const SPARK_FILE_ROUTE:&str = r"\spark.conf"; // Using the raw parameter for the string and declaring the route for the conf file

pub fn spark_config(){
    /*
    * This is a placeholder for the spark_config function, should iterate the config file for the
    * bootmanager and deliver it to the main function
    */
    println!("Parsing the default config file at {}...", SPARK_FILE_ROUTE); // Placeholder
}

pub fn boot_entries(){
    /*
    * This function is meant to parse the boot entries inside the boot installation, at this moment
    * it is still a placeholder, but should also deliver it as a return
    */
    println!("Parsing the config files for boot entries and saving the routes..."); // Placeholder
}
