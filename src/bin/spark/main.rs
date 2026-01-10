use std::env; // Provides the functionality for the arguments 
use std::process::exit; // Provides the correct function to exit the script
mod auto_detect; // Module for auto_detect the path of the kernels already installed 
mod remove; // For removing the Spark installation in the system
mod install; // For installing the Spark program in the system
mod utils;
// This is the main code flow for the general spark binary
fn main() {
    let argument: Vec<String> = env::args().collect(); // Detect user input
    if argument.len() < 2 { // dry-run
        utils::show_help(); // Shows help so the user know what to execute
        return;
    }
    // Arguments
    let skip_conf = utils::skip_confirmation(&argument); 
    match argument[1].as_str() { // Converts the argument into string 
        "install" => install::install(skip_conf), // Install argument invokes install function
        "remove" => remove::remove_installation(skip_conf),// Remove argument invokes remove function
        "help" => utils::show_help(), // Shows the help about the program
        "clean" => print!("This function is still work in progress."),
        "update" => print!("This function is still work in progress."),
        _ => { // If the user says something not contemplated before:
            eprintln!("Unknown argument: {}", argument[1]); // Unknown argument with the argument
            utils::show_help(); // Show help argument
            exit(1); // Exit the process 
        }
    }
}
