#![no_std] // Not using the std library (Since not OS is loaded.)
#![no_main] 
mod input;
use core::time::Duration; // using the duration method from core crate
use uefi::prelude::*; // Using the prelude functions of the uefi crate
use uefi::println;

#[entry] // Entry point of the binary
fn main() -> Status { // Main function MUST return a status code at the end.
    uefi::helpers::init().unwrap(); // Initializing the UEFI services
    println!("Hello world!"); // Printing in the console
    input::read_keyboard_events();
    let mut counting = 0; // Integer that will change during the runtime
    while counting < 100{ // While counting is less than 100 do 
        println!("{}",counting); // Print the counter
        boot::stall(Duration::from_secs(1)); // Wait 1 second
        counting += 1; // Increase the counter
    }

    boot::stall(Duration::from_secs(10)); // 10 hours of pure hello world glorious
    Status::SUCCESS // If the process reached this point means it had success.
}
