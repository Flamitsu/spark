// This file is for parsing the conf file 
use uefi::println;
// use uefi::prelude::*;
// use uefi::CString16;
// use uefi::fs::FileSystem;
const SPARK_FILE_ROUTE:&str = r"\spark.conf"; // Using the raw parameter for the string and declaring the route for the conf file

pub fn spark_config(){
    println!("Parsing the default config file at {}...", SPARK_FILE_ROUTE);
}

pub fn boot_entries(){
    println!("Parsing the config files for boot entries and saving the routes...")
}
