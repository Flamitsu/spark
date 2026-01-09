// This is the main code flow for the general spark binary.
use std::env; // Provides the functionality for the arguments 
use std::process::exit; // Provides the correct function to exit the script
mod auto_detect; // Module for auto_detect the path of the kernels already installed 
mod remove; // For removing the Spark installation in the system
mod install; // For installing the Spark program in the system
mod utils;
fn main() {
    let argument: Vec<String> = env::args().collect(); // Detect user input
    if argument.len() < 2 {
        auto_detect::detect_new_kernel(); // If the user executes only "spark", it detect new kernels
        return;
    }
    // Arguments
    match argument[1].as_str() { // Converts the argument into string 
        "install" => install::install(), // Install argument invokes install function
        "remove"  => remove::remove_installation(), // Remove argument invokes remove function
        _ => { // If the user says something not contemplated before:
            eprintln!("Unknown argument: {}", argument[1]); // Unknown argument with the argument
            exit(1); // Exit the process 
        }
    }
}
