#![no_main]
#![no_std] // Not using the std library (Since not OS is loaded.)

use core::time::Duration; // using the duration method from core crate
use log::info; // Using the info function from the log crate 
use uefi::prelude::*; // Using the prelude functions of the uefi crate

#[entry] // Entry point of the binary
fn main() -> Status { // Main function MUST return a status code at the end.
    uefi::helpers::init().unwrap(); // Initializing the UEFI services
    info!("Hello world!"); // Printing in the console
    boot::stall(Duration::from_hours(10)); // 10 hours of pure hello world glorious
    Status::SUCCESS // If the process reached this point means it had success.
}
