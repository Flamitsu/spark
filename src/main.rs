#![no_std] // No standard library imported (Since no OS is running for now)
#![no_main] // No main function execution, needed the #[entry] point

mod input; // Input module imported (Just handles user input)
mod configuration; // Configuration module imported (Parse the config file)
mod find_boot; // Find the boot entries
mod kernel; // Used to load the entry selected
use uefi::prelude::*; // prelude methods imported
use uefi::println; // Prinln macro of the uefi crate

#[entry] // Entry point of the program
fn main() -> Status {
    
    uefi::helpers::init().unwrap(); // Starting UEFI services
    find_boot::find_boot_entry(); // Find boots entries
    configuration::spark_config(); // Parse configuration from the config file 
    kernel::load_kernel();
    println!("---- Spark bootmanager ----"); // Print into the console

    if let Err(e) = input::read_keyboard_events() { // Invokes the function
        println!("Keyboard error: {:?}", e); // If the input handling had an error, shows which one
    }
    return Status::SUCCESS; // If the program reaches here, it returns success status and ends the program
}
